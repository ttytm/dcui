use ratatui::{
	buffer::Buffer,
	layout::{Constraint, Layout, Margin, Rect},
	style::{Color, Style, Stylize},
	symbols::{self, line::Set},
	widgets::{Block, LineGauge, Widget},
};

use crate::{utils::title_block, App, Pane, Setting};

use super::{BORDER_STYLE, SELECTED_STYLE};

impl App {
	pub fn render_settings(&self, area: Rect, buf: &mut Buffer) {
		let mut block = title_block("Settings");
		if self.selected.pane == Pane::Settings {
			block = block.border_style(Style::new().fg(BORDER_STYLE));
		};
		block.render(area, buf);
		let [brightness_area, contrast_area] =
			Layout::vertical([Constraint::Length(2); 2]).areas(area.inner(Margin { horizontal: 1, vertical: 1 }));
		self.render_brightness_gauge(brightness_area, buf);
		self.render_contrast_gauge(contrast_area, buf);
	}

	fn render_brightness_gauge(&self, area: Rect, buf: &mut Buffer) {
		let Some(mut monitor) = self.selected.monitor.selected() else {
			return;
		};
		if monitor >= self.monitors.len() {
			monitor = self.monitors.len() - 1
		}
		render_gauge(
			"Brightness",
			self.monitors[monitor].brightness,
			self.selected.setting == Setting::Brightness,
			area,
			buf,
		);
	}

	fn render_contrast_gauge(&self, area: Rect, buf: &mut Buffer) {
		let Some(mut monitor) = self.selected.monitor.selected() else {
			return;
		};
		if monitor >= self.monitors.len() {
			monitor = self.monitors.len() - 1
		}
		render_gauge(
			"Contrast",
			self.monitors[monitor].contrast,
			self.selected.setting == Setting::Contrast,
			area,
			buf,
		);
	}
}

fn render_gauge(title: &str, ratio: u16, is_selected: bool, area: Rect, buf: &mut Buffer) {
	let set = Set {
		// horizontal: "•",
		horizontal: "■",
		..symbols::line::NORMAL
	};
	LineGauge::default()
		.filled_style(Style::new().fg(if is_selected { SELECTED_STYLE } else { Color::Reset }))
		.unfilled_style(Style::new().fg(Color::DarkGray))
		.line_set(set)
		.block(Block::default().title(title.magenta().style(Style::new().fg(if is_selected {
			SELECTED_STYLE
		} else {
			Color::Reset
		}))))
		.label(format!("{:>3}%", ratio))
		.ratio(ratio as f64 / 100.0)
		.render(area, buf);
}
