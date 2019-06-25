use core::fmt::Debug;
use std::collections::*;
use std::any::*;
use std::marker::*;
use std::cell::*;
use std::rc::*;
use either::{Either, Either::*};
use wasm_bindgen::JsValue;

use crate::backend::browser;
use crate::reactive_sys::*;
use crate::program_sys::spec::{Spec};
use crate::program_sys::instances::*;
use crate::view_sys::dom::Dom;
use crate::view_sys::dsl::View;


///////////////////////////////////////////////////////////////////////////////
// STYLING - DATA
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Clone, Default)]
pub struct Styling {
    pub(crate) default: StyleList,
    pub(crate) state: Vec<StateSelector>,
    pub(crate) animations: Vec<Animation>,
    pub(crate) media: Vec<MediaCondition>,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Clone, Default)]
pub struct StyleList(pub(crate) Vec<Style>);

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Clone)]
pub struct Style {
    pub(crate) property: String,
    pub(crate) value: String,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Clone)]
pub struct Animation(pub(crate) Vec<AnimationInterval>);

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Clone)]
pub struct AnimationInterval {
    pub(crate) value: String,
    pub(crate) style: StyleList,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Clone)]
pub struct MediaCondition {
    pub(crate) condition: StyleList,
    pub(crate) body: StyleList,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Clone)]
pub struct StateSelector {
    pub(crate) name: StateSelectorName,
    pub(crate) body: StyleList,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Clone)]
pub(crate) enum StateSelectorName {
    Active,
    After,
    Before,
    Checked,
    Disabled,
    Empty,
    Enabled,
    FirstChild,
    FirstLetter,
    FirstLine,
    Focus,
    Hover,
    LastChild,
    OnlyChild,
    Link,
    Visited,
    SpellingError,
    GrammarError,
    Selection,
    Placeholder,
    Marker,
    Cue,
    Backdrop,
}


///////////////////////////////////////////////////////////////////////////////
// STYLING - API
///////////////////////////////////////////////////////////////////////////////

impl Styling {
    pub fn is_empty(&self) -> bool {
        self.default.0.is_empty() &&
        self.state.is_empty() &&
        self.animations.is_empty() &&
        self.media.is_empty()
    }
    pub fn extend(&mut self, new: Styling) {
        self.default.0.extend(new.default.0);
        self.state.extend(new.state);
        self.animations.extend(new.animations);
        self.media.extend(new.media);
    }
    pub fn add_style(&mut self, x: Style) {
        self.default.0.push(x);
    }
    pub fn add_state(&mut self, x: StateSelector) {
        self.state.push(x);
    }
    pub fn add_animation(&mut self, xs: Vec<AnimationInterval>) {
        self.animations.push(Animation(xs));
    }
    pub fn add_media(&mut self, condition: StyleList, body: StyleList) {
        self.media.push(MediaCondition{condition, body});
    }
}

impl StyleList {
    pub fn new() -> Self {
        StyleList(Vec::new())
    }
    pub fn push(&mut self, value: Style) {
        self.0.push(value);
    }
}

impl Style {
    pub fn new(property: &str, value: &str) -> Self {
        let property = String::from(property);
        let value = String::from(value);
        Style{property, value}
    }
}

impl StateSelector {
    pub fn new_active(body: StyleList) -> Self {
        StateSelector{name: StateSelectorName::Active, body}
    }
    pub fn new_after(body: StyleList) -> Self {
        StateSelector{name: StateSelectorName::After, body}
    }
    pub fn new_before(body: StyleList) -> Self {
        StateSelector{name: StateSelectorName::Before, body}
    }
    pub fn new_checked(body: StyleList) -> Self {
        StateSelector{name: StateSelectorName::Checked, body}
    }
    pub fn new_disabled(body: StyleList) -> Self {
        StateSelector{name: StateSelectorName::Disabled, body}
    }
    pub fn new_empty(body: StyleList) -> Self {
        StateSelector{name: StateSelectorName::Empty, body}
    }
    pub fn new_enabled(body: StyleList) -> Self {
        StateSelector{name: StateSelectorName::Enabled, body}
    }
    pub fn new_first_child(body: StyleList) -> Self {
        StateSelector{name: StateSelectorName::FirstChild, body}
    }
    pub fn new_first_letter(body: StyleList) -> Self {
        StateSelector{name: StateSelectorName::FirstLetter, body}
    }
    pub fn new_first_line(body: StyleList) -> Self {
        StateSelector{name: StateSelectorName::FirstLine, body}
    }
    pub fn new_focus(body: StyleList) -> Self {
        StateSelector{name: StateSelectorName::Focus, body}
    }
    pub fn new_hover(body: StyleList) -> Self {
        StateSelector{name: StateSelectorName::Hover, body}
    }
    pub fn new_last_child(body: StyleList) -> Self {
        StateSelector{name: StateSelectorName::LastChild, body}
    }
    pub fn new_only_child(body: StyleList) -> Self {
        StateSelector{name: StateSelectorName::OnlyChild, body}
    }
    pub fn new_link(body: StyleList) -> Self {
        StateSelector{name: StateSelectorName::Link, body}
    }
    pub fn new_visited(body: StyleList) -> Self {
        StateSelector{name: StateSelectorName::Visited, body}
    }
    pub fn new_spelling_error(body: StyleList) -> Self {
        StateSelector{name: StateSelectorName::SpellingError, body}
    }
    pub fn new_grammar_error(body: StyleList) -> Self {
        StateSelector{name: StateSelectorName::GrammarError, body}
    }
    pub fn new_selection(body: StyleList) -> Self {
        StateSelector{name: StateSelectorName::Selection, body}
    }
    pub fn new_placeholder(body: StyleList) -> Self {
        StateSelector{name: StateSelectorName::Placeholder, body}
    }
    pub fn new_marker(body: StyleList) -> Self {
        StateSelector{name: StateSelectorName::Marker, body}
    }
    pub fn new_cue(body: StyleList) -> Self {
        StateSelector{name: StateSelectorName::Cue, body}
    }
    pub fn new_backdrop(body: StyleList) -> Self {
        StateSelector{name: StateSelectorName::Backdrop, body}
    }
}

impl AnimationInterval {
    pub fn new(value: &str, style: StyleList) -> Self {
        AnimationInterval{value: String::from(value), style}
    }
}

impl StateSelectorName {
    pub fn as_str(&self) -> &str {
        match self {
            StateSelectorName::Active => ":active",
            StateSelectorName::After => "::after",
            StateSelectorName::Before => "::before",
            StateSelectorName::Checked => ":checked",
            StateSelectorName::Disabled => ":disabled",
            StateSelectorName::Empty => ":empty",
            StateSelectorName::Enabled => ":enabled",
            StateSelectorName::FirstChild => ":first-child",
            StateSelectorName::FirstLetter => "::first-letter",
            StateSelectorName::FirstLine => "::first-line",
            StateSelectorName::Focus => ":focus",
            StateSelectorName::Hover => ":hover",
            StateSelectorName::LastChild => ":last-child",
            StateSelectorName::OnlyChild => ":only-child",
            StateSelectorName::Link => ":link",
            StateSelectorName::Visited => ":visited",
            StateSelectorName::SpellingError => "::spelling-error",
            StateSelectorName::GrammarError => "::grammar-error",
            StateSelectorName::Selection => "::selection",
            StateSelectorName::Placeholder => "::placeholder",
            StateSelectorName::Marker => "::marker",
            StateSelectorName::Cue => "::cue",
            StateSelectorName::Backdrop => "::backdrop",
        }
    }
}


///////////////////////////////////////////////////////////////////////////////
// EVENTS
///////////////////////////////////////////////////////////////////////////////

impl<Msg: 'static> EventHandler<Msg> {
    pub fn new_on_click(cb: impl Fn() -> Msg + 'static) -> Self {
        EventHandler(Rc::new(OnClick(Box::new(cb))))
    }
    pub fn new_on_mouse_down(cb: impl Fn() -> Msg + 'static) -> Self {
        EventHandler(Rc::new(OnMouseDown(Box::new(cb))))
    }
    pub fn new_on_mouse_up<M>(cb: impl Fn() -> Msg + 'static) -> Self {
        EventHandler(Rc::new(OnMouseUp(Box::new(cb))))
    }
    pub fn new_on_mouse_enter(cb: impl Fn() -> Msg + 'static) -> Self {
        EventHandler(Rc::new(OnMouseEnter(Box::new(cb))))
    }
    pub fn new_on_mouse_leave(cb: impl Fn() -> Msg + 'static) -> Self {
        EventHandler(Rc::new(OnMouseLeave(Box::new(cb))))
    }
    pub fn new_on_mouse_over(cb: impl Fn() -> Msg + 'static) -> Self {
        EventHandler(Rc::new(OnMouseOver(Box::new(cb))))
    }
    pub fn new_on_mouse_out(cb: impl Fn() -> Msg + 'static) -> Self {
        EventHandler(Rc::new(OnMouseOut(Box::new(cb))))
    }
    pub fn new_on_input(cb: impl Fn(String) -> Msg + 'static) -> Self {
        EventHandler(Rc::new(OnInput(Box::new(cb))))
    }
    pub fn new_on_check(cb: impl Fn(bool) -> Msg + 'static) -> Self {
        EventHandler(Rc::new(OnCheck(Box::new(cb))))
    }
    pub fn new_on_submit(cb: impl Fn() -> Msg + 'static) -> Self {
        EventHandler(Rc::new(OnSubmit(Box::new(cb))))
    }
    pub fn new_on_blur(cb: impl Fn() -> Msg + 'static) -> Self {
        EventHandler(Rc::new(OnBlur(Box::new(cb))))
    }
    pub fn new_on_focus(cb: impl Fn() -> Msg + 'static) -> Self {
        EventHandler(Rc::new(OnFocus(Box::new(cb))))
    }
}

pub(crate) struct EventHandler<Msg>(pub Rc<EventHandlerImpl<Msg>>);
pub(crate) trait EventHandlerImpl<Msg> {
    fn apply(&self, event: JsValue) -> Msg;
    fn event_type(&self) -> String;
}
impl<Msg>  EventHandler<Msg> {
    pub(crate) fn apply(&self, event: JsValue) -> Msg {self.0.apply(event)}
    pub(crate) fn event_type(&self) -> String {self.0.event_type()}
}
impl<Msg> Clone for EventHandler<Msg> {
    fn clone(&self) -> Self {
        EventHandler(self.0.clone())
    }
}

#[derive(PartialEq)]
pub(crate) enum EventType {
    OnClick,
    OnMouseDown,
    OnMouseUp,
    OnMouseEnter,
    OnMouseLeave,
    OnMouseOver,
    OnMouseOut,
    OnInput,
    OnCheck,
    OnSubmit,
    OnBlur,
    OnFocus,
}

impl EventType {
    /// Gets the event name as a string.
    pub fn as_str(&self) -> &str {
        match self {
            EventType::OnClick => "click",
            EventType::OnMouseDown => "mousedown",
            EventType::OnMouseUp => "mouseup",
            EventType::OnMouseEnter => "mouseenter",
            EventType::OnMouseLeave => "mouseleave",
            EventType::OnMouseOver => "mouseover",
            EventType::OnMouseOut => "mouseout",
            EventType::OnInput => "input",
            EventType::OnCheck => "click",
            EventType::OnSubmit => "submit",
            EventType::OnBlur => "blur",
            EventType::OnFocus => "focus",
        }
    }
}

// MOUSE
pub struct OnClick<Msg>(Box<Fn() -> Msg>); // click
pub struct OnMouseDown<Msg>(Box<Fn()->Msg>); // mousedown
pub struct OnMouseUp<Msg>(Box<Fn()->Msg>); // mouseup
pub struct OnMouseEnter<Msg>(Box<Fn()->Msg>); // mouseenter
pub struct OnMouseLeave<Msg>(Box<Fn()->Msg>); // mouseleave
pub struct OnMouseOver<Msg>(Box<Fn()->Msg>); // mouseover
pub struct OnMouseOut<Msg>(Box<Fn()->Msg>); // mouseout


// FORMS
pub struct OnInput<Msg>(Box<Fn(String)->Msg>); // input
pub struct OnCheck<Msg>(Box<Fn(bool)->Msg>); // click
pub struct OnSubmit<Msg>(Box<Fn()->Msg>); // submit


// FOCUS
pub struct OnBlur<Msg>(Box<Fn()->Msg>); // blur
pub struct OnFocus<Msg>(Box<Fn()->Msg>); // focus

// MOUSE
impl<Msg> EventHandlerImpl<Msg> for OnClick<Msg> {
    fn apply(&self, event: JsValue) -> Msg {
        self.0()
    }
    fn event_type(&self) -> String {
        String::from(EventType::OnClick.as_str())
    }
}
impl<Msg> EventHandlerImpl<Msg> for OnMouseDown<Msg> {
    fn apply(&self, event: JsValue) -> Msg {
        self.0()
    }
    fn event_type(&self) -> String {
        String::from(EventType::OnMouseDown.as_str())
    }
}
impl<Msg> EventHandlerImpl<Msg> for OnMouseUp<Msg> {
    fn apply(&self, event: JsValue) -> Msg {
        self.0()
    }
    fn event_type(&self) -> String {
        String::from(EventType::OnMouseUp.as_str())
    }
}
impl<Msg> EventHandlerImpl<Msg> for OnMouseEnter<Msg> {
    fn apply(&self, event: JsValue) -> Msg {
        self.0()
    }
    fn event_type(&self) -> String {
        String::from(EventType::OnMouseEnter.as_str())
    }
}
impl<Msg> EventHandlerImpl<Msg> for OnMouseLeave<Msg> {
    fn apply(&self, event: JsValue) -> Msg {
        self.0()
    }
    fn event_type(&self) -> String {
        String::from(EventType::OnMouseLeave.as_str())
    }
}
impl<Msg> EventHandlerImpl<Msg> for OnMouseOver<Msg> {
    fn apply(&self, event: JsValue) -> Msg {
        self.0()
    }
    fn event_type(&self) -> String {
        String::from(EventType::OnMouseOver.as_str())
    }
}
impl<Msg> EventHandlerImpl<Msg> for OnMouseOut<Msg> {
    fn apply(&self, event: JsValue) -> Msg {
        self.0()
    }
    fn event_type(&self) -> String {
        String::from(EventType::OnMouseOut.as_str())
    }
}


// FORMS
impl<Msg> EventHandlerImpl<Msg> for OnInput<Msg> {
    fn apply(&self, event: JsValue) -> Msg {
        let value: String = browser::utils::get_oninput_value(&event);
        self.0(value)
    }
    fn event_type(&self) -> String {
        String::from(EventType::OnInput.as_str())
    }
}
impl<Msg> EventHandlerImpl<Msg> for OnCheck<Msg> {
    fn apply(&self, event: JsValue) -> Msg {
        let event: web_sys::Event = From::from(event.clone());
        let target: web_sys::EventTarget = event
            .target()
            .expect("target failed");
        let target: JsValue = From::from(target);
        let target: web_sys::HtmlInputElement = From::from(target);
        let value: bool = target.checked();
        self.0(value)
    }
    fn event_type(&self) -> String {
        String::from(EventType::OnCheck.as_str())
    }
}
impl<Msg> EventHandlerImpl<Msg> for OnSubmit<Msg> {
    fn apply(&self, event: JsValue) -> Msg {
        self.0()
    }
    fn event_type(&self) -> String {
        String::from(EventType::OnSubmit.as_str())
    }
}

// FOCUS
impl<Msg> EventHandlerImpl<Msg> for OnBlur<Msg> {
    fn apply(&self, event: JsValue) -> Msg {
        self.0()
    }
    fn event_type(&self) -> String {
        String::from(EventType::OnBlur.as_str())
    }
}
impl<Msg> EventHandlerImpl<Msg> for OnFocus<Msg> {
    fn apply(&self, event: JsValue) -> Msg {
        self.0()
    }
    fn event_type(&self) -> String {
        String::from(EventType::OnFocus.as_str())
    }
}

impl<Msg> std::fmt::Debug for EventHandler<Msg> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "EventHandler")
    }
}

///////////////////////////////////////////////////////////////////////////////
// ATTRIBUTES
///////////////////////////////////////////////////////////////////////////////

pub fn normalize_attribute_value(x: impl AttributeValue) -> Either<Value<String>, Value<bool>> {
    x.normalize()
}

#[derive(Debug)]
pub struct Attribute {
    pub key: String,
    pub value: Either<Value<String>, Value<bool>>,
}

pub trait AttributeValue {
    fn normalize(&self) -> Either<Value<String>, Value<bool>>;
}
impl AttributeValue for &str {
    fn normalize(&self) -> Either<Value<String>, Value<bool>> {
        Left(Value::Static(Rc::new(self.to_owned().to_string())))
    }
}
impl AttributeValue for str {
    fn normalize(&self) -> Either<Value<String>, Value<bool>> {
        Left(Value::Static(Rc::new(String::from(self))))
    }
}
impl AttributeValue for String {
    fn normalize(&self) -> Either<Value<String>, Value<bool>> {
        Left(Value::Static(Rc::new(self.clone())))
    }
}
impl AttributeValue for bool {
    fn normalize(&self) -> Either<Value<String>, Value<bool>> {
        Right(Value::Static(Rc::new(self.clone())))
    }
}
impl AttributeValue for &Signal<String> {
    fn normalize(&self) -> Either<Value<String>, Value<bool>> {
        let signal_output = self.signal_output();
        Left(Value::Dynamic(DynamicValue {
            current: Rc::new(RefCell::new(signal_output.get())),
            observer: signal_output,
        }))
    }
}
impl AttributeValue for &Signal<bool> {
    fn normalize(&self) -> Either<Value<String>, Value<bool>> {
        let signal_output = self.signal_output();
        Right(Value::Dynamic(DynamicValue {
            current: Rc::new(RefCell::new(signal_output.get())),
            observer: signal_output,
        }))
    }
}


// MAPPED-EVENT-HANDLER
impl<Msg: 'static> EventHandler<Msg> {
    pub(crate) fn map_msg_impl<T: 'static>(self, f: Rc<Fn(Msg)->T>) -> EventHandler<T> {
        EventHandler(Rc::new(MappedEventHandler {inner: self,f}))
    }
}
pub(crate) struct MappedEventHandler<T, U> {
    f: Rc<Fn(T)->U>,
    inner: EventHandler<T>,
}
impl<T, U> EventHandlerImpl<U> for MappedEventHandler<T, U> {
    fn apply(&self, event: JsValue) -> U {
        (self.f)(self.inner.apply(event))
    }
    fn event_type(&self) -> String {
        self.inner.event_type()
    }
}


///////////////////////////////////////////////////////////////////////////////
// MISCELLANEOUS
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub enum Value<T> {
    Static(Rc<T>),
    Dynamic(DynamicValue<T>),
}

#[derive(Debug)]
pub struct DynamicValue<T> {
    pub(crate) observer: SignalOutput<T>,
    pub(crate) current: Rc<RefCell<Rc<T>>>,
}

pub struct DynamicProducer<Msg>(pub(crate) Rc<Fn()->Option<View<Msg>>>);

impl<Msg> Clone for DynamicProducer<Msg> {
    fn clone(&self) -> Self {
        DynamicProducer(self.0.clone())
    }
}

impl<Msg> std::fmt::Debug for DynamicProducer<Msg> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "DynamicProducer")
    }
}

pub(crate) struct IfChangedWithOld<'a, T> {
    pub new: &'a T,
    pub old: &'a T,
}

impl<T> Value<T> where T: PartialEq + 'static {
    pub(crate) fn if_changed_with_old(&self, f: impl Fn(IfChangedWithOld<T>)) {
        match &self {
            Value::Dynamic(dynamic) => {
                let upstream = dynamic.observer.get();
                let unchanged = *dynamic.current.borrow() == upstream;
                if !unchanged {
                    f(IfChangedWithOld {
                        new: &upstream,
                        old: &dynamic.current.borrow(),
                    });
                    dynamic.current.replace(upstream);
                }
            }
            Value::Static(_) => {}
        }
    }
    pub(crate) fn if_changed(&self, f: impl Fn(&T)) {
        match &self {
            Value::Dynamic(dynamic) => {
                let upstream = dynamic.observer.get();
                let unchanged = *dynamic.current.borrow() == upstream;
                if !unchanged {
                    f(&upstream);
                    dynamic.current.replace(upstream);
                }
            }
            Value::Static(_) => {}
        }
    }
    pub(crate) fn get(&self) -> Rc<T> {
        match &self {
            Value::Dynamic(dynamic) => {
                let upstream = dynamic.observer.get();
                let unchanged = *dynamic.current.borrow() == upstream;
                if !unchanged {
                    dynamic.current.replace(upstream.clone());
                    upstream
                } else {
                    upstream
                }
            }
            Value::Static(value) => {value.clone()}
        }
    }
    pub(crate) fn is_dynamic(&self) -> bool {
        match &self {
            Value::Static(_) => {true}
            Value::Dynamic(_) => {false}
        }
    }
    pub(crate) fn is_static(&self) -> bool {
        match &self {
            Value::Static(_) => {true}
            Value::Dynamic(_) => {false}
        }
    }
}


impl<T> Clone for Value<T> {
    fn clone(&self) -> Self {
        match self {
            Value::Static(x) => Value::Static(x.clone()),
            Value::Dynamic(x) => Value::Dynamic(x.clone()),
        }
    }
}
impl<T> Clone for DynamicValue<T> {
    fn clone(&self) -> Self {
        DynamicValue {
            observer: self.observer.clone(),
            current: self.current.clone(),
        }
    }
}




