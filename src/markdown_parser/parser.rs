use super::Markdown;
use super::MarkdownInline;
use super::MarkdownText;

use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take, take_while1},
    character::is_digit,
    combinator::{map, not},
    multi::{many0, many1},
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};

pub fn parse_markdown(i: &str) -> IResult<&str, Vec<Markdown>> {
    many1(alt((
        map(parse_header, |e| Markdown::Heading(e.0, e.1)),
        map(parse_unordered_list, |e| Markdown::UnorderedList(e)),
        map(parse_ordered_list, |e| Markdown::OrderedList(e)),
        map(parse_code_block, |e| {
            Markdown::Codeblock(e.0.to_string(), e.1.to_string())
        }),
        map(parse_markdown_text, |e| Markdown::Line(e)),
    )))(i)
}

fn parse_boldtext(i: &str) -> IResult<&str, &str> {
    delimited(tag("**"), is_not("**"), tag("**"))(i)
}

fn parse_italics(i: &str) -> IResult<&str, &str> {
    delimited(tag("*"), is_not("*"), tag("*"))(i)
}

fn parse_inline_code(i: &str) -> IResult<&str, &str> {
    delimited(tag("`"), is_not("`"), tag("`"))(i)
}

fn parse_link(i: &str) -> IResult<&str, (&str, &str)> {
    pair(
        delimited(tag("["), is_not("]"), tag("]")),
        delimited(tag("("), is_not(")"), tag(")")),
    )(i)
}

fn parse_image(i: &str) -> IResult<&str, (&str, &str)> {
    pair(
        delimited(tag("!["), is_not("]"), tag("]")),
        delimited(tag("("), is_not(")"), tag(")")),
    )(i)
}

fn parse_plaintext(i: &str) -> IResult<&str, String> {
    map(
        many1(preceded(
            not(alt((tag("*"), tag("`"), tag("["), tag("!["), tag("\n")))),
            take(1u8),
        )),
        |vec| vec.join(""),
    )(i)
}

fn parse_markdown_inline(i: &str) -> IResult<&str, MarkdownInline> {
    alt((
        map(parse_italics, |s: &str| {
            MarkdownInline::Italic(s.to_string())
        }),
        map(parse_inline_code, |s: &str| {
            MarkdownInline::InlineCode(s.to_string())
        }),
        map(parse_boldtext, |s: &str| {
            MarkdownInline::Bold(s.to_string())
        }),
        map(parse_image, |(tag, url): (&str, &str)| {
            MarkdownInline::Image(tag.to_string(), url.to_string())
        }),
        map(parse_link, |(tag, url): (&str, &str)| {
            MarkdownInline::Link(tag.to_string(), url.to_string())
        }),
        map(parse_plaintext, |s| MarkdownInline::Plaintext(s)),
    ))(i)
}

fn parse_markdown_text(i: &str) -> IResult<&str, MarkdownText> {
    terminated(many0(parse_markdown_inline), tag("\n"))(i)
}

// this guy matches the literal character #
fn parse_header_tag(i: &str) -> IResult<&str, usize> {
    map(
        terminated(take_while1(|c| c == '#'), tag(" ")),
        |s: &str| s.len(),
    )(i)
}

// this combines a tuple of the header tag and the rest of the line
fn parse_header(i: &str) -> IResult<&str, (usize, MarkdownText)> {
    tuple((parse_header_tag, parse_markdown_text))(i)
}

fn parse_unordered_list_tag(i: &str) -> IResult<&str, &str> {
    terminated(tag("-"), tag(" "))(i)
}

fn parse_unordered_list_element(i: &str) -> IResult<&str, MarkdownText> {
    preceded(parse_unordered_list_tag, parse_markdown_text)(i)
}

fn parse_unordered_list(i: &str) -> IResult<&str, Vec<MarkdownText>> {
    many1(parse_unordered_list_element)(i)
}

fn parse_ordered_list_tag(i: &str) -> IResult<&str, &str> {
    terminated(
        terminated(take_while1(|d| is_digit(d as u8)), tag(".")),
        tag(" "),
    )(i)
}

fn parse_ordered_list_element(i: &str) -> IResult<&str, MarkdownText> {
    preceded(parse_ordered_list_tag, parse_markdown_text)(i)
}

fn parse_ordered_list(i: &str) -> IResult<&str, Vec<MarkdownText>> {
    many1(parse_ordered_list_element)(i)
}

fn parse_code_block(i: &str) -> IResult<&str, (String, &str)> {
    tuple((parse_code_block_lang, parse_code_block_body))(i)
}

fn parse_code_block_body(i: &str) -> IResult<&str, &str> {
    delimited(tag("\n"), is_not("```"), tag("```"))(i)
}

fn parse_code_block_lang(i: &str) -> IResult<&str, String> {
    alt((
        preceded(tag("```"), parse_plaintext),
        map(tag("```"), |_| "__UNKNOWN__".to_string()),
    ))(i)
}
/// Original author of this code is [Nathan Ringo](https://github.com/remexre)
/// Source: https://github.com/acmumn/mentoring/blob/master/web-client/src/view/markdown.rs
use pulldown_cmark::{Alignment, CodeBlockKind, Event, Options, Parser, Tag};
use yew::virtual_dom::{VNode, VTag, VText};
use yew::{html, Html};

/// Renders a string of Markdown to HTML with the default options (footnotes
/// disabled, tables enabled).
pub fn render_markdown(src: &str) -> Html {
    let mut elems = vec![];
    let mut spine = vec![];

    macro_rules! add_child {
        ($child:expr) => {{
            let l = spine.len();
            assert_ne!(l, 0);
            spine[l - 1].add_child($child);
        }};
    }

    for ev in Parser::new_ext(src, Options::empty()) {
        match ev {
            Event::Start(tag) => {
                spine.push(make_tag(tag));
            }
            Event::End(tag) => {
                let line_len = spine.len();
                assert!(line_len >= 1);
                let mut top = spine.pop().unwrap();
                if let Tag::CodeBlock(_) = tag {
                    let mut pre = VTag::new("pre");
                    pre.add_child(top.into());
                    top = pre;
                } else if let Tag::Table(aligns) = tag {
                    top.children_mut().into_iter().for_each(|vlist| {
                        vlist.iter_mut().for_each(|vnode| {
                            if let VNode::VTag(ref mut vtag) = vnode {
                                vtag.children_mut().into_iter().for_each(|cvlist| {
                                    cvlist.iter_mut().enumerate().for_each(|(idxnode, cvnode)| {
                                        if let VNode::VTag(ref mut cvtag) = cvnode {
                                            if let Some(class) = match aligns[idxnode] {
                                                Alignment::None => None,
                                                Alignment::Left => Some("text-left"),
                                                Alignment::Center => Some("text-center"),
                                                Alignment::Right => Some("text-right"),
                                            } {
                                                cvtag.add_attribute("class", class)
                                            }
                                        }
                                    })
                                })
                            }
                        })
                    });
                } else if let Tag::TableHead = tag {
                    top.children_mut().into_iter().for_each(|vlist| {
                        vlist.iter_mut().for_each(|vnode| {
                            if let VNode::VTag(ref mut vtag) = vnode {
                                vtag.add_attribute("scope", "col");
                            }
                        })
                    });
                }
                if line_len == 1 {
                    elems.push(top);
                } else {
                    spine[line_len - 2].add_child(top.into());
                }
            }
            Event::Text(text) => add_child!(VText::new(text.to_string()).into()),
            Event::SoftBreak => add_child!(VText::new("\n".to_string()).into()),
            Event::HardBreak => add_child!(VTag::new("br").into()),
            Event::Html(html) => add_child!(VText::new(html.to_string()).into()),
            Event::Code(code) => add_child!(VText::new(code.to_string()).into()),
            _ => panic!("Unknown event: {:#?}", ev),
        }
    }

    if elems.len() == 1 {
        VNode::VTag(Box::new(elems.pop().unwrap()))
    } else {
        html! {
            <div>{ for elems.into_iter() }</div>
        }
    }
}

#[inline]
fn add_class(vtag: &mut VTag, class: &str) {
    let mut classes = vtag
        .attributes
        .get_mut_index_map()
        .get("class")
        .map(AsRef::as_ref)
        .unwrap_or("")
        .to_owned();
    classes.push_str(class);
    vtag.add_attribute("class", class.to_owned())
}

fn make_tag(t: Tag) -> VTag {
    match t {
        Tag::Paragraph => VTag::new("p"),
        //Tag::Rule => VTag::new("hr"),
        Tag::Heading(n, _, _) => VTag::new(format!("h{}", n as u32)),
        Tag::BlockQuote => {
            let mut el = VTag::new("blockquote");
            add_class(&mut el, "blockquote");
            el
        }
        Tag::CodeBlock(kind) => {
            let mut el = VTag::new("code");
            if let CodeBlockKind::Fenced(lang) = kind {
                match lang.as_ref() {
                    "html" => add_class(&mut el, "html-language"),
                    "rust" => add_class(&mut el, "rust-language"),
                    "java" => add_class(&mut el, "java-language"),
                    "c" => add_class(&mut el, "c-language"),
                    "sh" => add_class(&mut el, "bash-language"),
                    _ => {} // Add your own language highlighting support
                };
            }
            el
        }
        Tag::List(None) => VTag::new("ul"),
        Tag::List(Some(1)) => VTag::new("ol"),
        Tag::List(Some(ref start)) => {
            let mut el = VTag::new("ol");
            el.add_attribute("start", start.to_string());
            el
        }
        Tag::Item => VTag::new("li"),
        Tag::Table(_) => {
            let mut el = VTag::new("table");
            add_class(&mut el, "table");
            el
        }
        Tag::TableHead => VTag::new("th"),
        Tag::TableRow => VTag::new("tr"),
        Tag::TableCell => VTag::new("td"),
        Tag::Emphasis => {
            let mut el = VTag::new("span");
            add_class(&mut el, "is-italic");
            el
        }
        Tag::Strong => {
            let mut el = VTag::new("span");
            add_class(&mut el, "has-text-weight-bold");
            el
        }
        Tag::Link(_type, ref href, ref title) => {
            let mut el = VTag::new("a");
            el.add_attribute("href", href.to_string());
            if title.as_ref() != "" {
                el.add_attribute("title", title.to_string());
            }
            el
        }
        Tag::Image(_type, ref src, ref title) => {
            let mut el = VTag::new("img");
            el.add_attribute("src", src.to_string());
            if title.as_ref() != "" {
                el.add_attribute("title", title.to_string());
            }
            el
        }
        Tag::Strikethrough => VTag::new("strike"),
        Tag::FootnoteDefinition(ref _footnote_id) => VTag::new("span"), // Footnotes are not rendered as anything special
    }
}
