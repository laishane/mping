use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Target hosts to ping
    #[arg(short, long, num_args = 1.., value_delimiter = ' ')]
    pub targets: Vec<String>,

    /// Display results in terminal
    #[arg(short, long, default_value = "true")]
    pub display: bool,

    /// Log file path
    #[arg(short, long)]
    pub log: Option<PathBuf>,

    /// Run in background
    #[arg(short, long, default_value = "false")]
    pub background: bool,

    /// Protocol to use (icmp/tcp)
    #[arg(short, long, default_value = "icmp")]
    pub protocol: String,

    /// Interval between pings in seconds
    #[arg(short, long, default_value = "1")]
    pub interval: u64,

    /// Number of pings (0 for unlimited)
    #[arg(short, long, default_value = "0")]
    pub count: u64,
} 