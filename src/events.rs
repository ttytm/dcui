use std::time::{Duration, Instant};

use anyhow::Result;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::{App, Pane, Setting};

impl App {
	fn handle_key(&mut self, key: KeyEvent, timeout: Duration) -> Result<()> {
		if key.kind != KeyEventKind::Press {
			return Ok(());
		}

		if key.modifiers.contains(KeyModifiers::CONTROL) {
			match key.code {
				KeyCode::Char('c') => self.quit(),
				KeyCode::Char('h') => self.selected.pane = Pane::Monitors,
				KeyCode::Char('j') => self.selected.pane = Pane::Presets,
				KeyCode::Char('k') => self.selected.pane = Pane::Monitors,
				KeyCode::Char('l') => self.selected.pane = Pane::Settings,
				_ => {}
			}
			return Ok(());
		}

		match key.code {
			KeyCode::Char(' ') | KeyCode::Enter => match self.selected.pane {
				Pane::Monitors => self.selected.pane = Pane::Settings,
				_ => {}
			},
			KeyCode::Char('q') => self.quit(),
			KeyCode::Char('d') => match self.selected.pane {
				Pane::Monitors => self.detect_monitors()?,
				_ => {}
			},
			KeyCode::Char('l') | KeyCode::Tab => match self.selected.pane {
				Pane::Monitors => self.selected.pane = Pane::Settings,
				Pane::Settings => self.selected.pane = Pane::Monitors,
				_ => {}
			},
			KeyCode::Char('h') | KeyCode::BackTab => match self.selected.pane {
				Pane::Monitors => self.selected.pane = Pane::Settings,
				Pane::Settings => self.selected.pane = Pane::Monitors,
				_ => {}
			},
			KeyCode::Char('L') | KeyCode::Right => {
				if self.selected.pane == Pane::Settings {
					match self.selected.setting {
						Setting::Brightness => self.increase_brightness(),
						Setting::Contrast => self.increase_contrast(),
					}
				}
			}
			KeyCode::Char('H') | KeyCode::Left => {
				if self.selected.pane == Pane::Settings {
					match self.selected.setting {
						Setting::Brightness => self.decrease_brightness(),
						Setting::Contrast => self.decrease_contrast(),
					}
				}
			}
			// KeyCode::Char('G') => self.set_brightness(100),
			KeyCode::Char('j') | KeyCode::Down => match self.selected.pane {
				Pane::Monitors => self.selected.monitor.select_next(),
				Pane::Presets => self.selected.preset.select_next(),
				Pane::Settings => match self.selected.setting {
					Setting::Brightness => self.selected.setting = Setting::Contrast,
					Setting::Contrast => self.selected.setting = Setting::Brightness,
				},
			},
			KeyCode::Char('k') | KeyCode::Up => match self.selected.pane {
				Pane::Monitors => self.selected.monitor.select_previous(),
				Pane::Presets => self.selected.preset.select_previous(),
				Pane::Settings => match self.selected.setting {
					Setting::Brightness => self.selected.setting = Setting::Contrast,
					Setting::Contrast => self.selected.setting = Setting::Brightness,
				},
			},
			KeyCode::Char('?') => self.show_help = !self.show_help,
			KeyCode::Esc => self.show_help = false,
			KeyCode::Char(digit) if digit.is_numeric() => {
				if let Some((last_digit, last_key_time)) = self.last_key.take() {
					if last_key_time.elapsed() <= timeout {
						let val = last_digit.to_digit(10).unwrap() * 10 + digit.to_digit(10).unwrap();
						match self.selected.setting {
							Setting::Brightness => self.set_brightness(val as u16),
							Setting::Contrast => self.set_contrast(val as u16),
						}
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
			if self.selected.pane == Pane::Settings {
				return Ok(());
			}
			if let Some((last_digit, _)) = self.last_key.take() {
				let last_digit = last_digit.to_digit(10).unwrap() as u16;
				match self.selected.setting {
					Setting::Brightness => self.set_brightness(last_digit),
					Setting::Contrast => self.set_contrast(last_digit),
				}
			}
			return Ok(());
		}

		if let Event::Key(key) = event::read()? {
			self.handle_key(key, timeout)?;
		};

		Ok(())
	}
}
