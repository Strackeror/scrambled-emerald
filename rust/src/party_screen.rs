use alloc::boxed::Box;
use alloc::vec;
use core::ffi::c_void;
use core::future::Future;
use core::pin::Pin;
use core::ptr::addr_of;
use core::task::{Context, Poll};

use arrayvec::ArrayVec;
use data::{get_item, Pokemon};
use graphics::{
    set_gpu_registers, Background, Palette, PokemonSpritePic, Sprite, SpriteImage, Tilemap,
    Tileset, DUMMY_SPRITE_ANIMS,
};

use crate::future::Executor;
use crate::pokeemerald::*;
use crate::resources::{CheckPtrCast as _, CheckPtrCastMut as _, LoadedResource, Resource};
use crate::{include_res_lz, mgba_print};
static EXECUTOR: Executor = Executor::new();

extern "C" {
    static gAgbMainLoop_sp: *const c_void;
}
#[inline(never)]
fn print_stack_addr() {
    let mut offset = 0;
    offset = unsafe { gAgbMainLoop_sp.offset_from(addr_of!(offset).cast()) };
    mgba_print!(2, "size_of_stack: {:?}", offset);
}

#[no_mangle]
extern "C" fn Init_Full_Summary_Screen(back: MainCallback) {
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

static TILESET: Resource = include_res_lz!("../../graphics/party_menu_full/tiles.4bpp");
static PAL: Resource = include_res_lz!("../../graphics/party_menu_full/tiles.gbapal");
static BG_MAP: Resource = include_res_lz!("../../graphics/party_menu_full/bg.bin");

async fn item_sprites(pokes: &[Pokemon]) -> ArrayVec<Option<Sprite>, 6> {
    let mut item_sprites: ArrayVec<Option<Sprite>, 6> = ArrayVec::new();
    for (index, poke) in pokes.iter().enumerate() {
        let Some(item) = poke.item() else {
            item_sprites.push(None);
            continue;
        };
        let item_info = get_item(item);
        let palette = Palette {
            buffer: Resource::from_lz_ptr(item_info.iconPalette.c_cast(), 32).into(),
            index: 6 + index,
        };

        let base_icon = Resource::from_lz_ptr(item_info.iconPic.c_cast(), 24 * 24 / 2);
        let mut buffer = vec![0; 32 * 32 / 2].into_boxed_slice();
        unsafe {
            CopyItemIconPicTo4x4Buffer(base_icon.load().as_ptr(), buffer.as_mut_ptr().c_cast_mut())
        };

        let image = SpriteImage {
            resource: LoadedResource::Compressed(buffer),
            size: SPRITE_SIZE_32x32,
        };
        sleep(1).await;

        let sprite = Sprite::load(&DUMMY_SPRITE_ANIMS, image, palette).await;
        item_sprites.push(Some(sprite));
        sleep(1).await;
    }
    item_sprites
}

async fn summary_screen(back: MainCallback) {
    clear_ui().await;

    let tileset = TILESET.load();
    sleep(1).await;
    let bg_map = BG_MAP.load();
    sleep(1).await;

    set_gpu_registers(&[
        (REG_OFFSET_DISPCNT, &[DISPCNT_OBJ_ON, DISPCNT_OBJ_1D_MAP]),
        (
            REG_OFFSET_BLDCNT,
            &[BLDCNT_TGT1_BG1, BLDCNT_EFFECT_BLEND, BLDCNT_TGT2_ALL],
        ),
        (REG_OFFSET_BLDY, &[]),
    ]);

    let mut palette = Palette {
        index: 0,
        buffer: (&PAL).into(),
    };
    let mut tileset = Tileset::new(1, 0, &tileset);
    let mut tilemap_bg = Tilemap {
        buffer: &bg_map,
        map: 0,
    };
    let bg = Background::load(3, 3, &mut tileset, &mut tilemap_bg, &mut palette).await;
    bg.show();

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
    let mut item_sprites = item_sprites(&pokes).await;
    for (index, sprite) in item_sprites.iter_mut().enumerate() {
        let Some(sprite) = sprite else {
            continue;
        };
        let (x, y) = MON_POS[index];
        sprite.handle().set_pos(x + 20, y + 20);
    }

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
