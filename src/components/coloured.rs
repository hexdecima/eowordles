use leptos::{html::*, prelude::*};

#[derive(Copy, Clone)]
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

impl AsRef<Colour> for Colour {
    fn as_ref(&self) -> &Colour {
        self
    }
}

pub fn coloured(text: impl AsRef<str>, colour: impl AsRef<Colour>) -> impl IntoView {
    p().child(text.as_ref().to_owned())
        .class(format!("coloured {}", colour.as_ref().css_class()))
}
