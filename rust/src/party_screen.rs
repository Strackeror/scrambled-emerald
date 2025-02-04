use alloc::boxed::Box;
use core::ffi::c_void;
use core::fmt::Write;
use core::future::Future;
use core::mem::size_of;
use core::pin::Pin;
use core::ptr::null;
use core::task::{Context, Poll};

use crate::aformat;
use crate::future::Executor;
use crate::pokeemerald::*;
static EXECUTOR: Executor = Executor::new();

extern "C" fn main_cb() {
    unsafe {
        DoScheduledBgTilemapCopiesToVram();
        UpdatePaletteFade();
        // MgbaPrintf(3, aformat!(10, "Poll\0").as_ptr());
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

static TILESET: &[u8] = include_bytes!("../../graphics/summary_screen/bw/tiles.4bpp.lz");
static PAL: &[u8] = include_bytes!("../../graphics/summary_screen/bw/tiles.gbapal.lz");
static BG_MAP: &[u8] = include_bytes!("../../graphics/summary_screen/bw/scroll_bg.bin.lz");

#[no_mangle]
extern "C" fn Init_Full_Summary_Screen(back: MainCallback) {
    unsafe {
        MgbaPrintf(2, "Opened new menu".as_ptr());
    }
    unsafe { SetMainCallback2(Some(main_cb)) }
    EXECUTOR.set(summary_screen(back));
}

async fn summary_screen(back: MainCallback) {
    let _bg = init().await;
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

async fn init() -> BackgroundTilemap<0x800> {
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
        SetBgMode(0);
        let mut bg = init_bg();
        sleep(1).await;

        bg.load_tileset(&TILESET);
        sleep(1).await;

        bg.load_tilemap(&BG_MAP);
        sleep(1).await;

        bg.load_palette(&PAL, 8);
        sleep(1).await;

        SetVBlankCallback(Some(vblank_cb));
        bg
    }
}

struct BackgroundTilemap<const C: usize> {
    index: u8,
    tile_buffer: *mut c_void,
}

impl<const C: usize> BackgroundTilemap<C> {
    fn new(template: &BgTemplate) -> BackgroundTilemap<C> {
        unsafe {
            let tilemap = BackgroundTilemap::<C> {
                index: template.bg() as u8,
                tile_buffer: Alloc_(C as u32, null()),
            };
            InitBgFromTemplate(&raw const *template);
            SetBgTilemapBuffer(tilemap.index as u32, tilemap.tile_buffer);
            ScheduleBgCopyTilemapToVram(tilemap.index);
            ShowBg(tilemap.index as _);
            tilemap
        }
    }

    fn load_tileset(&self, tileset: &[u8]) {
        unsafe {
            DecompressAndCopyTileDataToVram(self.index, tileset.as_ptr() as _, 0, 0, 0);
        }
    }

    fn load_tilemap(&mut self, tilemap: &[u8]) {
        unsafe {
            LZ77UnCompWram(tilemap.as_ptr() as *const _, self.tile_buffer);
            ScheduleBgCopyTilemapToVram(self.index);
            // MgbaPrintf(2, aformat!(50, "{:x?}\0", &self.tile_buffer[0..2]).as_ptr());
            // MgbaPrintf(2, aformat!(50, "{:x?}\0", self.tile_buffer.as_mut_ptr()).as_ptr());
        }
    }

    fn load_palette(&self, palette: &[u8], size: usize) {
        unsafe {
            const PALETTE_ENTRY_SIZE: u32 = 16;
            LoadCompressedPalette(
                palette.as_ptr() as _,
                BG_PLTT_OFFSET + self.index as u32 * PALETTE_ENTRY_SIZE,
                size as u32 * 16 * size_of::<u16>() as u32,
            );
            MgbaPrintf(2, aformat!(20, "palette loaded: {size}\0").as_ptr());
        }
    }
}

impl<const C: usize> Drop for BackgroundTilemap<C> {
    fn drop(&mut self) {
        unsafe {
            UnsetBgTilemapBuffer(self.index as _);
            Free(self.tile_buffer);
        }
    }
}

fn init_bg() -> BackgroundTilemap<0x800> {
    let mut template = BgTemplate::default();
    unsafe {
        MgbaPrintf(2, aformat!(100, "{template:?}").as_ptr());
    }
    template.set_bg(0);
    template.set_charBaseIndex(1);
    template.set_mapBaseIndex(0);
    template.set_paletteMode(0);
    template.set_priority(0);
    template.set_baseTile(0);
    BackgroundTilemap::new(&template)
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
