use ddc_hi::{Ddc, Display};

use crate::App;

use anyhow::Result;

pub struct Monitor {
	pub display: Display,
	pub brightness: u16,
	pub contrast: u16,
	pub brightness_columns: u16,
	pub contrast_columns: u16,
}

const BRIGHTNESS_CODE: u8 = 0x10;
const CONTRAST_CODE: u8 = 0x12;

pub fn detect(term_width: u16) -> Result<Vec<Monitor>> {
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

impl App {
	pub fn set_brightness(&mut self, amount: u16) {
		let Some(selected_monitor) = self.selected_monitor.selected() else { return };
		self.monitors[selected_monitor].brightness = amount;
		self.update_gauges();
	}

	pub fn increase_brightness(&mut self) {
		let Some(selected_monitor) = self.selected_monitor.selected() else { return };
		if self.monitors[selected_monitor].brightness == 100 {
			return;
		}
		self.monitors[selected_monitor].brightness += 1;
		self.update_gauges();
	}

	pub fn decrease_brightness(&mut self) {
		let Some(selected_monitor) = self.selected_monitor.selected() else { return };
		if self.monitors[selected_monitor].brightness == 0 {
			return;
		}
		self.monitors[selected_monitor].brightness -= 1;
		self.update_gauges();
	}

	fn update_gauges(&mut self) {
		let Some(selected_monitor) = self.selected_monitor.selected() else { return };
		let val = self.monitors[selected_monitor].brightness;
		_ = self.monitors[selected_monitor]
			.display
			.handle
			.set_vcp_feature(BRIGHTNESS_CODE, val);
		self.monitors[selected_monitor].brightness_columns =
			self.terminal_width / 100 * self.monitors[selected_monitor].brightness;
	}
}
