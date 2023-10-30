
use bevy::prelude::*;
pub mod controller;
pub mod keyboard;
pub mod gamepad;


#[cfg(feature = "virtual_joystick")]
pub mod joystick;

#[cfg(feature = "virtual_joystick")]
use crate::joystick::{initialize_joystick,JoystickTypeAxis};

use crate::GameState;
use crate::controller::PlayerController;

use self::controller::PlayerInputState;
use self::gamepad::process_gamepad;

#[cfg(feature = "virtual_joystick")]
use self::joystick::process_joystick;

use self::keyboard::process_keyboard;

#[derive(Component)]
pub struct PlayerControlled;

pub fn cache_current_inputstate<const MAXVIEW : u32>(
    mut input_state: ResMut<PlayerInputState>,
    controller: Res<PlayerController<MAXVIEW>>) 
{
    input_state.brake = controller.get_brake();
    input_state.jumped = controller.get_jump();
    input_state.throttle = controller.get_throttle();
    input_state.steering = controller.get_steering();
    input_state.direction = controller.get_move_direction().to_array();
}

pub struct InputPlugin<const MAXVIEW:u32>;

impl<const MAXVIEW:u32> Plugin for InputPlugin<MAXVIEW> {
    fn build(&self, app: &mut App) {

        #[cfg(feature = "virtual_joystick")]
        app.add_systems(OnExit(GameState::Loading),initialize_joystick)
        .add_plugins(VirtualJoystickPlugin::<JoystickTypeAxis>::default());

        app.init_resource::<PlayerController::<MAXVIEW>>()
        .insert_resource(PlayerInputState::default())
        .add_systems(Update, 
            (process_keyboard::<MAXVIEW>,
                     process_gamepad::<MAXVIEW>,
                     #[cfg(feature = "virtual_joystick")] process_joystick::<MAXVIEW>)
        .run_if(in_state(GameState::Menu).or_else(in_state(GameState::Playing))).before(cache_current_inputstate::<MAXVIEW>))
        .add_systems(Update, cache_current_inputstate::<MAXVIEW>.run_if(in_state(GameState::Menu).or_else(in_state(GameState::Playing))));

    }
}