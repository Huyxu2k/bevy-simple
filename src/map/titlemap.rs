use bevy::math::{URect, UVec2};

pub struct TitlemapSprite{
    pub name: &'static str,
    pub pixel_x: u32,
    pub pixel_y: u32
}


pub const TITLE_MAP: TitlemapDefinition = TitlemapDefinition {
    title_width: 32,
    title_height: 32,
    atlas_width: 256,
    atlas_height: 320,
    sprites: &[
        TitlemapSprite {
            name: "dirt",
            pixel_x: 128,
            pixel_y: 0,
        }
    ],
};

pub  struct TitlemapDefinition{
    pub title_width: u32,
    pub title_height: u32,
    pub atlas_width: u32,
    pub atlas_height: u32,
    pub sprites: & 'static [TitlemapSprite]
}

impl TitlemapDefinition {
    pub const fn title_size(&self) -> UVec2{
        UVec2::new(self.title_width, self.title_height)
    }

    pub const fn atlas_size(&self) -> UVec2{
        UVec2::new(self.atlas_width, self.atlas_height)
    }
    
    pub fn sprite_index(&self, name: &str) -> Option<usize>{
        self.sprites.iter().position(|item| item.name == name)
    }

    pub fn sprite_rect(&self, index: usize) -> URect{
        let sprite = &self.sprites[index];
        let min = UVec2::new(sprite.pixel_x, sprite.pixel_y);
        URect::from_corners(min, min + self.title_size())
    }
}