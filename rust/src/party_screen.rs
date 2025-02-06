use alloc::boxed::Box;
use core::ffi::c_void;
use core::fmt::Write;
use core::future::Future;
use core::mem::size_of;
use core::pin::Pin;
use core::ptr::null;
use core::task::{Context, Poll};

use graphics::{g_sprite, Background, Palette, Tilemap, Tileset};

use crate::future::Executor;
use crate::pokeemerald::*;
use crate::resources::{LoadedResource, Resource};
use crate::{aformat, include_res_lz, mgba_print};
static EXECUTOR: Executor = Executor::new();

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
    }
}

mod graphics;

static TILESET: Resource = include_res_lz!("../../graphics/party_menu_full/tiles.4bpp");
static PAL: Resource = include_res_lz!("../../graphics/party_menu_full/tiles.gbapal");
static BG_MAP: Resource = include_res_lz!("../../graphics/party_menu_full/bg.bin");

#[no_mangle]
extern "C" fn Init_Full_Summary_Screen(back: MainCallback) {
    unsafe {
        MgbaPrintf(2, "Opened new menu".as_ptr());
    }
    unsafe { SetMainCallback2(Some(main_cb)) }
    EXECUTOR.set(summary_screen(back));
}

async fn summary_screen(back: MainCallback) {
    clear_ui().await;

    let tileset = TILESET.load();
    sleep(1).await;
    let bg_map = BG_MAP.load();
    sleep(1).await;

    unsafe {
        SetGpuReg(REG_OFFSET_DISPCNT as _, DISPCNT_OBJ_ON as u16 | DISPCNT_OBJ_1D_MAP as u16);
        SetGpuReg(REG_OFFSET_BLDCNT as _, (BLDCNT_TGT1_BG1 | BLDCNT_EFFECT_BLEND | BLDCNT_TGT2_ALL) as _);
        SetGpuReg(REG_OFFSET_BLDY as _, 0);
    }

    let mut palette = Palette { offset: 0, buffer: &PAL };
    let mut tileset = Tileset::new(1, 0, &tileset);
    let mut tilemap_bg = Tilemap { buffer: &bg_map, map: 0 };
    let bg = Background::load(3, 3, &mut tileset, &mut tilemap_bg, &mut palette).await;
    bg.show();

    let sprite_id =
        unsafe { CreateMonPicSprite_Affine(3, 0, 0xff, MON_PIC_AFFINE_FRONT as _, 120, 40, 0, TAG_NONE as _) };
    mgba_print!(2, "{sprite_id:?}");
    let sprite_id =
        unsafe { CreateMonPicSprite_Affine(4, 0, 0xff, MON_PIC_AFFINE_FRONT as _, 140, 40, 1, TAG_NONE as _) };
    mgba_print!(2, "{sprite_id:?}");
    unsafe {
        (*g_sprite(sprite_id as _)).oam.set_priority(2);
    }

    unsafe {
        SetVBlankCallback(Some(vblank_cb));
        ShowBg(0);
    };

    loop {
        if unsafe { gMain.newKeys } & 0x1 != 0 {
            break;
        }
        sleep(1).await;
    }
    unsafe {
        FreeAndDestroyMonPicSprite(sprite_id);
        SetMainCallback2(back);
    }
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
