#![allow(dead_code)]

use crate::markdown_parser::{html, markdown_parse, Html, Markdown, MarkdownInline};
use crate::translate::*;
// use crate::translate::*;

pub const ABOUTME: &str = include_str!("../data/aboutme.txt");
pub const CONTENT: &str = include_str!("../data/contents.md");

#[derive(Clone, Debug, PartialEq)]
pub struct AboutMe {
    pub name: String,
    pub age: i32,
    pub work: String,
    pub education: String,
    pub hobby: Vec<String>,
    pub hastags: Vec<String>,
    pub image_url: String,
    pub abouts: String,
}

impl AboutMe {
    pub fn new() -> Self {
        let items = ABOUTME
            .split('\n')
            .map(|v| match v.split('=').collect::<Vec<_>>().last() {
                Some(expr) => expr.trim(),
                None => "None",
            })
            .filter(|f| !f.is_empty())
            .collect::<Vec<_>>();

        if items.len() < 8 {
            return AboutMe::new();
        }
        Self {
            name: items[0].to_string(),
            age: match items[1].parse::<i32>() {
                Ok(n) => n,
                Err(_) => 0,
            },
            work: items[2].to_string(),
            education: items[3].to_string(),
            hobby: items[4]
                .split(',')
                .map(|s| s.to_string())
                .collect::<Vec<_>>(),
            hastags: items[5]
                .split(',')
                .map(|s| s.to_string())
                .collect::<Vec<_>>(),
            image_url: items[6].to_string(),
            abouts: items[7].to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ContentItem {
    pub name: Option<Html>,
    pub isi: Option<Html>,
    pub link: Option<Html>,
    pub image: Option<Html>,
}
impl ContentItem {
    pub fn new() -> Self {
        Self {
            name: None,
            isi: None,
            link: None,
            image: None,
        }
    }

    pub fn from(markdown_str: &str) -> Self {
        let mut content: ContentItem = ContentItem::new();
        match markdown_parse(markdown_str) {
            Some((_, _cntn_md)) => {
                println!("{:?}", _cntn_md);
                content.name = Some(html! { for _cntn_md.iter().filter_map(|md| match md {
                    Markdown::Heading(size, md)=> Some(translate_header(*size, md.to_vec())),
                    _ => None,
                }) });
                let line = _cntn_md.iter().filter_map(|md| match md {
                    Markdown::Line(text) => Some(text),
                    _ => None,
                });
                content.isi = Some(html! { for line.clone().map(|mdvec| {
                    html! { for mdvec.into_iter().filter_map(|md| match md {
                        MarkdownInline::Bold(_str)=> Some(translate_boldtext(_str.to_string())),
                        MarkdownInline::Italic(_str) => Some(translate_italic(_str.to_string())),
                        MarkdownInline::Plaintext(_str) => Some(html!{ _str.to_string() }),
                        _ => None
                    })}
                })});
                content.link = Some(
                    html! { for line.clone().map(|md| html! { for md.into_iter().filter_map(|mdline| match mdline {
                        MarkdownInline::Link(name, link) => Some(translate_link(name.to_string(), link.to_string())),
                        _ => None
                    }) }) },
                );
                content.image = Some(html! {
                    for line.clone().map(|md| html! {
                        for md.into_iter().filter_map(|mdline| match mdline {
                            MarkdownInline::Image(name, link) => Some(translate_image(name.to_string(), link.to_string())),
                            _ => None
                        })
                    })
                })
            }

            None => {}
        };
        Self {
            name: content.name,
            isi: content.isi,
            link: content.link,
            image: content.image,
        }
    }
    // add code here
}
impl std::fmt::Display for ContentItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "=== content ===")?;
        writeln!(f, " - name: {:#?}", self.name)?;
        writeln!(f, " - isi : {:#?}", self.isi)?;
        writeln!(f, " - link: {:#?}", self.link)?;
        writeln!(f, " - image: {:#?}", self.image)?;
        writeln!(f, "===============")
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ContentLists {
    pub contents: Vec<ContentItem>,
}

impl ContentLists {
    pub fn parse_contents_from_md() -> Self {
        let contents = CONTENT
            .split("---")
            .into_iter()
            .map(|strmd| ContentItem::from(strmd))
            .collect();
        Self { contents }
    }
}
