use anyhow::Result;
use reqwest::Client;
use std::time::Duration;

pub struct VaroorClient {
    client: Client,
}

impl VaroorClient {
    pub fn new() -> Result<Self> {
        let client = Client::builder()
            .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
            .timeout(Duration::from_secs(10))
            .connect_timeout(Duration::from_secs(5))
            .build()?;

        Ok(Self { client })
    }

    pub fn client(&self) -> &Client {
        &self.client
    }
}