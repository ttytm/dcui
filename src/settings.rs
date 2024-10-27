use ratatui::{
	buffer::Buffer,
	layout::{Constraint, Layout, Margin, Rect},
	style::{Color, Style},
	widgets::{LineGauge, Widget},
};

use crate::{utils::title_block, App, Pane};

impl App {
	pub fn render_settings(&self, area: Rect, buf: &mut Buffer) {
		let mut block = title_block("Settings");
		if self.current_pane == Pane::Settings {
			block = block.border_style(Style::new().fg(Color::Magenta));
		};
		block.render(area, buf);
		let [brightness_area, contrast_area] =
			Layout::vertical([Constraint::Length(2); 2]).areas(area.inner(Margin { horizontal: 1, vertical: 1 }));
		self.render_brightness_gauge(brightness_area, buf);
		self.render_contrast_gauge(contrast_area, buf);
	}

	fn render_brightness_gauge(&self, area: Rect, buf: &mut Buffer) {
		let Some(mut selected_monitor) = self.selected_monitor.selected() else {
			return;
		};
		if selected_monitor >= self.monitors.len() {
			selected_monitor = self.monitors.len() - 1
		}
		LineGauge::default()
			.filled_style(Style::new().fg(Color::Blue))
			.unfilled_style(Style::new().fg(Color::DarkGray))
			.label(format!("Brightness: {}", self.monitors[selected_monitor].brightness))
			// .label(format!("{selected_monitor}"))
			.ratio(self.monitors[selected_monitor].brightness as f64 / 100.0)
			.render(area, buf);
	}

	fn render_contrast_gauge(&self, area: Rect, buf: &mut Buffer) {
		let Some(mut selected_monitor) = self.selected_monitor.selected() else {
			return;
		};
		if selected_monitor >= self.monitors.len() {
			selected_monitor = self.monitors.len() - 1
		}
		LineGauge::default()
			.filled_style(Style::new().fg(Color::Blue))
			.unfilled_style(Style::new().fg(Color::DarkGray))
			.label(format!("Contrast: {}", self.monitors[selected_monitor].contrast))
			// .label(format!("{selected_monitor}"))
			.ratio(self.monitors[selected_monitor].contrast as f64 / 100.0)
			.render(area, buf);
	}
}
