mod cli;
use clap::Parser;

use bhed::{Config, Processor};

fn main() -> anyhow::Result<()> {
    let args = cli::Args::parse();
    let config = Config::from(args.clone());

    let mut processor = Processor::new(&config)?;
    let new_count = processor.process_stdin(&config)?;

    if args.count {
        println!("{}", new_count);
    }

    Ok(())
}