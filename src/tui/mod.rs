mod footer;
mod info;
mod settings;
mod sidebar;

use ratatui::{
	buffer::Buffer,
	layout::{Constraint, Layout, Rect},
	style::{Color, Stylize},
	widgets::{Block, Widget},
};

use crate::App;

const BORDER_STYLE: Color = Color::Blue;
const SELECTED_STYLE: Color = Color::Blue;

impl Widget for &mut App {
	fn render(self, area: Rect, buf: &mut Buffer) {
		let [main_area, footer_area] = Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).areas(area);
		let [left, right] = Layout::horizontal([Constraint::Length(35), Constraint::Fill(1)]).areas(main_area);

		if self.show_grayscale {
			// Use a black background to enhance contrast and enable more reliable grayscale calibration.
			let block = Block::new().bg(Color::Rgb(0, 0, 0));
			block.render(area, buf);
		}
		self.render_sidebar(left, buf);
		self.render_settings(right, buf);
		footer::render(footer_area, buf);
		if self.show_help {
			self.render_help(main_area, buf);
		}
	}
}
