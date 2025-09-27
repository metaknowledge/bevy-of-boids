// use std::{ops::Div, borrow::Borrow};

use bevy::{prelude::*, window::PrimaryWindow, sprite::MaterialMesh2dBundle, render::{render_resource::PrimitiveTopology, mesh::Indices}};
use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use rand::prelude::*;

pub const NUMBER_OF_BOIDS: usize = 30;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(WorldInspectorPlugin::new())
    .init_resource::<Configuration>() // `ResourceInspectorPlugin` won't initialize the resource
    .register_type::<Configuration>() // you need to register your type to display it
    .add_plugin(ResourceInspectorPlugin::<Configuration>::default())// also works with built-in resources, as long as they implement `Reflect`
    .add_startup_system(spawn_camera)
    .add_startup_system(spawn_boids)
    .add_system(update_boids)
    .add_system(confine_boids)
    .add_system(push_boids_away_from_walls)
    .run();
}

#[derive(Component, Default)]
pub struct Boid;

#[derive(Component, Default, InspectorOptions)]
pub struct Velocity{
  pub vel: Vec3
}

// `InspectorOptions` are completely optional
#[derive(Reflect, Resource, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct Configuration {
    #[inspector(min = 1.0, max = 10000.0)]
    attraction: f32,
    #[inspector(min = 1.0, max = 1000.0)]
    repulsion: f32,
    #[inspector(min = 1.0, max = 100.0)]
    direction: f32,
    #[inspector(min = 1.0, max = 1000.0)]
    closeness: f32,
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn spawn_boids(
  mut cmd: Commands,
  window_query:Query<&Window, With<PrimaryWindow>>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  let window = window_query.get_single().unwrap();

  for _ in 1..NUMBER_OF_BOIDS {
    let x= random::<f32>() * window.width() / 2.0;
    let y = random::<f32>() * window.height() / 2.0;
    let velocity = Vec3::new(random::<f32>(), random::<f32>(), 0.0);
    cmd.spawn(
        (MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(5.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
            ..default()
        }, Boid, Velocity{
          vel: velocity.normalize()
        }),
        );
  }
}



pub fn update_boids(
  mut boids_query: Query<(&mut Transform, &mut Velocity), With<Boid>>,
  config_asset: Res<Configuration>,
) {
  let mut combonations = boids_query.iter_combinations_mut();
  if config_asset.attraction == 0.0 {
    return;
  }
  let scaler: f32 = (NUMBER_OF_BOIDS - 1) as f32;
  while let Some([(transform1, mut velocity1), (transform2, mut velocity2)]) = combonations.fetch_next() {
    if transform1.translation.distance_squared(transform2.translation) < 40000.0 {
      let im_velocity1 = velocity1.vel;
      let im_velocity2 = velocity2.vel;
      // move the direction towards other boids direction
      velocity1.vel += (velocity2.vel - im_velocity1) / (scaler * config_asset.direction);
      velocity2.vel += (velocity1.vel - im_velocity2) / (scaler * config_asset.direction);
      // move towards other boids center of mass
      velocity1.vel += (transform2.translation - transform1.translation) / (scaler * config_asset.attraction);
      velocity2.vel += (transform1.translation - transform1.translation) / (scaler * config_asset.attraction);
      // move away from close boids
      if transform1.translation.distance_squared(transform2.translation) < config_asset.closeness.powi(2){

        velocity1.vel += (transform1.translation - transform2.translation) / config_asset.repulsion;
        velocity2.vel += (transform2.translation - transform1.translation) / config_asset.repulsion;
      }
    }
  }

  for (mut transform, velocity) in boids_query.iter_mut() {
    transform.translation += velocity.vel;
  }
}


pub fn push_boids_away_from_walls(
  mut boids_query: Query<(&mut Transform, &mut Velocity), With<Boid>>,
  window_query:Query<&Window, With<PrimaryWindow>>,
) {
  let window = window_query.get_single().unwrap();

  for (transform, mut velocity) in boids_query.iter_mut() {
    // println!("boids: {}", velocity.vel.y);
    if transform.translation.y < 50.0 {
      velocity.vel.y += 0.5;
    } else if transform.translation.y > window.height() - 50.0 {
      velocity.vel.y -= 0.5;
    }

    if transform.translation.x < 50.0 {
      velocity.vel.x += 1.0;
    } else if transform.translation.x > window.width() - 50.0 {
      velocity.vel.x -= 1.0;
    }
    velocity.vel = velocity.vel.clamp_length(0.1, 1.0);
  }
}

pub fn confine_boids(
  mut boids_query: Query<(&mut Transform, &mut Velocity), With<Boid>>,
  window_query:Query<&Window, With<PrimaryWindow>>,
) {
  let window = window_query.get_single().unwrap();

  for (mut transform, _) in boids_query.iter_mut() {
    if transform.translation.x < 0.0 {
      transform.translation.x = 0.0;
    } else if transform.translation.x > window.width() {
      transform.translation.x = window.width();
    }
    if transform.translation.y < 0.0 {
      transform.translation.y = 0.0;
    } else if transform.translation.y > window.height() {
      transform.translation.y = window.height();
      // println!("{}", vel.vel.y);
    }
  }
}

fn create_triangle() -> Mesh {
  let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vec![[1.0, 0.0, 0.0], [0.0, 2.0, 0.0], [1.0, 0.0, 0.0]]);
    mesh.set_indices(Some(Indices::U32(vec![0,1,2])));
    mesh
}
