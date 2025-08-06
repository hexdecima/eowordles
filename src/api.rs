use eowordle_lib::{Enemy, EnemyDiff};
use reqwasm::http::{Request, RequestMode};

const ENDPOINT: &str = "https://eowordles-qb4c.shuttle.app";

fn make_route(route: impl AsRef<str>) -> String {
    format!("{ENDPOINT}{}", route.as_ref())
}

pub async fn get_daily_diff(id: u16) -> Option<EnemyDiff> {
    Request::get(&make_route(format!("/daily/diff/{id}"))).mode(RequestMode::Cors).send().await.ok()?.json::<EnemyDiff>().await.ok()
}

pub async fn get_yesterday() -> Option<Enemy> {
    Request::get(&make_route("/yesterday")).mode(RequestMode::Cors).send().await.ok()?.json::<Enemy>().await.ok()
}
