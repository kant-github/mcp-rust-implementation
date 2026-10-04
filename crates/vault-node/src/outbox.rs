use anyhow::{Ok, Result};

use crate::envelope::Envelope;

#[derive(Clone, Debug)]
pub struct Outbox {
    client: reqwest::Client,
    node_urls: Vec<String>,
}

impl Outbox {
    pub fn new(node_urls: Vec<String>) -> Self {
        Outbox {
            client: reqwest::Client::new(),
            node_urls,
        }
    }

    pub fn to_url(&self, index: u16) -> &str {
        let url = self.node_urls.get(index as usize);
        match url {
            Some(u) => u,
            None => "unknown node",
        }
    }

    pub async fn send(&self, to: u16, envelope: &Envelope) -> Result<()> {
        let url = format!("{}/v1/message", self.to_url(to));
        let client_sent = self.client.post(url).json(envelope).send().await?;
        client_sent.error_for_status()?;
        Ok(())
    }
}
