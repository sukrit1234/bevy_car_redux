use bevy::prelude::*;
use virtual_joystick::*;
use crate::game_asset::GameAssets;
use crate::controller::PlayerController;

#[derive(Default, Reflect, Hash, Clone, PartialEq, Eq)]
pub enum JoystickTypeAxis {
    #[default]
    X,
    Y,
}

const MARGIN: Val = Val::Px(35.);
const KNOB_SIZE: Vec2 = Vec2::new(70., 70.);
const AREA_SIZE: Val = Val::Px(150.);
const BG: BackgroundColor = BackgroundColor(Color::rgba(1.0, 0.27, 0.0, 0.1));

pub fn initialize_joystick(mut cmd: Commands, game_asset: Res<GameAssets>) {
    cmd.spawn((
        VirtualJoystickBundle::new(VirtualJoystickNode {
            border_image: game_asset.joystick_horizon_border_image.clone(),
            knob_image: game_asset.joystick_knob_image.clone(),
            knob_size: KNOB_SIZE,
            dead_zone: 0.,
            id: JoystickTypeAxis::X,
            axis: VirtualJoystickAxis::Horizontal,
            behaviour: VirtualJoystickType::Fixed,
        })
        .set_color(TintColor(Color::WHITE))
        .set_style(Style {
            width: AREA_SIZE,
            height: AREA_SIZE,
            position_type: PositionType::Absolute,
            left: MARGIN,
            bottom: MARGIN,
            ..default()
        }),
        BG,
        VirtualJoystickInteractionArea,
    ));

    cmd.spawn((
        VirtualJoystickBundle::new(VirtualJoystickNode {
            border_image: game_asset.joystick_verticle_border_image.clone(),
            knob_image: game_asset.joystick_knob_image.clone(),
            knob_size: KNOB_SIZE,
            dead_zone: 0.,
            id: JoystickTypeAxis::Y,
            axis: VirtualJoystickAxis::Vertical,
            behaviour: VirtualJoystickType::Fixed,
        })
        .set_color(TintColor(Color::WHITE))
        .set_style(Style {
            width: AREA_SIZE,
            height: AREA_SIZE,
            position_type: PositionType::Absolute,
            right: MARGIN,
            bottom: MARGIN,
            ..default()
        }),
        BG,
        VirtualJoystickInteractionArea,
    ));
}

pub fn process_joystick<const MAXVIEW : u32>(
    mut controller: ResMut<PlayerController<MAXVIEW>>,
    mut virtual_joystick_events: EventReader<VirtualJoystickEvent<JoystickTypeAxis>>,
) {
    let mut move_dir = Vec3::new(0.0, 0.0, 0.0);
    let mut throttle : f32 = 0.0;
    let mut steering : f32 = 0.0;

    for j in virtual_joystick_events.iter() {
        let Vec2 { x, y } = j.axis();
        match j.id() {
            JoystickTypeAxis::X => {
                steering += x;
                move_dir += Vec3::new(x, 0.0, 0.0);
            }
            JoystickTypeAxis::Y => {
                throttle += y;
                move_dir += Vec3::new(0.0, 0.0, y);
            }
        }
    }
    controller.joystick.move_dir = move_dir;
    controller.joystick.throttle = throttle;
    controller.joystick.steering = steering;
}
