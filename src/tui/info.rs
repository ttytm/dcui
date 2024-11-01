use ratatui::{
	buffer::Buffer,
	layout::{Constraint, Layout, Margin, Rect},
	style::Stylize,
	text::Line,
	widgets::{Clear, Row, Table, Widget},
};

use crate::{
	utils::{center, title_block},
	App,
};

impl App {
	pub fn render_help(&mut self, area: Rect, buf: &mut Buffer) {
		let widths = [Constraint::Fill(1), Constraint::Fill(2)];
		let info_data = [
			("Version", env!("CARGO_PKG_VERSION")),
			("Repository", "https://github.com/ttytm/dcui"),
		];
		let info_rows: Vec<Row> = info_data
			.into_iter()
			.map(|(k, v)| Row::new([Line::from(k), Line::from(v).right_aligned()]))
			.collect();
		let info_table = Table::new(info_rows, widths).header(Row::new(vec!["General".bold()]));

		let keys_data = [
			("Toggle Info", "?"),
			("Quit", "q"),
			("Next Pane", "<tab>, l"),
			("Previous Pane", "<S-tab>, h"),
			("Down", "down, j"),
			("Up", "up, k"),
			("Increase", "left, <S-h>"),
			("Decrease", "right, <S-l>"),
			("Toggle Grayscale", "<S-s>"),
		];
		let keys_rows = keys_data
			.into_iter()
			.map(|(desc, key)| Row::new([Line::from(desc), Line::from(key).right_aligned()]))
			.collect::<Vec<Row>>();
		let keys_table = Table::new(keys_rows, widths).header(Row::new(vec!["Keys".bold()]));

		let area = center(
			area,
			Constraint::Length(55),
			Constraint::Length((info_data.len() + keys_data.len() + 5) as u16),
		);
		let [info_area, keys_area] =
			Layout::vertical([Constraint::Length((info_data.len() + 2) as u16), Constraint::Fill(1)])
				.areas(area.inner(Margin { horizontal: 1, vertical: 1 }));

		Clear.render(area, buf);
		title_block("Info").render(area, buf);
		info_table.render(info_area, buf);
		keys_table.render(keys_area, buf);
	}
}
