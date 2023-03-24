mod parser;
mod translate;

pub use parser::render_markdown;
use yew::virtual_dom::VNode;


pub fn render_markdown_from_bytes(bytes: &Vec<u8>) -> VNode {
    let s = match String::from_utf8(bytes.clone()) {
        Ok(v) => v,
        Err(_) => "# NO README PROVIDED\n".to_owned(),
    };
    render_markdown(&s)
}
