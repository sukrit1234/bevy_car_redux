
use bevy::prelude::*;
use bevy::pbr::DirectionalLightShadowMap;

pub struct GraphicSettingPlugin;

impl Plugin for GraphicSettingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa::Sample4)
           .insert_resource(DirectionalLightShadowMap::default());
    }
}