use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(
    name = "bhed",
    about = "High-performance duplicate line filter written in Rust, inspired by anew",
    long_about = None
)]
pub struct Args {
    #[arg(value_name = "FILE", help = "Output file to append unique lines")]
    output: Option<String>,

    #[arg(short, long, help = "Suppress stdout output")]
    silent: bool,

    #[arg(short, long, help = "Preview mode: show what would be added without writing to file")]
    preview: bool,

    #[arg(short, long, help = "Trim leading/trailing whitespace before comparing")]
    normalize: bool,

    #[arg(short, long, help = "Compare lines case-insensitively")]
    ignore_case: bool,

    #[arg(short, long, help = "Print only the count of new unique lines")]
    pub count: bool,

    #[arg(short = 'b', long, help = "Skip blank/empty lines")]
    skip_blanks: bool,

    #[arg(long, default_value = "\n", help = "Custom line separator (default: newline)")]
    sep: String,
}

impl From<Args> for crate::Config {
    fn from(args: Args) -> Self {
        Self {
            silent: args.silent,
            preview: args.preview,
            normalize: args.normalize,
            ignore_case: args.ignore_case,
            count_only: args.count,
            skip_blanks: args.skip_blanks,
            separator: args.sep,
            output: args.output,
        }
    }
}