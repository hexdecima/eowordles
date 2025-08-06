use std::ops::Deref;

use crate::components::{guesses, searcher, info, intro};
use components::victory;
use data::Guess;
use eowordle_lib::Enemy;
use leptos::{html::*, prelude::*, task};

mod components;
mod data;
mod api;

#[derive(Copy, Clone)]
pub struct GuessContext(ReadSignal<GuessManager>, WriteSignal<GuessManager>);

pub fn app() -> impl IntoView {
    let guess_signal = signal(
        GuessManager::default()
    );
    let ui_signal = RwSignal::new(UIManager::default());
    let yesterday = RwSignal::<Option<Enemy>>::new(None);

    provide_context(GuessContext(guess_signal.0, guess_signal.1));
    provide_context(ui_signal);
    provide_context(yesterday);

    task::spawn_local(async move {
        yesterday.set(api::get_yesterday().await);
    });

    let ui = use_context::<RwSignal<UIManager>>().unwrap();

    let children = move || if ui.get().show_victory {
        victory().into_any()
    } else {
        (intro(), searcher(), guesses(), info()).into_any()
    };

    body().child(children)
}

#[derive(Clone)]
pub struct GuessManager {
    inner: Vec<Guess>,
    pub correct: Option<Enemy>
}

impl GuessManager {
    /// Inserts a guess at the start.
    pub fn add(&mut self, guess: Guess) {
        self.inner.insert(0, guess);
    }
    /// Checks if an enemy has been guessed already.
    pub fn has(&self, enemy: &Enemy) -> bool {
        self.inner.iter().any(|guess| guess.enemy == *enemy)
    }
}

impl Default for GuessManager {
    fn default() -> Self {
        Self { inner: vec![], correct: None }
    }
}

impl Deref for GuessManager {
    type Target = Vec<Guess>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[derive(Clone)]
pub struct UIManager {
    pub show_victory: bool
}

impl Default for UIManager {
    fn default() -> Self {
        Self { show_victory: false }
    }
}
