
use std::io;
use actions::set_path;
use clap::Parser;

mod actions;
mod cli;
mod ui;
mod parser;

use cli::BasaltArgs;

fn main() -> io::Result<()> {
    
    let cli = BasaltArgs::parse();

    match &cli.command {
        cli::Commands::SetPath(path) => {
            set_path(path);
        },
        cli::Commands::Peek => {
            // for (kind, content) in elements {
            //     println!("{:?} : {}", kind ,content)
            // }
            ui::render()
        },
        cli::Commands::Edit => {
            println!("not implemented yet !")
        }
    };
    
    Ok(())
}
