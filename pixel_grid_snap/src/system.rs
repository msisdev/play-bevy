use bevy::{
    camera::RenderTarget,
    color::palettes::css::GRAY,
    prelude::*,
    render::render_resource::{
        Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
    },
    window::WindowResized,
};
use crate::component;
use crate::config;

pub fn setup_sprite(mut commands: Commands, asset_server: Res<AssetServer>) {
    // The sample sprite that will be rendered to the pixel-perfect canvas
    commands.spawn((
        Sprite::from_image(asset_server.load("pixel/bevy_pixel_dark.png")),
        Transform::from_xyz(-45., 20., 2.),
        component::Rotate,
        config::PIXEL_PERFECT_LAYERS,
    ));

    // The sample sprite that will be rendered to the high-res "outer world"
    commands.spawn((
        Sprite::from_image(asset_server.load("pixel/bevy_pixel_light.png")),
        Transform::from_xyz(-45., -20., 2.),
        component::Rotate,
        config::HIGH_RES_LAYERS,
    ));
}

/// Spawns a capsule mesh on the pixel-perfect layer.
pub fn setup_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Mesh2d(meshes.add(Capsule2d::default())),
        MeshMaterial2d(materials.add(Color::BLACK)),
        Transform::from_xyz(25., 0., 2.).with_scale(Vec3::splat(32.)),
        component::Rotate,
        config::PIXEL_PERFECT_LAYERS,
    ));
}

pub fn setup_camera(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let canvas_size = Extent3d {
        width: config::RES_WIDTH,
        height: config::RES_HEIGHT,
        ..default()
    };

    // This Image serves as a canvas representing the low-resolution game screen
    let mut canvas = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size: canvas_size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    // Fill image.data with zeroes
    canvas.resize(canvas_size);

    let image_handle = images.add(canvas);

    // This camera renders whatever is on `PIXEL_PERFECT_LAYERS` to the canvas
    commands.spawn((
        Camera2d,
        Camera {
            // Render before the "main pass" camera
            order: -1,
            clear_color: ClearColorConfig::Custom(GRAY.into()),
            ..default()
        },
        RenderTarget::Image(image_handle.clone().into()),
        Msaa::Off,
        component::InGameCamera,
        config::PIXEL_PERFECT_LAYERS,
    ));

    // Spawn the canvas
    commands.spawn((Sprite::from_image(image_handle), component::Canvas, config::HIGH_RES_LAYERS));

    // The "outer" camera renders whatever is on `HIGH_RES_LAYERS` to the screen.
    // here, the canvas and one of the sample sprites will be rendered by this camera
    commands.spawn((Camera2d, Msaa::Off, component::OuterCamera, config::HIGH_RES_LAYERS));
}

/// Rotates entities to demonstrate grid snapping.
pub fn rotate(time: Res<Time>, mut transforms: Query<&mut Transform, With<component::Rotate>>) {
    for mut transform in &mut transforms {
        let dt = time.delta_secs();
        transform.rotate_z(dt);
    }
}

/// Scales camera projection to fit the window (integer multiples only).
pub fn fit_canvas(
    mut resize_messages: MessageReader<WindowResized>,
    mut projection: Single<&mut Projection, With<component::OuterCamera>>,
) {
    let Projection::Orthographic(projection) = &mut **projection else {
        return;
    };
    for window_resized in resize_messages.read() {
        let h_scale = window_resized.width / config::RES_WIDTH as f32;
        let v_scale = window_resized.height / config::RES_HEIGHT as f32;
        projection.scale = 1. / h_scale.min(v_scale).round();
    }
}
