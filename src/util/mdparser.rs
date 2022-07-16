use yew::prelude::*;

pub fn get_link(arg: &str, first: usize, end: usize) -> (String, String) {
    (
        arg[first + 1..end].to_string(),
        arg[end + 2..arg.len() - 1].to_string(),
    )
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ImageLink {
    pub name: String,
    pub link: String,
}
impl ImageLink {

    pub fn new() -> Self {
        Self {
            name: String::new(),
            link: String::new(),
        }
    }

    pub fn from_tuple(name: String, link: String) -> Self {
        Self { name, link }
    }


    pub fn push(&mut self, _self: Self) {
        self.name = _self.name;
        self.link = _self.link;
    }

    fn from<S>(item: S) -> Self
    where
        S: AsRef<str>,
    {
        let (name, link) = get_link(item.as_ref(), 1, item.as_ref().find(']').unwrap());
        Self { name, link }
    }
}

#[derive(Debug)]
pub enum Head {
    H1(String),
    H2(String),
    H3(String),
    H4(String),
    H5(String),
}

#[derive(Debug)]
pub enum Markdowns {
    Headers(Head),
    Paragraphs(String),
    Link(String, String),
    Image(ImageLink),
    OrderedList(String),
    UnorderedList(String),
    None,
}

pub fn parse_line<S>(_ln: S) -> Markdowns
where
    S: AsRef<str>,
{
    let line = _ln.as_ref().trim();
    if line.is_empty() {
        return Markdowns::None;
    } else {
        match line.chars().nth(0).unwrap() {
            '#' => {
                let mut lines = line.to_string();
                let sz = lines.matches('#').count() - 1;
                lines.replace_range(0..sz+1, "");
                match sz {
                    0 => Markdowns::Headers(Head::H1(lines.trim().to_string())),
                    1 => Markdowns::Headers(Head::H2(lines.trim().to_string())),
                    2 => Markdowns::Headers(Head::H3(lines.trim().to_string())),
                    3 => Markdowns::Headers(Head::H4(lines.trim().to_string())),
                    4 => Markdowns::Headers(Head::H5(lines.trim().to_string())),
                    _ => Markdowns::None,
                }
            }
            '[' => {
                let (name, link) = get_link(line, 0, line.find(']').unwrap());
                Markdowns::Link(name, link)
            }
            '!' => {
                Markdowns::Image(ImageLink::from(line))
            }
            '-' => Markdowns::UnorderedList(line.trim_start_matches('-').to_string()),
            '1' => {
                let rmved = line
                    .chars()
                    .filter(|ch| !ch.is_numeric())
                    .collect::<String>();
                Markdowns::OrderedList(rmved.trim_start().to_string())
            }

            _ => Markdowns::Paragraphs(line.to_string()),
        }
    }
}

pub fn parse_markdown(source: &str) -> Vec<Vec<Markdowns>> {
    source
        .split("---")
        .map(|d| {
            d.split('\n')
                .into_iter()
                .map(|line| parse_line(line))
                .collect::<Vec<Markdowns>>()
        })
        .collect::<Vec<_>>()
}
