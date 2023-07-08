use std::fs::File;
use std::io::{self, BufRead};

mod parser;
use crate::parser::{
    MarkdownElement,
    parse
};

fn main() -> io::Result<()> {
    let file = File::open("../../Documents/obsidian.main/To-do.md")?;
    let reader = io::BufReader::new(file);

    let mut elements: Vec<MarkdownElement> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let element = parse(line);
        elements.push(element);
    }

    for elem in elements {
        println!("{:?}", elem)
    }

    Ok(())
}
