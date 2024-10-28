use std::time::{Duration, Instant};

use anyhow::Result;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::{App, Pane};

impl App {
	fn handle_key(&mut self, key: KeyEvent, timeout: Duration) -> Result<()> {
		if key.kind != KeyEventKind::Press {
			return Ok(());
		}

		if key.modifiers.contains(KeyModifiers::CONTROL) {
			match key.code {
				KeyCode::Char('c') => self.quit(),
				KeyCode::Char('h') => self.current_pane = Pane::Monitors,
				KeyCode::Char('j') => self.current_pane = Pane::Presets,
				KeyCode::Char('k') => self.current_pane = Pane::Monitors,
				KeyCode::Char('l') => self.current_pane = Pane::Settings,
				_ => {}
			}
			return Ok(());
		}

		match key.code {
			// KeyCode::Char(' ') | KeyCode::Enter => self.start(),
			KeyCode::Char('q') => self.quit(),
			KeyCode::Char('l') | KeyCode::Tab => match self.current_pane {
				Pane::Monitors => self.current_pane = Pane::Presets,
				Pane::Presets => self.current_pane = Pane::Settings,
				Pane::Settings => self.current_pane = Pane::Monitors,
			},
			KeyCode::Char('h') | KeyCode::BackTab => match self.current_pane {
				Pane::Settings => self.current_pane = Pane::Presets,
				Pane::Presets => self.current_pane = Pane::Monitors,
				Pane::Monitors => self.current_pane = Pane::Settings,
			},
			KeyCode::Char('L') | KeyCode::Right => self.increase(),
			KeyCode::Char('H') | KeyCode::Left => self.decrease(),
			KeyCode::Char('G') => self.max(),
			KeyCode::Char('j') | KeyCode::Down => match self.current_pane {
				Pane::Monitors => self.selected_monitor.select_next(),
				// TODO: require confirm to change preset.
				Pane::Presets => self.selected_preset.select_next(),
				_ => {}
			},
			KeyCode::Char('k') | KeyCode::Up => match self.current_pane {
				Pane::Monitors => self.selected_monitor.select_previous(),
				Pane::Presets => self.selected_preset.select_previous(),
				_ => {}
			},
			KeyCode::Char('?') => self.show_help = !self.show_help,
			KeyCode::Esc => self.show_help = false,
			KeyCode::Char(digit) if digit.is_numeric() => {
				if let Some((last_digit, last_key_time)) = self.last_key.take() {
					if last_key_time.elapsed() <= timeout {
						self.set((last_digit.to_digit(10).unwrap() * 10 + digit.to_digit(10).unwrap()) as u16);
					}
				} else {
					self.last_key = Some((digit, Instant::now()));
				}
				return Ok(());
			}
			_ => {}
		}

		self.last_key = None;

		Ok(())
	}

	pub fn handle_events(&mut self) -> Result<()> {
		let timeout = Duration::from_millis(750);

		if !event::poll(timeout)? {
			if let Some((last_digit, _)) = self.last_key.take() {
				self.set(last_digit.to_digit(10).unwrap() as u16);
			}
			return Ok(());
		}

		if let Event::Key(key) = event::read()? {
			self.handle_key(key, timeout)?;
		};

		Ok(())
	}
}
