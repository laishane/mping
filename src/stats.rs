use std::time::Duration;
use std::collections::HashMap;
use crate::pinger::PingResult;

#[derive(Debug, Default)]
pub struct PingStats {
    pub sent: u64,
    pub received: u64,
    pub min_rtt: Option<Duration>,
    pub max_rtt: Option<Duration>,
    pub total_rtt: Duration,
}

impl PingStats {
    pub fn update(&mut self, result: &PingResult) {
        self.sent += 1;
        if result.success {
            self.received += 1;
            self.total_rtt += result.rtt;
            
            if let Some(min_rtt) = self.min_rtt {
                if result.rtt < min_rtt {
                    self.min_rtt = Some(result.rtt);
                }
            } else {
                self.min_rtt = Some(result.rtt);
            }

            if let Some(max_rtt) = self.max_rtt {
                if result.rtt > max_rtt {
                    self.max_rtt = Some(result.rtt);
                }
            } else {
                self.max_rtt = Some(result.rtt);
            }
        }
    }

    pub fn average_rtt(&self) -> Option<Duration> {
        if self.received > 0 {
            Some(Duration::from_nanos((self.total_rtt.as_nanos() / self.received as u128) as u64))
        } else {
            None
        }
    }

    pub fn loss_percentage(&self) -> f64 {
        if self.sent == 0 {
            0.0
        } else {
            ((self.sent - self.received) as f64 / self.sent as f64) * 100.0
        }
    }

    pub fn format_summary(&self) -> String {
        let loss_pct = self.loss_percentage();
        let avg_rtt = self.average_rtt().map(|d| d.as_millis()).unwrap_or(0);
        let min_rtt = self.min_rtt.map(|d| d.as_millis()).unwrap_or(0);
        let max_rtt = self.max_rtt.map(|d| d.as_millis()).unwrap_or(0);

        format!(
            "Packets: Sent = {}, Received = {}, Lost = {} ({:.1}% loss)\n\
            Round Trip Times: Min = {}ms, Max = {}ms, Avg = {}ms",
            self.sent,
            self.received,
            self.sent - self.received,
            loss_pct,
            min_rtt,
            max_rtt,
            avg_rtt
        )
    }
}

#[derive(Debug, Default)]
pub struct StatsCollector {
    stats: HashMap<String, PingStats>,
}

impl StatsCollector {
    pub fn new() -> Self {
        Self {
            stats: HashMap::new(),
        }
    }

    pub fn update(&mut self, result: &PingResult) {
        self.stats
            .entry(result.target.clone())
            .or_default()
            .update(result);
    }

    #[allow(dead_code)]
    pub fn get_stats(&self, target: &str) -> Option<&PingStats> {
        self.stats.get(target)
    }

    pub fn format_all_stats(&self) -> String {
        let mut output = String::new();
        for (target, stats) in &self.stats {
            output.push_str(&format!("Statistics for {}:\n{}", target, stats.format_summary()));
        }
        output
    }
} 