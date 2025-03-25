mod cli;
mod pinger;
mod stats;

use anyhow::Result;
use clap::Parser;
use cli::Args;
use pinger::{IcmpPinger, Pinger, TcpPinger};
use stats::StatsCollector;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use tokio::time::{Duration, sleep};
use futures::future::join_all;
use tokio::task::JoinHandle;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    if args.targets.is_empty() {
        println!("Error: At least one target must be specified");
        std::process::exit(1);
    }

    // Setup logger file if specified
    let mut log_file = if let Some(log_path) = args.log.as_ref() {
        Some(
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(log_path)
                .await?,
        )
    } else {
        None
    };

    // Setup Ctrl+C handler with specific configuration
    let running = Arc::new(AtomicBool::new(true));
    
    #[cfg(windows)]
    {
        use ctrlc::set_handler;
        let r = running.clone();
        set_handler(move || {
            println!("\nReceived Ctrl+C, stopping all ping tasks...");
            r.store(false, Ordering::SeqCst);
        })?;
    }

    #[cfg(unix)]
    {
        use signal_hook::consts::SIGINT;
        use signal_hook::flag::register;
        use signal_hook::iterator::Signals;
        
        // 创建一个新的信号处理器
        let mut signals = Signals::new(&[SIGINT])?;
        let r = running.clone();
        
        // 在后台线程中处理信号
        std::thread::spawn(move || {
            for _ in signals.forever() {
                println!("\nReceived Ctrl+C, stopping all ping tasks...");
                r.store(false, Ordering::SeqCst);
                break;
            }
        });
    }

    // Initialize pinger based on protocol
    let pinger: Box<dyn Pinger> = match args.protocol.to_lowercase().as_str() {
        "icmp" => Box::new(IcmpPinger::new()?),
        "tcp" => Box::new(TcpPinger::new()),
        _ => {
            println!("Error: Invalid protocol specified. Use 'icmp' or 'tcp'");
            std::process::exit(1);
        }
    };

    let stats_collector = Arc::new(tokio::sync::Mutex::new(StatsCollector::new()));
    let interval = Duration::from_secs(args.interval);
    let mut tasks = Vec::new();

    // Create a task for each target
    for target in args.targets {
        let pinger = pinger.box_clone();
        let running = running.clone();
        let stats_collector = stats_collector.clone();
        let display = args.display;
        let mut log_file = log_file.take();
        let count = args.count;
        let interval = interval;

        let handle: JoinHandle<Result<()>> = tokio::spawn(async move {
            let mut current_count = 0;
            while running.load(Ordering::SeqCst) {
                if count > 0 && current_count >= count {
                    break;
                }

                let result = pinger.ping(&target).await?;
                {
                    let mut stats = stats_collector.lock().await;
                    stats.update(&result);

                    if display {
                        let status = if result.success {
                            "Success".to_string()
                        } else {
                            result.error_msg.clone().unwrap_or_else(|| "Failed".to_string())
                        };
                        
                        println!(
                            "Target: {} Time: {} Status: {} RTT: {}ms",
                            result.target,
                            result.timestamp.format("%Y-%m-%d %H:%M:%S"),
                            status,
                            result.rtt.as_millis()
                        );
                    }
                }

                if let Some(ref mut file) = log_file {
                    let status = if result.success {
                        "Success".to_string()
                    } else {
                        result.error_msg.clone().unwrap_or_else(|| "Failed".to_string())
                    };
                    
                    let log_entry = format!(
                        "{},{},{},{}\n",
                        result.target,
                        result.timestamp.format("%Y-%m-%d %H:%M:%S"),
                        status,
                        result.rtt.as_millis()
                    );
                    file.write_all(log_entry.as_bytes()).await?;
                }

                current_count += 1;
                sleep(interval).await;
            }
            Ok(())
        });

        tasks.push(handle);
    }

    // Wait for all tasks to complete
    join_all(tasks).await;

    // Print final statistics
    let stats = stats_collector.lock().await;
    let stats_text = stats.format_all_stats();
    println!("Final Statistics:");
    println!("{}", stats_text);

    // Write final statistics to log file if specified
    if let Some(ref mut file) = log_file {
        let log_entry = format!(
            "\n--- Final Statistics ---\n{}\n--- End of Statistics ---\n",
            stats_text
        );
        file.write_all(log_entry.as_bytes()).await?;
    }

    Ok(())
}
