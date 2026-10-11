use std::borrow::Cow;

use super::common::menu::MenuItem;

#[derive(Default, Clone)]
pub struct User {
  pub username: String,
  pub name:     Option<String>,
}

impl MenuItem for User {
  fn format(&self) -> Cow<'_, str> {
    self.name.as_ref().map_or_else(
      || Cow::Borrowed(self.username.as_str()),
      |name| Cow::Owned(format!("{name} ({})", self.username)),
    )
  }
}
