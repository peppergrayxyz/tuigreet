use std::ops::Deref;

pub(in crate::integration) struct Output(pub String);

impl Deref for Output {
  type Target = String;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
