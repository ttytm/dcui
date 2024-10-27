mod events;
mod footer;
mod help;
mod settings;
mod sidebar;
mod tui;
mod utils;

use anyhow::Result;
use ddc_hi::{Ddc, Display};
use ratatui::{widgets::ListState, DefaultTerminal};

#[derive(Default)]
struct App {
	should_exit: bool,
	show_help: bool,
	monitors: Vec<Monitor>,
	current_pane: Pane,
	terminal_width: u16,
	selected_monitor: ListState,
	selected_preset: ListState,
	// styles: Styles,
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

/* struct Styles {
	border_style: Style,
	border_type: BorderType,
} */

#[derive(Default, PartialEq)]
enum Pane {
	#[default]
	Monitors,
	Settings,
	Presets,
}

const BRIGHTNESS_CODE: u8 = 0x10;
const CONTRAST_CODE: u8 = 0x12;

fn main() -> Result<()> {
	let terminal = ratatui::init();
	let app_result = App::init(&terminal)?.run(terminal);
	ratatui::restore();
	app_result
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
