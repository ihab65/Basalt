use clap:: {
    Parser,
    Subcommand
};

use crate::actions::PathToSet;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct BasaltArgs {
    #[command(subcommand)]
    pub command: Commands
}

#[derive(Subcommand)]
pub enum Commands {
    /// set the path to the you to-do list 
    SetPath(PathToSet),
    /// view your curent tasks
    Peek,
    /// check, edit, add your tasks
    Edit
}
