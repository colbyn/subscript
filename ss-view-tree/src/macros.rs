#[macro_export]
macro_rules! html {
    () => {{}};
    ($x:tt) => {{

    }};
}

pub fn dev() {
    html!{};
    html!{{
        css::display(flex());
        h1{
            display: "flex";
            justify_content: "center";
            text("Hello World")
        }
        div {
        }
    }};
}





