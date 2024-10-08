use ratatui::{
	layout::Alignment,
	prelude::*,
	style::{Color, Style},
	widgets::{Block, Borders, Paragraph, *},
	Frame,
};

use crate::app::App;

// BUG: panic on end of short input
impl App {
	pub fn render(&self, frame: &mut Frame) {
		let &(start_line, start_offset) = self.line_index.get(self.sample_start_index).unwrap();
		let &(cur_line, cur_offset) = self.line_index.get(self.sample_start_index + self.text.cur_char).unwrap();
		let &(end_line, end_offset) = self.line_index.get(self.sample_start_index + self.sample_len).unwrap();
		let mut lines: Vec<String> = self.book_lines.clone();
		let num_rows = frame.area().height as usize - 2; // TODO fix crash
		let rows_to_center = num_rows / 2 - 2; // TODO fix crash

		let first_row = usize::checked_sub(rows_to_center, self.display_line).unwrap_or(0);

		let num_skipped_lines = usize::checked_sub(self.display_line, rows_to_center).unwrap_or(0);
		lines = lines.split_off(usize::min(num_skipped_lines, lines.len()));
		lines.truncate(num_rows - first_row);

		let mut display_lines: Vec<Line> = Vec::new();
		for (mut i, s) in lines.iter().enumerate() {
			i += num_skipped_lines;
			match i {
				_ if i == cur_line => match (i == start_line, i == end_line) {
					(true, true) => {
						display_lines.push(Line::from(vec![
							s.chars().take(start_offset).collect::<String>().dim(),
							s.chars().take(cur_offset).skip(start_offset).collect::<String>().white(),
							s.chars().nth(cur_offset).unwrap().to_string().black().bg(Color::White),
							s.chars().take(end_offset).skip(cur_offset + 1).collect::<String>().blue(),
							s.chars().skip(end_offset).collect::<String>().dim(),
						]));
					}
					(true, false) => {
						display_lines.push(Line::from(vec![
							s.chars().take(start_offset).collect::<String>().dim(),
							s.chars().take(cur_offset).skip(start_offset).collect::<String>().white(),
							s.chars().nth(cur_offset).unwrap().to_string().black().bg(Color::White),
							s.chars().skip(cur_offset + 1).collect::<String>().blue(),
						]));
					}
					(false, true) => {
						display_lines.push(Line::from(vec![
							s.chars().take(cur_offset).collect::<String>().white(),
							s.chars().nth(cur_offset).unwrap().to_string().black().bg(Color::White),
							s.chars().take(end_offset).skip(cur_offset + 1).collect::<String>().blue(),
							s.chars().skip(end_offset).collect::<String>().dim(),
						]));
					}
					(false, false) => {
						display_lines.push(Line::from(vec![
							s.chars().take(cur_offset).collect::<String>().white(),
							s.chars().nth(cur_offset).unwrap().to_string().black().bg(Color::White),
							s.chars().skip(cur_offset + 1).collect::<String>().blue(),
						]));
					}
				},
				_ if i < cur_line => match i {
					_ if i == start_line => {
						display_lines.push(Line::from(vec![
							s.chars().take(start_offset).collect::<String>().dim(),
							s.chars().skip(start_offset).collect::<String>().white(),
						]));
					}
					_ if i < start_line => {
						display_lines.push(s.clone().dim().into());
					}
					_ => {
						display_lines.push(s.clone().white().into());
					}
				},
				_ if i == end_line => {
					display_lines.push(Line::from(vec![
						s.chars().take(end_offset).collect::<String>().blue(),
						s.chars().skip(end_offset).collect::<String>().dim(),
					]));
				}
				_ if i < end_line => {
					display_lines.push(s.clone().blue().into());
				}
				_ => {
					display_lines.push(s.clone().dim().into());
				}
			}
		}

		let graph = Paragraph::new::<Text>(display_lines.into()).style(Style::default());

		let screen = Rect::new(0, 0, frame.area().width, frame.area().height);

		let vert = Layout::default()
			.direction(Direction::Vertical)
			.constraints([Constraint::Length(first_row as u16 + 1), Constraint::Percentage(100)])
			.split(screen);
		let horiz = Layout::default()
			.direction(Direction::Horizontal)
			.constraints([
				Constraint::Percentage((100 - self.text_width_percent) / 2),
				Constraint::Percentage(self.text_width_percent),
				Constraint::Percentage((100 - self.text_width_percent) / 2),
			])
			.split(vert[1])[1];

		// Render into the second chunk of the layout.
		frame.render_widget(graph, horiz);
		frame.render_widget(
			Block::default()
				.title("Booktyping")
				.title(block::Title::from(format!("{}", self.get_rolling_average().unwrap())).alignment(Alignment::Right))
				.borders(Borders::ALL)
				.border_style(Style::new().white()),
			screen,
		);
	}
}
