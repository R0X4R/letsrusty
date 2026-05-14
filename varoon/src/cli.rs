use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(
    name = "varoon",
    about = "XSS parameter reflection scanner written in Rust",
    long_about = None
)]
pub struct Args {
    #[arg(
        short = 'c',
        long,
        default_value = "50",
        help = "Number of concurrent requests"
    )]
    pub concurrency: usize,
}