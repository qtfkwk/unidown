use clap::{CommandFactory, Parser};
use std::path::PathBuf;

#[derive(Parser)]
#[command(about, version, max_term_width = 80)]
struct Cli {
    /// Input file(s)
    #[arg(short, value_name = "PATH")]
    input_files: Vec<PathBuf>,

    /// Markdown string(s)
    #[arg(value_name = "STRING")]
    input_strings: Vec<String>,
}

fn main() {
    let cli = Cli::parse();

    // Print help if no files or arguments
    if cli.input_strings.is_empty() && cli.input_files.is_empty() {
        let mut cmd = Cli::command();
        cmd.build();
        cmd.print_help().unwrap();
        std::process::exit(0);
    }

    // Process arguments
    for input in &cli.input_strings {
        println!("{}", unidown::convert(input));
    }

    // Process files
    for f in &cli.input_files {
        let input = if f.as_os_str() == "-" {
            std::io::read_to_string(std::io::stdin()).unwrap()
        } else {
            std::fs::read_to_string(f).unwrap()
        };
        print!("{}", unidown::convert(&input));
    }
}
