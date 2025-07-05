#[derive(Clone, Default)]
pub struct MatchInfo {
    pub map: Option<String>,
    pub server_version: Option<u32>,
    pub match_id: Option<u64>,
}
