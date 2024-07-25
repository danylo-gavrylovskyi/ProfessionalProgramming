use crate::command::Command;

#[derive(Debug, Clone)]
pub enum Song {
    Intro,
    Starman,
    ShowMustGoOn,
    LetItBe,
    ButInTheEnd,
}

pub struct MusicPlayer {
    current_song: Song,
}

impl MusicPlayer {
    pub fn new() -> Self {
        MusicPlayer {
            current_song: Song::Intro,
        }
    }

    pub fn transition(&mut self, command: Command) {
        self.current_song = match (&self.current_song, command) {
            (Song::Intro, Command::Dangerous) => Song::LetItBe,
            (Song::Intro, Command::Fun) => Song::Starman,
            (Song::Intro, Command::Sad) => Song::ButInTheEnd,

            (Song::Starman, Command::Silly) => Song::Intro,
            (Song::Starman, Command::Dangerous) => Song::ShowMustGoOn,
            (Song::Starman, Command::Fun) => Song::ButInTheEnd,

            (Song::ShowMustGoOn, Command::Sad) => Song::LetItBe,
            (Song::ShowMustGoOn, Command::Fun) => Song::Starman,

            (Song::LetItBe, Command::Dangerous) => Song::Intro,
            (Song::LetItBe, Command::Silly) => Song::ShowMustGoOn,

            (Song::ButInTheEnd, _) => Song::ButInTheEnd,

            (current_song, _) => current_song.clone(),
        };
    }

    pub fn print_current_song(&self) {
        let song_name = match self.current_song {
            Song::Intro => "Intro",
            Song::Starman => "Starman",
            Song::ShowMustGoOn => "Show must go on",
            Song::LetItBe => "Let it be",
            Song::ButInTheEnd => "But in the end, it doesn't even matter",
        };
        println!("{}", song_name);
    }

    pub fn is_termination_state(&self) -> bool {
        matches!(self.current_song, Song::ButInTheEnd)
    }
}
