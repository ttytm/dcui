use ratatui::{
	buffer::Buffer,
	layout::{Constraint, Flex, Layout, Margin, Rect},
	style::{Color, Style, Stylize},
	symbols::{self},
	widgets::{Block, Borders, LineGauge, Widget},
};

use crate::{utils::title_block, App, Pane, Setting};

use super::{BORDER_STYLE, SELECTED_STYLE};

impl App {
	pub fn render_settings(&self, area: Rect, buf: &mut Buffer) {
		let mut block = title_block("Settings");

		if self.selected.pane == Pane::Settings {
			block = block.border_style(Style::new().fg(BORDER_STYLE));
		};
		let [brightness_area, contrast_area] =
			Layout::vertical([Constraint::Length(2); 2]).areas(area.inner(Margin { horizontal: 1, vertical: 1 }));

		block.render(area, buf);
		self.render_brightness_gauge(brightness_area, buf);
		self.render_contrast_gauge(contrast_area, buf);
		if self.show_grayscale {
			self.render_grayscale(area, buf);
		}
	}

	fn render_grayscale(&self, area: Rect, buf: &mut Buffer) {
		let [area] = Layout::vertical([Constraint::Percentage(50)]).flex(Flex::End).areas(area);
		let set = symbols::border::Set {
			horizontal_top: symbols::line::HORIZONTAL,
			top_left: symbols::line::VERTICAL_RIGHT,
			top_right: symbols::line::VERTICAL_LEFT,
			..symbols::border::ROUNDED
		};
		let mut block = Block::bordered().border_set(set);
		if self.selected.pane == Pane::Settings {
			block = block.border_style(Style::new().fg(BORDER_STYLE));
		};
		block.render(area, buf);

		let total = 255;
		let levels = match self.terminal_width {
			..100 => 10,
			100..150 => 15,
			_ => 21,
		};
		// ^= `total / levels * i` but with integer ceiling.
		let values: Vec<_> = (0..=levels).map(|i| (total * i + levels - 1) / levels).collect();
		let layout = Layout::horizontal(vec![Constraint::Ratio(1, levels as u32); levels + 1])
			.split(area.inner(Margin { horizontal: 6, vertical: 3 }))
			.to_vec();

		for (val, area) in values.into_iter().zip(layout) {
			let v = val as u8;
			let color = Color::Rgb(v, v, v);
			let block = Block::new()
				.borders(Borders::BOTTOM)
				.border_set(symbols::border::EMPTY)
				.border_style(Style::new().bg(Color::Rgb(0, 0, 0)))
				.bg(color)
				.title_bottom(val.to_string().gray().into_centered_line());
			Widget::render(block, area, buf);
		}
	}

	fn render_brightness_gauge(&self, area: Rect, buf: &mut Buffer) {
		let Some(mut monitor) = self.selected.monitor.selected() else {
			return;
		};
		if monitor >= self.monitors.len() {
			monitor = self.monitors.len() - 1;
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
			monitor = self.monitors.len() - 1;
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
	let set = symbols::line::Set {
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
		.label(format!("{ratio:>3}%"))
		.ratio(f64::from(ratio) / 100.0)
		.render(area, buf);
}
