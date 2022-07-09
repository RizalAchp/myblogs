use serde_derive::{Deserialize, Serialize};
const ABOUTME: &str = include_str!("../data/aboutme.txt");
const ABOUT_CONTENT: &str = include_str!("../data/about.txt");

#[derive(Debug, Serialize, Deserialize)]
pub struct AboutMe {
    pub name: String,
    pub age: i32,
    pub work: String,
    pub education: String,
    pub hobby: Vec<String>,
    pub hastags: Vec<String>,
    pub image_url: String,
    pub abouts: String
}

impl AboutMe {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            age: 0,
            work: String::new(),
            education: String::new(),
            hobby: Vec::new(),
            hastags: Vec::new(),
            image_url: String::new(),
            abouts: String::new()
        }
    }
    pub fn from(arg: Vec<&str>) -> Self {
        if arg.len() < 8 {
            return AboutMe::new();
        }
        Self {
            name: arg[0].to_string(),
            age: match arg[1].parse::<i32>() {
                Ok(n) => n,
                Err(_) => 0,
            },
            work: arg[2].to_string(),
            education: arg[3].to_string(),
            hobby: arg[4].split(',').map(|s| s.to_string()).collect::<Vec<_>>(),
            hastags: arg[5].split(',').map(|s| s.to_string()).collect::<Vec<_>>(),
            image_url: arg[6].to_string(),
            abouts: arg[7].to_string()
        }
    }
}

pub fn about_me() -> AboutMe {
    let items = ABOUTME
        .split('\n')
        .map(|v| match v.split('=').collect::<Vec<_>>().last() {
            Some(expr) => expr.trim(),
            None => "None",
        })
        .filter(|f| !f.is_empty())
        .collect::<Vec<_>>();

    AboutMe::from(items)
}


fn title_case(word: &str) -> String {
    let idx = match word.chars().next() {
        Some(c) => c.len_utf8(),
        None => 0,
    };

    let mut result = String::with_capacity(word.len());
    result.push_str(&word[..idx].to_uppercase());
    result.push_str(&word[idx..]);
    result
}
