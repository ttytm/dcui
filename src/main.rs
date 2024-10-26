use std::time::Duration;

use anyhow::Result;
use ddc_hi::{Ddc, Display};
use ratatui::{
	buffer::Buffer,
	crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
	layout::{Alignment, Constraint, Layout, Margin, Rect},
	style::{Color, Style, Stylize},
	text::{Line, Span},
	widgets::{Block, BorderType, LineGauge, List, ListState, Paragraph, Widget},
	DefaultTerminal,
};
use utils::style_number;

mod utils;

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
	monitors: Vec<Monitor>,
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

// TODO: abstract frame
// TODO: help popup with keymaps
// TODO: footer
// TODO: line gauges
// TODO: directly enter a number, but then use longer timeout
impl App {
	fn init(terminal: &DefaultTerminal) -> Result<App> {
		let mut app = App {
			monitors: get_monitors(terminal.size()?.width)?,
			..Default::default()
		};
		// TODO: require confirm, for now use differnt color
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

			terminal.draw(|frame| {
				let [main_area, footer_area] =
					Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).areas(frame.area());

				frame.render_widget(footer(), footer_area);

				let [left, right] = Layout::horizontal([Constraint::Length(35), Constraint::Fill(1)]).areas(main_area);

				let mut block = title_block("Settings", style_number(3, false));
				if self.current_pane == Pane::Settings {
					block = block.border_style(Style::new().fg(Color::Magenta));
				};
				frame.render_widget(Block::new(), left);
				frame.render_widget(block, right);
				frame.render_widget(&self, right);

				let constraints = if self.current_pane == Pane::Monitors {
					[Constraint::Percentage(62), Constraint::Fill(1)]
				} else {
					[Constraint::Fill(1), Constraint::Fill(1)]
				};
				let [list_area, preset_area] = Layout::vertical(constraints).areas(left);

				// This should be stored outside of the function in your application state.
				let monitors: Vec<String> =
					self.monitors.iter().filter_map(|m| m.display.info.model_name.clone()).collect();

				let mut title = title_block("Monitors", style_number(1, false));
				let mut monitors = List::new(monitors);

				if self.current_pane == Pane::Monitors {
					title = title.border_style(Style::new().fg(Color::Magenta));
					monitors = monitors.highlight_style(Style::new().bg(Color::Blue))
				} else {
					monitors = monitors.highlight_style(Style::new().fg(Color::Magenta))
				};
				monitors = monitors.block(title);
				frame.render_stateful_widget(monitors, list_area, &mut self.selected_monitor);

				let items = ["None".italic(), "Day".into(), "Evening".into(), "Night".into()];
				// self.selected_preset.select_next();
				let mut title = title_block("Presets", style_number(2, false));
				let mut presets = List::new(items);
				if self.current_pane == Pane::Presets {
					title = title.border_style(Style::new().fg(Color::Magenta));
					presets = presets.highlight_style(Style::new().bg(Color::Blue))
				} else {
					presets = presets.highlight_style(Style::new().fg(Color::Magenta))
				};
				presets = presets.block(title);
				frame.render_stateful_widget(presets, preset_area, &mut self.selected_preset);
			})?;
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

	fn handle_events(&mut self) -> Result<()> {
		let timeout = Duration::from_secs_f32(1.0 / 20.0);

		if !event::poll(timeout)? {
			return Ok(());
		}

		if let Event::Key(key) = event::read()? {
			self.handle_key(key)?;
		};

		Ok(())
	}

	fn quit(&mut self) {
		self.should_exit = true;
	}
}

impl Widget for &App {
	fn render(self, area: Rect, buf: &mut Buffer) {
		let [brightness_area, contrast_area] =
			Layout::vertical([Constraint::Length(2); 2]).areas(area.inner(Margin { horizontal: 1, vertical: 1 }));
		self.render_brightness_gauge(brightness_area, buf);
		self.render_contrast_gauge(contrast_area, buf);
	}
}

/* fn header() -> impl Widget {
	Paragraph::new("Ratatui Line Gauge Example")
		.bold()
		.alignment(Alignment::Center)
		.fg(CUSTOM_LABEL_COLOR)
} */

fn footer() -> impl Widget {
	Paragraph::new("Press ENTER / SPACE to start")
		.alignment(Alignment::Right)
		.fg(Color::DarkGray)
		.bold()
}

impl App {
	/* fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
		let block = Block::new()
			.title(Line::raw("TODO List").centered())
			.borders(Borders::TOP)
			.border_style(TODO_HEADER_STYLE)

		// Iterate through all elements in the `items` and stylize them.
		let items: Vec<ListItem> = self
			.todo_list
			.items
			.iter()
			.enumerate()
			.map(|(i, todo_item)| {
				let color = alternate_colors(i);
				ListItem::from(todo_item).bg(color)
			})
			.collect();

		// Create a List from all list items and highlight the currently selected one
		let list = List::new(items) .block(block)

		// We need to disambiguate this trait method as both `Widget` and `StatefulWidget` share the
		// same method name `render`.
		StatefulWidget::render(list, area, buf, &mut self.todo_list.state);
	} */

	fn render_brightness_gauge(&self, area: Rect, buf: &mut Buffer) {
		let Some(mut selected_monitor) = self.selected_monitor.selected() else {
			return;
		};
		if selected_monitor >= self.monitors.len() {
			selected_monitor = self.monitors.len() - 1
		}
		LineGauge::default()
			.filled_style(Style::new().fg(Color::Blue))
			.unfilled_style(Style::new().fg(Color::DarkGray))
			.label(format!("Brightness: {}", self.monitors[selected_monitor].brightness))
			// .label(format!("{selected_monitor}"))
			.ratio(self.monitors[selected_monitor].brightness as f64 / 100.0)
			.render(area, buf);
	}

	fn render_contrast_gauge(&self, area: Rect, buf: &mut Buffer) {
		let Some(mut selected_monitor) = self.selected_monitor.selected() else {
			return;
		};
		if selected_monitor >= self.monitors.len() {
			selected_monitor = self.monitors.len() - 1
		}
		LineGauge::default()
			.filled_style(Style::new().fg(Color::Blue))
			.unfilled_style(Style::new().fg(Color::DarkGray))
			.label(format!("Contrast: {}", self.monitors[selected_monitor].contrast))
			// .label(format!("{selected_monitor}"))
			.ratio(self.monitors[selected_monitor].contrast as f64 / 100.0)
			.render(area, buf);
	}
}

fn title_block(title: &str, key_map: String) -> Block {
	Block::bordered()
		.border_type(BorderType::Rounded)
		.border_style(Style::default().fg(Color::DarkGray))
		.title(Line::from(vec![key_map.into(), Span::styled(title, Style::default().white())]))
}
