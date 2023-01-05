use bevy::{prelude::*, time::FixedTimestep};

mod arena;
mod camera;
mod snake;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Snake Game".to_string(),
                width: 500.,
                height: 500.,
                ..default()
            },
            ..default()
        }))
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(snake::SnakeSegments::default())
        .insert_resource(snake::LastTailPosition::default())
        .add_event::<snake::GrowthEvent>()
        .add_event::<arena::GameOverEvent>()
        .add_startup_system(camera::setup_camera)
        .add_startup_system(snake::spawn_snake)
        .add_system(snake::snake_movement_input.before(snake::snake_movement))
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(arena::position_translation)
                .with_system(arena::size_scaling),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(arena::food_spawner),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.150))
                .with_system(snake::snake_movement)
                .with_system(snake::snake_eating.after(snake::snake_movement))
                .with_system(snake::snake_growth.after(snake::snake_eating)),
        )
        .add_system(arena::game_over.after(snake::snake_movement))
        .run();
}
