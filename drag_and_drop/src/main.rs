use bevy::prelude::*;

#[derive(Component)]
struct DropArea;

#[derive(Component)]
struct DraggableButton;

#[derive(Component)]
struct GhostPreview;

#[derive(Component)]
struct DroppedElement;

const AREA_SIZE: f32 = 500.0;
const BUTTON_WIDTH: f32 = 150.0;
const BUTTON_HEIGHT: f32 = 50.0;
const ELEMENT_SIZE: f32 = 25.0;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MeshPickingPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let container = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Start,
                ..default()
            },
            Pickable::IGNORE,
        ))
        .id();

    let button = commands
        .spawn((
            DraggableButton,
            Node {
                width: Val::Px(BUTTON_WIDTH),
                height: Val::Px(BUTTON_HEIGHT),
                margin: UiRect::all(Val::Px(10.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(1.0, 0.0, 0.0)),
        ))
        .observe(on_drag_start)
        .observe(on_drag_end)
        .id();

    let button_text = commands
        .spawn((
            Text::new("Drag from me"),
            TextColor(Color::WHITE),
            Pickable::IGNORE,
        ))
        .id();

    commands.entity(button).add_child(button_text);
    commands.entity(container).add_child(button);

    let drop_area = commands
        .spawn((
            DropArea,
            Mesh2d(meshes.add(Rectangle::new(AREA_SIZE, AREA_SIZE))),
            MeshMaterial2d(materials.add(Color::srgb(0.1, 0.4, 0.1))),
            Transform::IDENTITY,
        ))
        .observe(on_drag_enter)
        .observe(on_drag_over)
        .observe(on_drag_drop)
        .observe(on_drag_leave)
        .id();

    let drop_area_text = commands
        .spawn((
            Text2d::new("Drop here"),
            TextFont::from_font_size(50.),
            TextColor(Color::BLACK),
            Pickable::IGNORE,
            Transform::from_translation(Vec3::Z),
        ))
        .id();

    commands.entity(drop_area).add_child(drop_area_text);
}

fn on_drag_start(
    mut event: On<Pointer<DragStart>>,
    mut button_color: Single<&mut BackgroundColor, With<DraggableButton>>,
) { 
    // Changes the button color to orange to indicate dragging has started
    button_color.0 = Color::srgb(1.0, 0.5, 0.0);
    event.propagate(false);
}

fn on_drag_end(
    mut event: On<Pointer<DragEnd>>,
    mut button_color: Single<&mut BackgroundColor, With<DraggableButton>>,
) {
    // Resets the button color to red when dragging ends
    button_color.0 = Color::srgb(1.0, 0.0, 0.0);
    event.propagate(false);
}

fn on_drag_enter(
    mut event: On<Pointer<DragEnter>>,
    button: Single<Entity, With<DraggableButton>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Spawns a translucent ghost preview of the element being dragged
    // when the dragged item enters the drop area
    if event.dragged == *button {
        let Some(position) = event.hit.position else {
            return;
        };
        commands.spawn((
            GhostPreview,
            Mesh2d(meshes.add(Circle::new(ELEMENT_SIZE))),
            MeshMaterial2d(materials.add(Color::srgba(1.0, 1.0, 0.6, 0.5))),
            Transform::from_translation(position + 2. * Vec3::Z),
            Pickable::IGNORE,
        ));
        event.propagate(false);
    }
}

fn on_drag_over(
    mut event: On<Pointer<DragOver>>,
    button: Single<Entity, With<DraggableButton>>,
    mut ghost_transform: Single<&mut Transform, With<GhostPreview>>,
) {
    // Updates the position of the ghost preview to follow the mouse pointer
    // while hovering over the drop area
    if event.dragged == *button {
        let Some(position) = event.hit.position else {
            return;
        };
        ghost_transform.translation = position;
        event.propagate(false);
    }
}

fn on_drag_drop(
    mut event: On<Pointer<DragDrop>>,
    button: Single<Entity, With<DraggableButton>>,
    mut commands: Commands,
    ghost: Single<Entity, With<GhostPreview>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Replaces the ghost preview with the actual dropped element
    // at the drop location
    if event.dropped == *button {
        commands.entity(*ghost).despawn();
        let Some(position) = event.hit.position else {
            return;
        };
        commands.spawn((
            DroppedElement,
            Mesh2d(meshes.add(Circle::new(ELEMENT_SIZE))),
            MeshMaterial2d(materials.add(Color::srgb(1.0, 1.0, 0.6))),
            Transform::from_translation(position + 2. * Vec3::Z),
            Pickable::IGNORE,
        ));
        event.propagate(false);
    }
}

fn on_drag_leave(
    mut event: On<Pointer<DragLeave>>,
    button: Single<Entity, With<DraggableButton>>,
    mut commands: Commands,
    ghost: Single<Entity, With<GhostPreview>>,
) {
    // Despawns the ghost preview when the dragged item leaves the drop area
    if event.dragged == *button {
        commands.entity(*ghost).despawn();
        event.propagate(false);
    }
}
