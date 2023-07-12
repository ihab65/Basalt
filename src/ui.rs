// ui code is going here
// '└─' use this to make braches and these too '✔', '✗'
//                                "   └─[ ]"
//                                "      ∙"


use ansi_term::Colour::*;
use crate::actions::load_path;
use std::fs::File;
use std::io;
use std::io::BufRead;
use crate::parser::{MarkdownElement, parse_line};

type Element = (MarkdownElement, String);

fn get_elements() -> io::Result<Vec<Element>>{
    let path = load_path().unwrap();
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut elements: Vec<Element> = Vec::new();

    for line in reader.lines() {
        let element = parse_line(line?);
        elements.push(element);
    }
    Ok(elements)
}

pub fn render() {
    let list: Vec<Element> = get_elements().unwrap();

    for (kind, text) in list {
        match kind {
            MarkdownElement::Title => println!("{} :", Yellow.bold().paint(text)),
            MarkdownElement::Note  => println!("  ∙ {}", text),
            MarkdownElement::Task  => {
                print!(" {}",
                Yellow.bold().paint("[ ]")
                );
                println!(" {}", text);
            },
            MarkdownElement::Done  => {
                print!(" {}{}{}",
                    Yellow.bold().paint("["),
                    Green.bold().paint("✔"),
                    Yellow.bold().paint("]")
                );
                println!(" {}", text);
            },
            MarkdownElement::SubTask => {
                print!(" {}",
                    Yellow.bold().paint(" └─[ ]")
                );
                println!(" {}", text);

            },
            MarkdownElement::SubDone  => {
                print!(" {}{}{}",
                    Yellow.bold().paint(" └─["),
                    Green.bold().paint("✔"),
                    Yellow.bold().paint("]")
                );
                println!(" {}", text);
            },
            MarkdownElement::SubNote => println!("  {} ∙ {}",Yellow.bold().paint("└─"), text),
            _ => {}
        }
    }


}