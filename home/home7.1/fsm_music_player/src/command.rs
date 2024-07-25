#[derive(Debug)]
pub enum Command {
    Sad,
    Fun,
    Silly,
    Dangerous,
}

impl Command {
    pub fn from_str(command: &str) -> Option<Command> {
        match command {
            "sad" => Some(Command::Sad),
            "fun" => Some(Command::Fun),
            "silly" => Some(Command::Silly),
            "dangerous" => Some(Command::Dangerous),
            _ => None,
        }
    }
}
