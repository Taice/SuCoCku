use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum Mode {
    Command,
    Normal,
    Highlight(u8),
    Insert,
    Note,
    Go(u8),
    Custom(String),
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::Go(_) => write!(f, "go"),
            Mode::Custom(s) => write!(f, "{s}"),
            _ => write!(f, "{}", format!("{self:?}").to_lowercase()),
        }
    }
}

#[test]
fn display_mode_works() {
    assert_eq!("command".to_string(), Mode::Command.to_string());
    assert_eq!("go".to_string(), Mode::Go(4).to_string());
    assert_eq!(
        "banana".to_string(),
        Mode::Custom("banana".to_string()).to_string()
    );
}
