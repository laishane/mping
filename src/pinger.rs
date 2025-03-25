use anyhow::Result;
use async_trait::async_trait;
use chrono::Utc;
use std::time::{Duration, Instant};
use surge_ping::{Client, PingIdentifier, PingSequence};
use tokio::net::TcpStream;
use rand::random;
use tokio::time::timeout;

#[derive(Debug, Clone)]
pub struct PingResult {
    pub target: String,
    pub timestamp: chrono::DateTime<Utc>,
    pub success: bool,
    pub rtt: Duration,
    pub error_msg: Option<String>,
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
        let start = Instant::now();
        let addr_result = tokio::net::lookup_host(format!("{}:0", target)).await;

        match addr_result {
            Ok(mut addrs) => {
                if let Some(addr) = addrs.next() {
                    let payload = b"Hello";
                    let mut pinger = self.client.pinger(addr.ip(), PingIdentifier(random::<u16>())).await;
                    pinger.timeout(Duration::from_secs(1));

                    match timeout(Duration::from_secs(1), pinger.ping(PingSequence(1), payload)).await {
                        Ok(Ok((_response, duration))) => Ok(PingResult {
                            target: target.to_string(),
                            timestamp: Utc::now(),
                            success: true,
                            rtt: duration,
                            error_msg: None,
                        }),
                        Ok(Err(e)) => {
                            let error_msg = match e.to_string().to_lowercase().as_str() {
                                s if s.contains("timeout") => "Request timed out".to_string(),
                                s if s.contains("network is unreachable") => "Network unreachable".to_string(),
                                s if s.contains("host is unreachable") => "Host unreachable".to_string(),
                                s if s.contains("no route to host") => "No route to host".to_string(),
                                s if s.contains("connection refused") => "Connection refused".to_string(),
                                _ => format!("ICMP error: {}", e),
                            };
                            Ok(PingResult {
                                target: target.to_string(),
                                timestamp: Utc::now(),
                                success: false,
                                rtt: start.elapsed(),
                                error_msg: Some(error_msg),
                            })
                        }
                        Err(_) => Ok(PingResult {
                            target: target.to_string(),
                            timestamp: Utc::now(),
                            success: false,
                            rtt: start.elapsed(),
                            error_msg: Some("Request timed out".to_string()),
                        })
                    }
                } else {
                    Ok(PingResult {
                        target: target.to_string(),
                        timestamp: Utc::now(),
                        success: false,
                        rtt: start.elapsed(),
                        error_msg: Some("Could not resolve hostname".to_string()),
                    })
                }
            }
            Err(e) => Ok(PingResult {
                target: target.to_string(),
                timestamp: Utc::now(),
                success: false,
                rtt: start.elapsed(),
                error_msg: Some(format!("DNS resolution failed: {}", e)),
            })
        }
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

        match timeout(Duration::from_secs(1), TcpStream::connect(&addr)).await {
            Ok(Ok(_)) => Ok(PingResult {
                target: target.to_string(),
                timestamp: Utc::now(),
                success: true,
                rtt: start.elapsed(),
                error_msg: None,
            }),
            Ok(Err(e)) => {
                let error_msg = match e.kind() {
                    std::io::ErrorKind::ConnectionRefused => "Connection refused".to_string(),
                    std::io::ErrorKind::ConnectionReset => "Connection reset".to_string(),
                    std::io::ErrorKind::NetworkUnreachable => "Network unreachable".to_string(),
                    std::io::ErrorKind::HostUnreachable => "Host unreachable".to_string(),
                    std::io::ErrorKind::WouldBlock => "Connection timed out".to_string(),
                    _ => format!("TCP connection failed: {}", e),
                };
                Ok(PingResult {
                    target: target.to_string(),
                    timestamp: Utc::now(),
                    success: false,
                    rtt: start.elapsed(),
                    error_msg: Some(error_msg),
                })
            }
            Err(_) => Ok(PingResult {
                target: target.to_string(),
                timestamp: Utc::now(),
                success: false,
                rtt: start.elapsed(),
                error_msg: Some("Connection timed out".to_string()),
            }),
        }
    }

    fn box_clone(&self) -> Box<dyn Pinger> {
        Box::new(self.clone())
    }
} 