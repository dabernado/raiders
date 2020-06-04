use crate::state::{
    PlayerEntity,
};
use amethyst::{
    shrev::*,
    ecs::*,
    core::{
        transform::components::Transform,
    },
    derive::SystemDesc,
    input::{InputHandler, StringBindings},
};
/* Keyboard controller system */
// This is for controlling the camera with WASD keys
// during testing
#[derive(SystemDesc)]
pub struct ControllerSystem;

impl<'s> System<'s> for ControllerSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        Read<'s, PlayerEntity>,
        Read<'s, InputHandler<StringBindings>>,
        Entities<'s>,
        );

    fn run (&mut self, (mut transforms, player_entity, input, entities): Self::SystemData) {
        let player = entities.entity(player_entity.index());
        if let Some(transform) = transforms.get_mut(player) {
            if let Some(move_side) = input.axis_value("left_right") {
                let scaled_amount = -0.2 * move_side as f32;
                transform.append_translation_xyz(scaled_amount, 0.0, 0.0);
            }
            if let Some(move_forward) = input.axis_value("forward_back") {
                let scaled_amount = -0.2 * move_forward as f32;
                transform.append_translation_xyz(0.0, 0.0, scaled_amount);
            }
            if let Some(mouse_x) = input.axis_value("mouse_x") {
                let scaled_amount = 0.2 * mouse_x as f32;
                transform.append_rotation_y_axis(scaled_amount);
            }
            if let Some(mouse_y) = input.axis_value("mouse_y") {
                let scaled_amount = 0.2 * mouse_y as f32;
                transform.append_rotation_x_axis(scaled_amount);
            }
        }
    }
}
