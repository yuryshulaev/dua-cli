use tui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Paragraph, Widget},
};

pub struct Header;

impl Header {
    pub fn render(&self, bg_color: Color, area: Rect, buf: &mut Buffer) {
        let standard = Style {
            fg: bg_color.into(),
            bg: Color::Reset.into(),
            add_modifier: Modifier::REVERSED,
            ..Default::default()
        };
        let text = |text: &'static str| Span::styled(text, standard);

        let spans = vec![
            text(" Disk Usage Analyzer v"),
            text(env!("CARGO_PKG_VERSION")),
            text("    (press ? for help)"),
        ];
        Paragraph::new(Text::from(Spans::from(spans)))
            .style(Style {
                fg: bg_color.into(),
                add_modifier: Modifier::REVERSED,
                ..Default::default()
            })
            .render(area, buf);
    }
}
