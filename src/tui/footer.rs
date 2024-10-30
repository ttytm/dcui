use ratatui::{
	buffer::Buffer,
	layout::{Constraint, Layout, Rect},
	style::{Color, Stylize},
	text::Line,
	widgets::Widget,
};

pub fn render(area: Rect, buf: &mut Buffer) {
	let [left] = Layout::horizontal([Constraint::Fill(1)]).areas(area);
	// NOTE: Show keymaps related to current pane?
	Line::from(" Help [?]".fg(Color::DarkGray)).render(left, buf);
}
