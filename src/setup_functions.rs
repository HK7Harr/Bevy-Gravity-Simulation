
use crate::helpers::*;
use crate::components::Planet;
use crate::internal_imports::*;

fn spawn_planet(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    radius: i32,
    id: i32,
    color: Color,
    transform: Transform,
) {
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(radius as f32))),
        MeshMaterial2d(materials.add(color)),
        transform,
        Planet::new(id, radius, DVec3::ZERO),
    ));
}

fn random_color() -> Color {
    let mut rng = rand::rng();
    Color::srgb(rng.random(), rng.random(), rng.random())
}
fn random_transform(
    x: (i32, i32),
    y: (i32, i32)
) -> Transform {
    let mut rng = rand::rng();
    
    Transform::from_xyz( rng.random_range(x.0..=x.1) as f32, rng.random_range(y.0..=y.1) as f32, 0.0_f32)
}

fn spawn_random_planets(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    radius_range: (i32, i32),
    ids: Vec<i32>, // the length of the vec is the amount of planets
    x_range: (i32, i32),
    y_range: (i32, i32)
) {
    let mut rng = rand::rng();
    let mut info: Vec<(Transform, i32)> = Vec::new();
    for id in ids {
        let mut radius = rng.random_range(radius_range.0..=radius_range.1);
        let mut transform = random_transform(x_range, y_range);
        loop {
            if info.is_empty() == true {
                info.push((transform, radius));
                break;
            }
            for i in &info {
                if com_length_pixels_2d(transform.translation, i.0.translation) <= (radius + i.1) as f64 {
                    radius = rng.random_range(radius_range.0..=radius_range.1);
                    transform = random_transform(x_range, y_range);
                }
            }
            info.push((transform, radius));
            break;
        }
        spawn_planet(commands, meshes, materials, radius, id, random_color(), transform);
    }
}

pub fn spawn_planets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    
    spawn_random_planets(&mut commands, &mut meshes, &mut materials, (20, 350), (1..=35).collect(), (-10000, 10000), (-10000, 10000));

    
    /*
    TEMPLATE    
    spawn_planet(
        &mut commands, 
        &mut meshes, 
        &mut materials, 
        rx, 
        id, 
        Color::srgb(0.0, 0.0, 0.0), 
        Transform::from_xyz(0.0, 0.0, 0.0));
     */
}
