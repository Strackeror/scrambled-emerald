#![allow(unused)]

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::marker::PhantomData;
use core::ops::{Add, BitOr, Deref, Mul};

use derive_more::{Add, Constructor, Div, Mul, Sub};

use super::{data, sleep};
use crate::charmap::Pkstr;
use crate::mgba_warn;
use crate::pokeemerald::{self, *};
use crate::resources::{Buffer, StaticWrapper};

pub fn set_gpu_registers(list: &[(u32, &[u32])]) {
    for (offset, flags) in list {
        let flag = flags.iter().fold(0u32, BitOr::bitor);
        unsafe { SetGpuReg(*offset as _, flag as _) };
    }
}

#[derive(Debug, Clone, Copy, Add, Mul, Div, Sub, Constructor)]
pub struct Vec2D<T> {
    pub x: T,
    pub y: T,
}

impl Vec2D<u8> {
    pub const ZERO: Self = Vec2D::new(0, 0);
    pub fn tile_to_pixel(self) -> Vec2D<i16> {
        Vec2D {
            x: self.x as i16 * 8,
            y: self.y as i16 * 8,
        }
    }
}
impl Vec2D<u16> {
    pub const ZERO: Self = Vec2D::new(0, 0);
}

impl<T: Mul + Copy> Vec2D<T> {
    pub fn size(&self) -> T::Output {
        self.x * self.y
    }
}

#[derive(Debug, Clone, Copy, Add, Mul, Div, Sub, Constructor)]
pub struct Rect<T> {
    pub x: T,
    pub y: T,
    pub width: T,
    pub height: T,
}

impl<T: Copy> Rect<T> {
    pub const fn from_vecs(pos: Vec2D<T>, size: Vec2D<T>) -> Self {
        Rect {
            x: pos.x,
            y: pos.y,
            width: size.x,
            height: size.y,
        }
    }

    pub const fn dim(&self) -> Vec2D<T> {
        Vec2D {
            x: self.width,
            y: self.height,
        }
    }

    pub const fn pos(&self) -> Vec2D<T> {
        Vec2D {
            x: self.x,
            y: self.y,
        }
    }
}

impl Rect<u8> {
    pub fn tile_to_pixel(self) -> Rect<u16> {
        Rect {
            x: self.x as u16 * 8,
            y: self.y as u16 * 8,
            width: self.width as u16 * 8,
            height: self.height as u16 * 8,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct BgPalette {
    index: u8,
}
#[derive(Clone, Copy, Debug)]
pub struct ObjPalette {
    index: u8,
}

pub fn load_bg_palette(index: u8, data: &[u16]) -> BgPalette {
    unsafe {
        let size = data.len() * size_of::<u16>();
        let data = data.as_ptr().cast();
        LoadPalette(data, BG_PLTT_OFFSET + index as u32 * 16, size as u32);
        BgPalette { index }
    }
}

pub fn load_obj_palette(index: u8, data: &[u16]) -> ObjPalette {
    unsafe {
        let size = data.len() * size_of::<u16>();
        let data = data.as_ptr().cast();
        LoadPalette(data, OBJ_PLTT_OFFSET + index as u32 * 16, size as u32);
        ObjPalette { index }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct TileBitmap4bpp(pub [u8; 32]);
#[derive(Clone, Copy, Debug)]
pub struct Tile4bpp(pub u16);
#[derive(Clone, Copy, Debug)]
pub struct TilePlain(pub u8);

#[derive(Clone, Copy, Debug)]
pub struct Tileset<Buf: Buffer<TileBitmap4bpp>> {
    pub char_base: u16,
    pub offset: u16,
    pub tiles: Buf,
    pub palette: BgPalette,
}

pub struct Tilemap<Buf: Buffer<Tile4bpp>> {
    pub map: u16,
    pub buffer: Buf,
}

#[derive(Debug, Clone, Copy)]
pub enum BackgroundIndex {
    Background0 = 0,
    Background1 = 1,
    Background2 = 2,
    Background3 = 3,
}

#[derive(Debug, Clone, Copy)]
pub struct BgHandle<'a>(BackgroundIndex, PhantomData<&'a ()>);
impl Deref for BgHandle<'_> {
    type Target = BackgroundIndex;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct Background<Set, Map>
where
    Set: Buffer<TileBitmap4bpp>,
    Map: Buffer<Tile4bpp>,
{
    index: BackgroundIndex,
    tileset_buffer: Set,
    tilemap_buffer: Map,
}

impl<Set, Map> Background<Set, Map>
where
    Set: Buffer<TileBitmap4bpp>,
    Map: Buffer<Tile4bpp>,
{
    pub async fn load(
        index: BackgroundIndex,
        priority: u16,
        tileset: Tileset<Set>,
        tilemap: Tilemap<Map>,
    ) -> Self {
        let mut template: BgTemplate = BgTemplate::default();
        template.set_bg(index as u16);
        template.set_charBaseIndex(tileset.char_base);
        template.set_mapBaseIndex(tilemap.map);
        template.set_baseTile(tileset.offset);
        template.set_paletteMode(0);
        template.set_priority(priority);
        template.set_screenSize(0);

        unsafe { InitBgFromTemplate(&raw const template) };
        let tiles = &tileset.tiles;
        unsafe {
            LoadBgTiles(
                index as u32,
                tiles.get().as_ptr().cast(),
                tiles.size_bytes() as _,
                tileset.offset,
            )
        };

        let map = &tilemap.buffer;
        unsafe { SetBgTilemapBuffer(index as _, map.as_ptr().cast_mut().cast()) };
        unsafe { LoadBgTilemap(index as _, map.as_ptr().cast(), map.size_bytes() as _, 0) };
        sleep(1).await;

        Background {
            index,
            tileset_buffer: tileset.tiles,
            tilemap_buffer: tilemap.buffer,
        }
    }

    pub fn handle(&self) -> BgHandle {
        BgHandle(self.index, PhantomData)
    }

    pub fn show(&self) {
        unsafe {
            ShowBg(self.index as _);
        }
    }

    pub fn copy_tilemap_to_vram(&self) {
        unsafe {
            CopyBgTilemapBufferToVram(self.index as _);
        }
    }

    pub fn set_pos(&self, x: u8, y: u8) {
        unsafe {
            ChangeBgX(self.index as _, BG_COORD_SET as _, x);
            ChangeBgY(self.index as _, BG_COORD_SET as _, y);
        }
    }

    pub fn fill(&self, rect: Rect<u8>, tile_index: u16, palette: BgPalette) {
        unsafe {
            let bg = self.index as u32;
            FillBgTilemapBufferRect(
                bg,
                tile_index,
                rect.x,
                rect.y,
                rect.width,
                rect.height,
                palette.index,
            );
            ScheduleBgCopyTilemapToVram(self.index as _);
        }
    }
}

pub struct SpriteHandle {
    sprite_index: u16,
}

#[unsafe(link_section = ".ewram")]
static G_SPRITES: StaticWrapper<pokeemerald::Sprite> =
    StaticWrapper::new_from_arr(&raw mut gSprites);
impl SpriteHandle {
    pub fn set_pos(&mut self, pos: Vec2D<i16>) {
        let mut sprite = G_SPRITES.index_mut(self.sprite_index as usize);
        sprite.x = pos.x;
        sprite.y = pos.y;
    }
    pub fn set_palette(&mut self, palette: u16) {
        let mut sprite = G_SPRITES.index_mut(self.sprite_index as usize);
        sprite.oam.set_paletteNum(palette);
    }
    pub fn set_priority(&self, priority: u8) {
        let mut sprite = G_SPRITES.index_mut(self.sprite_index as usize);
        sprite.oam.set_priority(priority as _);
    }
    pub fn set_invisible(&mut self, invisible: bool) {
        let mut sprite = G_SPRITES.index_mut(self.sprite_index as usize);
        sprite.set_invisible(invisible.then_some(1).unwrap_or(0));
    }
    pub fn animate(&mut self) {
        unsafe {
            let mut sprite = G_SPRITES.index_mut(self.sprite_index as usize);
            AnimateSprite(&raw mut *sprite);
        }
    }
    pub fn request_copy(&self) {
        unsafe {
            let sprite = G_SPRITES.index_mut(self.sprite_index as usize);
            RequestSpriteFrameImageCopy(0, sprite.oam.tileNum(), sprite.images);
        }
    }
}

#[derive(Clone, Copy)]
pub struct SpriteAnims {
    anims: *const *const AnimCmd,
    affine_anims: *const *const AffineAnimCmd,
}
unsafe impl Sync for SpriteAnims {}
pub static DUMMY_SPRITE_ANIMS: SpriteAnims = unsafe {
    SpriteAnims {
        anims: gDummySpriteAnimTable.as_ptr(),
        affine_anims: gDummySpriteAffineAnimTable.as_ptr(),
    }
};

pub struct SpriteImage<Buf: Buffer<TileBitmap4bpp>> {
    pub buf: Buf,
    pub size: u32,
}

pub struct Sprite<Img: Buffer<TileBitmap4bpp>> {
    _own: (ObjPalette, Img, Box<SpriteFrameImage>),
    sprite: SpriteHandle,
}

impl<Img: Buffer<TileBitmap4bpp>> Sprite<Img> {
    pub async fn load(image: SpriteImage<Img>, anims: SpriteAnims, palette: ObjPalette) -> Self {
        let mut template = SpriteTemplate::default();
        template.affineAnims = anims.affine_anims;
        template.anims = anims.anims;
        template.callback = Some(SpriteCallbackDummy);

        let mut frame = SpriteFrameImage::default();
        frame.data = image.buf.as_ptr().cast();
        frame.relativeFrames = 0;
        frame.size = image.buf.size_bytes() as u16;
        let frame = Box::new(frame);
        template.images = &raw const *frame;

        let mut oam = OamData::default();
        oam.set_size((image.size >> 2) & 0b11);
        oam.set_shape(image.size & 0b11);

        template.oam = &raw const oam;
        template.tileTag = TAG_NONE as _;
        template.paletteTag = TAG_NONE as _;

        let sprite_index = unsafe { CreateSprite(&raw const template, 0, 0, 0) };
        let mut sprite = SpriteHandle {
            sprite_index: sprite_index as _,
        };
        sprite.set_palette(palette.index as u16);
        sprite.request_copy();
        Sprite {
            _own: (palette, image.buf, frame),
            sprite,
        }
    }

    #[allow(unused)]
    pub fn debug(&self) {
        mgba_warn!("{:?} {:?}", self._own.1.as_ptr(), &raw const *self._own.2);
    }

    pub fn handle(&mut self) -> &mut SpriteHandle {
        &mut self.sprite
    }
}
impl<T: Buffer<TileBitmap4bpp>> Drop for Sprite<T> {
    fn drop(&mut self) {
        unsafe {
            DestroySpriteAndFreeResources(
                &raw mut *&mut *G_SPRITES.index_mut(self.sprite.sprite_index as _),
            );
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
            PokemonSpritePic {
                sprite: SpriteHandle { sprite_index },
            }
        }
    }

    pub fn handle(&mut self) -> &mut SpriteHandle {
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

#[derive(Clone, Copy)]
pub struct TextPrinterParams {
    pub font: u8,
    pub speed: u8,
    pub fg_color: u8,
    pub bg_color: u8,
    pub shadow_color: u8,
    pub letter_spacing: u8,
    pub line_spacing: u8,
}

impl TextPrinterParams {
    pub fn font(font_id: u8) -> Self {
        let font_info = unsafe { &*gFonts.offset(font_id as _) };
        TextPrinterParams {
            font: font_id,
            speed: 0,
            fg_color: font_info.fgColor(),
            bg_color: font_info.bgColor(),
            shadow_color: font_info.shadowColor(),
            line_spacing: font_info.lineSpacing,
            letter_spacing: font_info.letterSpacing,
        }
    }
}

pub struct WindowHandle {
    index: u8,
}

impl WindowHandle {
    pub fn fill(&self, fill: u8) {
        unsafe { FillWindowPixelBuffer(self.index.into(), fill) };
    }

    pub fn fill_rect(&self, fill: u8, rect: Rect<u16>) {
        let Rect {
            x,
            y,
            width,
            height,
        } = rect;
        unsafe {
            FillWindowPixelRect(self.index.into(), fill, x, y, width, height);
        }
    }

    pub fn clear(&self) {
        unsafe { ClearWindowTilemap(self.index as _) }
    }

    pub fn put_tilemap(&self) {
        unsafe { PutWindowTilemap(self.index.into()) }
    }

    pub fn copy_to_vram(&self) {
        unsafe { CopyWindowToVram(self.index.into(), COPYWIN_FULL) };
    }

    pub fn print_text(&self, text: &Pkstr, pos: Vec2D<u8>, font: TextPrinterParams) {
        let mut template = TextPrinterTemplate {
            currentChar: text.as_ptr(),
            windowId: self.index,
            fontId: font.font,
            x: pos.x,
            y: pos.y,
            currentX: pos.x,
            currentY: pos.y,
            letterSpacing: font.letter_spacing,
            lineSpacing: font.line_spacing,
            _bitfield_1: TextPrinterTemplate::new_bitfield_1(
                0,
                font.fg_color,
                font.bg_color,
                font.shadow_color,
            ),
            _bitfield_align_1: [],
            __bindgen_padding_0: 0,
        };

        unsafe { AddTextPrinter(&raw mut template, font.speed, None) };
    }

    pub fn blit_bitmap(&self, pixels: &[u8], rect: Rect<u16>) {
        unsafe {
            BlitBitmapToWindow(
                self.index as _,
                pixels.as_ptr(),
                rect.x,
                rect.y,
                rect.width,
                rect.height,
            )
        };
    }
    pub fn copy_tilemap(&self, tileset: &[TileBitmap4bpp], tilemap: &[TilePlain], rect: Rect<u8>) {
        let mut buffer = Vec::with_capacity(
            rect.width as usize * rect.height as usize * size_of::<TileBitmap4bpp>(),
        );
        for y in 0..rect.height {
            for x in 0..rect.width {
                let offset = y * rect.width + x;
                let tile = tilemap[offset as usize].0;
                buffer.extend_from_slice(&tileset[tile as usize].0);
            }
        }
        self.blit_bitmap(&buffer, rect.tile_to_pixel());
    }
}

pub struct Window {
    handle: WindowHandle,
}

impl<'a> Deref for Window {
    type Target = WindowHandle;
    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}

impl Window {
    pub fn create(bg: BgHandle<'_>, rect: Rect<u8>, palette: BgPalette, base_block: u16) -> Window {
        let mut window_template = WindowTemplate::default();
        window_template.baseBlock = base_block;
        window_template.bg = bg.0 as _;
        window_template.paletteNum = palette.index;
        window_template.tilemapLeft = rect.x;
        window_template.tilemapTop = rect.y;
        window_template.width = rect.width;
        window_template.height = rect.height;
        let index = unsafe { AddWindow(&raw const window_template) };
        let handle = WindowHandle { index: index as u8 };
        Window { handle }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe { RemoveWindow(self.handle.index as _) };
    }
}
