use std::ops::Deref;

use crate::components::{guesses, searcher, info, intro};
use components::{status, victory};
use data::Guess;
use eowordle_lib::prelude::Enemy;
use leptos::{html::*, prelude::*, task};

mod components;
mod data;
mod api;

#[derive(Copy, Clone)]
pub struct GuessContext(ReadSignal<GuessManager>, WriteSignal<GuessManager>);

#[derive(Clone)]
pub struct StatusData {
    title: Option<String>,
    description: Option<String>
}

impl Default for StatusData {
    fn default() -> Self {
        Self {
            title: None,
            description: None
        }
    }
}

pub fn app() -> impl IntoView {
    let guess_signal = signal(
        GuessManager::default()
    );
    let ui_signal = RwSignal::new(UIManager::default());
    let yesterday = RwSignal::<Option<Enemy>>::new(None);
    let status_data = RwSignal::new(StatusData::default());

    provide_context(GuessContext(guess_signal.0, guess_signal.1));
    provide_context(ui_signal);
    provide_context(yesterday);
    provide_context(status_data);

    task::spawn_local(async move {
        if let Some(enemy) = api::get_yesterday().await {
            yesterday.set(Some(enemy));
        } else {
            status_data.update(|data| {
                data.title = Some("Service Unavailable".into());
                data.description = Some(
                    "Failed to connect to the game server, likely because it is offline. A few minutes of daily downtime is expected (or rarely a few hours), so you can try again in a bit."
                        .into()
                );
            });

            ui_signal.update(|ui| {
                ui.screen = Screen::Status;
            });
        }
    });

    let ui = use_context::<RwSignal<UIManager>>().unwrap();

    let children = move || match ui.get().screen {
        Screen::Game => {
        (intro(), searcher(), guesses(), info()).into_any()
        },
        Screen::Victory => {
            victory().into_any()
        },
        Screen::Status => {
            status().into_any()
        }
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

#[derive(Debug, Clone)]
pub enum Screen {
    Game,
    Victory,
    Status
}

#[derive(Clone)]
pub struct UIManager {
    pub screen: Screen,
}

impl Default for UIManager {
    fn default() -> Self {
        Self { screen: Screen::Game }
    }
}
