use anyhow::Result;
use async_trait::async_trait;
use chrono::Utc;
use std::time::{Duration, Instant};
use surge_ping::{Client, PingIdentifier, PingSequence};
use tokio::net::TcpStream;
use rand::random;

#[derive(Debug, Clone)]
pub struct PingResult {
    pub target: String,
    pub timestamp: chrono::DateTime<Utc>,
    pub success: bool,
    pub rtt: Duration,
}

#[async_trait]
pub trait Pinger: Send + Sync {
    async fn ping(&self, target: &str) -> Result<PingResult>;
    fn box_clone(&self) -> Box<dyn Pinger>;
}

#[derive(Clone)]
pub struct IcmpPinger {
    client: Client,
}

impl IcmpPinger {
    pub fn new() -> Result<Self> {
        let client = Client::new(&surge_ping::Config::default())?;
        Ok(Self { client })
    }
}

#[async_trait]
impl Pinger for IcmpPinger {
    async fn ping(&self, target: &str) -> Result<PingResult> {
        let addr = tokio::net::lookup_host(format!("{}:0", target))
            .await?
            .next()
            .ok_or_else(|| anyhow::anyhow!("Failed to resolve host"))?;

        let payload = b"Hello";
        let mut pinger = self.client.pinger(addr.ip(), PingIdentifier(random::<u16>())).await;
        pinger.timeout(Duration::from_secs(5));

        let (_response, duration) = pinger.ping(PingSequence(1), payload).await?;

        Ok(PingResult {
            target: target.to_string(),
            timestamp: Utc::now(),
            success: true,
            rtt: duration,
        })
    }

    fn box_clone(&self) -> Box<dyn Pinger> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
pub struct TcpPinger;

impl TcpPinger {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Pinger for TcpPinger {
    async fn ping(&self, target: &str) -> Result<PingResult> {
        let start = Instant::now();
        let addr = if !target.contains(':') {
            format!("{}:80", target)
        } else {
            target.to_string()
        };

        let result = TcpStream::connect(&addr).await;
        let duration = start.elapsed();

        Ok(PingResult {
            target: target.to_string(),
            timestamp: Utc::now(),
            success: result.is_ok(),
            rtt: duration,
        })
    }

    fn box_clone(&self) -> Box<dyn Pinger> {
        Box::new(self.clone())
    }
} 