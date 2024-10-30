use ratatui::{
	buffer::Buffer,
	layout::{Constraint, Layout, Rect},
	style::{Color, Style, Stylize},
	symbols::line::THICK_VERTICAL,
	widgets::{List, StatefulWidget},
};

use crate::{utils::title_block, App, Pane};

impl App {
	pub fn render_sidebar(&mut self, area: Rect, buf: &mut Buffer) {
		let constraints = if self.selected.pane == Pane::Monitors {
			[Constraint::Percentage(62), Constraint::Fill(1)]
		} else {
			[Constraint::Fill(1), Constraint::Fill(1)]
		};
		let [monitors_area, preset_area] = Layout::vertical(constraints).areas(area);
		self.render_monitors(monitors_area, buf);
		self.render_presets(preset_area, buf);
	}

	fn render_monitors(&mut self, area: Rect, buf: &mut Buffer) {
		let monitors: Vec<String> = self.monitors.iter().filter_map(|m| m.display.info.model_name.clone()).collect();
		let mut title = title_block("Monitors");
		let mut monitors = List::new(monitors);
		if self.selected.pane == Pane::Monitors {
			title = title.border_style(Style::new().fg(Color::Magenta));
			monitors = monitors.highlight_style(Style::new().bg(Color::Blue))
		} else {
			monitors = monitors.highlight_style(Style::new().fg(Color::Magenta))
		};
		monitors = monitors.block(title);
		StatefulWidget::render(monitors, area, buf, &mut self.selected.monitor);
	}

	fn render_presets(&mut self, area: Rect, buf: &mut Buffer) {
		// TODO:
		let items = ["None".italic(), "Day".into(), "Evening".into(), "Night".into()];
		let mut title = title_block("Presets");
		let mut presets = List::new(items);
		if self.selected.pane == Pane::Presets {
			title = title.border_style(Style::new().fg(Color::Magenta));
			presets = presets.highlight_style(Style::new().bg(Color::Blue))
		} else {
			presets = presets.highlight_style(Style::new().fg(Color::Magenta))
		};
		presets = presets.block(title);
		StatefulWidget::render(presets, area, buf, &mut self.selected.preset);
	}
}
