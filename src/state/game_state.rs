use bevy::prelude::*;

pub fn game_status(game_state: Res<State<GameState>>) {
    match game_state.get() {
        GameState::Waiting => println!("Waiting for players"),
        GameState::Playing => println!("Playing game..."),
        GameState::End => println!("End of game"),
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GameState {
    Waiting,
    #[default]
    Playing,
    End,
}
