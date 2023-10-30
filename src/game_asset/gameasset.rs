
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;
use bevy::prelude::*;

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see <https://github.com/NiklasEi/bevy_asset_loader>)

/*All of game asset file must be list here.*/

#[derive(AssetCollection, Resource)]
pub struct GameAssets {

    #[asset(path = "textures/bevy.png")]
    pub texture_bevy: Handle<Image>,

    #[asset(path = "audio/flying.ogg")]
    pub flying: Handle<AudioSource>,
    
    #[asset(path = "wheelRacing.glb#Scene0")]
    pub wheel_body: Handle<Scene>,

    #[asset(path = "car-race.glb#Scene0")]
    pub car_body: Handle<Scene>,
       
    #[asset(path = "overheadLights.glb#Scene0")]
    pub overhead_light_body: Handle<Scene>,

    #[asset(path = "joystick/Horizontal_Outline_Arrows.png")]
    pub joystick_horizon_border_image: Handle<Image>,

    #[asset(path = "joystick/Vertical_Outline_Arrows.png")]
    pub joystick_verticle_border_image: Handle<Image>,

    #[asset(path = "joystick/Outline.png")]
    pub joystick_knob_image: Handle<Image>,

    #[asset(path = "fonts/FiraMono-Medium.ttf")]
    pub dash_font : Handle<Font>,

    #[asset(path = "overheadLights.glb#Scene0")]
    pub light_prob : Handle<Scene>
}