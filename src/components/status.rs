use leptos::{html::*, prelude::*};

use crate::StatusData;

pub fn status() -> impl IntoView {
    let status_data = use_context::<RwSignal<StatusData>>().unwrap();

    div().id("status").child((
        img().src("assets/enemies/614.gif"),
        b().child(status_data.get().title.unwrap_or("No title.".into())),
        p().child(
            status_data
                .get()
                .description
                .unwrap_or("No description.".into()),
        ),
    ))
}
