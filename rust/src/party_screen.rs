use alloc::boxed::Box;
use core::ffi::c_void;
use core::fmt::Write;
use core::future::Future;
use core::mem::size_of;
use core::pin::Pin;
use core::ptr::null;
use core::task::{Context, Poll};

use graphics::{Background, Palette, Tilemap, Tileset};

use crate::future::Executor;
use crate::pokeemerald::*;
use crate::resources::Resource;
use crate::{aformat, include_res_lz};
static EXECUTOR: Executor = Executor::new();

extern "C" fn main_cb() {
    unsafe {
        DoScheduledBgTilemapCopiesToVram();
        UpdatePaletteFade();
    }
    EXECUTOR.poll();
}

extern "C" fn vblank_cb() {
    unsafe {
        ProcessSpriteCopyRequests();
        LoadOam();
        TransferPlttBuffer();
    }
}

mod graphics {
    use core::marker::PhantomData;

    use super::sleep;
    use crate::pokeemerald::*;
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
                LoadBgTiles(
                    bg_index as _,
                    self.buffer.buffer(),
                    self.buffer.len() as _,
                    self.offset,
                );
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
}

static TILESET: Resource = include_res_lz!("../../graphics/summary_screen/bw/tiles.4bpp");
static PAL: Resource = include_res_lz!("../../graphics/summary_screen/bw/tiles.gbapal");
static BG_MAP: Resource = include_res_lz!("../../graphics/summary_screen/bw/scroll_bg.bin");
static FG_MAP: Resource = include_res_lz!("../../graphics/summary_screen/bw/page_info.bin");

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
    let fg_map = FG_MAP.load();
    sleep(1).await;

    let mut palette = Palette { offset: 0, buffer: &PAL };
    let mut tileset = Tileset::new(1, 0, &tileset);
    let mut tilemap_bg = Tilemap { buffer: &bg_map, map: 0 };
    let mut tilemap_fg = Tilemap { buffer: &fg_map, map: 1 };
    let fg = Background::load(0, 0, &mut tileset, &mut tilemap_fg, &mut palette).await;
    let bg = Background::load(1, 1, &mut tileset, &mut tilemap_bg, &mut palette).await;

    fg.show();
    bg.show();
    unsafe { SetVBlankCallback(Some(vblank_cb)) };

    loop {
        if unsafe { gMain.newKeys } & 0x1 != 0 {
            break;
        }
        sleep(1).await;
    }
    unsafe {
        FreeTempTileDataBuffersIfPossible();
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
