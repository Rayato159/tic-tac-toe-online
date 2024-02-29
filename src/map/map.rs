use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::settings::settings::{WINDOW_RESOLUTION, WINDOW_SIZE_FACTOR};

#[derive(Component)]
pub struct MainCamera;

enum PastelPink {
    Level1,
    Level2,
    Level3,
    Level4,
}

fn pastel_pink(color: PastelPink) -> Color {
    match color {
        PastelPink::Level1 => Color::rgb(255. / 255., 230. / 255., 230. / 255.),
        PastelPink::Level2 => Color::rgb(225. / 255., 175. / 255., 209. / 255.),
        PastelPink::Level3 => Color::rgb(173. / 255., 136. / 255., 198. / 255.),
        PastelPink::Level4 => Color::rgb(116. / 255., 105. / 255., 182. / 255.),
    }
}

pub fn initialize_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

pub fn draw_backgorund(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((MaterialMesh2dBundle {
        mesh: meshes.add(Rectangle::default()).into(),
        transform: Transform {
            translation: Vec3::new(0., 0., -1.),
            scale: Vec3::new(
                WINDOW_SIZE_FACTOR * WINDOW_RESOLUTION.0,
                WINDOW_SIZE_FACTOR * WINDOW_RESOLUTION.1,
                0.,
            ),
            ..default()
        },
        material: materials.add(pastel_pink(PastelPink::Level3)),
        ..default()
    },));
}

// Grid size for each cell
// [-320./2., 320./2.]  [0., 320./2.]   [320./2., 320./2.]
// [-320./2., 0.]       [0., 0.]        [320./2., 0.]
// [-320./2., -320./2.] [0., -320./2.]   [320./2., -320./2.]
pub fn draw_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let offset = 8.;

    for row in -1..2 {
        for col in -1..2 {
            commands.spawn((MaterialMesh2dBundle {
                mesh: meshes.add(Rectangle::default()).into(),
                transform: Transform {
                    translation: Vec3::new(
                        WINDOW_SIZE_FACTOR * WINDOW_RESOLUTION.0 / 3. * col as f32,
                        WINDOW_SIZE_FACTOR * WINDOW_RESOLUTION.1 / 3. * row as f32,
                        0.,
                    ),
                    scale: Vec3::new(
                        WINDOW_SIZE_FACTOR * WINDOW_RESOLUTION.0 / 3. - offset,
                        WINDOW_SIZE_FACTOR * WINDOW_RESOLUTION.1 / 3. - offset,
                        0.,
                    ),
                    ..default()
                },
                material: materials.add(pastel_pink(PastelPink::Level1)),
                ..default()
            },));
        }
    }
}
