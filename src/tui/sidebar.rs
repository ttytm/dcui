use ratatui::{
	buffer::Buffer,
	layout::{Constraint, Layout, Rect},
	style::Style,
	widgets::{List, StatefulWidget},
};

use crate::{utils::title_block, App, Pane};

use super::{BORDER_STYLE, SELECTED_STYLE};

impl App {
	pub fn render_sidebar(&mut self, area: Rect, buf: &mut Buffer) {
		let [monitors_area] = Layout::vertical([Constraint::Fill(1)]).areas(area);
		self.render_monitors(monitors_area, buf);
	}

	fn render_monitors(&mut self, area: Rect, buf: &mut Buffer) {
		let monitors: Vec<String> = self.monitors.iter().filter_map(|m| m.display.info.model_name.clone()).collect();
		let mut title = title_block("Monitors");
		let mut monitors = List::new(monitors);
		if self.selected.pane == Pane::Monitors {
			title = title.border_style(Style::new().fg(BORDER_STYLE));
			monitors = monitors.highlight_style(Style::new().bg(SELECTED_STYLE))
		} else {
			monitors = monitors.highlight_style(Style::new().fg(SELECTED_STYLE))
		};
		monitors = monitors.block(title);
		StatefulWidget::render(monitors, area, buf, &mut self.selected.monitor);
	}
}
