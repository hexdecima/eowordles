use std::fmt::Display;

use leptos::{html::*, prelude::*};

use crate::GuessContext;
use eowordle_lib::{biomes::Biome, events::Event, layers::Layer, prelude::*};

use super::coloured::{coloured, Colour};

const ARROW_DOWN: char = '↓';
const ARROW_UP: char = '↑';

pub fn guesses() -> impl IntoView {
    let GuessContext(guess_reader, _) = use_context::<GuessContext>().unwrap();

    if guess_reader.get().is_empty() {
        p().id("guess-none")
            .child("Guesses will appear here.")
            .into_any()
    } else {
        table()
            .id("guesses")
            .child((
                tr().id("guesses-header").child((
                    th().class("guess-image guess-cell").child("NPC"),
                    th().class("guess-life guess-cell").child("Life"),
                    th().class("guess-defence guess-cell").child("Defence"),
                    th().class("guess-coins guess-cell").child("Coins"),
                    th().class("guess-biomes guess-cell").child("Environments"),
                    th().class("guess-events guess-cell").child("Events"),
                    th().class("guess-layers guess-cell").child("Layers"),
                    th().class("guess-rarity guess-cell").child("Rarity"),
                )),
                guess_reader
                    .get()
                    .iter()
                    .map(|g| guess(&g.enemy, &g.diff))
                    .collect_view(),
            ))
            .into_any()
    }
}

pub fn guess(enemy: &Enemy, diff: &EnemyDiff) -> impl IntoView {
    tr().class("guess").child((
        guess_image(enemy.id, diff.name),
        guess_life(enemy.life, &diff.life),
        guess_defence(enemy.defence, &diff.defence),
        guess_coins(&enemy.coins, &diff.coins),
        guess_biomes(&enemy.biomes, &diff.biomes),
        guess_events(&enemy.events, &diff.events),
        guess_layers(&enemy.layers, &diff.layers),
        guess_rarity(&enemy.rarity, &diff.rarity),
    ))
}

fn guess_image(id: u16, correct: bool) -> impl IntoView {
    let hl = if correct {
        "guess-correct"
    } else {
        "guess-wrong"
    };
    let classes = format!("guess-image guess-cell {hl}");

    td().class(classes)
        .child(img().src(format!("assets/enemies/{}.gif", id)))
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

fn guess_biomes(biomes: &[Biome], diff: &Diff<Biome>) -> impl IntoView {
    let items = make_coloured_children(biomes, diff);

    td().class("guess-biomes guess-cell")
        .child(div().class("guess-group").child(items))
}

fn guess_events(events: &[Event], diff: &Diff<Event>) -> impl IntoView {
    let items = make_coloured_children(events, diff);

    td().class("guess-events guess-cell")
        .child(div().class("guess-group").child(items))
}

fn guess_layers(layers: &[Layer], diff: &Diff<Layer>) -> impl IntoView {
    let items = make_coloured_children(layers, diff);

    td().class("guess-layers guess-cell")
        .child(div().class("guess-group").child(items))
}

fn guess_rarity(rarity: &Rarity, diff: &OrderingText) -> impl IntoView {
    td().class("guess-rarity guess-cell")
        .child(coloured(rarity.to_string(), to_colour(diff)))
}

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

/// Logic shared between some component functions split into a separate function.
fn make_coloured_children<T>(items: &[T], diff: &Diff<T>) -> Vec<impl IntoView>
where
    T: Clone + Eq + Display,
{
    let mut items = Vec::with_capacity(items.len());
    let colour = if diff.right.is_empty() {
        // If nothing is right.
        Colour::Red
    } else if diff.missing {
        // If something is right but also missing.
        Colour::Yellow
    } else {
        // If something is right and nothing is missing, then everything is right.
        Colour::Green
    };

    diff.right
        .iter()
        .map(|i| coloured(i.to_string(), colour))
        .for_each(|e| items.push(e));
    diff.wrong
        .iter()
        .map(|i| coloured(i.to_string(), colour))
        .for_each(|e| items.push(e));

    items
}
