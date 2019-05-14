#![allow(dead_code, unused, unused_variables)]

pub mod core;
pub mod values;
pub mod properties;
pub mod selectors;


pub fn main() {
    use crate::core::*;
    use crate::values::*;
    use crate::properties::*;
    use crate::selectors::media::*;
    use crate::selectors::state::*;
    
    let mut stylesheet = Stylesheet::new("id-text");
    stylesheet.add_locals(vec![
        display(flex()),
    ]);
    stylesheet.add_media(
        media(
            vec![min_width(px(0))],
            vec![display(flex())]
        )
    );
    println!("{}", render_stylesheets(vec![stylesheet]));
}

