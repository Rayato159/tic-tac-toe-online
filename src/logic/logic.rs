use std::{collections::HashMap, hash::Hash};

use bevy::prelude::*;

use crate::settings::settings::{WINDOW_RESOLUTION, WINDOW_SIZE_FACTOR};

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum BoardPosition {
    Col1Row1,
    Col2Row1,
    Col3Row1,
    Col1Row2,
    Col2Row2,
    Col3Row2,
    Col1Row3,
    Col2Row3,
    Col3Row3,
}

#[derive(Component, Debug, Clone)]
pub struct BoardUsage(pub HashMap<BoardPosition, bool>);

pub fn init_board_usage(mut commands: Commands) {
    let mut board_usage = BoardUsage(HashMap::<BoardPosition, bool>::new());

    board_usage.0.insert(BoardPosition::Col1Row1, false);
    board_usage.0.insert(BoardPosition::Col2Row1, false);
    board_usage.0.insert(BoardPosition::Col3Row1, false);
    board_usage.0.insert(BoardPosition::Col1Row2, false);
    board_usage.0.insert(BoardPosition::Col2Row2, false);
    board_usage.0.insert(BoardPosition::Col3Row2, false);
    board_usage.0.insert(BoardPosition::Col1Row3, false);
    board_usage.0.insert(BoardPosition::Col2Row3, false);
    board_usage.0.insert(BoardPosition::Col3Row3, false);

    commands.spawn(board_usage);
}

pub fn get_board_pos(board_state: BoardPosition) -> Vec3 {
    let window_x_size = WINDOW_SIZE_FACTOR * WINDOW_RESOLUTION.0;
    let window_y_size = WINDOW_SIZE_FACTOR * WINDOW_RESOLUTION.1;

    match board_state {
        BoardPosition::Col1Row1 => Vec3::new(window_x_size / 3. * -1., window_y_size / 3. * 1., 1.),
        BoardPosition::Col2Row1 => Vec3::new(0., window_y_size / 3. * 1., 1.),
        BoardPosition::Col3Row1 => Vec3::new(window_x_size / 3. * 1., window_y_size / 3. * 1., 1.),
        BoardPosition::Col1Row2 => Vec3::new(window_x_size / 3. * -1., 0., 1.),
        BoardPosition::Col2Row2 => Vec3::new(0., 0., 1.),
        BoardPosition::Col3Row2 => Vec3::new(window_x_size / 3. * 1., 0., 1.),
        BoardPosition::Col1Row3 => {
            Vec3::new(window_x_size / 3. * -1., window_y_size / 3. * -1., 1.)
        }
        BoardPosition::Col2Row3 => Vec3::new(0., window_y_size / 3. * -1., 1.),
        BoardPosition::Col3Row3 => Vec3::new(window_x_size / 3. * 1., window_y_size / 3. * -1., 1.),
    }
}
