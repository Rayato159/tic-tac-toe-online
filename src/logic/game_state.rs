use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GameState {
    Waiting,
    #[default]
    Playing,
    End,
}

pub fn get_game_status(game_state: Res<State<GameState>>) {
    match game_state.get() {
        GameState::Waiting => println!("Waiting for players"),
        GameState::Playing => println!("Playing game..."),
        GameState::End => println!("End of game"),
    }
}

#[derive(Component, Debug, Clone, PartialEq)]
pub enum Player {
    Player1,
    Player2,
}

#[derive(Component, Debug, Clone)]
pub struct PlayerTurn(pub Player);

pub fn init_player_turn(mut commands: Commands) {
    commands.spawn(PlayerTurn(Player::Player1));
}
