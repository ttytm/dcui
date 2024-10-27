mod footer;
mod help;
mod settings;
mod sidebar;

use ratatui::{
	buffer::Buffer,
	layout::{Constraint, Layout, Rect},
	widgets::Widget,
};

use crate::App;

impl Widget for &mut App {
	fn render(self, area: Rect, buf: &mut Buffer) {
		let [main_area, footer_area] = Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).areas(area);
		let [left, right] = Layout::horizontal([Constraint::Length(35), Constraint::Fill(1)]).areas(main_area);

		self.render_sidebar(left, buf);
		self.render_settings(right, buf);
		footer::render(footer_area, buf);
		if self.show_help {
			self.render_help(main_area, buf);
		}
	}
}
