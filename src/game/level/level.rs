use bevy::{
    //color::palettes::tailwind,
    prelude::*, 
    //render::view::RenderLayers,
};
use avian3d::prelude::*;

pub struct LevelPlugin;

/*
#[derive(Debug, Component)]
struct WorldModelCamera;
const DEFAULT_RENDER_LAYER: usize = 0;
const VIEW_MODEL_RENDER_LAYER: usize = 1;
*/


impl Plugin for LevelPlugin
{
    fn build(&self, app: &mut App)
    {
     app.add_systems(Startup, init_level);
    }
}

fn  init_level(
    mut commands : Commands,
    mut meshes : ResMut<Assets<Mesh>>,
    mut materials : ResMut<Assets<StandardMaterial>>, )
{
    let level_material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        ..default()
    });

    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(20. ,0. ,20.),
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y,  Vec2::splat(20.)))),
        MeshMaterial3d(level_material.clone()),
        Transform::IDENTITY,
        ));

    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(2.,2.,2.),
        Mesh3d(meshes.add(Cuboid::from_length(2.))),
        MeshMaterial3d(level_material.clone()),
        Transform::from_xyz(0.0, 0.25, -3.0),
        ));

    commands.spawn((
        DirectionalLight {
            color: Color::srgb(0.98, 0.95, 0.82),
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 20.0, 0.0)
            .looking_at(Vec3::new(-0.15, -0.1, -0.15), Vec3::Y),

    ));
    /*commands.spawn((
        PointLight {
            color: Color::from(tailwind::ROSE_300),
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(-2.0, 4.0, -0.75),
        // The light source illuminates both the world model and the view model.
        RenderLayers::from_layers(&[DEFAULT_RENDER_LAYER, VIEW_MODEL_RENDER_LAYER]),
    ));*/

}

