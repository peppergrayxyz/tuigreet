use greetd_ipc::Request;

pub trait SafeDebug {
  fn safe_repr(&self) -> String;
}

impl SafeDebug for Request {
  fn safe_repr(&self) -> String {
    match self {
      msg @ (&Self::CancelSession
      | &Self::CreateSession { .. }
      | &Self::StartSession { .. }) => format!("{msg:?}"),
      &Self::PostAuthMessageResponse { .. } => {
        "PostAuthMessageResponse".to_string()
      },
    }
  }
}

macro_rules! fl {
  ($message_id:literal) => {{
    $crate::ui::MESSAGES.get($message_id).replace(&['\u{2068}', '\u{2069}'], "")
  }};

  ($message_id:literal, $($key:ident = $value:expr),*) => {{
    let mut args = std::collections::HashMap::new();
    $(args.insert(stringify!($key), $value);)*
    $crate::ui::MESSAGES.get_args($message_id, args).replace(&['\u{2068}', '\u{2069}'], "")
  }};
}
