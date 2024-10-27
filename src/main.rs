mod events;
mod popup;
mod settings;
mod utils;

use anyhow::Result;
use ddc_hi::{Ddc, Display};
use ratatui::{
	buffer::Buffer,
	layout::{Alignment, Constraint, Flex, Layout, Rect},
	style::{Color, Style, Stylize},
	widgets::{Clear, List, ListState, Paragraph, StatefulWidget, Widget},
	DefaultTerminal,
};

use utils::title_block;

const BRIGHTNESS_CODE: u8 = 0x10;
const CONTRAST_CODE: u8 = 0x12;

fn main() -> Result<()> {
	let terminal = ratatui::init();
	let app_result = App::init(&terminal)?.run(terminal);
	ratatui::restore();
	app_result
}

#[derive(Default)]
struct App {
	should_exit: bool,
	show_help: bool,
	monitors: Vec<Monitor>,
	// presets: Vec<Monitor>,
	current_pane: Pane,
	terminal_width: u16,
	selected_monitor: ListState,
	selected_preset: ListState,
}

/* struct Monitors {
	items: Vec<Monitor>,
	state: ListState
} */

struct Monitor {
	display: Display,
	brightness: u16,
	contrast: u16,
	brightness_columns: u16,
	contrast_columns: u16,
}

#[derive(Default, PartialEq)]
enum Pane {
	#[default]
	Monitors,
	Settings,
	Presets,
}

fn get_monitors(term_width: u16) -> Result<Vec<Monitor>> {
	let displays: Vec<Display> = Display::enumerate()
		.into_iter()
		.filter_map(|mut d| d.handle.get_vcp_feature(BRIGHTNESS_CODE).ok().map(|_| d))
		.collect();

	let mut monitors = Vec::with_capacity(displays.len());
	for mut d in displays {
		let brightness = d.handle.get_vcp_feature(BRIGHTNESS_CODE)?.value();
		let contrast = d.handle.get_vcp_feature(CONTRAST_CODE)?.value();
		let brightness_columns = term_width / 100 * brightness;
		let contrast_columns = term_width / 100 * contrast;
		monitors.push(Monitor {
			display: d,
			brightness,
			contrast,
			brightness_columns,
			contrast_columns,
		})
	}
	Ok(monitors)
}

// TODO: help popup with keymaps.
// TODO: footer.
// TODO: line gauges.
// TODO: directly enter a number, but then use longer timeout.
impl App {
	fn init(terminal: &DefaultTerminal) -> Result<App> {
		let mut app = App {
			monitors: get_monitors(terminal.size()?.width)?,
			..Default::default()
		};
		// TODO: require confirm to change preset.
		app.selected_preset.select(Some(0));
		app.selected_monitor.select(Some(0));
		Ok(app)
	}

	fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
		loop {
			self.terminal_width = terminal.size()?.width;
			self.handle_events()?;
			if self.should_exit {
				return Ok(());
			}
			terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
		}
	}

	fn increase(&mut self) {
		let Some(selected_monitor) = self.selected_monitor.selected() else { return };
		if self.monitors[selected_monitor].brightness == 100 {
			return;
		}
		self.monitors[selected_monitor].brightness += 1;
		self.update();
	}

	fn max(&mut self) {
		let Some(selected_monitor) = self.selected_monitor.selected() else { return };
		self.monitors[selected_monitor].brightness = 100;
		self.update();
	}

	fn decrease(&mut self) {
		let Some(selected_monitor) = self.selected_monitor.selected() else { return };
		if self.monitors[selected_monitor].brightness == 0 {
			return;
		}
		self.monitors[selected_monitor].brightness -= 1;
		self.update();
	}

	fn update(&mut self) {
		let Some(selected_monitor) = self.selected_monitor.selected() else { return };
		let val = self.monitors[selected_monitor].brightness;
		_ = self.monitors[selected_monitor]
			.display
			.handle
			.set_vcp_feature(BRIGHTNESS_CODE, val);
		self.monitors[selected_monitor].brightness_columns =
			self.terminal_width / 100 * self.monitors[selected_monitor].brightness;
	}

	fn quit(&mut self) {
		self.should_exit = true;
	}
}

impl Widget for &mut App {
	fn render(self, area: Rect, buf: &mut Buffer) {
		let [main_area, footer_area] = Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).areas(area);
		let [left, right] = Layout::horizontal([Constraint::Length(35), Constraint::Fill(1)]).areas(main_area);
		self.render_sidebar(left, buf);
		self.render_settings(right, buf);
		self.render_help(main_area, buf);
		footer().render(footer_area, buf);
	}
}

fn footer() -> impl Widget {
	Paragraph::new("[ Repo](https://github.com/ttytm/dcui)  ")
		.alignment(Alignment::Right)
		.fg(Color::DarkGray)
}

fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
	let [area] = Layout::horizontal([horizontal]).flex(Flex::Center).areas(area);
	let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
	area
}

impl App {
	fn render_help(&mut self, area: Rect, buf: &mut Buffer) {
		if !self.show_help {
			return;
		}
		let area = center(area, Constraint::Percentage(30), Constraint::Length(3));
		Clear.render(area, buf);
		let popup = title_block("Help");
		Widget::render(popup, area, buf);
	}

	fn render_sidebar(&mut self, area: Rect, buf: &mut Buffer) {
		let constraints = if self.current_pane == Pane::Monitors {
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
		if self.current_pane == Pane::Monitors {
			title = title.border_style(Style::new().fg(Color::Magenta));
			monitors = monitors.highlight_style(Style::new().bg(Color::Blue))
		} else {
			monitors = monitors.highlight_style(Style::new().fg(Color::Magenta))
		};
		monitors = monitors.block(title);
		StatefulWidget::render(monitors, area, buf, &mut self.selected_monitor);
	}

	fn render_presets(&mut self, area: Rect, buf: &mut Buffer) {
		// TODO:
		let items = ["None".italic(), "Day".into(), "Evening".into(), "Night".into()];
		let mut title = title_block("Presets");
		let mut presets = List::new(items);
		if self.current_pane == Pane::Presets {
			title = title.border_style(Style::new().fg(Color::Magenta));
			presets = presets.highlight_style(Style::new().bg(Color::Blue))
		} else {
			presets = presets.highlight_style(Style::new().fg(Color::Magenta))
		};
		presets = presets.block(title);
		StatefulWidget::render(presets, area, buf, &mut self.selected_preset);
	}
}
