use bevy::prelude::*;

use crate::prelude::*;

// Move the quad around.
pub fn move_canvas(mut query: Query<&mut Transform, With<Canvas>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        // println!("Time: {:?}", (time.elapsed_seconds() * 0.5).sin() * 1000.0);
        transform.translation.x = (time.elapsed_seconds() * 0.5).sin() * 100.0;
    }
}

// Move the world around.
pub fn move_world(mut query: Query<&mut Transform>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        // println!("Time: {:?}", (time.elapsed_seconds() * 0.5).sin() * 1000.0);
        transform.translation.y = (time.elapsed_seconds() * 0.5).sin() * 100.0;
    }
}

// Move the world around with keyboard input.
pub fn move_world_with_input(
    query: Query<&Handle<CustomMaterial>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut speed = 100.0;
    if keyboard_input.pressed(KeyCode::ShiftLeft) {
        speed = 200.0;
    }

    let mut translation = Vec2::ZERO;
    if keyboard_input.pressed(KeyCode::KeyW) {
        translation.y -= speed;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        translation.y += speed;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        translation.x += speed;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        translation.x -= speed;
    }
    translation *= time.delta_seconds();

    for handle in query.iter() {
        if let Some(material) = materials.get_mut(handle) {
            material.real_coord += translation.x * material.world_scale;
            material.imag_coord += translation.y * material.world_scale;
        }
    }
}

// Update the shader uniforms.
pub fn change_number_of_iterations(
    query: Query<&Handle<CustomMaterial>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Minus) {
        for handle in query.iter() {
            if let Some(material) = materials.get_mut(handle) {
                material.max_iterations /= 2;
            }
        }
    } else if keyboard_input.just_pressed(KeyCode::Equal) {
        for handle in query.iter() {
            if let Some(material) = materials.get_mut(handle) {
                material.max_iterations *= 2;
            }
        }
    }
}

// Update the shader uniforms.
pub fn change_world_scale(
    query: Query<&Handle<CustomMaterial>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let mut speed = 1.0e-3;
    if keyboard_input.pressed(KeyCode::ShiftLeft) {
        speed = 1.0e-2;
    }

    if keyboard_input.pressed(KeyCode::KeyE) {
        for handle in query.iter() {
            if let Some(material) = materials.get_mut(handle) {
                material.world_scale *= 1.0 - speed;
            }
        }
    } else if keyboard_input.pressed(KeyCode::KeyQ) {
        for handle in query.iter() {
            if let Some(material) = materials.get_mut(handle) {
                material.world_scale *= 1.0 + speed;
            }
        }
    }
}
