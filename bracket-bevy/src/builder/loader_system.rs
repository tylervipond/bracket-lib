use crate::{
    consoles::SparseConsole, fonts::FontStore, BTermBuilder, BracketContext, SimpleConsole,
    TerminalLayer,
};
use bevy::{
    asset::Handle,
    prelude::{AssetServer, Assets, Camera2dBundle, Commands, Component, Mesh, Res, ResMut},
    render::texture::{
        Image, ImageAddressMode, ImageFilterMode, ImageLoaderSettings, ImageSampler,
        ImageSamplerDescriptor,
    },
    sprite::ColorMaterial,
};

#[derive(Component)]
pub struct BracketCamera;

pub(crate) fn load_terminals(
    context: Res<BTermBuilder>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    if context.with_ortho_camera {
        commands
            .spawn(Camera2dBundle::default())
            .insert(BracketCamera);
    }

    // Setup the new context
    let mut new_context = BracketContext::new(context.palette.clone());
    new_context.scaling_mode = context.scaling_mode;

    // Load the fonts
    for font in context.fonts.iter() {
        let texture_handle: Handle<Image> = asset_server.load_with_settings(
            &font.filename,
            |settings: &mut ImageLoaderSettings| {
                settings.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
                    label: None,
                    address_mode_u: ImageAddressMode::ClampToEdge,
                    address_mode_v: ImageAddressMode::ClampToEdge,
                    address_mode_w: ImageAddressMode::ClampToEdge,
                    mag_filter: ImageFilterMode::Nearest,
                    min_filter: ImageFilterMode::Nearest,
                    mipmap_filter: ImageFilterMode::Nearest,
                    lod_min_clamp: 0.0,
                    lod_max_clamp: 32.0,
                    compare: None,
                    anisotropy_clamp: 1,
                    border_color: None,
                })
            },
        );
        // let texture_handle = asset_server.load(&font.filename);
        let material_handle = materials.add(ColorMaterial::from(texture_handle.clone()));
        new_context.fonts.push(FontStore::new(
            texture_handle,
            material_handle,
            font.chars_per_row,
            font.n_rows,
            font.font_height_pixels,
        ));
    }

    // Setup the consoles
    for (idx, terminal) in context.layers.iter().enumerate() {
        match terminal {
            TerminalLayer::Simple {
                font_index,
                width,
                height,
                features,
            } => {
                let mut console = SimpleConsole::new(*font_index, *width, *height);
                console.initialize(&new_context.fonts, &mut meshes, features);
                console.spawn(
                    &mut commands,
                    new_context.fonts[*font_index].material_handle.clone(),
                    idx,
                );
                new_context.terminals.lock().push(Box::new(console));
            }
            TerminalLayer::Sparse {
                font_index,
                width,
                height,
                features,
            } => {
                let mut console = SparseConsole::new(*font_index, *width, *height);
                console.initialize(&new_context.fonts, &mut meshes, features);
                console.spawn(
                    &mut commands,
                    new_context.fonts[*font_index].material_handle.clone(),
                    idx,
                );
                new_context.terminals.lock().push(Box::new(console));
            }
        }
    }

    // Clean up after the building process
    commands.remove_resource::<BTermBuilder>();
    commands.insert_resource(new_context);
}
