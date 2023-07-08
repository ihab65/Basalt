#[derive(Debug)]
pub enum MarkdownElement {
    CheckboxEmpty(String),
    CheckboxChecked(String),
    Bullet(String),
    Heading(String),
    Unknown(String),
}

pub fn parse(line: String) -> MarkdownElement {
    match line {
        l if l.starts_with("- [ ]") => MarkdownElement::CheckboxEmpty(l[5..].trim().to_string()),
        l if l.starts_with("- [x]") => MarkdownElement::CheckboxChecked(l[5..].trim().to_string()),
        l if l.starts_with('-') => MarkdownElement::Bullet(l[1..].trim().to_string()),
        l if l.starts_with('#') => MarkdownElement::Heading(l.trim().to_string()),
        _ => MarkdownElement::Unknown(line),
    }
}