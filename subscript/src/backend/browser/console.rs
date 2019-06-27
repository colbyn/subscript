pub mod internal_reexports {
    pub use ::wasm_bindgen::JsValue;
    pub use ::web_sys;
}


#[macro_export]
macro_rules! console {
    ($($rest:tt)*) => {{
        use $crate::backend::browser::console::internal_reexports::*;

        let value: String = format!($($rest)*);
        let value: JsValue = JsValue::from_str(value.as_str());
        web_sys::console::log_1(&value);
    }};
}