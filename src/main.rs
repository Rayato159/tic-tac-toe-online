use bevy::prelude::*;
use tictactoe::logic::game_state::{get_game_status, init_player_turn, GameState};
use tictactoe::logic::logic::init_board_usage;
use tictactoe::map::map::{draw_backgorund, draw_board, initialize_camera};
use tictactoe::player::animate::animate_sprite;
use tictactoe::player::marker::place_marker;
use tictactoe::plugins::default_plugins::get_defaults_plugins;

fn main() {
    App::new()
        .add_plugins(get_defaults_plugins())
        .init_resource::<State<GameState>>()
        .add_systems(
            Startup,
            (
                init_player_turn,
                initialize_camera,
                draw_backgorund,
                draw_board,
                init_board_usage,
                get_game_status,
            ),
        )
        .add_systems(Update, place_marker)
        .add_systems(Update, animate_sprite)
        .run();
}
