#![allow(unused)]

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::any::Any;
use core::array;
use core::marker::PhantomData;
use core::ops::{Add, BitOr, Deref, Mul};
use core::ptr::{null, null_mut};

use derive_more::{Add, Constructor, Div, Mul, Sub};

use super::{data, sleep};
use crate::charmap::Pkstr;
use crate::mgba_warn;
use crate::pokeemerald::{self, *};
use crate::resources::{AllocBuf, Buffer, StaticWrapper};

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
impl ObjPalette {
    pub unsafe fn raw(index: u8) -> Self {
        ObjPalette { index }
    }
}

pub fn load_bg_palette(index: u8, data: &[u16]) -> BgPalette {
    unsafe {
        let size = data.len() * size_of::<u16>();
        let data = data.as_ptr().cast();
        LoadPalette(data, BG_PLTT_OFFSET + index as u32 * 16, size as u32);
        BgPalette { index }
    }
}

pub fn load_bg_palettes<const C: usize>(index: u8, data: &[u16]) -> [BgPalette; C] {
    if data.len() / 16 < C - 1 {
        panic!(
            "Palette not big enough {} colors, {C} palettes expected",
            data.len()
        );
    }

    let size = data.len() * size_of::<u16>();
    let src = data.as_ptr().cast();
    unsafe { LoadPalette(src, BG_PLTT_OFFSET + index as u32 * 16, size as u32) };
    array::from_fn(|i| BgPalette {
        index: index + i as u8,
    })
}

pub fn load_obj_palette(index: u8, data: &[u16]) -> ObjPalette {
    unsafe {
        let size = data.len() * size_of::<u16>();
        let data = data.as_ptr().cast();
        LoadPalette(data, OBJ_PLTT_OFFSET + index as u32 * 16, size as u32);
        ObjPalette { index }
    }
}

pub fn load_user_window_gfx(bg: BgHandle, offset: u16, pal: u8) -> TilesetHandle {
    let char_base = unsafe { GetBgAttribute(bg.0 as _, BG_ATTR_CHARBASEINDEX) };
    unsafe {
        LoadUserWindowBorderGfxOnBg(bg.0 as u8, offset, pal * 16);
    }
    TilesetHandle {
        char_base,
        offset,
        palette: BgPalette { index: pal },
    }
}

pub fn load_msg_box_gfx(bg: BgHandle, offset: u16, pal: u8) -> TilesetHandle {
    let char_base = unsafe { GetBgAttribute(bg.0 as _, BG_ATTR_CHARBASEINDEX) };
    unsafe { LoadBgTiles(bg.0 as _, gMessageBox_Gfx.as_ptr().cast(), 0x1C0, offset) };
    unsafe { LoadPalette(GetOverworldTextboxPalettePtr().cast(), pal as u32 * 16, 32) };
    TilesetHandle {
        char_base,
        offset,
        palette: BgPalette { index: pal },
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

pub struct TilesetHandle {
    pub char_base: u16,
    pub offset: u16,
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
impl BgHandle<'_> {
    pub fn show(&self) {
        unsafe {
            ShowBg(self.0 as _);
        }
    }

    pub fn copy_tilemap_to_vram(&self) {
        unsafe {
            CopyBgTilemapBufferToVram(self.0 as _);
        }
    }

    pub fn set_pos(&self, x: u8, y: u8) {
        unsafe {
            ChangeBgX(self.0 as _, BG_COORD_SET as _, x);
            ChangeBgY(self.0 as _, BG_COORD_SET as _, y);
        }
    }

    pub fn fill(&self, rect: Rect<u8>, tile_index: u16, palette: BgPalette) {
        unsafe {
            let bg = self.0 as u32;
            FillBgTilemapBufferRect(
                bg,
                tile_index,
                rect.x,
                rect.y,
                rect.width,
                rect.height,
                palette.index,
            );
            ScheduleBgCopyTilemapToVram(self.0 as _);
        }
    }
}

pub struct Background<Set, Map> {
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

    pub fn handle<'a>(&'a self) -> BgHandle<'a> {
        BgHandle(self.index, PhantomData)
    }
}

pub struct SpriteHandle {
    sprite_index: u16,
}

#[unsafe(link_section = ".ewram")]
static G_SPRITES: StaticWrapper<pokeemerald::Sprite> =
    StaticWrapper::new_from_arr(&raw mut gSprites);
impl SpriteHandle {
    pub fn set_pos(&self, pos: Vec2D<i16>) {
        let mut sprite = G_SPRITES.index_mut(self.sprite_index as usize);
        sprite.x = pos.x;
        sprite.y = pos.y;
    }

    pub fn get_pos(&self) -> Vec2D<i16> {
        let mut sprite = G_SPRITES.index_mut(self.sprite_index as usize);
        Vec2D::new(sprite.x, sprite.y)
    }
    pub fn set_palette(&self, palette: u16) {
        let mut sprite = G_SPRITES.index_mut(self.sprite_index as usize);
        sprite.oam.set_paletteNum(palette);
    }
    pub fn set_priority(&self, priority: u8) {
        let mut sprite = G_SPRITES.index_mut(self.sprite_index as usize);
        sprite.oam.set_priority(priority as _);
    }
    pub fn set_subpriority(&self, priority: u8) {
        let mut sprite = G_SPRITES.index_mut(self.sprite_index as usize);
        sprite.subpriority = priority;
    }
    pub fn set_invisible(&self, invisible: bool) {
        let mut sprite = G_SPRITES.index_mut(self.sprite_index as usize);
        sprite.set_invisible(invisible.then_some(1).unwrap_or(0));
    }
    pub fn animate(&self) {
        unsafe {
            let mut sprite = G_SPRITES.index_mut(self.sprite_index as usize);
            AnimateSprite(&raw mut *sprite);
        }
    }

    pub fn start_animation(&self, index: u8) {
        unsafe {
            let mut sprite = G_SPRITES.index_mut(self.sprite_index as usize);
            StartSpriteAnim(&raw mut *sprite, index);
        }
    }
    pub fn request_copy(&self) {
        unsafe {
            let sprite = G_SPRITES.index_mut(self.sprite_index as usize);
            RequestSpriteFrameImageCopy(0, sprite.oam.tileNum(), sprite.images);
        }
    }
}

pub struct SpriteSheet<B> {
    _own: B,
    tilestart: u16,
    tag: u16,
    size: u8,
}

impl<B: Buffer<TileBitmap4bpp>> SpriteSheet<B> {
    pub fn load(buffer: B, tag: u16, size: u8) -> Self {
        let sheet = pokeemerald::SpriteSheet {
            data: buffer.as_ptr().cast(),
            size: buffer.size_bytes() as u16,
            tag,
        };
        let index = unsafe { LoadSpriteSheet(&raw const sheet) };
        SpriteSheet {
            _own: buffer,
            tilestart: index,
            tag,
            size,
        }
    }
}

impl<B> Drop for SpriteSheet<B> {
    fn drop(&mut self) {
        unsafe { FreeSpriteTilesByTag(self.tag) };
    }
}

#[derive(Clone, Copy)]
pub struct SpriteAnims {
    pub anims: *const *const AnimCmd,
    pub affine_anims: *const *const AffineAnimCmd,
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

pub struct SheetSprite<'a> {
    handle: SpriteHandle,
    _own: PhantomData<&'a ()>,
}

impl<'a> SheetSprite<'a> {
    pub async fn load(
        sheet: &'a SpriteSheet<impl Any>,
        anims: SpriteAnims,
        palette: ObjPalette,
    ) -> Self {
        let mut template = SpriteTemplate::default();
        template.affineAnims = anims.affine_anims;
        template.anims = anims.anims;
        template.callback = Some(SpriteCallbackDummy);

        let size = sheet.size as u32;
        let mut oam = OamData::default();
        oam.set_size((size >> 2) & 0b11);
        oam.set_shape(size & 0b11);
        template.oam = &raw const oam;

        template.paletteTag = TAG_NONE as _;
        template.tileTag = sheet.tag;
        let sprite_index = unsafe { CreateSprite(&raw const template, 0, 0, 0) };
        let handle = SpriteHandle {
            sprite_index: sprite_index as u16,
        };
        handle.set_palette(palette.index as u16);
        SheetSprite {
            handle,
            _own: PhantomData,
        }
    }
}

impl Deref for SheetSprite<'_> {
    type Target = SpriteHandle;
    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}

impl Drop for SheetSprite<'_> {
    fn drop(&mut self) {
        unsafe {
            DestroySprite(&raw mut *G_SPRITES.index_mut(self.handle.sprite_index as usize));
        }
    }
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

    pub fn handle(&self) -> &SpriteHandle {
        &self.sprite
    }
}

impl<T: Buffer<TileBitmap4bpp>> Drop for Sprite<T> {
    fn drop(&mut self) {
        unsafe {
            DestroySpriteAndFreeResources(
                &raw mut *G_SPRITES.index_mut(self.sprite.sprite_index as _),
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
pub struct Font {
    pub font: u8,
    pub fg_color: u8,
    pub bg_color: u8,
    pub shadow_color: u8,
    pub letter_spacing: u8,
    pub line_spacing: u8,
}

impl Font {
    pub fn new(font_id: u8) -> Self {
        let font_info = unsafe { &*gFonts.offset(font_id as _) };
        Font {
            font: font_id,
            fg_color: font_info.fgColor(),
            bg_color: font_info.bgColor(),
            shadow_color: font_info.shadowColor(),
            line_spacing: font_info.lineSpacing,
            letter_spacing: font_info.letterSpacing,
        }
    }

    pub fn width_for(&self, str: &Pkstr) -> u16 {
        unsafe { GetStringWidth(self.font, str.as_ptr(), self.letter_spacing as i16) as u16 }
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

    pub fn draw_border(&self, tileset: TilesetHandle) {
        unsafe {
            DrawTextBorderOuter(self.index, tileset.offset, tileset.palette.index);
        }
    }

    pub fn set_palette(&self, palette: BgPalette) {
        unsafe { SetWindowAttribute(self.index.into(), WINDOW_PALETTE_NUM, palette.index.into()) };
    }

    pub fn clear(&self) {
        unsafe { ClearWindowTilemap(self.index as _) }
    }

    pub fn clear_with_border(&self) {
        unsafe {
            rbox_fill_rectangle(self.index as _);
        }
    }

    pub fn put_tilemap(&self) {
        unsafe { PutWindowTilemap(self.index.into()) }
    }

    pub fn copy_to_vram(&self) {
        unsafe { CopyWindowToVram(self.index.into(), COPYWIN_FULL) };
    }

    pub fn print_text(&self, text: &Pkstr, pos: Vec2D<u8>, font: Font) {
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

        unsafe { AddTextPrinter(&raw mut template, 0, None) };
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

pub struct ListMenu<'a> {
    index: u8,
    _w: PhantomData<&'a ()>,
}

impl<'a> ListMenu<'a> {
    pub fn create(
        window: &'a Window,
        items: &'a [ListMenuItem],
        x_offset: u8,
        shown: u8,
        vertical_padding: u8,
        bg_color: u8,
        cursor_colors: (u8, u8),
        font: u32,
    ) -> Self {
        let mut template = ListMenuTemplate {
            items: items.as_ptr(),
            moveCursorFunc: None,
            itemPrintFunc: None,
            _bitfield_align_1: [],
            _bitfield_1: ListMenuTemplate::new_bitfield_1(items.len() as _, shown as u32, 0),
            windowId: window.index,
            header_X: 0,
            item_X: x_offset,
            cursor_X: 0,
            _bitfield_align_2: [],
            _bitfield_2: ListMenuTemplate::new_bitfield_2(
                0,
                cursor_colors.0,
                bg_color,
                cursor_colors.1,
                0,
                vertical_padding,
                0,
                font as _,
                CURSOR_BLACK_ARROW as _,
            ),
        };
        let index = unsafe { ListMenuInit(&raw mut template, 0, 0) };
        ListMenu {
            index: index as u8,
            _w: PhantomData,
        }
    }

    pub async fn wait_for_result(&self) -> Option<i32> {
        loop {
            sleep(1).await;
            let result = unsafe { ListMenu_ProcessInput(self.index as _) };
            if result == LIST_NOTHING_CHOSEN {
                continue;
            }
            if result < 0 {
                return None;
            }
            return Some(result);
        }
    }
}

impl<'a> Drop for ListMenu<'a> {
    fn drop(&mut self) {
        unsafe {
            DestroyListMenuTask(self.index, null_mut(), null_mut());
        }
    }
}
