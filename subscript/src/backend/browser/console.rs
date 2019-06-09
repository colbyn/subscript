#[macro_export]
macro_rules! console {
    ($($rest:tt)*) => {{
        use wasm_bindgen::JsValue;
        let value: String = format!($($rest)*);
        let value: JsValue = JsValue::from_str(value.as_str());
        web_sys::console::log_1(&value);
    }};
}