use pulldown_cmark::{Event, Options, Parser, html};

pub fn render(content: &str) -> String {
    let parser = Parser::new_ext(content, Options::all());
    let events = parser.map(|event| match event {
        Event::Html(t) | Event::InlineHtml(t) => Event::Text(t),
        e => e,
    });
    let mut out = String::new();
    html::push_html(&mut out, events);
    out
}
