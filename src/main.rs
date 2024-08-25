mod args;
mod count;
use std::process::exit;

use args::{Cli, OutputOptions};
use clap::Parser;
use count::{get_counts, ResultOutput};

fn print_results(arr: &[ResultOutput], _options: OutputOptions) {
    // dbg!(options);
    let mut total = ResultOutput::new("total".to_string());
    for result in arr {
        total += result.clone();
        print!("{result}");
    }
    if arr.len() > 1 {
        print!("{total}");
    }
}
fn main() {
    let cli = Cli::parse();
    let output = cli.get_output_settings();
    if !output.character_count {
        eprintln!("Failed to activate the only working flag. Exiting...");
        exit(1);
    }
    let mut results = Vec::with_capacity(cli.files.len());
    for file in cli.files {
        if let Ok(result) = get_counts(file.clone()) {
            results.push(result);
        } else {
            eprintln!("Failed to open file {file}");
        }
    }
    print_results(&results, output);
}
