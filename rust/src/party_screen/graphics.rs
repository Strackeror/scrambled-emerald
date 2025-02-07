use core::marker::PhantomData;
use core::mem;

use super::{data, sleep};
use crate::pokeemerald::{self, *};
use crate::resources::{LoadedResource, Resource};

pub struct Tileset<'a> {
    char_base: u8,
    offset: u16,
    buffer: &'a LoadedResource,
    loaded: bool,
}

impl<'a> Tileset<'a> {
    pub const fn new(char_base: u8, offset: u16, buffer: &'a LoadedResource) -> Self {
        Tileset { char_base, offset, buffer, loaded: false }
    }

    fn load(&mut self, bg_index: u8) {
        if self.loaded {
            return;
        }
        self.loaded = true;
        unsafe {
            LoadBgTiles(bg_index as _, self.buffer.buffer(), self.buffer.len() as _, self.offset);
        }
    }
}

pub struct Palette<'a> {
    pub offset: usize,
    pub buffer: &'a Resource,
}

impl<'a> Palette<'a> {
    fn load(&mut self) {
        unsafe {
            LoadCompressedPalette(
                self.buffer.buffer() as *const _,
                self.offset as _,
                self.buffer.len() as u32,
            );
        }
    }
}

pub struct Tilemap<'a> {
    pub map: u8,
    pub buffer: &'a LoadedResource,
}

pub struct Background<'a> {
    bg_index: u8,
    _phantom_data: PhantomData<&'a ()>,
}

impl<'a> Background<'a> {
    pub async fn load(
        index: u8,
        priority: u8,
        tileset: &mut Tileset<'a>,
        tilemap: &mut Tilemap<'a>,
        palette: &mut Palette<'a>,
    ) -> Background<'a> {
        let mut template: BgTemplate = BgTemplate::default();
        template.set_bg(index as _);
        template.set_charBaseIndex(tileset.char_base as _);
        template.set_mapBaseIndex(tilemap.map as _);
        template.set_baseTile(tileset.offset as _);
        template.set_paletteMode(0);
        template.set_priority(priority as _);
        template.set_screenSize(0);

        unsafe {
            InitBgFromTemplate(&raw const template);
            SetBgTilemapBuffer(index as _, tilemap.buffer.buffer());
            LoadBgTilemap(index as _, tilemap.buffer.buffer(), tilemap.buffer.len() as _, 0);
        };
        sleep(1).await;

        tileset.load(index);
        sleep(1).await;

        palette.load();
        sleep(1).await;

        Background { bg_index: index, _phantom_data: PhantomData }
    }

    pub fn show(&self) {
        unsafe {
            ShowBg(self.bg_index as _);
        }
    }
}

pub struct SpriteHandle {
    sprite_index: u16,
}
pub unsafe fn g_sprite(id: u8) -> *mut pokeemerald::Sprite {
    (*(&raw mut gSprites)).as_mut_ptr().add(id as _)
}

impl SpriteHandle {
    pub fn set_pos(&mut self, x: i16, y: i16) {
        unsafe {
            let sprite = &mut *(g_sprite(self.sprite_index as _));
            sprite.x = x;
            sprite.y = y;
        }
    }
}

pub struct PokemonSpritePic {
    sprite: SpriteHandle,
}

impl PokemonSpritePic {
    pub fn new(poke: &data::Pokemon, slot: u8) -> PokemonSpritePic {
        let species = poke.species();
        let personality = poke.personality();
        let shiny = poke.shiny();
        unsafe {
            let sprite_index = CreateMonPicSprite_Affine(
                species,
                shiny as _,
                personality,
                MON_PIC_AFFINE_FRONT as _,
                0,
                0,
                slot,
                TAG_NONE as _,
            );
            PokemonSpritePic { sprite: SpriteHandle { sprite_index } }
        }
    }

    pub fn sprite(&mut self) -> &mut SpriteHandle {
        &mut self.sprite
    }
}

impl Drop for PokemonSpritePic {
    fn drop(&mut self) {
        unsafe {
            FreeAndDestroyMonPicSprite(self.sprite.sprite_index);
        }
    }
}
