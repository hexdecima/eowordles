use leptos::{html::*, prelude::*};

use crate::GuessContext;
use eowordle_lib::*;

use super::coloured::{coloured, Colour};

const ARROW_DOWN: char = '↓';
const ARROW_UP: char = '↑';

fn to_colour(o: &OrderingText) -> Colour {
    match o {
        OrderingText::Equal => Colour::Green,
        _ => Colour::Red,
    }
}

fn make_arrow(o: &OrderingText) -> String {
    match o {
        OrderingText::Less => format!(" {ARROW_UP}"),
        OrderingText::Equal => String::new(),
        OrderingText::Greater => format!(" {ARROW_DOWN}"),
    }
}

pub fn guesses() -> impl IntoView {
    let GuessContext(guess_reader, _) = use_context::<GuessContext>().unwrap();

    table().id("guesses").child((
        tr().id("guesses-header").child((
            th().class("guess-name guess-cell").child("Name"),
            th().class("guess-life guess-cell").child("Life"),
            th().class("guess-defence guess-cell").child("Defence"),
            th().class("guess-coins guess-cell").child("Coins"),
            th().class("guess-environments guess-cell")
                .child("Environments"),
            th().class("guess-layers guess-cell").child("Layers"),
            th().class("guess-rarity guess-cell").child("Rarity"),
        )),
        (if guess_reader.get().is_empty() {
            p().id("guess-none").child("Guesses will appear here.").into_any()
        } else {
            (move || {
                guess_reader
                    .get()
                    .iter()
                    .map(|g| guess(&g.enemy, &g.diff))
                    .collect_view()
            })()
            .into_any()
        }),
    ))
}

pub fn guess(enemy: &Enemy, diff: &EnemyDiff) -> impl IntoView {
    tr().class("guess").child((
        guess_name(enemy.name.to_owned(), diff.name),
        guess_life(enemy.life, &diff.life),
        guess_defence(enemy.defence, &diff.defence),
        guess_coins(&enemy.coins, &diff.coins),
        guess_environments(&enemy.environments, &diff.environments),
        guess_layers(&enemy.layers, &diff.layers),
        guess_rarity(&enemy.rarity, &diff.rarity),
    ))
}

fn guess_name(name: impl AsRef<str>, correct: bool) -> impl IntoView {
    let colour = if correct { Colour::Green } else { Colour::Red };

    td().class("guess-name guess-cell")
        .child(p().child(coloured(name.as_ref().to_owned(), colour)))
}

fn guess_life(life: u16, diff: &OrderingText) -> impl IntoView {
    let text = format!("{life}{}", make_arrow(diff));

    td().class("guess-life guess-cell")
        .child(coloured(text, to_colour(diff)))
}

fn guess_defence(def: u16, diff: &OrderingText) -> impl IntoView {
    let text = format!("{def}{}", make_arrow(diff));

    td().class("guess-defence guess-cell")
        .child(coloured(text, to_colour(diff)))
}

fn guess_coins(coins: &Coins, diff: &OrderingText) -> impl IntoView {
    let text = format!("{coins}{}", make_arrow(diff));

    td().class("guess-coins guess-cell")
        .child(coloured(text, to_colour(diff)))
}

fn guess_environments(envs: &[Environment], diff: &EnvironmentDiff) -> impl IntoView {
    let mut items = Vec::with_capacity(envs.len());
    diff.right
        .iter()
        .map(|env| coloured(env.to_string(), Colour::Green))
        .for_each(|e| items.push(e));
    diff.wrong
        .iter()
        .map(|env| coloured(env.to_string(), Colour::Red))
        .for_each(|e| items.push(e));

    td().class("guess-environments guess-cell")
        .child(div().class("guess-group").child(items))
}

fn guess_layers(layers: &[Layer], diff: &LayerDiff) -> impl IntoView {
    let mut items = Vec::with_capacity(layers.len());
    diff.right
        .iter()
        .map(|lay| coloured(lay.to_string(), Colour::Green))
        .for_each(|e| items.push(e));
    diff.wrong
        .iter()
        .map(|lay| coloured(lay.to_string(), Colour::Red))
        .for_each(|e| items.push(e));

    td().class("guess-layers guess-cell")
        .child(div().class("guess-group").child(items))
}

fn guess_rarity(rarity: &Rarity, diff: &OrderingText) -> impl IntoView {
    td().class("guess-rarity guess-cell")
        .child(coloured(rarity.to_string(), to_colour(diff)))
}
