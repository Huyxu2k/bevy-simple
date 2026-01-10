use bevy::{prelude::*, sprite::Anchor};
use bevy_procedural_tilemaps::prelude::*;

use crate::map::titlemap::TITLE_MAP;

#[derive(Clone)]
pub struct SpawnableAsset {
    sprite_name: &'static str,
    grid_offset: GridDelta,
    offset: Vec3,
    component_spawner: fn(&mut EntityCommands),
}

impl SpawnableAsset {
    pub fn new(sprite_name: &'static str) -> SpawnableAsset {
        SpawnableAsset {
            sprite_name,
            grid_offset: GridDelta::new(0, 0, 0),
            offset: Vec3::ZERO,
            component_spawner: |_| {},
        }
    }
    
    pub fn with_grid_offset(mut self, offset: GridDelta) -> SpawnableAsset{
        self.grid_offset = offset;
        self
    }
}


#[derive(Clone)]
pub struct TilemapHandles{
    pub image: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}
impl TilemapHandles {
    pub fn sprite(&self, atlas_index: usize) -> Sprite{
        Sprite::from_atlas_image(
            self.image.clone(), 
            TextureAtlas::from(self.layout.clone()).with_index(atlas_index),
        )
    }
}

pub fn prepare_tilemap_handles(
    asset_server: &Res<AssetServer>,
    atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    assets_directory: &str,
    tilemap_file: &str,
) -> TilemapHandles {
    let image = asset_server.load::<Image>(format!("{assets_directory}/{tilemap_file}"));
    let mut layout = TextureAtlasLayout::new_empty(TITLE_MAP.atlas_size());
    for index in 0..TITLE_MAP.sprites.len()  {
        layout.add_texture(TITLE_MAP.sprite_rect(index));
    }

    let layout = atlas_layouts.add(layout);

    TilemapHandles { image, layout }
}

pub fn load_assets(
    tilemap_handles: &TilemapHandles,
    assets_definitions: Vec<Vec<SpawnableAsset>>,
) -> ModelsAssets<Sprite> {
    let mut models_assets: ModelsAssets<Sprite> = ModelsAssets::<Sprite>::new();
    for (model_index, assets) in assets_definitions.into_iter().enumerate() {
        for asset_def in assets {
            let SpawnableAsset {
                sprite_name,
                grid_offset,
                offset,
                component_spawner,
            } = asset_def;

            let Some(atlas_index) = TITLE_MAP.sprite_index(sprite_name) else {
                panic!("Unknown atlas sprite '{}'",sprite_name);
            };

            models_assets.add(
                model_index, 
                ModelAsset { 
                    assets_bundle: tilemap_handles.sprite(atlas_index), 
                    spawn_commands: component_spawner, 
                    grid_offset, 
                    world_offset: offset 
                }
            );
        }
    }
    models_assets
}