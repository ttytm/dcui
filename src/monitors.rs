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

impl App {
	// TODO: processing indicator.
	pub fn detect_monitors(&mut self) -> Result<()> {
		let displays: Vec<Display> = Display::enumerate()
			.into_iter()
			.filter_map(|mut d| d.handle.get_vcp_feature(BRIGHTNESS_CODE).ok().map(|_| d))
			.collect();

		let mut monitors = Vec::with_capacity(displays.len());
		for mut d in displays {
			let brightness = d.handle.get_vcp_feature(BRIGHTNESS_CODE)?.value();
			let contrast = d.handle.get_vcp_feature(CONTRAST_CODE)?.value();
			let brightness_columns = self.terminal_width / 100 * brightness;
			let contrast_columns = self.terminal_width / 100 * contrast;
			monitors.push(Monitor {
				display: d,
				brightness,
				contrast,
				brightness_columns,
				contrast_columns,
			})
		}

		self.monitors = monitors;
		Ok(())
	}

	// TODO: refactor reduce redundancy.
	// == Brightness =============================================================
	pub fn set_brightness(&mut self, amount: u16) {
		let Some(selected_monitor) = self.selected.monitor.selected() else { return };
		self.monitors[selected_monitor].brightness = amount;
		self.update_brightness_gauge();
	}

	pub fn increase_brightness(&mut self) {
		let Some(selected_monitor) = self.selected.monitor.selected() else { return };
		if self.monitors[selected_monitor].brightness == 100 {
			return;
		}
		self.monitors[selected_monitor].brightness += 1;
		self.update_brightness_gauge();
	}

	pub fn decrease_brightness(&mut self) {
		let Some(selected_monitor) = self.selected.monitor.selected() else { return };
		if self.monitors[selected_monitor].brightness == 0 {
			return;
		}
		self.monitors[selected_monitor].brightness -= 1;
		self.update_brightness_gauge();
	}

	fn update_brightness_gauge(&mut self) {
		let Some(selected_monitor) = self.selected.monitor.selected() else { return };
		let val = self.monitors[selected_monitor].brightness;
		self.monitors[selected_monitor]
			.display
			.handle
			.set_vcp_feature(BRIGHTNESS_CODE, val)
			.unwrap();
		self.monitors[selected_monitor].brightness_columns =
			self.terminal_width / 100 * self.monitors[selected_monitor].brightness;
	}

	// == Contrast ===============================================================
	pub fn set_contrast(&mut self, amount: u16) {
		let Some(selected_monitor) = self.selected.monitor.selected() else { return };
		self.monitors[selected_monitor].contrast = amount;
		self.update_contrast_gauge();
	}

	pub fn increase_contrast(&mut self) {
		let Some(selected_monitor) = self.selected.monitor.selected() else { return };
		if self.monitors[selected_monitor].contrast == 100 {
			return;
		}
		self.monitors[selected_monitor].contrast += 1;
		self.update_contrast_gauge();
	}

	pub fn decrease_contrast(&mut self) {
		let Some(selected_monitor) = self.selected.monitor.selected() else { return };
		if self.monitors[selected_monitor].contrast == 0 {
			return;
		}
		self.monitors[selected_monitor].contrast -= 1;
		self.update_contrast_gauge();
	}

	fn update_contrast_gauge(&mut self) {
		let Some(selected_monitor) = self.selected.monitor.selected() else { return };
		let val = self.monitors[selected_monitor].contrast;
		self.monitors[selected_monitor]
			.display
			.handle
			.set_vcp_feature(CONTRAST_CODE, val)
			.unwrap();
		self.monitors[selected_monitor].contrast_columns =
			self.terminal_width / 100 * self.monitors[selected_monitor].contrast;
	}
}
