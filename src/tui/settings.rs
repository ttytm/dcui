use ratatui::{
	buffer::Buffer,
	layout::{Constraint, Layout, Margin, Rect},
	style::{Color, Style},
	symbols::{self, line::Set},
	widgets::{Block, LineGauge, Widget},
};

use crate::{utils::title_block, App, Pane, Setting};

impl App {
	pub fn render_settings(&self, area: Rect, buf: &mut Buffer) {
		let mut block = title_block("Settings");
		if self.selected.pane == Pane::Settings {
			block = block.border_style(Style::new().fg(Color::Magenta));
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
		/* LineGauge::default()
		.filled_style(Style::new().fg(Color::Blue))
		.unfilled_style(Style::new().fg(Color::DarkGray).bg(Color::Black))
		.block(Block::default().title("Brightness"))
		.label(format!("{}%", self.monitors[selected.monitor].brightness))
		// .label(format!("{selected.monitor}"))
		.ratio(self.monitors[selected.monitor].brightness as f64 / 100.0)
		.render(area, buf); */
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
		// LineGauge::default()
		// 	.filled_style(Style::new().fg(Color::Blue))
		// 	.unfilled_style(Style::new().fg(Color::DarkGray).bg(Color::Black))
		// 	.block(Block::default().title("Contrast"))
		// 	.label(format!(" {}%", self.monitors[selected.monitor].contrast))
		// 	// .label(format!("{selected.monitor}"))
		// 	.ratio(self.monitors[selected.monitor].contrast as f64 / 100.0)
		// 	.render(area, buf);
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
		horizontal: "•",
		..symbols::line::NORMAL
	};
	let bg_color = if is_selected { Color::Blue } else { Color::Reset };
	LineGauge::default()
		.filled_style(
			Style::new()
				.fg(if is_selected { Color::Gray } else { Color::Reset })
				.bg(bg_color),
		)
		.unfilled_style(Style::new().fg(Color::Black).bg(bg_color))
		.line_set(set)
		.block(Block::default().title(title))
		.label(format!(" {}%", ratio))
		// .label(format!("{selected.monitor}"))
		.ratio(ratio as f64 / 100.0)
		.render(area, buf);
}
