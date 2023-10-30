use bevy::prelude::*;
use crate::controller::PlayerController;

pub fn process_keyboard<const MAXVIEW : u32>(
    mut controller: ResMut<PlayerController<MAXVIEW>>,
    input: Res<Input<KeyCode>>,
) 
{
    let mut move_dir : Vec3 = Vec3::new(0.0,0.0,0.0);
    let mut throttle : f32 = 0.0;
    let mut steering : f32 = 0.0;
    
    if input.pressed(KeyCode::Up) || input.pressed(KeyCode::W) {
        move_dir += Vec3::new(0.0,0.0,1.0);
        throttle += 1.0;
    }
    if input.pressed(KeyCode::Down) || input.pressed(KeyCode::S) {
        move_dir += Vec3::new(0.0,0.0,-1.0);
        throttle += -1.0;
    }

    if input.pressed(KeyCode::Left) || input.pressed(KeyCode::A) {
        move_dir += Vec3::new(-1.0,0.0,0.0);
        steering += -1.0;
    }
    if input.pressed(KeyCode::Right) || input.pressed(KeyCode::D) {
        move_dir += Vec3::new(1.0,0.0,0.0);
        steering += 1.0;
    }

    controller.keyboard.move_dir = move_dir;
    controller.keyboard.throttle = throttle;
    controller.keyboard.steering = steering;
    
    if input.pressed(KeyCode::Space) {
        controller.keyboard.jump = true;
        controller.keyboard.brake = true;
    }
    if input.just_released(KeyCode::Space) {
        controller.keyboard.jump = false;
        controller.keyboard.brake = false;
    }
    if input.just_released(KeyCode::Escape) {
        controller.keyboard.pause_game = !controller.keyboard.pause_game;
    }
    if input.just_pressed(KeyCode::V) {
        controller.next_view();
    }

    if input.just_pressed(KeyCode::Key1) {
        controller.set_view_index(0);
    }
    if input.just_pressed(KeyCode::Key2) {
        controller.set_view_index(1);
    }
    if input.just_pressed(KeyCode::Key3) {
        controller.set_view_index(2);
    }
    if input.just_pressed(KeyCode::Key4) {
        controller.set_view_index(3);
    }
    if input.just_pressed(KeyCode::Key5) {
        controller.set_view_index(4);
    }
    if input.just_pressed(KeyCode::Key6) {
        controller.set_view_index(5);
    }

}
