use eowordle_lib::EnemyDiff;
use reqwasm::http::{Request, RequestMode};

const ENDPOINT: &str = "http://localhost:3010";

fn make_route(route: impl AsRef<str>) -> String {
    format!("{ENDPOINT}{}", route.as_ref())
}

pub async fn get_daily_diff(id: u16) -> Option<EnemyDiff> {
    Request::get(&make_route(format!("/daily/diff/{id}"))).mode(RequestMode::Cors).send().await.ok()?.json::<EnemyDiff>().await.ok()
}
