use alloc::boxed::Box;
use alloc::vec;
use core::future::Future;
use core::pin::Pin;
use core::ptr::addr_of;
use core::task::{Context, Poll};

use arrayvec::ArrayVec;
use data::{get_item, Pokemon};
use graphics::{Sprite, Window, *};

use crate::future::Executor;
use crate::pokeemerald::*;
use crate::resources::{lz_ptr_res, AllocBuf, Buffer};
use crate::{include_res_lz, mgba_warn};
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

const MON_POS: [(i16, i16); 6] = [
    (40, 40),
    (120, 40),
    (200, 40),
    (40, 100),
    (120, 100),
    (200, 100),
];

mod data;
mod graphics;

include_res_lz!(TILESET, "../../graphics/party_menu_full/tiles.4bpp");
include_res_lz!(PAL, "../../graphics/party_menu_full/tiles.gbapal");
include_res_lz!(SCROLL_BG_MAP, "../../graphics/party_menu_full/bg.bin");
include_res_lz!(MON_BG_MAP, "../../graphics/party_menu_full/mon_bg.bin");

type OwnedSprite = Sprite<AllocBuf<TileBitmap4bpp>>;
async fn item_sprite(poke: &Pokemon, index: usize) -> Option<OwnedSprite> {
    let Some(item) = poke.item() else {
        return None;
    };
    let item_info = get_item(item);
    let palette = lz_ptr_res::<{ 2 * 16 }>(item_info.iconPalette.cast());
    let palette = load_obj_palette(6 + index as u8, &palette.load().get());

    const ICON_SIZE: usize = size_of::<TileBitmap4bpp>() * 3 * 3;
    const SPRITE_SIZE: usize = size_of::<TileBitmap4bpp>() * 4 * 4;

    let icon: AllocBuf<TileBitmap4bpp> = lz_ptr_res::<ICON_SIZE>(item_info.iconPic.cast()).load();
    let sprite_buffer = vec![0; SPRITE_SIZE].into_boxed_slice();
    let sprite_buffer: AllocBuf<TileBitmap4bpp> = AllocBuf::new(sprite_buffer);
    mgba_warn!("{} {}", icon.size_bytes(), sprite_buffer.size_bytes());
    unsafe { CopyItemIconPicTo4x4Buffer(icon.as_ptr().cast(), sprite_buffer.as_mut_ptr().cast()) };
    let image = SpriteImage {
        buf: sprite_buffer,
        size: SPRITE_SIZE_32x32,
    };
    sleep(1).await;

    let sprite = Sprite::load(image, DUMMY_SPRITE_ANIMS, palette).await;
    sprite.debug();
    Some(sprite)
}

async fn summary_screen(back: MainCallback) {
    clear_ui().await;

    set_gpu_registers(&[
        (REG_OFFSET_DISPCNT, &[DISPCNT_OBJ_ON, DISPCNT_OBJ_1D_MAP]),
        (REG_OFFSET_BLDCNT, &[]),
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
    let bg = Background::load(BackgroundIndex::Background3, 3, tileset, tilemap).await;
    bg.show();

    let empty_tilemap = AllocBuf::new(vec![0u8; bg_map.size_bytes()].into_boxed_slice());
    let empty_tilemap = Tilemap {
        map: 1,
        buffer: empty_tilemap,
    };

    let bg2 = Background::load(BackgroundIndex::Background2, 2, tileset, empty_tilemap).await;
    bg2.set_pos(0, 0);
    bg2.fill(Rect::new(0, 0, 32, 20), 15, palette);
    bg2.show();

    let pokes: ArrayVec<Pokemon, 6> = (0..6)
        .filter_map(|i| Pokemon::get_player_party(i))
        .collect();
    let mut poke_sprites: ArrayVec<PokemonSpritePic, 6> = pokes
        .iter()
        .enumerate()
        .map(|(index, p)| PokemonSpritePic::new(p, index as _))
        .collect();
    for (index, sprite) in poke_sprites.iter_mut().enumerate() {
        let (x, y) = MON_POS[index];
        sprite.sprite().set_pos(x, y);
    }

    let mut item_sprites: ArrayVec<Option<OwnedSprite>, 6> = ArrayVec::new();
    for (index, poke) in pokes.iter().enumerate() {
        item_sprites.push(item_sprite(poke, index).await);
    }

    for (index, sprite) in item_sprites.iter_mut().enumerate() {
        let Some(sprite) = sprite else { continue };
        let (x, y) = MON_POS[index];
        sprite.handle().set_pos(x + 20, y + 20);
    }

    let mon_bg: AllocBuf<TilePlain> = MON_BG_MAP.load();
    sleep(1).await;

    let win = Window::create(bg2.handle(), Rect::new(1, 1, 10, 10), palette, 0x100);
    win.copy_tilemap(&tileset.tiles.get(), &mon_bg.get(), Rect::new(0, 0, 10, 10));
    win.display();

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
