use crate::{BracketCamera, BracketContext, TerminalScalingMode};
use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    ecs::event::Events,
    prelude::*,
    sprite::Mesh2dHandle,
    window::WindowResized,
};

use super::{BracketMesh, ScreenScaler};

pub(crate) fn update_consoles(
    mut ctx: ResMut<BracketContext>,
    mut meshes: ResMut<Assets<Mesh>>,
    mesh_handle_query: Query<(&BracketMesh, &Mesh2dHandle)>,
    scaler: Res<ScreenScaler>,
) {
    let mut new_meshes: Vec<(Mesh2dHandle, Mesh2dHandle, bool)> = Vec::new();
    {
        let mut terms = ctx.terminals.lock();
        for (id, handle) in mesh_handle_query.iter() {
            let terminal_id = id.0;
            let new_mesh = terms[terminal_id].new_mesh(&ctx, &mut meshes, &scaler);
            if let Some(new_mesh) = new_mesh {
                let old_mesh = handle.clone();
                new_meshes.push((old_mesh, new_mesh.into(), false));
            }
        }
    }

    new_meshes
        .drain(..)
        .for_each(|m| ctx.mesh_replacement.push(m));
}

// can this be combined with update_consoles?
pub(crate) fn replace_meshes(
    mut ctx: ResMut<BracketContext>,
    mut ev_asset: EventReader<AssetEvent<Mesh>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut update_mesh: Query<&mut Mesh2dHandle, With<BracketMesh>>,
) {
    for ev in ev_asset.read() {
        if let AssetEvent::Added { id } = ev {
            for (old, new, done) in ctx.mesh_replacement.iter_mut() {
                if *id == new.0.id() {
                    let old_id = old.0.id();
                    update_mesh.for_each_mut(|mut m| {
                        if old_id == m.0.id() {
                            *m = new.clone();
                        }
                    });
                    *done = true;
                }
            }
        }
    }

    for (old, _, _) in ctx.mesh_replacement.iter().filter(|(_, _, done)| *done) {
        meshes.remove(old.0.clone());
    }
    ctx.mesh_replacement.retain(|(_, _, done)| !done);
}

pub(crate) fn update_timing(mut ctx: ResMut<BracketContext>, diagnostics: Res<DiagnosticsStore>) {
    if let Some(fps_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(fps_avg) = fps_diagnostic.measurement() {
            ctx.fps = fps_avg.value.round();
        }
    }

    if let Some(frame_time) = diagnostics.get(FrameTimeDiagnosticsPlugin::FRAME_TIME) {
        if let Some(frame_time_avg) = frame_time.measurement() {
            ctx.frame_time_ms = (frame_time_avg.value * 1000.0).round();
        }
    }
}

pub(crate) fn window_resize(
    mut context: ResMut<BracketContext>,
    resize_event: Res<Events<WindowResized>>,
    mut scaler: ResMut<ScreenScaler>,
) {
    let mut reader = resize_event.get_reader();
    let terminal_pixel_size = context.get_pixel_size();
    let largest_font = context.largest_font();
    for e in reader.read(&resize_event) {
        scaler.set_screen_size(e.width, e.height);
        if let TerminalScalingMode::ResizeTerminals = context.scaling_mode {
            context.resize_terminals(&scaler);
        }
        scaler.recalculate(terminal_pixel_size, largest_font);
    }
}

pub(crate) fn apply_all_batches(mut context: ResMut<BracketContext>) {
    context.render_all_batches();
}

pub(crate) fn update_mouse_position(
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<BracketCamera>>,
    mut context: ResMut<BracketContext>,
    scaler: Res<ScreenScaler>,
) {
    let window = window_query.single();
    let (camera, camera_transform) = camera_query.single();

    if let Some(screen_pos) = window.cursor_position() {
        let window_size = Vec2::new(window.width(), window.height());
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
        let world_pos: Vec2 = world_pos.truncate();

        let mouse_position = (world_pos.x, world_pos.y);
        context.set_mouse_pixel_position(mouse_position, &scaler);
    }
}
