use serde::{Serialize, Deserialize};
use ss_view_tree::View;
use ss_program::Subscriptions;
use ss_program::StartupInfo;
use ss_program::Init;
use ss_program::SubSystems;
use ss_program::Component;
use ss_program::Program;
use ss_program::Spec;



#[derive(Debug, PartialEq, Clone)]
pub enum Msg {
    NoOp,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct Model {}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct AppSpec {

}

impl Spec for AppSpec {
    type Model = Model;
    type Msg = Msg;

    fn init(&self, startup: StartupInfo<Self>) -> Init<Self> {
    	Init {
    		model: Model::default(),
    		subs: Subscriptions::default(),
    	}
    }
    fn update(&self, model: &mut Self::Model, msg: Self::Msg, sys: &SubSystems) {

    }
    fn view(&self, model: &Self::Model) -> View<Self::Msg> {
    	view!{
    		h1{"Hello World!!!!";}
    	}
    }
}


pub fn main() {
	let program = Program::from_component(Component {
		name: "app",
		spec: AppSpec::default(),
	});
	program.start();
}