use ratatui::{
	style::{Color, Style, Stylize},
	widgets::{Block, BorderType},
};

/* pub fn style_number(mut num: i32, sub: bool) -> String {
	const SUPERSCRIPT_DIGITS: [char; 10] = ['⁰', '¹', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹'];
	const SUBSCRIPT_DIGITS: [char; 10] = ['₀', '₁', '₂', '₃', '₄', '₅', '₆', '₇', '₈', '₉'];

	let digits = if sub { SUBSCRIPT_DIGITS } else { SUPERSCRIPT_DIGITS };

	let mut result = String::new();

	if num == 0 {
		result.push(digits[0]);
		return result;
	}

	if num < 0 {
		num = -num;
		result.push(if sub { '₋' } else { '⁻' });
	}

	let mut started = false;
	let mut power_of_ten = 1_000_000_000;
	for _ in 0..10 {
		let digit = num / power_of_ten;
		num -= digit * power_of_ten;
		power_of_ten /= 10;
		if digit != 0 || started {
			started = true;
			result.push(digits[digit as usize])
		}
	}

	result
} */

pub fn title_block(title: &str) -> Block {
	Block::bordered()
		.border_type(BorderType::Rounded)
		.border_style(Style::default().fg(Color::DarkGray))
		.title(title.white())
}
