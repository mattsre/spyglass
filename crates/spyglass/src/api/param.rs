use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SearchParam<'r> {
    pub term: &'r str,
}

#[derive(Debug, Deserialize)]
pub struct QueueItemParam<'r> {
    pub url: &'r str,
    pub force_crawl: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateStatusParam {
    pub toggle_pause: Option<bool>,
}
