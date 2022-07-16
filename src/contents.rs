pub use crate::util::mdparser::{parse_markdown, Head, Markdowns, ImageLink};
use yew::prelude::*;

const ABOUTME: &str = include_str!("../data/aboutme.txt");
const CONTENT: &str = include_str!("../data/contents.md");

#[derive(Clone, Debug, PartialEq, Properties)]
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

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct LinkName {
    name: String,
    link: String,
}
impl LinkName {
    fn new() -> Self {
        Self {
            name: String::new(),
            link: String::new(),
        }
    }
    fn from(name: String, link: String) -> Self {
        Self { name, link }
    }

    fn push(&mut self, _self: Self) {
        self.name = _self.name;
        self.link = _self.link;
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Contents {
    pub name: String,
    pub isi: String,
    pub linkrepo: LinkName,
    pub imageurl: ImageLink,
}

impl Contents {
    pub fn from(item: Vec<Markdowns>) -> Self {
        let mut isi = String::new();
        let mut name = String::new();
        let mut linkrepo = LinkName::new();
        let mut imageurl = ImageLink::new();

        println!("len in vec {}", item.len());
        let _ = item.into_iter().map(|x| match x {
            Markdowns::Headers(Head::H2(s)) => {
                name.push_str(&s);
                s.to_string()
            },
            Markdowns::Paragraphs(s) => {
                isi.push_str(&s);
                s.to_string()
            },
            Markdowns::Link(s,d) =>{
                linkrepo.push(LinkName{name: s.to_string(), link:d.to_string()});
                s.to_string()
            },
            Markdowns::Image(s) => {
                imageurl.push(s);
                s.name.to_string()
            }
            _ => String::new(),
        }).collect::<Vec<_>>();

        Self {
            name,
            isi,
            linkrepo,
            imageurl,
        }
    }
}

pub fn get_contents() -> Vec<Contents> {
    parse_markdown(CONTENT).into_iter().map(|dt| Contents::from(dt)).collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use crate::contents::get_contents;
    #[test]
    fn test_datacontents() {
        let cntns = get_contents();
        for content in cntns {
            println!("{:#?}", content)
        }
        assert!(false, "is just testring")
    }
}
