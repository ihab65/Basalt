use std::fs::File;
use std::io::{self, BufRead};

mod parser;
use parser::{parse_line, MarkdownElement};



fn main() -> io::Result<()> {
    let file = File::open("/home/ihab/Documents/obsidian.main/To-do.md")?;
    let reader = io::BufReader::new(file);

    let mut elements: Vec<(MarkdownElement, String)> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let element = parse_line(line);
        elements.push(element);
    }

    for elem in elements {
        println!("{:?} :\n{}", elem.0 ,elem.1)
    }

    Ok(())
}
