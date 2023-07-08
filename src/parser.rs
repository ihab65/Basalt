#[derive(Debug)]
pub enum MarkdownElement {
    Note(String), Title(String), SubTask(String), Emtpy ,
    Task(String), Other(String), SubDone(String),
    Done(String), Line         , SubNote(String),
}


pub fn parse(line: String) -> MarkdownElement {
    match line {
        l if l.starts_with("---") => MarkdownElement::Line,
        l if l.is_empty() => MarkdownElement::Emtpy,

        l if l.starts_with("- [ ]") => {
            MarkdownElement::Task(l[5..].trim().to_string())
        },

        l if l.starts_with("\t- [ ]") => {
            MarkdownElement::SubTask(l[6..].trim().to_string())
        },

        l if l.starts_with("- [x]") => {
            MarkdownElement::Done(l[5..].trim().to_string())
        },

        l if l.starts_with("\t- [x]") => {
            MarkdownElement::SubDone(l[6..].trim().to_string())
        },

        l if l.starts_with('-') => {
            MarkdownElement::Note(l[1..].trim().to_string())
        },
        l if l.starts_with("\t-") => {
            MarkdownElement::SubNote(l[2..].trim().to_string())
        },
        
        l if l.starts_with('#') => MarkdownElement::Title(l.trim_matches('#').trim().to_string()),
        _ => MarkdownElement::Other(line),
    }
}