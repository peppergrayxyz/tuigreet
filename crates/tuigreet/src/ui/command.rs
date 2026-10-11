use std::error::Error;

use tui::{
  layout::{Constraint, Direction, Layout, Rect},
  text::Span,
  widgets::{Block, BorderType, Borders, Clear, Paragraph},
};

use super::common::style::Themed;
use crate::{
  Greeter,
  ui::{
    Frame,
    prompt_value,
    util::{get_cursor_offset, get_input_width, get_rect_bounds, titleize},
  },
};

pub fn draw_with_area(
  greeter: &mut Greeter,
  f: &mut Frame,
  area: Rect,
) -> Result<(u16, u16), Box<dyn Error>> {
  let theme = &greeter.theme;

  let size = area;
  let (x, y, width, height) = get_rect_bounds(greeter, size, 0);

  let container_padding = greeter.container_padding();

  let container = Rect::new(x, y, width, height);
  let frame = Rect::new(
    x + container_padding,
    y + container_padding,
    width - container_padding,
    height - container_padding,
  );

  if greeter.animation.is_some() {
    f.render_widget(Clear, container);
  }

  let block = Block::default()
    .title(titleize(&fl!("title_command")))
    .title_style(theme.of(&[Themed::Title]))
    .style(theme.of(&[Themed::Container]))
    .borders(Borders::ALL)
    .border_type(BorderType::Plain)
    .border_style(theme.of(&[Themed::Border]));

  f.render_widget(block, container);

  let constraints = [
    Constraint::Length(1), // Username
  ];

  let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints(constraints.as_ref())
    .split(frame);
  let cursor = chunks[0];

  let command_label = fl!("new_command");
  let command_label_width = u16::try_from(command_label.chars().count())?;
  let input_width = get_input_width(greeter, width, Some(&command_label));
  let command_label_text = prompt_value(theme, Some(command_label));
  let command_label =
    Paragraph::new(command_label_text).style(theme.of(&[Themed::Prompt]));
  let command_value_text = Span::from(&greeter.buffer);
  let command_value =
    Paragraph::new(command_value_text).style(theme.of(&[Themed::Input]));

  f.render_widget(command_label, chunks[0]);
  f.render_widget(
    command_value,
    Rect::new(
      1 + chunks[0].x + command_label_width,
      chunks[0].y,
      input_width,
      1,
    ),
  );

  let command_length = greeter.buffer.chars().count();
  let offset = u16::try_from(get_cursor_offset(greeter, command_length)?)?;

  Ok((2 + cursor.x + command_label_width + offset, cursor.y + 1))
}
