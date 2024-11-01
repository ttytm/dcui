#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(
	clippy::module_name_repetitions,
	clippy::cast_possible_truncation,
	clippy::cast_sign_loss
)]

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
	show_grayscale: bool,
	monitors: Vec<Monitor>,
	terminal_width: u16,
	selected: SelectionState,
	// Last keyboard input key. Used in an event handling context.
	// E.g. to allow directly entering a multi-digit brightness number, like `80`.
	last_key: Option<(char, Instant)>,
}

struct SelectionState {
	pane: Pane,
	monitor: ListState,
	preset: ListState,
	setting: Setting,
}

#[derive(Default, PartialEq)]
enum Pane {
	Monitors,
	Presets,
	#[default]
	Settings,
}

#[derive(Debug, Default, PartialEq)]
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

impl Default for SelectionState {
	fn default() -> Self {
		Self {
			pane: Pane::default(),
			setting: Setting::default(),
			monitor: ListState::default().with_selected(Some(0)),
			preset: ListState::default().with_selected(Some(0)),
		}
	}
}

impl App {
	fn init(terminal: &DefaultTerminal) -> Result<App> {
		let terminal_width = terminal.size()?.width;
		let mut app = App { terminal_width, ..Default::default() };
		app.detect_monitors()?;

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
