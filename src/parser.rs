#[derive(Debug)]
pub enum MarkdownElement {
    Note, Title, SubTask, Emtpy,
    Task, Other, SubDone,
    Done, Line , SubNote,
}


pub fn parse_line(line: String) -> (MarkdownElement, String) {
    match line {
        l if l.starts_with("---") => (MarkdownElement::Line, String::from("---")),
        l if l.is_empty() => (MarkdownElement::Emtpy, String::from("")),

        l if l.starts_with("- [ ]") => {
            let text = l[5..].trim().to_string();
            (MarkdownElement::Task, text)
        },

        l if l.starts_with("\t- [ ]") => {
            let text = l[6..].trim().to_string();
            (MarkdownElement::SubTask, text)
        },

        l if l.starts_with("- [x]") => {
            let text = l[5..].trim().to_string();
            (MarkdownElement::Done, text) 
        },

        l if l.starts_with("\t- [x]") => {
            let text = l[6..].trim().to_string();
            (MarkdownElement::SubDone, text)
        },

        l if l.starts_with('-') => {
            let text = l[1..].trim().to_string();
            (MarkdownElement::Note, text)
        },
        l if l.starts_with("\t-") => {
            let text = l[2..].trim().to_string();
            (MarkdownElement::SubNote, text)
        },
        
        l if l.starts_with('#') => {
            (MarkdownElement::Title, l.trim_matches('#').trim().to_string())
        },
        _ => (MarkdownElement::Other, String::from("not defined yet"))
    }
}