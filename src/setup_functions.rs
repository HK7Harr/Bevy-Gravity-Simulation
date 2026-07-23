
use crate::helpers::*;
use crate::components::Planet;
use crate::internal_imports::*;

fn spawn_planet(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    radius: i32,
    id: i32,
    color: Color,
    transform: Transform,
) {
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(radius as f32))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: color,
            ..default()
        })),
        transform,
        Planet::new(id, radius, DVec3::ZERO)
    ));
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)),
        PanOrbitCamera::default(),
    ));
}


pub fn setup_lighting(mut commands: Commands) {
    // Uniformly lights every object in the world, regardless of position
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY, // flat, even lighting
            shadow_maps_enabled: false, // turn off for now, huge scenes can make shadow cascades expensive/weird
            ..default()
        },
        Transform::default().looking_at(Vec3::new(-1.0, -1.0, -1.0), Vec3::Y),
    ));

    // Fills in the unlit side of spheres so they aren't pure black
    commands.insert_resource(GlobalAmbientLight {
        color: Color::WHITE,
        brightness: B,
        ..default()
    });
}


fn random_material() -> StandardMaterial {
    let mut rng = rand::rng();
    StandardMaterial {
        base_color: Color::srgb(rng.random(), rng.random(), rng.random()),
        ..default()
    }
}

fn random_color() -> Color {
    let mut rng = rand::rng();
    Color::srgb(rng.random(), rng.random(), rng.random())
}

fn random_transform(
    x: (i32, i32),
    y: (i32, i32),
    z: (i32, i32)
) -> Transform {
    let mut rng = rand::rng();
    
    Transform::from_xyz( rng.random_range(x.0..=x.1) as f32, rng.random_range(y.0..=y.1) as f32, rng.random_range(z.0..=z.1) as f32)
}

fn spawn_random_planets(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    ids: Vec<i32>, // the length of the vec is the amount of planets
) {
    let mut rng = rand::rng();
    let mut info: Vec<(Transform, i32)> = Vec::new();
    for id in ids {
        let mut radius = rng.random_range(MIN_R..=MAX_R);
        let mut transform = random_transform((MIN_WS, MAX_WS), (MIN_WS, MAX_WS), (MIN_WS, MAX_WS));
        loop {
            if info.is_empty() == true {
                info.push((transform, radius));
                break;
            }
            for i in &info {
                if com_length_wu_3d(transform.translation, i.0.translation) <= (radius + i.1) as f64 {
                    radius = rng.random_range(MIN_R..=MAX_R);
                    transform = random_transform((MIN_WS, MAX_WS), (MIN_WS, MAX_WS), (MIN_WS, MAX_WS));
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
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    
    spawn_random_planets(&mut commands, &mut meshes, &mut materials, (1..=100).collect());

    
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
