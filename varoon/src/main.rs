mod cli;
mod client;
mod reflector;

use anyhow::Result;
use clap::Parser;
use client::VaroorClient;
use reflector::{check_reflection, check_unfiltered_chars};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::sync::mpsc;
use tokio::sync::Semaphore;

async fn worker(
    mut rx: mpsc::Receiver<String>,
    client: Arc<VaroorClient>,
    semaphore: Arc<Semaphore>,
    completed: Arc<AtomicUsize>,
) {
    while let Some(url) = rx.recv().await {
        let _guard = semaphore.acquire().await.unwrap();

        let reflected_params = match check_reflection(&client, &url).await {
            Ok(params) => params,
            Err(_) => {
                completed.fetch_add(1, Ordering::Relaxed);
                continue;
            }
        };

        if reflected_params.is_empty() {
            completed.fetch_add(1, Ordering::Relaxed);
            continue;
        }

        for (param, _) in reflected_params {
            let unfiltered = check_unfiltered_chars(&client, &url, &param).await;

            if !unfiltered.is_empty() {
                println!(
                    "URL: {} Param: [ {} ] Unfiltered: [ {} ]",
                    url,
                    param,
                    unfiltered.iter().collect::<String>()
                );
            }
        }

        completed.fetch_add(1, Ordering::Relaxed);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = cli::Args::parse();

    let client = Arc::new(VaroorClient::new()?);
    let semaphore = Arc::new(Semaphore::new(args.concurrency));
    let completed = Arc::new(AtomicUsize::new(0));

    let (tx, rx) = mpsc::channel(args.concurrency * 2);

    let worker_client = Arc::clone(&client);
    let worker_semaphore = Arc::clone(&semaphore);
    let worker_completed = Arc::clone(&completed);

    tokio::spawn(async move {
        worker(rx, worker_client, worker_semaphore, worker_completed).await;
    });

    let mut count = 0;
    for line in std::io::stdin().lines() {
        if let Ok(url) = line {
            let url = url.trim().to_string();
            if url.is_empty() {
                continue;
            }

            if tx.send(url).await.is_err() {
                break;
            }
            count += 1;
        }
    }

    drop(tx);

    while completed.load(Ordering::Relaxed) < count {
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }

    Ok(())
}