use clap::Parser;
use serde::Deserialize;

/// bulk invoice generator tool
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// path to the input file in toml format
    #[arg(short, long)]
    input: std::path::PathBuf,
    /// path to the invoice/letter template in tinytemplate format
    #[arg(short, long)]
    template: std::path::PathBuf,
}
fn main() {
    let args = Args::parse();
    dbg!(args.input);
}
