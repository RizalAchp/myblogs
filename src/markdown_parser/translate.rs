// use super::*;

// pub fn translate(md: Vec<Markdown>) -> yew::Html {
//     html! {
//     <div class="markdowns-parsers-class">
//         { for md.iter().map(| bit | match bit {
//             Markdown::Heading(size, line) => translate_header(*size, line.to_vec()),
//             Markdown::UnorderedList(lines) => translate_unordered_list(lines.to_vec()),
//             Markdown::OrderedList(lines) => translate_ordered_list(lines.to_vec()),
//             Markdown::Codeblock(lang, code) => {
//                 translate_codeblock(lang.to_string(), code.to_string())
//             }
//             Markdown::Line(line) => translate_line(line.to_vec()),
//         })}
//     </div>
//     }
// }

// #[inline]
// pub fn translate_boldtext(boldtext: String) -> yew::Html {
//     html! {<b>{boldtext.clone()}</b>}
// }

// #[inline]
// pub fn translate_italic(italic: String) -> yew::Html {
//     html! {<i>{italic.clone()}</i>}
// }

// #[inline]
// pub fn translate_inline_code(codestr: String) -> yew::Html {
//     html! {<code>{codestr.clone()}</code>}
// }

// #[inline]
// pub fn translate_link(text: String, url: String) -> yew::Html {
//     html! {<a href={url.clone()}>{text.clone()}</a>}
// }

// #[inline]
// pub fn translate_image(text: String, url: String) -> yew::Html {
//     html! {<img src={url.clone()} alt={text.clone()} />}
// }

// pub fn translate_list_elements(lines: Vec<MarkdownText>) -> yew::Html {
//     html! {
//     <div class="listelements">
//         { for lines.iter().map(|line| html!{
//                     <li class="listitem">
//                         { translate_text(line.to_vec())}
//                     </li>})}
//     </div>
//     }
// }

// pub fn translate_header(size: usize, text: MarkdownText) -> yew::Html {
//     match size {
//         1 => html! { <h1 class="title is-1"> {translate_text(text)} </h1> },
//         2 => html! { <h2 class="title is-2"> {translate_text(text)} </h2> },
//         3 => html! { <h3 class="title is-3"> {translate_text(text)} </h3> },
//         4 => html! { <h4 class="title is-4"> {translate_text(text)} </h4> },
//         5 => html! { <h5 class="title is-5"> {translate_text(text)} </h5> },
//         _ => html! { <h6 class="title is-6">  {translate_text(text)} </h6>},
//     }
// }

// pub fn translate_unordered_list(lines: Vec<MarkdownText>) -> yew::Html {
//     html! { <ul> { translate_list_elements(lines.to_vec()) } </ul> }
// }

// pub fn translate_ordered_list(lines: Vec<MarkdownText>) -> yew::Html {
//     html! { <ol> { translate_list_elements(lines.to_vec()) } </ol> }
// }

// // fn translate_code(code: MarkdownText) -> String {
// //     format!("<code>{}</code>", translate_text(code))
// // }

// pub fn translate_codeblock(lang: String, codestr: String) -> yew::Html {
//     html! { <pre><code class={String::from("lang-") + &lang}>{codestr.clone()}</code></pre>}
// }

// pub fn translate_line(text: MarkdownText) -> yew::Html {
//     html! {<p> {translate_text(text)} </p> }
// }

// pub fn translate_text(text: MarkdownText) -> yew::Html {
//     html! {
//     <div class="markdowns-texts">
//         { for text.iter()
//         .map(|part| match part {
//             MarkdownInline::Bold(text) => translate_boldtext(text.to_string()),
//             MarkdownInline::Italic(text) => translate_italic(text.to_string()),
//             MarkdownInline::InlineCode(code) => translate_inline_code(code.to_string()),
//             MarkdownInline::Link(text, url) => translate_link(text.to_string(), url.to_string()),
//             MarkdownInline::Image(text, url) => translate_image(text.to_string(), url.to_string()),
//             MarkdownInline::Plaintext(text) => html!{text.to_string()},
//         })}
//     </div>
//     }
// }
