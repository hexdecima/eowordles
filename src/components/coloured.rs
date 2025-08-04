use leptos::{html::*, prelude::*};

pub enum Colour {
    Yellow,
    Green,
    Red,
}

impl Colour {
    pub fn css_class(&self) -> String {
        match self {
            Colour::Yellow => "coloured-yellow",
            Colour::Green => "coloured-green",
            Colour::Red => "coloured-red",
        }
        .to_owned()
    }
}

pub fn coloured(text: impl AsRef<str>, colour: Colour) -> impl IntoView {
    p().child(text.as_ref().to_owned())
        .class(format!("coloured {}", colour.css_class()))
}
