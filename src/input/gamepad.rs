
use bevy::prelude::*;
use crate::controller::PlayerController;

pub fn process_gamepad<const MAXVIEW : u32>(
    mut controller: ResMut<PlayerController<MAXVIEW>>,
    buttons: Res<Input<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,
    gamepads: Res<Gamepads>,
) {
   
    let mut move_dir : Vec3 = Vec3::new(0.0,0.0,0.0);
    let mut throttle : f32 = 0.0;
    let mut steering : f32 = 0.0;

    let mut brake_pressed_count : u32 = 0;
    let mut brake_released_count : u32 = 0;

    let mut jump_pressed_count : u32 = 0;
    let mut jump_released_count : u32 = 0;
    let mut next_view_count : u32 = 0;

    for gamepad in gamepads.iter() {
       
        let left_stick_x = axes.get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX)).unwrap();
        let right_stick_z = axes.get(GamepadAxis::new(gamepad, GamepadAxisType::RightStickY)).unwrap();
       
        move_dir += Vec3::new(left_stick_x,0.0,1.0);
        move_dir += Vec3::new(0.0,0.0,right_stick_z);

        throttle += right_stick_z;
        steering += left_stick_x;

  
        if buttons.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::Start)) {
            controller.gamepad.pause_game = !controller.gamepad.pause_game;
        }
        if buttons.pressed(GamepadButton::new(gamepad, GamepadButtonType::South)) {
            brake_pressed_count += 1;
            jump_pressed_count += 1;
        }
         else if buttons.just_released(GamepadButton::new(gamepad, GamepadButtonType::South)) {
            brake_released_count += 1;
            jump_released_count += 1;
        }
        if buttons.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::RightTrigger)) {
           next_view_count += 1;
        }
    }

    controller.gamepad.move_dir = move_dir;
    controller.gamepad.throttle = throttle;
    controller.gamepad.steering = steering;

    if brake_pressed_count > 0
    {
        controller.gamepad.brake = true;
    }
    else if brake_released_count > 0
    {
        controller.gamepad.brake = false;
    }

    if jump_pressed_count > 0
    {
        controller.gamepad.jump = true;
    }
    else if jump_released_count > 0 {
        controller.gamepad.jump = false;
    }
    if next_view_count > 0
    {
        controller.next_view();
    }

}
