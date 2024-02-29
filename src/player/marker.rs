use bevy::{prelude::*, utils::Uuid};

use crate::{
    logic::{
        game_state::{Player, PlayerTurn},
        logic::{get_board_pos, BoardPosition, BoardUsage},
    },
    map::map::MainCamera,
    settings::settings::{WINDOW_RESOLUTION, WINDOW_SIZE_FACTOR},
};

use super::animate::{AnimationIndices, AnimationTimer, FPS};

#[derive(Component, Debug, Clone)]
pub struct Marker {
    pub id: Uuid,
    pub number_of_player: u8,
    pub positions: Vec3,
    pub positions_value: bool,
    pub animation_indices: AnimationIndices,
}

pub fn place_marker(
    mut commands: Commands,
    mouse_button_input: Res<'_, ButtonInput<MouseButton>>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut player_turn_query: Query<&mut PlayerTurn>,
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut board_useage_query: Query<&mut BoardUsage>,
) {
    let window = window_query.single();
    let (camera, camera_transform) = camera_query.single();
    let mut board_usage = board_useage_query.single_mut();
    let mut player_turn = player_turn_query.single_mut();
    let asset = asset_server.clone();

    if let Some(world_pos) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        let mut world_pos_vec3 = Vec3::new(world_pos.x, world_pos.y, 1.);

        if mouse_button_input.just_pressed(MouseButton::Left) {
            let board_pos = get_place_pos(&world_pos_vec3);

            let is_used = match board_usage.0.get(&board_pos) {
                Some(value) => *value,
                None => false,
            };

            if is_used {
                println!("This place is already used");
                return;
            }

            board_usage.0.insert(board_pos.clone(), true);
            world_pos_vec3 = get_board_pos(board_pos.clone());

            let (texture, marker) = match player_turn.0 {
                Player::Player1 => get_player_by_turn(
                    asset.clone(),
                    "cat/orange_cat.png".to_string(),
                    1,
                    world_pos_vec3.clone(),
                ),
                Player::Player2 => get_player_by_turn(
                    asset.clone(),
                    "cat/gray_cat.png".to_string(),
                    2,
                    world_pos_vec3.clone(),
                ),
            };

            let layout = TextureAtlasLayout::from_grid(Vec2::new(32., 32.), 8, 1, None, None);
            let texture_atlas_layout = texture_atlas_layouts.add(layout);

            let sprite_sheet = SpriteSheetBundle {
                texture,
                atlas: TextureAtlas {
                    layout: texture_atlas_layout,
                    index: 1,
                },
                transform: Transform {
                    translation: world_pos_vec3.clone(),
                    scale: Vec3::splat(4.0),
                    ..default()
                },
                ..default()
            };

            let animation_timer =
                AnimationTimer(Timer::from_seconds(1. / FPS, TimerMode::Repeating));

            commands.spawn((sprite_sheet, marker, animation_timer));

            match player_turn.0 {
                Player::Player1 => player_turn.0 = Player::Player2,
                Player::Player2 => player_turn.0 = Player::Player1,
            }
        }
    }
}

fn get_player_by_turn(
    asset_server: AssetServer,
    asset_path: String,
    number_of_player: u8,
    pos: Vec3,
) -> (Handle<Image>, Marker) {
    let texture = asset_server.load(asset_path);
    let animation_indices = AnimationIndices { first: 1, last: 7 };

    let marker = Marker {
        id: Uuid::new_v4(),
        number_of_player,
        positions: pos.clone(),
        positions_value: true,
        animation_indices: animation_indices.clone(),
    };

    (texture, marker)
}

fn get_place_pos(mouse_pos: &Vec3) -> BoardPosition {
    let window_x_size = WINDOW_SIZE_FACTOR * WINDOW_RESOLUTION.0;
    let window_y_size = WINDOW_SIZE_FACTOR * WINDOW_RESOLUTION.1;

    if mouse_pos.x < -(window_x_size / 4.) && mouse_pos.y > (window_y_size / 4.) {
        BoardPosition::Col1Row1
    } else if mouse_pos.x < (window_x_size / 4.) && mouse_pos.y > (window_y_size / 4.) {
        BoardPosition::Col2Row1
    } else if mouse_pos.x < (window_x_size / 2.) && mouse_pos.y > (window_y_size / 4.) {
        BoardPosition::Col3Row1
    } else if mouse_pos.x < -(window_x_size / 4.) && mouse_pos.y > -(window_y_size / 4.) {
        BoardPosition::Col1Row2
    } else if mouse_pos.x < (window_x_size / 4.) && mouse_pos.y > -(window_y_size / 4.) {
        BoardPosition::Col2Row2
    } else if mouse_pos.x < (window_x_size / 2.) && mouse_pos.y > -(window_y_size / 4.) {
        BoardPosition::Col3Row2
    } else if mouse_pos.x < -(window_x_size / 4.) && mouse_pos.y > -(window_y_size / 2.) {
        BoardPosition::Col1Row3
    } else if mouse_pos.x < (window_x_size / 4.) && mouse_pos.y > -(window_y_size / 2.) {
        BoardPosition::Col2Row3
    } else {
        BoardPosition::Col3Row3
    }
}
