mod events;
mod monitors;
mod tui;
mod utils;

use std::time::Instant;

use anyhow::Result;
use monitors::Monitor;
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
	selected_setting: ListState,
	// Last keyboard input key. E.g. to allow directly entering a multi-digit brightness number, like `80`.
	last_key: Option<(char, Instant)>,
	// TODO:
	// styles: Styles,
}

/* struct Styles {
	border_style: Style,
	border_type: BorderType,
} */

#[derive(Default, PartialEq)]
enum Pane {
	Monitors,
	Presets,
	#[default]
	Settings,
}

#[derive(Default, PartialEq)]
enum Setting {
	#[default]
	Brightness,
	Contrast,
}

fn main() -> Result<()> {
	let terminal = ratatui::init();
	let app_result = App::init(&terminal)?.run(terminal);
	ratatui::restore();
	app_result
}

// TODO: line gauge styles.
// TODO: setting selection.
impl App {
	fn init(terminal: &DefaultTerminal) -> Result<App> {
		let mut app = App {
			monitors: monitors::detect(terminal.size()?.width)?,
			..Default::default()
		};
		// TODO: remember state.
		// app.selected_preset.select(Some(0));
		app.selected_monitor.select(Some(0));
		app.selected_setting.select(Some(0));

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

	fn quit(&mut self) {
		self.should_exit = true;
	}
}
