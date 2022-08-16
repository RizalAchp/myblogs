pub mod parser;
pub mod translate;

pub use yew::html::Scope;
pub use yew::prelude::*;
pub use yew_router::prelude::*;

pub type MarkdownText = Vec<MarkdownInline>;

#[derive(Clone, Debug, PartialEq)]
pub enum Markdown {
    Heading(usize, MarkdownText),
    OrderedList(Vec<MarkdownText>),
    UnorderedList(Vec<MarkdownText>),
    Line(MarkdownText),
    Codeblock(String, String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MarkdownInline {
    Link(String, String),
    Image(String, String),
    InlineCode(String),
    Bold(String),
    Italic(String),
    Plaintext(String),
}

pub fn markdown_to_yew(md: &str) -> yew::Html {
    match parser::parse_markdown(md) {
        Ok((_, m)) => translate::translate(m),
        Err(_) => html! {},
    }
}

pub fn markdown_parse(md: &str) -> Option<(&str, Vec<Markdown>)> {
    match parser::parse_markdown(md) {
        Ok(item) => Some(item),
        Err(_) => None,
    }
}
