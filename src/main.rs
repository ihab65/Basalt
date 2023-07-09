use std::fs::File;
use std::io::{self, BufRead};
use actions::set_path;
use clap::Parser;

mod actions;
mod cli;
mod parser;

use cli::BasaltArgs;
use parser::{parse_line, MarkdownElement};



fn main() -> io::Result<()> {
    
    let cli = BasaltArgs::parse();

    match &cli.command {
        cli::Commands::SetPath(path) => {
            set_path(path);
            let file = File::open(path)?;
            let reader = io::BufReader::new(file);
            let mut elements: Vec<(MarkdownElement, String)> = Vec::new();

            for line in reader.lines() {
                let line = line?;
                let element = parse_line(line);
                elements.push(element);
            }

            for (kind, content) in elements {
                println!("{:?} : {}", kind ,content)
            }
        },
        cli::Commands::Peek => {},
        cli::Commands::Peek => {},
        _ => {}
    };
    
    // let file = File::open("/home/ihab/Documents/obsidian.main/To-do.md")?;
    
    

    

    

    Ok(())
}
