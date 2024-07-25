mod command;
mod music_player;

use std::io;
use command::Command;
use music_player::MusicPlayer;

fn main() {
    let mut music_player = MusicPlayer::new();
    let mut input = String::new();

    loop {
        music_player.print_current_song();
        println!("Enter command (sad, fun, silly, dangerous):");
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let command_str = input.trim();

        if let Some(command) = Command::from_str(command_str) {
            music_player.transition(command);
            if music_player.is_termination_state() {
                music_player.print_current_song();
                println!("Terminating the program.");
                break;
            }
        } else {
            println!("Invalid command. Please try again.");
        }
    }
}
