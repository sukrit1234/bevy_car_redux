use bevy_rapier3d::prelude::*;
use bevy::prelude::*;

#[derive(Resource, Copy, Clone, Debug)]
pub struct PhysicsParams {
    pub max_velocity_iters: usize,
    pub max_velocity_friction_iters: usize,
    pub max_stabilization_iters: usize,
    pub substeps: usize,
}

impl PhysicsParams {
    fn default_wasm32() -> PhysicsParams{
        PhysicsParams {
            max_velocity_iters: 32,
            max_velocity_friction_iters: 32,
            max_stabilization_iters: 16,
            substeps: 8,
        }
    }
    fn default_not_wasm32() -> PhysicsParams{
        PhysicsParams{
            max_velocity_iters: 64,
            max_velocity_friction_iters: 64,
            max_stabilization_iters: 16,
            substeps: 20,
        }
    }
    pub fn make_default() ->PhysicsParams{
        #[cfg(target_arch = "wasm32")]
        return PhysicsParams::default_wasm32();
       
        #[cfg(not(target_arch = "wasm32"))]
        return PhysicsParams::default_not_wasm32();
    }
}

impl Default for PhysicsParams {
    fn default() -> Self {
        PhysicsParams::make_default()
    }
}

pub fn rapier_config_start_system(mut c: ResMut<RapierContext>, ph: Res<PhysicsParams>) {
    c.integration_parameters.max_velocity_iterations = ph.max_velocity_iters;
    c.integration_parameters.max_velocity_friction_iterations = ph.max_velocity_friction_iters;
    c.integration_parameters.max_stabilization_iterations = ph.max_stabilization_iters;
    // c.integration_parameters.max_ccd_substeps = 16;
    // c.integration_parameters.allowed_linear_error = 0.000001;
    c.integration_parameters.erp = 0.99;
    // c.integration_parameters.erp = 1.;
    // c.integration_parameters.max_penetration_correction = 0.0001;
    // c.integration_parameters.prediction_distance = 0.01;
    dbg!(c.integration_parameters);
}

