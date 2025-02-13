use alloc::boxed::Box;
use alloc::vec;
use core::future::Future;
use core::pin::Pin;
use core::ptr::addr_of;
use core::task::{Context, Poll};

use arrayvec::ArrayVec;
use data::{get_item, Pokemon};
use graphics::{Sprite, Window, *};

use crate::charmap::ArrayPkstr;
use crate::future::Executor;
use crate::pokeemerald::*;
use crate::resources::{lz_ptr_res, AllocBuf, Buffer};
use crate::{aformat, include_res_lz, mgba_warn};
static EXECUTOR: Executor = Executor::new();

#[no_mangle]
extern "C" fn Init_Full_Summary_Screen(back: MainCallback) {
    mgba_warn!(
        "Heap: {:?} {:?} {:?} {:?} {:?}",
        addr_of!(gHeap),
        addr_of!(TILESET),
        addr_of!(PAL),
        addr_of!(SCROLL_BG_MAP),
        addr_of!(MON_BG_MAP)
    );
    let fut = Box::new(summary_screen(back));
    unsafe { SetMainCallback2(Some(main_cb)) }
    EXECUTOR.set(fut);
}

extern "C" fn main_cb() {
    unsafe {
        AnimateSprites();
        BuildOamBuffer();
        DoScheduledBgTilemapCopiesToVram();
        UpdatePaletteFade();
    }
    EXECUTOR.poll();
}

extern "C" fn vblank_cb() {
    unsafe {
        LoadOam();
        ProcessSpriteCopyRequests();
        TransferPlttBuffer();
        ChangeBgX(3, 64, BG_COORD_ADD as _);
        ChangeBgY(3, 64, BG_COORD_ADD as _);
    }
}

const MON_POS: [(u8, u8); 6] = [(1, 1), (11, 1), (21, 1), (1, 10), (11, 10), (21, 10)];

mod data;
mod graphics;

include_res_lz!(TILESET, "../../graphics/party_menu_full/tiles.4bpp");
include_res_lz!(PAL, "../../graphics/party_menu_full/tiles.gbapal");
include_res_lz!(SCROLL_BG_MAP, "../../graphics/party_menu_full/bg.bin");
include_res_lz!(MON_BG_MAP, "../../graphics/party_menu_full/mon_bg.bin");
include_res_lz!(HP_MAP, "../../graphics/party_menu_full/hp.plain.bin");

type OwnedSprite = Sprite<AllocBuf<TileBitmap4bpp>>;
async fn item_sprite(poke: &Pokemon, index: usize) -> Option<OwnedSprite> {
    let Some(item) = poke.item() else {
        return None;
    };
    let item_info = get_item(item);
    let palette = lz_ptr_res::<{ 2 * 16 }>(item_info.iconPalette.cast());
    let palette = load_obj_palette(6 + index as u8, &palette.load().get());
    sleep(1).await;

    const ICON_SIZE: usize = size_of::<TileBitmap4bpp>() * 3 * 3;
    const SPRITE_SIZE: usize = size_of::<TileBitmap4bpp>() * 4 * 4;

    let icon: AllocBuf<TileBitmap4bpp> = lz_ptr_res::<ICON_SIZE>(item_info.iconPic.cast()).load();
    let sprite_buffer = vec![0; SPRITE_SIZE].into_boxed_slice();
    let sprite_buffer: AllocBuf<TileBitmap4bpp> = AllocBuf::new(sprite_buffer);
    sleep(1).await;

    unsafe { CopyItemIconPicTo4x4Buffer(icon.as_ptr().cast(), sprite_buffer.as_mut_ptr().cast()) };
    let image = SpriteImage {
        buf: sprite_buffer,
        size: SPRITE_SIZE_32x32,
    };
    sleep(1).await;

    let sprite = Sprite::load(image, DUMMY_SPRITE_ANIMS, palette).await;
    Some(sprite)
}

struct Entry {
    poke: Pokemon,
    sprite: PokemonSpritePic,
    item_sprite: Option<OwnedSprite>,
    bg_window: Window,
    fg_window: Window,
}

impl Entry {
    fn print_info(&self, tileset: &[TileBitmap4bpp], hp_tilemap: &[TilePlain]) {
        let fg = &self.fg_window;
        const HP_BAR_RECT: Rect<u8> = Rect::new(0, 7, 9, 1);
        const HP_FILL_RECT: Rect<u16> = Rect::new(16, 59, 48, 3);
        // fg.fill_rect(0, HP_BAR_RECT.tile_to_pixel());
        fg.copy_tilemap(tileset, hp_tilemap, HP_BAR_RECT);

        let printer_params = TextPrinterParams {
            bg_color: 0,
            ..TextPrinterParams::font(FONT_SMALL_NARROWER as _)
        };

        fg.print_text(&self.poke.name(), Vec2D::new(4, 0), printer_params);

        let lv = aformat!(5, "Lv{}", self.poke.level());
        let lv = aformat!(6, "{:>6}", lv);
        let lv = ArrayPkstr::<7>::from_str(&lv);
        fg.print_text(&lv, Vec2D::new(48, 0), printer_params);

        let hp = aformat!(10, "{:<3}/{:<3}", self.poke.hp(), self.poke.max_hp());
        let hp = ArrayPkstr::<11>::from_str(&hp);
        fg.print_text(&hp, Vec2D { x: 3, y: 45 }, printer_params);
        
        let mut hp_fill_rect = HP_FILL_RECT;
        hp_fill_rect.width = self.poke.hp() * HP_FILL_RECT.width / self.poke.max_hp();
        let color = match hp_fill_rect.width {
            0..12 => 15,
            12..24 => 13,
            _ => 14,
        };
        fg.fill_rect(color, hp_fill_rect);

    }
}

async fn create_entry(
    poke: Pokemon,
    bg: BgHandle<'_>,
    fg: BgHandle<'_>,
    tileset: Tileset<&AllocBuf<TileBitmap4bpp>>,
    index: u8,
) -> Entry {
    const BG_DIM: Vec2D<u8> = Vec2D::new(9, 8);
    const FG_DIM: Vec2D<u8> = Vec2D::new(9, 8);
    const BASE_BLOCK: u16 = 0x20;
    const BLOCK_SIZE: u16 = (BG_DIM.x * BG_DIM.y + FG_DIM.x * FG_DIM.y) as u16;
    const POKE_SPRITE_OFFS: Vec2D<i16> = Vec2D::new(36, 32);
    const ITEM_SPRITE_OFFS: Vec2D<i16> = Vec2D::new(56, 48);

    let (tile_x, tile_y) = MON_POS[index as usize];
    let tile_pos = Vec2D::new(tile_x, tile_y);

    let mut sprite = PokemonSpritePic::new(&poke, index);
    sleep(1).await;
    let handle = sprite.handle();
    handle.set_pos(tile_pos.tile_to_pixel() + POKE_SPRITE_OFFS);
    handle.set_priority(2);

    let mut item_sprite = item_sprite(&poke, index.into()).await;
    if let Some(item_sprite) = &mut item_sprite {
        let handle = item_sprite.handle();
        handle.set_pos(tile_pos.tile_to_pixel() + ITEM_SPRITE_OFFS);
        handle.set_priority(2);
    }

    let tiles = &tileset.tiles.get();
    let block = BLOCK_SIZE * index as u16 + BASE_BLOCK;

    let rect = Rect::from_vecs(tile_pos, BG_DIM);
    let bg_window = Window::create(bg, rect, tileset.palette, block);
    sleep(1).await;

    let rect = Rect::from_vecs(tile_pos, FG_DIM);
    let fg_window = Window::create(fg, rect, tileset.palette, block + BG_DIM.size() as u16);
    fg_window.put_tilemap();
    sleep(1).await;

    let mon_bg_map = MON_BG_MAP.load();
    let rect = Rect::from_vecs(Vec2D::new(0, 0), BG_DIM);
    bg_window.copy_tilemap(tiles, &mon_bg_map.get(), rect);
    bg_window.put_tilemap();
    bg_window.copy_to_vram();
    sleep(1).await;

    Entry {
        poke,
        sprite,
        item_sprite,
        bg_window,
        fg_window,
    }
}

async fn summary_screen(back: MainCallback) {
    clear_ui().await;

    set_gpu_registers(&[
        (REG_OFFSET_DISPCNT, &[DISPCNT_OBJ_ON, DISPCNT_OBJ_1D_MAP]),
        // (
        //     REG_OFFSET_BLDCNT,
        //     &[BLDCNT_EFFECT_BLEND, BLDCNT_TGT1_BG1, BLDCNT_TGT2_OBJ],
        // ),
        // (REG_OFFSET_BLDALPHA, &[15 << 8, 15]),
        (REG_OFFSET_BLDY, &[]),
    ]);

    let tileset_data = TILESET.load();
    sleep(1).await;
    let bg_map = SCROLL_BG_MAP.load();
    sleep(1).await;
    let palette = PAL.load();
    sleep(1).await;

    let palette = load_bg_palette(0, &palette.get());
    sleep(1).await;

    let tileset = Tileset {
        char_base: 1,
        offset: 0,
        palette,
        tiles: &tileset_data,
    };
    let tilemap = Tilemap {
        map: 0,
        buffer: &bg_map,
    };
    let scroll_bg = Background::load(BackgroundIndex::Background3, 3, tileset, tilemap).await;
    scroll_bg.show();

    let empty_tilemap = AllocBuf::new(vec![0u8; bg_map.size_bytes()].into_boxed_slice());
    let empty_tilemap = Tilemap {
        map: 2,
        buffer: empty_tilemap,
    };
    let fixed_bg = Background::load(BackgroundIndex::Background2, 2, tileset, empty_tilemap).await;
    fixed_bg.set_pos(0, 0);
    fixed_bg.fill(Rect::new(0, 0, 32, 20), 15, palette);
    fixed_bg.show();

    let buffer_fg = AllocBuf::new(vec![0u8; bg_map.size_bytes()].into_boxed_slice());
    let empty_tilemap = Tilemap {
        map: 4,
        buffer: &buffer_fg,
    };
    let fg = Background::load(BackgroundIndex::Background1, 1, tileset, empty_tilemap).await;
    fg.fill(Rect::new(0, 0, 32, 20), 15, palette);
    fg.set_pos(0, 0);
    fg.show();

    let hp_tilemap = HP_MAP.load();


    let mut entries: ArrayVec<Entry, 6> = ArrayVec::new();
    for i in 0..6 {
        let Some(poke) = Pokemon::get_player_party(i) else {
            continue;
        };
        entries.push(create_entry(poke, fixed_bg.handle(), fg.handle(), tileset, i).await);
    }

    for entry in entries.iter() {
        sleep(1).await;
        entry.print_info(&tileset_data.get(), &hp_tilemap.get());
    }

    unsafe { CopyBgTilemapBufferToVram(1) };
    unsafe { SetVBlankCallback(Some(vblank_cb)) };
    loop {
        if unsafe { gMain.newKeys } & 0x1 != 0 {
            break;
        }
        sleep(1).await;
    }
    unsafe { SetMainCallback2(back) };
}

async fn clear_ui() {
    unsafe {
        SetVBlankHBlankCallbacksToNull();
        ResetVramOamAndBgCntRegs();
        ClearScheduledBgCopiesToVram();
        sleep(1).await;

        ResetPaletteFade();
        sleep(1).await;

        ResetSpriteData();
        sleep(1).await;

        FreeAllSpritePalettes();
        sleep(1).await;

        ResetBgsAndClearDma3BusyFlags(0);
    }
}

fn sleep(frames: usize) -> impl Future<Output = ()> {
    struct WaitUntil(usize);
    impl Future for WaitUntil {
        type Output = ();
        fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
            if self.0 == 0 {
                return Poll::Ready(());
            }
            self.0 -= 1;
            Poll::Pending
        }
    }
    WaitUntil(frames)
}
