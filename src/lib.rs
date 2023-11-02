use reqwest::Client;
use thiserror::Error;
use url::Url;

#[derive(Debug, Error)]
pub enum PduDaemonError {
    #[error("Could not parse url: {0}")]
    ParseUrlError(#[from] url::ParseError),
    #[error("Http request failed: {0}")]
    ReqwestError(#[from] reqwest::Error),
}

#[derive(Debug, Clone)]
pub struct PduDaemon {
    client: Client,
    url: Url,
}

impl PduDaemon {
    /// Create a new Pdudaemon client
    pub fn new(url: &str) -> Result<Self, PduDaemonError> {
        let client = Client::new();
        let url = url.parse()?;
        Ok(Self { client, url })
    }

    fn build_url(&self, command: &str, hostname: &str, port: &str) -> Result<Url, PduDaemonError> {
        let mut url = self.url.join("power/control/")?.join(command)?;

        url.query_pairs_mut()
            .append_pair("hostname", hostname)
            .append_pair("port", port);
        Ok(url)
    }

    async fn send(&self, url: Url) -> Result<(), PduDaemonError> {
        self.client.get(url).send().await?.error_for_status()?;
        Ok(())
    }

    /// Send the on command to a given pdu hostname and port
    pub async fn on(&self, hostname: &str, port: &str) -> Result<(), PduDaemonError> {
        let url = self.build_url("on", hostname, port)?;
        self.send(url).await
    }

    /// Send the off command to a given pdu hostname and port
    pub async fn off(&self, hostname: &str, port: &str) -> Result<(), PduDaemonError> {
        let url = self.build_url("off", hostname, port)?;
        self.send(url).await
    }

    /// Send the reboot command to a given pdu hostname and port with an optional delay
    pub async fn reboot(
        &self,
        hostname: &str,
        port: &str,
        delay: Option<u32>,
    ) -> Result<(), PduDaemonError> {
        let mut url = self.build_url("reboot", hostname, port)?;
        if let Some(delay) = delay {
            url.query_pairs_mut()
                .append_pair("delay", &delay.to_string());
        }
        self.send(url).await
    }
}
