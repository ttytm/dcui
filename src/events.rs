use std::time::Duration;

use anyhow::Result;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::{App, Pane};

impl App {
	fn handle_key(&mut self, key: KeyEvent) -> Result<()> {
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
		} else if key.modifiers.contains(KeyModifiers::SHIFT) {
			match key.code {
				KeyCode::Char('L') => self.increase(),
				KeyCode::Char('H') => self.decrease(),
				KeyCode::Char('G') => self.max(),
				_ => {}
			}
			return Ok(());
		}

		match key.code {
			// KeyCode::Char(' ') | KeyCode::Enter => self.start(),
			KeyCode::Char('1') => self.current_pane = Pane::Monitors,
			KeyCode::Char('2') => self.current_pane = Pane::Presets,
			KeyCode::Char('3') => self.current_pane = Pane::Settings,
			KeyCode::Char('q') => self.quit(),
			KeyCode::Char('l') => match self.current_pane {
				Pane::Monitors => self.current_pane = Pane::Presets,
				Pane::Presets => self.current_pane = Pane::Settings,
				_ => {}
			},
			KeyCode::Char('h') => match self.current_pane {
				Pane::Settings => self.current_pane = Pane::Presets,
				Pane::Presets => self.current_pane = Pane::Monitors,
				_ => {}
			},
			KeyCode::Right => self.increase(),
			KeyCode::Left => self.decrease(),
			KeyCode::Char('j') | KeyCode::Down => match self.current_pane {
				Pane::Monitors => self.selected_monitor.select_next(),
				Pane::Presets => self.selected_preset.select_next(),
				_ => {}
			},
			KeyCode::Char('k') | KeyCode::Up => match self.current_pane {
				Pane::Monitors => self.selected_monitor.select_previous(),
				Pane::Presets => self.selected_preset.select_previous(),
				_ => {}
			},
			_ => {}
		}

		Ok(())
	}

	pub fn handle_events(&mut self) -> Result<()> {
		let timeout = Duration::from_secs_f32(1.0 / 20.0);

		if !event::poll(timeout)? {
			return Ok(());
		}

		if let Event::Key(key) = event::read()? {
			self.handle_key(key)?;
		};

		Ok(())
	}
}
