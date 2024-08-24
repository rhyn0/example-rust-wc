mod args;
use args::Cli;
use clap::Parser;
fn main() {
    let cli = Cli::parse();
    println!("Cli gave us {:?}", cli.get_output_settings());
}
