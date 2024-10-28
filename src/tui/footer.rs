use ratatui::{
	buffer::Buffer,
	layout::{Constraint, Layout, Rect},
	style::{Color, Stylize},
	text::Line,
	widgets::Widget,
};

pub fn render(area: Rect, buf: &mut Buffer) {
	let [left, right] = Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(area);
	// NOTE: Show keymaps related to current pane?
	Line::from(" Help [?]".fg(Color::DarkGray)).render(left, buf);
	Line::from(" Repo [https://github.com/ttytm/dcui] ".fg(Color::DarkGray))
		.right_aligned()
		.render(right, buf);
}
