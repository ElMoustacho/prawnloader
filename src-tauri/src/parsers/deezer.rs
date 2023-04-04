use async_trait::async_trait;

use super::{ParserResult, SongParser};

pub struct DeezerParser {}

impl DeezerParser {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl SongParser for DeezerParser {
    async fn parse_url(&self, _url: &String) -> ParserResult {
        Ok(Vec::new())
    }
}
