use bevy::prelude::*;
use tictactoe::map::map::{draw_backgorund, draw_board};
use tictactoe::plugins::default_plugins::get_defaults_plugins;
use tictactoe::state::game_state::GameState;

fn main() {
    App::new()
        .add_plugins(get_defaults_plugins())
        .init_resource::<State<GameState>>()
        .add_systems(Startup, (draw_backgorund, draw_board))
        .run();
}
