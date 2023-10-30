use crate::car::Car;
use bevy::prelude::*;
use crate::input::{*,controller::*};
use crate::controller::PlayerController;
use crate::camera::CameraConfig;
use crate::gamestate::GameState;

pub fn do_input<const MAXVIEW : u32>(
    mut controller: ResMut<PlayerController<MAXVIEW>>,
    mut camera_config: ResMut<CameraConfig>,
    mut cars: Query<&mut Car, With<PlayerControlled>>,
    state : Res<State<GameState>>,
) {
    for mut car in cars.iter_mut() 
    {
        car.steering = controller.get_steering();
        let throttle = controller.get_throttle();
        if throttle < 0. {
            car.brake = -throttle / 0.75;
            car.gas = 0.0;
        } else {
            car.gas = throttle / 0.75;
            car.brake = 0.0;
        }
        if controller.get_brake(){
            car.brake = 1.0;
            car.gas = 0.0;
        }
    }

    if ((*state.get()) == GameState::Playing) && controller.is_view_index_changed()
    {
        let view_index = controller.get_view_index();
        if view_index == 0 {
            camera_config.driver();
        }
        if view_index == 1 {
            camera_config.near();
        }
        if view_index == 2 {
            camera_config.mid();
        }
        if view_index == 3 {
            camera_config.far();
        }
        if view_index == 4 {
            camera_config.wheel();
        }
        if view_index == 5 {
            camera_config.free();
        }
    }
    controller.consum_state();
}


pub fn do_input_from_state(mut query: Query<(&PlayerInputState, &mut Car)>) {
    for (input,mut car) in query.iter_mut() {
        car.steering = input.steering;
        let throttle = input.throttle;
        if throttle < 0. {
            car.brake = -throttle / 0.75;
            car.gas = 0.0;
        } else {
            car.gas = throttle / 0.75;
            car.brake = 0.0;
        }
        if input.brake{
            car.brake = 1.0;
            car.gas = 0.0;
        }
    }
}