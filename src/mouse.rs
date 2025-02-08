use bevy::{input::touch::TouchPhase, prelude::*, utils::HashMap, window::PrimaryWindow};

use bevy::render::camera::ViewportConversionError;

#[derive(Default)]
pub struct MouseCoordsPlugin;

impl Plugin for MouseCoordsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MouseWorldCoords>()
            .init_resource::<TouchTracker>()
            .add_systems(
                Update,
                (update_mouse_world_coords, update_touch_world_coords),
            );
    }
}

/// We will store the world position of the mouse cursor here.
#[derive(Resource, Default)]
pub struct MouseWorldCoords {
    curr_pos: Vec2,
    prev_pos: Vec2,
    curr_screen_pos: Vec2,
    prev_screen_pos: Vec2,
}

#[derive(Default)]
pub struct TouchInfo {
    pub initial_pos: Vec2,
    pub curr_pos: Vec2,
}

#[derive(Resource, Default)]
pub struct TouchTracker {
    touches: HashMap<u64, TouchInfo>,
}

impl TouchTracker {
    pub fn touch(&mut self, id: u64, pos: Vec2) {
        if let Some(touch) = self.touches.get_mut(&id) {
            touch.curr_pos = pos;
        } else {
            self.touches.insert(
                id,
                TouchInfo {
                    curr_pos: pos,
                    initial_pos: pos,
                },
            );
        }
    }

    pub fn end_touch(&mut self, id: u64) {
        self.touches.remove(&id);
    }

    pub fn get_touch(&self, id: u64) -> Option<&TouchInfo> {
        self.touches.get(&id)
    }
}

impl MouseWorldCoords {
    pub fn get_pos(&self) -> Vec2 {
        self.curr_pos
    }

    pub fn get_screen_pos(&self) -> Vec2 {
        self.curr_screen_pos
    }

    pub fn get_prev_pos(&self) -> Vec2 {
        self.prev_pos
    }

    /// Gets the difference between the last two reported positions.
    pub fn get_diff(&self) -> Vec2 {
        self.curr_pos - self.prev_pos
    }

    pub(crate) fn set_pos(&mut self, new_pos: Vec2, new_screen_pos: Vec2) {
        self.prev_pos = self.curr_pos;
        self.curr_pos = new_pos;
        self.prev_screen_pos = self.curr_screen_pos;
        self.curr_screen_pos = new_screen_pos;
    }
}

/// Used to help identify our main camera
#[derive(Component)]
pub struct MainCamera;

fn update_mouse_world_coords(
    mut mouse_world_coords: ResMut<MouseWorldCoords>,
    // query to get the window (so we can read the current cursor position)
    q_window: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so Query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // There is only one primary window, so we can similarly get it from the query:
    let window = q_window.single();

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|coords| screen_to_world_coords(coords, camera, camera_transform).ok())
    {
        mouse_world_coords.set_pos(world_position, window.cursor_position().unwrap());
    }
}

fn update_touch_world_coords(
    mut touch_events: EventReader<TouchInput>,
    mut touch_tracker: ResMut<TouchTracker>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = q_camera.single();

    for touch_event in touch_events.read() {
        match touch_event.phase {
            TouchPhase::Started | TouchPhase::Moved => {
                if let Ok(pos) =
                    screen_to_world_coords(touch_event.position, camera, camera_transform)
                {
                    touch_tracker.touch(touch_event.id, pos);
                }
            }
            TouchPhase::Ended | TouchPhase::Canceled => {
                touch_tracker.end_touch(touch_event.id);
            }
        }
    }
}

pub fn screen_to_world_coords(
    coords: Vec2,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Result<Vec2, ViewportConversionError> {
    camera
        .viewport_to_world(camera_transform, coords)
        .map(|ray| ray.origin.truncate())
}
