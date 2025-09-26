use eowordle_lib::{enemies::list_enemies, prelude::Enemy};
use leptos::{attr::Value, ev, html::*, prelude::*, task, IntoView};

use crate::{api, data::Guess, GuessContext, UIManager};

#[derive(Clone)]
struct SearchManager {
    /// Tracks all enemies to render in the search suggestion.
    filtered: Vec<Enemy>,
    /// What enemy (index) is selected.
    selected: Option<usize>,
    /// A reference to the selected search DOM item.
    ref_selected: NodeRef<Button>
}

impl SearchManager {
    /// Gets the amount of filtered items.
    pub fn count(&self) -> usize {
        self.filtered.len()
    }

    /// Checks if an item is selected by its index.
    pub fn is_selected(&self, idx: usize) -> bool {
        if let Some(selected) = self.selected {
            selected == idx
        } else { false }
    }

    /// Selects the next item, circularly.
    pub fn next(&mut self) {
        if self.count() == 0 {
            return;
        }

        if let Some(selected) = self.selected {
            let next_idx = selected + 1;

            if next_idx > self.count() {
                self.select(Some(0));
            } else {
                self.select(Some(next_idx));
            }
        } else {
            self.select(Some(0));
        }
    }

    /// Selects the previous item, circularly.
    ///
    /// Does nothing without items.
    pub fn prev(&mut self) {
        if self.count() == 0 {
            return;
        }

        if let Some(selected) = self.selected {
            let prev_idx = selected.checked_sub(1).unwrap_or(self.count() - 1);
            self.select(Some(prev_idx));
        } else {
            self.select(Some(0));
        }
    }

    pub fn select(&mut self, idx: Option<usize>) {
        self.selected = idx;
    }

    /// Update the list of filtered enemies and reset the current selected one.
    pub fn set_filtered(&mut self, new: Vec<Enemy>) {
        self.filtered = new;
        self.selected = Some(0);
    }
}

impl Default for SearchManager {
    fn default() -> Self {
        Self { filtered: vec![], selected: None, ref_selected: NodeRef::new() }
    }
}

pub fn searcher() -> impl IntoView {
    let enemies = list_enemies();

    let query = RwSignal::new(String::new());
    let search_mgr = RwSignal::new(SearchManager::default());

    Effect::new(move |_| {
        let filtered: Vec<Enemy> = if query.get().len() == 0 {
            vec![]
        } else {
                enemies
                .iter()
                .filter(|e| e.name.to_lowercase().contains(&query.get()))
                .cloned()
                .collect()
        };

        search_mgr.update(|selection| selection.set_filtered(filtered));
    });

    div().class("searcher").child((
        input()
            .r#type("text")
            .id("searcher-input")
            .on(ev::keydown, move |kb_ev| {
                match kb_ev.key().as_str() {
                    "ArrowUp" => search_mgr.update(|s| s.prev()),
                    "ArrowDown" => search_mgr.update(|s| s.next()),
                    _ => {}
                }
            })
            .value(query)
            .bind(Value, query)
            .placeholder("Type here to search... (To be implemented soon!)"),
        div().id("searcher-picker").child(move || {
            search_mgr
                .get()
                .filtered
                .iter()
                .cloned()
                .enumerate()
                .map(|(i, enemy)| {
                    let mgr = search_mgr.write();
                    let is_selected = search_mgr.get().is_selected(i);
                    let item = picker_item(enemy, &is_selected);

                    item
                })
                .collect_view()
        }),
    ))
}



pub fn picker_item(enemy: Enemy, selected: bool) -> impl IntoView {
    let GuessContext(guess_reader, guess_writer) = use_context::<GuessContext>().unwrap();
    let ui = use_context::<RwSignal<UIManager>>().unwrap();
    let class = if selected { "picker-item selected-item" } else { "picker-item" };

    button()
        .class(class)
        .child((
                img().src(format!("assets/enemies/{}.gif", enemy.id)).aria_label(enemy.name.clone()),
                p().child(format!("{} life", enemy.life)),
                p().child(format!("{}", enemy.rarity))))
        .on(ev::click, move |_| {
            let enemy = enemy.clone();

            task::spawn_local(async move {
                let guesses = guess_reader.get_untracked();
                if guesses.has(&enemy) {
                    return;
                };

                let Some(diff) = api::get_daily_diff(enemy.id).await else {
                    return;
                };

                let guess = Guess {
                    enemy: enemy.clone(),
                    diff: diff.clone(),
                };
                guess_writer.update(|g| g.add(guess));

                if diff.is_same() {
                    guess_writer.update(|g| g.correct = Some(enemy.clone()));
                    ui.update(|ui| ui.show_victory = true);
                }
            });

        })
}
