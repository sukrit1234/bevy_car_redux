
use bevy::prelude::*;
use bevy::math::*;
use serde::{Serialize,Deserialize};

pub struct ControllerSignal {
    pub move_dir: Vec3,
    pub throttle : f32,
    pub steering: f32,
    pub brake: bool,
    pub jump: bool,
    pub pause_game : bool,
}

impl Default for ControllerSignal {
    fn default() -> Self {
        Self {
            move_dir: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
            throttle : 0.0,
            steering: 0.0,
            brake: false,
            jump: false,
            pause_game : false,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, Component, Resource)]
pub struct PlayerInputState
{
    pub brake : bool,
    pub jumped : bool,
    pub throttle : f32,
    pub steering : f32,
    pub direction : [f32;3]
}

#[derive(Resource)]
pub struct PlayerController<const MAXVIEW : u32> {
    pub keyboard : ControllerSignal,
    pub gamepad : ControllerSignal,
    pub joystick: ControllerSignal,
    view_index : u32,

    last_brake : bool,
    last_jumped : bool,
    last_pause_game : bool,
    last_view_index : u32,
}


impl<const MAXVIEW : u32> Default for PlayerController<MAXVIEW> {
    fn default() -> Self {
        Self {
            keyboard : ControllerSignal::default(),
            gamepad : ControllerSignal::default(),
            joystick: ControllerSignal::default(),
            view_index : 0,
            last_brake : false,
            last_jumped : false,
            last_pause_game : false,
            last_view_index : 0,
        }
    }
}
impl<const MAXVIEW : u32> PlayerController<MAXVIEW> {
    pub fn set_view_index(&mut self,index :u32) -> () {
        self.view_index = index % MAXVIEW;
    }
    pub fn next_view(&mut self) -> (){
        self.set_view_index(self.view_index + 1);
    }
    pub fn prev_view(&mut self) -> (){
        if self.view_index == 0
        {
            self.view_index = MAXVIEW - 1;
        }
        else {
            self.set_view_index(self.view_index - 1);
        }
    }
    pub fn get_move_direction(&self) -> Vec3{
        (self.keyboard.move_dir + self.gamepad.move_dir + self.joystick.move_dir).normalize_or_zero()
    }
    pub fn get_throttle(&self) ->f32 {
        (self.keyboard.throttle + self.gamepad.throttle + self.joystick.throttle).clamp(-1.0, 1.0)
    }
    pub fn get_steering(&self) ->f32 {
        (self.keyboard.steering + self.gamepad.steering + self.joystick.steering).clamp(-1.0, 1.0)
    }
    pub fn get_brake(&self) ->bool {
        self.keyboard.brake || self.gamepad.brake || self.joystick.brake
    }
    pub fn get_jump(&self) ->bool {
        self.keyboard.jump || self.gamepad.jump || self.joystick.jump
    }
    pub fn get_pause_game(&self) ->bool {
        self.keyboard.pause_game || self.gamepad.pause_game || self.joystick.pause_game
    }
    pub fn get_view_index(&self) -> u32 {
        self.view_index
    }
    pub fn is_view_index_changed(&self) -> bool {
        self.last_view_index != self.view_index
    }
    pub fn is_jump_changed(&self) -> bool {
        self.last_jumped != self.get_jump()
    }
    pub fn is_brake_changed(&self) -> bool {
        self.last_jumped != self.get_brake()
    }
    pub fn is_pause_game_changed(&self) -> bool {
        self.last_pause_game != self.get_pause_game()
    }
    pub fn consum_state(&mut self) ->(){
        self.last_brake = self.get_brake();
        self.last_jumped = self.get_jump();
        self.last_pause_game = self.get_pause_game();
        self.last_view_index = self.get_view_index();
    }
}