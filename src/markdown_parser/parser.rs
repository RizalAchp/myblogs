use pulldown_cmark::{Alignment, CodeBlockKind, Event, Options, Parser, Tag};
use yew::virtual_dom::{VNode, VTag, VText};
use yew::{html, Html};

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
            Event::Start(tag) => spine.push(make_tag(tag)),
            Event::End(tag) => {
                let line_len = spine.len();
                let mut top = spine.pop().unwrap();
                if let Tag::CodeBlock(codetag) = tag {
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
                                            let class = match aligns.get(idxnode) {
                                                Some(Alignment::None) => "text-left",
                                                Some(Alignment::Left) => "text-left",
                                                Some(Alignment::Center) => "text-center",
                                                Some(Alignment::Right) => "text-right",
                                                _ => "text-left",
                                            };
                                            cvtag.add_attribute("class", class)
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
            _ => continue,
        }
    }

    if elems.len() == 1 {
        VNode::VTag(Box::new(elems.pop().unwrap()))
    } else {
        html!(<div>{ for elems.into_iter() }</div>)
    }
}

fn make_tag(t: Tag) -> VTag {
    match t {
        Tag::Paragraph => VTag::new("p"),
        //Tag::Rule => VTag::new("hr"),
        Tag::Heading(n, _, _) => VTag::new(format!("h{}", n as u32)),
        Tag::BlockQuote => {
            let mut el = VTag::new("blockquote");
            el.add_attribute("class", "blockquote");
            el
        }
        Tag::CodeBlock(kind) => {
            let mut el = VTag::new("code");
            if let CodeBlockKind::Fenced(lang) = kind {
                match lang.as_ref() {
                    "html" => el.add_attribute("class", "html-language"),
                    "rust" | "rs" => el.add_attribute("class", "rust-language"),
                    "python" | "py" => el.add_attribute("class", "python-language"),
                    "c" | "c++" => el.add_attribute("class", "c-language"),
                    "sh" | "bash" | "shell" => el.add_attribute("class", "bash-language"),
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
            el.add_attribute("class", "table");
            el
        }
        Tag::TableHead => VTag::new("th"),
        Tag::TableRow => VTag::new("tr"),
        Tag::TableCell => VTag::new("td"),
        Tag::Emphasis => {
            let mut el = VTag::new("span");
            el.add_attribute("class", "is-italic");
            el
        }
        Tag::Strong => {
            let mut el = VTag::new("span");
            el.add_attribute("class", "has-text-weight-bold");
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
