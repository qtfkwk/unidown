use anyhow::Result;
use clap::{CommandFactory, Parser};
use std::path::PathBuf;

#[cfg(unix)]
use pager::Pager;

const README: &str = include_str!("../../README.md");

#[derive(Parser)]
#[command(about, version, max_term_width = 80)]
struct Cli {
    /// Style
    #[arg(short)]
    style: Option<unidown::Style>,

    /// Input file(s)
    #[arg(short, value_name = "PATH")]
    input_files: Vec<PathBuf>,

    /// Print readme
    #[arg(short)]
    readme: bool,

    /// Markdown string(s)
    #[arg(value_name = "STRING")]
    input_strings: Vec<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Print readme
    if cli.readme {
        #[cfg(unix)]
        Pager::with_pager("bat -pl md").setup();

        print!("{README}");
        return Ok(());
    }

    // Print help if no files or arguments
    if cli.input_strings.is_empty() && cli.input_files.is_empty() {
        let mut cmd = Cli::command();
        cmd.build();
        cmd.print_help().unwrap();
        return Ok(());
    }

    // Process arguments
    for input in &cli.input_strings {
        print!(
            "{}",
            if let Some(style) = &cli.style {
                style.convert(input)
            } else {
                unidown::convert(input)
            }
        );
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

    Ok(())
}
