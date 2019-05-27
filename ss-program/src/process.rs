use std::convert::AsRef;
use std::borrow::Cow;
use std::any::*;
use std::marker::*;
use std::fmt::{self, Debug};
use std::cell::*;
use std::rc::*;
use std::collections::*;
use std::hash::{Hash, Hasher};
use either::{Either, Left, Right};
use wasm_bindgen::JsValue;
use ss_web_utils::dom;
use ss_web_utils::prelude::*;
use ss_web_utils::js::{self, console};
use ss_trees::tree::*;
use ss_view_tree::*;
use ss_view_tree::attributes::*;
use ss_view_tree::events::*;
use ss_dom_tree::*;
use crate::spec::*;
use owning_ref::*;


///////////////////////////////////////////////////////////////////////////////
// GLOBAL-EVENTS REGISTRY
///////////////////////////////////////////////////////////////////////////////


thread_local! {
    pub static GLOBAL_EVENTS_REGISTRY: GlobalEventsRegistry = {
    	GlobalEventsRegistry {
    		events: RefCell::new(VecDeque::new()),
    	}
    };
}

pub struct GlobalEventsRegistry {
    pub(crate) events: RefCell<VecDeque<Rc<Any>>>,
}



///////////////////////////////////////////////////////////////////////////////
// PROCESS_REGISTRY
///////////////////////////////////////////////////////////////////////////////

thread_local! {
    pub static PROCESS_REGISTRY: RefCell<ProcessRegistry> = {
    	RefCell::new(ProcessRegistry {
    		processes: HashMap::new(),
    	})
    };
}

pub struct ProcessRegistry {
    processes: HashMap<ProcessId, Box<ProcessObject>>
}

impl ProcessRegistry {
	pub(crate) fn upsert_and_tick<S: 'static +  Spec>(&mut self, name: &str, spec_value: &S, reg: &GlobalTickRegistry) {
		let key = ProcessId::of::<S>(name, spec_value);
		let mut set_new_process = None;
		if let Some(process) = self.processes.get(&key) {
			let process: &Process<S> = cast_to_process::<S>(process.as_ref());
			process.tick(Some(&spec_value), reg);
		} else {
			let startup_info: StartupInfo<S> = StartupInfo {
				name: String::from(name),
				saved_model: load_saved_model::<S>(&key),
			};
			let Init{model, subs} = spec_value.init(startup_info);
			let model_hash = RefCell::new(calculate_model_hash::<S>(&model));
			let view = spec_value.view(&model);
			let view = RefCell::new(LiveView::start(view));
			let model = RefCell::new(model);
			let new_process = Process {
				view,
				model: unimplemented!(),
				model_hash,
				subs,
				pid: key.clone(),
				spec: RefCell::new(spec_value.clone()),
				sys: SubSystems::default(),
			};
			set_new_process = Some((key, new_process));
		}
		if let Some((key, new_process)) = set_new_process {
			self.processes.insert(key, Box::new(new_process));
		}
	}
	pub(crate) fn unchanged<S: 'static +  Spec>(&self, name: &str, spec_value: &S) -> bool {
		let key = ProcessId::of::<S>(name, spec_value);
		match self.processes.get(&key) {
			None => {false}
			Some(value) => {
				let process: &Process<S> = cast_to_process(value.as_ref());
				let is_unchanged = {
					let x: &S = &process.spec.borrow();
					x == spec_value
				};
				is_unchanged
			}
		}
	}
	pub(crate) fn recyclable<S: 'static +  Spec>(&self, name: &str, spec_value: &S) -> bool {
		let key = ProcessId::of::<S>(name, spec_value);
		match self.processes.get(&key) {
			None => {false}
			Some(_) => {true}
		}
	}
	pub(crate) fn remove<S: 'static +  Spec>(&mut self, name: &str, spec_value: &S) {
		let key = ProcessId::of::<S>(name, spec_value);
		self.processes.remove(&key);
	}
	pub(crate) fn prune_and_tick_global_events(&mut self, reg: &GlobalTickRegistry) {
		let mut orphaned_pids = Vec::new();
		let used_pids = cast_ids(&reg.components.borrow());
		for key in self.processes.keys() {
			let is_used = used_pids.contains(&key);
			if !is_used {
				orphaned_pids.push(key.clone());
			}
		}
		for key in orphaned_pids {
			self.processes.remove(&key);
		}
		let ref global_events = GLOBAL_EVENTS_REGISTRY.with(move |reg| {
		    reg.events.borrow_mut().drain(..).collect::<Vec<Rc<Any>>>()
		});
		if !global_events.is_empty() {
			for key in used_pids {
				let process = self.processes
					.get_mut(&key)
					.expect("ProcessRegistry.prune_and_tick_global_events should not fail here!")
					.tick_global_events(global_events);
			}
		}
	}
}


pub(crate) trait ProcessObject {
	fn tick_global_events(&self, global_events: &Vec<Rc<Any>>);
	fn to_any(&self) -> &Any;
}

pub(crate) fn cast_to_process<S: 'static +  Spec>(value: &ProcessObject) -> &Process<S> {
	let value: &Any = value.to_any();
	let value: Option<&Process<S>> = value.downcast_ref::<Process<S>>();
	match value {
		None => panic!(),
		Some(value) => value,
	}
}


///////////////////////////////////////////////////////////////////////////////
// PROCESS - LIVE-TREE-HANDLE
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct Component<S: Spec> {
	pub name: &'static str,
	pub spec: S,
}


impl<S: 'static +  Spec> ViewComponent for Component<S> {
	fn box_clone(&self) -> Box<ViewComponent> {
		Box::new(Component {
			name: self.name,
			spec: self.spec.clone(),
		})
	}
	fn tick(&self, reg: &GlobalTickRegistry) -> Box<Any> {
		PROCESS_REGISTRY.with(|proc_reg| {
			proc_reg.borrow_mut().upsert_and_tick::<S>(&self.name, &self.spec, reg);
		});
		let pid: ProcessId = ProcessId::of::<S>(&self.name, &self.spec);
		Box::new(pid)
	}
	fn unchanged(&self, other: &Box<Any>) -> bool {
		let other: &Component<S> = other.downcast_ref()
			.expect("Component.unchanged should not fail here!");
		assert!(self.name == other.name);
		PROCESS_REGISTRY.with(|reg| {
			reg.borrow().unchanged::<S>(&self.name, &other.spec)
		})
	}
	fn recyclable(&self, other: &Box<Any>) -> bool {
		let other: &Component<S> = other.downcast_ref()
			.expect("Component.unchanged should not fail here!");
		assert!(self.name == other.name);
		PROCESS_REGISTRY.with(|reg| {
			reg.borrow().recyclable::<S>(&self.name, &other.spec)
		})
	}
}


///////////////////////////////////////////////////////////////////////////////
// PROCESS-ID
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ProcessId {
	spec_id: TypeId,
	name: String,
}

impl ProcessId {
	pub fn of<S: 'static +  Spec>(name: &str, spec: &S) -> Self {
		ProcessId {
			spec_id: TypeId::of::<S>(),
			name: String::from(name),
		}
	}
}

pub fn cast_ids(xs: &Vec<Box<InternalComponentId>>) -> HashSet<ProcessId> {
	let mut pids = HashSet::with_capacity(xs.len());
	for cid in xs {
		let cid: &InternalComponentId = cid.as_ref();
		let cid: Option<&ProcessId> = cid.downcast_ref::<ProcessId>();
		match cid {
			None => {panic!()}
			Some(pid) => {pids.insert(pid.clone());}
		}
	}
	pids
}

///////////////////////////////////////////////////////////////////////////////
// PROCESS - CORE
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq)]
pub struct Process<S: Spec> {
	pid: ProcessId,
	spec: RefCell<S>,
	view: RefCell<LiveView<S::Msg>>,
	model: RefCell<S::Model>,
	model_hash: RefCell<u64>,
	subs: Subscriptions<S::Msg>,
	sys: SubSystems<S>,
}

impl<S: Spec> Process<S> where S::Msg: 'static {
	fn force_update_view(&self) {
		let x: &S::Model = &self.model.borrow();
		let new_view = self.spec.borrow().view(unimplemented!());
		self.view.borrow_mut().sync(new_view);
	}
	fn run_sys_requests(&self) {
		for req in self.sys.requests.borrow_mut().drain(..) {
			match req {
				SystemRequest::Save => {
					save_model::<S>(&self.pid, &self.model.borrow());
				}
				SystemRequest::Broadcast(x) => {
					broadcast::<S>(x);
				}
				SystemRequest::Navigate(x) => {
					navigate(x);
				}
			}
		}
	}
	fn tick(&self, new: Option<&S>, reg: &GlobalTickRegistry) {
		// SPECIAL
		let mut force_update_view = false;
		if let Some(new) = new {
			let spec_value_unchanged = &*self.spec.borrow() == new;
			if !spec_value_unchanged {
				self.spec.replace(new.clone());
				force_update_view = true;
			}
		}
		// SETUP
		let messages = {
			// SETUP
            let mut env = TickEnv::default();
            // FIRST - HTML DOM EVENTS
            self.view.borrow().tick(&mut env, reg);
            // SECOND - SUBSCRIPTIONS
            // self.subs.tick(&mut env.messages, reg);
            // DONE
            env.messages
		};
		if !messages.is_empty() {
			// PROCESS EVENTS
			for msg in messages {
			    self.spec.borrow().update(&mut self.model.borrow_mut(), msg, &self.sys);
			}
			let new_hash = calculate_model_hash::<S>(&self.model.borrow());
			// // PROCESS VIEW
			let unchanged = *self.model_hash.borrow() == new_hash;
			if force_update_view || !unchanged {
				// SET NEW HASH
				self.model_hash.replace(new_hash);
				// UPDATE VIEW
				self.force_update_view();
			}
			// PROCESS COMMANDS
			self.run_sys_requests();
		} else if force_update_view {
			self.force_update_view();
		}
	}
}

impl<S: 'static +  Spec> ProcessObject for Process<S> {
	fn to_any(&self) -> &(Any + 'static) {self}
	fn tick_global_events(&self, global_events: &Vec<Rc<Any>>) {
		let mut messages = Vec::new();
		self.subs.tick(&mut messages, global_events);
		if !messages.is_empty() {
			// PROCESS EVENTS
			for msg in messages {
			    self.spec.borrow().update(&mut self.model.borrow_mut(), msg, &self.sys);
			}
			let new_hash = calculate_model_hash::<S>(&self.model.borrow());
			// // PROCESS VIEW
			let unchanged = *self.model_hash.borrow() == new_hash;
			if !unchanged {
				// SET NEW HASH
				self.model_hash.replace(new_hash);
				// UPDATE VIEW
				self.force_update_view();
			}
			// PROCESS COMMANDS
			self.run_sys_requests();
		}
	}
}

///////////////////////////////////////////////////////////////////////////////
// SYSTEM-REQUEST HELPERS
///////////////////////////////////////////////////////////////////////////////

pub fn save_model<S: Spec>(pid: &ProcessId, model: &S::Model) {
    let spec_type_id = format!("{:?}-{}", &pid.spec_id, &pid.name);
    dom::window().local_storage.set::<S::Model>(spec_type_id.as_ref(), model);
}

pub fn load_saved_model<S: Spec>(pid: &ProcessId) -> Option<S::Model> {
    let spec_type_id = format!("{:?}-{}", &pid.spec_id, &pid.name);
    dom::window().local_storage.get::<S::Model>(spec_type_id.as_ref())
}

fn broadcast<S: Spec>(msg: Rc<Any>) {
	GLOBAL_EVENTS_REGISTRY.with(|reg| {
		reg.events.borrow_mut().push_back(msg);
	});
}

fn navigate(route: String) {
	dom::window()
	    .history
	    .push_state(route.as_str());
}


///////////////////////////////////////////////////////////////////////////////
// MISCELLANEOUS
///////////////////////////////////////////////////////////////////////////////


fn calculate_model_hash<S: Spec>(model: &S::Model) -> u64 {
	use std::collections::hash_map::DefaultHasher;
    let mut s = DefaultHasher::new();
    model.hash(&mut s);
    s.finish()
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
	use std::collections::hash_map::DefaultHasher;
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}


