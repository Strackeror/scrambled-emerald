use alloc::boxed::Box;
use alloc::vec;
use core::cmp::min;
use core::mem::swap;

use arrayvec::ArrayVec;
use bindings::charmap::{ArrayPkstr, Pkstr};
use bindings::data::{Pokemon, get_item};
use bindings::future::{sleep, Executor, RefCellSync};
use bindings::graphics::{ListMenu, Sprite, SpriteSheet, Tileset, Window, *};
use bindings::input::Button;
use bindings::pokeemerald::{self, *};
use bindings::resources::{AllocBuf, Buffer, lz_ptr_res, static_deref};
use bindings::{aformat, include_res_lz, mgba_warn, pkstr};
use derive_more::TryFrom;

static EXECUTOR: Executor = Executor::new();
static STORED_CALLBACK: RefCellSync<MainCallback> = RefCellSync::new(None);
static SELECTED_POKE: RefCellSync<u8> = RefCellSync::new(0);

const TYPE_ANIMS: &[*const AnimCmd] = &[
    [anim_frame(0, 0, false, false), anim_end()].as_ptr(),
    [anim_frame(4, 0, false, false), anim_end()].as_ptr(),
    [anim_frame(8, 0, false, false), anim_end()].as_ptr(),
    [anim_frame(12, 0, false, false), anim_end()].as_ptr(),
    [anim_frame(16, 0, false, false), anim_end()].as_ptr(),
    [anim_frame(20, 0, false, false), anim_end()].as_ptr(),
    [anim_frame(24, 0, false, false), anim_end()].as_ptr(),
    [anim_frame(28, 0, false, false), anim_end()].as_ptr(),
    [anim_frame(32, 0, false, false), anim_end()].as_ptr(),
    [anim_frame(36, 0, false, false), anim_end()].as_ptr(),
    [anim_frame(40, 0, false, false), anim_end()].as_ptr(),
    [anim_frame(44, 0, false, false), anim_end()].as_ptr(),
    [anim_frame(48, 0, false, false), anim_end()].as_ptr(),
    [anim_frame(52, 0, false, false), anim_end()].as_ptr(),
    [anim_frame(56, 0, false, false), anim_end()].as_ptr(),
    [anim_frame(60, 0, false, false), anim_end()].as_ptr(),
    [anim_frame(64, 0, false, false), anim_end()].as_ptr(),
    [anim_frame(68, 0, false, false), anim_end()].as_ptr(),
    [anim_frame(72, 0, false, false), anim_end()].as_ptr(),
    [anim_frame(76, 0, false, false), anim_end()].as_ptr(),
    [anim_frame(80, 0, false, false), anim_end()].as_ptr(),
    [anim_frame(84, 0, false, false), anim_end()].as_ptr(),
    [anim_frame(88, 0, false, false), anim_end()].as_ptr(),
];

const STATUS_ANIMS: &[*const AnimCmd] = &[
    [anim_frame(0, 0, false, false), anim_end()].as_ptr(),
    [anim_frame(4, 0, false, false), anim_end()].as_ptr(),
    [anim_frame(8, 0, false, false), anim_end()].as_ptr(),
    [anim_frame(12, 0, false, false), anim_end()].as_ptr(),
    [anim_frame(16, 0, false, false), anim_end()].as_ptr(),
    [anim_frame(20, 0, false, false), anim_end()].as_ptr(),
    [anim_frame(24, 0, false, false), anim_end()].as_ptr(),
    [anim_frame(28, 0, false, false), anim_end()].as_ptr(),
];

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
#[expect(unused, reason = "external")]
pub enum Style {
    Party = 0,
    ReadOnly = 1,
}

#[unsafe(no_mangle)]
extern "C" fn InitFullSummaryScreen(
    back: MainCallback,
    style: Style,
    pokemons: *mut pokeemerald::Pokemon,
    count: usize,
) {
    let fut = Box::new(summary_screen(back, style, pokemons, count, 0));
    mgba_warn!(
        "heap: {:?} fut:{:?}-{:x?}",
        &raw const gHeap,
        &raw const *fut,
        (&raw const *fut as usize) + size_of_val(&*fut)
    );

    unsafe { SetMainCallback2(Some(main_cb)) }
    *STORED_CALLBACK.borrow_mut() = back;
    EXECUTOR.set(fut);
}

extern "C" fn return_from_party_callback() {
    let index = unsafe { *static_deref(&raw mut gPartyMenu.slotId) } as u8;
    let back = *STORED_CALLBACK.borrow();
    let count = unsafe { gPlayerPartyCount };
    let mons = &raw mut gPlayerParty;
    let fut = Box::new(summary_screen(
        back,
        Style::Party,
        mons.cast(),
        count as usize,
        index,
    ));
    unsafe { SetMainCallback2(Some(main_cb)) }
    EXECUTOR.set(fut);
}

extern "C" fn return_from_give_hold_item_callback() {
    if let Some(poke) = Pokemon::get_player_party(*SELECTED_POKE.borrow()) {
        let item_to_give = unsafe { gSpecialVar_ItemId };
        unsafe { RemoveBagItem(item_to_give, 1) };
        if let Some(item) = poke.item() {
            unsafe { AddBagItem(item as u16, 1) };
        }
        poke.set_item(item_to_give);
    }
    let index = *SELECTED_POKE.borrow();
    let back = *STORED_CALLBACK.borrow();
    let count = unsafe { gPlayerPartyCount };
    let mons = &raw mut gPlayerParty;
    let fut = Box::new(summary_screen(
        back,
        Style::Party,
        mons.cast(),
        count as usize,
        index,
    ));
    unsafe { SetMainCallback2(Some(main_cb)) }
    EXECUTOR.set(fut);
}

extern "C" fn main_cb() {
    EXECUTOR.poll();
    unsafe {
        AnimateSprites();
        BuildOamBuffer();
        DoScheduledBgTilemapCopiesToVram();
        UpdatePaletteFade();
    }
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

include_res_lz!(TILESET, "../graphics/party_menu_full/tiles.4bpp");
include_res_lz!(PAL, "../graphics/party_menu_full/tiles.gbapal");
include_res_lz!(SCROLL_BG_MAP, "../graphics/party_menu_full/bg.bin");
include_res_lz!(MON_BG_MAP, "../graphics/party_menu_full/mon_bg.bin");
include_res_lz!(HP_MAP, "../graphics/party_menu_full/hp.plain.bin");

include_res_lz!(
    TERA_SPRITE,
    "../graphics/party_menu_full/tera/tera_types.4bpp"
);
include_res_lz!(
    TERA_SPRITE_PAL,
    "../graphics/party_menu_full/tera/tera_types.gbapal"
);

include_res_lz!(
    STATUS_SHEET,
    "../graphics/party_menu_full/status_icons.4bpp"
);

include_res_lz!(
    STATUS_PAL,
    "../graphics/party_menu_full/status_icons.gbapal"
);

type OwnedSprite = Sprite<AllocBuf<TileBitmap4bpp>>;
async fn item_sprite(poke: &Pokemon, index: usize) -> Option<OwnedSprite> {
    let item = poke.item()?;
    let item_info = get_item(item);
    let palette = lz_ptr_res::<{ 2 * 16 }>(item_info.iconPalette.cast());
    let palette = load_obj_palette(6 + index as u8, &palette.load().get());

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
    sprite.handle().set_priority(2);
    Some(sprite)
}

fn type_palette(pktype: u16) -> ObjPalette {
    unsafe { ObjPalette::raw(gTypesInfo[pktype as usize].palette) }
}

fn load_type_palettes() {
    load_obj_palette(13, &TERA_SPRITE_PAL.load().get());
}

type TeraSprite<'a> = SheetSprite<'a>;
fn tera_sprite<'a>(
    poke: &Pokemon,
    sheet: &'a SpriteSheet<AllocBuf<TileBitmap4bpp>>,
) -> Option<TeraSprite<'a>> {
    if poke.is_egg() {
        return None;
    }

    let tera = poke.tera_type();
    let anims = SpriteAnims {
        anims: TYPE_ANIMS.as_ptr(),
        ..DUMMY_SPRITE_ANIMS
    };
    let sprite = SheetSprite::load(sheet, anims, type_palette(tera));
    sprite.start_animation(tera as u8);
    sprite.set_priority(2);
    sprite.set_subpriority(1);
    Some(sprite)
}

fn status_sprite<'a>(
    poke: &Pokemon,
    sheet: &'a SpriteSheet<AllocBuf<TileBitmap4bpp>>,
    pal: ObjPalette,
) -> Option<SheetSprite<'a>> {
    let status = poke.status();
    if status == 0 {
        return None;
    }

    let anims = SpriteAnims {
        anims: STATUS_ANIMS.as_ptr(),
        ..DUMMY_SPRITE_ANIMS
    };
    let sprite = SheetSprite::load(sheet, anims, pal);
    sprite.start_animation(poke.status() - 1);
    sprite.set_priority(2);
    Some(sprite)
}

enum BackgroundStyle {
    Focused,
    Unfocused,
    SwitchFocused,
    SwitchUnfocused,
    KoFocused,
    KoUnfocused,
}

impl BackgroundStyle {
    fn palette_index(self) -> usize {
        match self {
            BackgroundStyle::Focused => 0,
            BackgroundStyle::Unfocused => 1,
            BackgroundStyle::SwitchFocused => 2,
            BackgroundStyle::SwitchUnfocused => 3,
            BackgroundStyle::KoFocused => 4,
            BackgroundStyle::KoUnfocused => 5,
        }
    }
}

struct Entry<'a> {
    index: u8,
    poke: Pokemon,
    sprite: PokemonSpritePic,
    item_sprite: Option<OwnedSprite>,
    tera_sprite: Option<TeraSprite<'a>>,
    status_sprite: Option<SheetSprite<'a>>,
    bg_window: Window,
    fg_window: Window,
}

impl<'a> Entry<'a> {
    const POKE_SPRITE_OFFS: Vec2D<i16> = Vec2D::new(36, 32);
    const ITEM_SPRITE_OFFS: Vec2D<i16> = Vec2D::new(62, 52);
    const TERA_SPRITE_OFFS: Vec2D<i16> = Vec2D::new(62, 20);
    const STATUS_SPRITE_OFFS: Vec2D<i16> = Vec2D::new(12, 44);

    fn print_info(&self, resource: &Resources) {
        const HP_BAR_RECT: Rect<u8> = Rect::new(0, 7, 9, 1);
        const HP_FILL_RECT_1: Rect<u16> = Rect::new(16, 59, 48, 1);
        const HP_FILL_RECT_2: Rect<u16> = Rect::new(16, 60, 48, 2);

        let fg = &self.fg_window;

        let font = Font {
            bg_color: 0,
            fg_color: 4,
            shadow_color: 7,
            ..Font::new(FONT_SMALL_NARROWER as _)
        };

        fg.print_text(&self.poke.name(), Vec2D::new(4, 0), font);
        if self.poke.is_egg() {
            return;
        }

        let lv = aformat!(5, "Lv{}", self.poke.level());
        let lv = ArrayPkstr::<6>::new_str(&lv);
        let lv_pos = Vec2D::new(69 - font.width_for(&lv) as u8, 0);
        fg.print_text(&lv, lv_pos, font);
        fg.copy_tilemap(
            &resource.tileset.get(),
            &resource.hp_bar_map.get(),
            HP_BAR_RECT,
        );

        let hp = aformat!(10, "{:<3}/{:<3}", self.poke.hp(), self.poke.max_hp());
        let hp = ArrayPkstr::<11>::new_str(&hp);
        fg.print_text(&hp, Vec2D { x: 3, y: 45 }, font);

        let width = self.poke.hp() * HP_FILL_RECT_1.width / self.poke.max_hp();
        let (color1, color2) = match width {
            0..12 => (15, 14),
            12..24 => (11, 10),
            _ => (13, 12),
        };
        let rect1 = Rect {
            width,
            ..HP_FILL_RECT_1
        };
        fg.fill_rect(color1, rect1);

        let rect2 = Rect {
            width,
            ..HP_FILL_RECT_2
        };
        fg.fill_rect(color2, rect2);
        fg.copy_to_vram();
    }

    fn update_bg(&self, bg_type: BackgroundStyle, palettes: &[BgPalette]) {
        let bg = &self.bg_window;
        let ko = self.poke.hp() == 0;
        let bg_type = match bg_type {
            BackgroundStyle::Focused if ko => BackgroundStyle::KoFocused,
            BackgroundStyle::Unfocused if ko => BackgroundStyle::KoUnfocused,
            style => style,
        };
        let pal_index = bg_type.palette_index();
        bg.set_palette(palettes[pal_index]);
        bg.put_tilemap();
        bg.copy_to_vram();
    }

    fn update_pos(&mut self, pos: Vec2D<i16>) {
        self.sprite.handle().set_pos(pos + Entry::POKE_SPRITE_OFFS);
        if let Some(item_sprite) = &self.item_sprite {
            item_sprite.handle().set_pos(pos + Entry::ITEM_SPRITE_OFFS);
        }
        if let Some(tera_sprite) = &self.tera_sprite {
            tera_sprite.set_pos(pos + Entry::TERA_SPRITE_OFFS);
        }
        if let Some(status_sprite) = &self.status_sprite {
            status_sprite.set_pos(pos + Entry::STATUS_SPRITE_OFFS);
        }
    }

    async fn switch(&mut self, other: &mut Entry<'a>, frames: i16) {
        self.fg_window.fill(0);
        self.fg_window.copy_to_vram();
        other.fg_window.fill(0);
        other.fg_window.copy_to_vram();
        sleep(1).await;

        let pos_self = MON_POS[self.index as usize];
        let pos_self = Vec2D::new(pos_self.0, pos_self.1).tile_to_pixel();
        let pos_other = MON_POS[other.index as usize];
        let pos_other = Vec2D::new(pos_other.0, pos_other.1).tile_to_pixel();
        for i in 1..=frames {
            let fpos_self = pos_other * i / frames + pos_self * (frames - i) / frames;
            let fpos_other = pos_self * i / frames + pos_other * (frames - i) / frames;
            self.update_pos(fpos_self);
            other.update_pos(fpos_other);
            sleep(1).await
        }

        swap(&mut self.status_sprite, &mut other.status_sprite);
        swap(&mut self.tera_sprite, &mut other.tera_sprite);
        swap(&mut self.item_sprite, &mut other.item_sprite);
        swap(&mut self.sprite, &mut other.sprite);
        Pokemon::swap(&mut self.poke, &mut other.poke);
    }

    async fn create(
        poke: Pokemon,
        resources: &'a Resources,
        bg: BgHandle<'_>,
        fg: BgHandle<'_>,
        index: u8,
    ) -> Entry<'a> {
        const BG_DIM: Vec2D<u8> = Vec2D::new(9, 8);
        const FG_DIM: Vec2D<u8> = Vec2D::new(9, 8);
        const BASE_BLOCK: u16 = 0x20;
        const BLOCK_SIZE: u16 = (BG_DIM.x * BG_DIM.y + FG_DIM.x * FG_DIM.y) as u16;

        let (tile_x, tile_y) = MON_POS[index as usize];
        let tile_pos = Vec2D::new(tile_x, tile_y);

        let mut sprite = PokemonSpritePic::new(&poke, index);
        sprite.handle().set_priority(2);
        sleep(1).await;
        let tera_sprite = tera_sprite(&poke, &resources.tera_sheet);
        let item_sprite = item_sprite(&poke, index.into()).await;
        let status_sprite = status_sprite(&poke, &resources.status_sheet, resources.status_pal);

        let tiles = &resources.tileset.get();
        let block = BLOCK_SIZE * index as u16 + BASE_BLOCK;

        let rect = Rect::from_vecs(tile_pos, BG_DIM);
        let bg_window = Window::create(bg, rect, resources.bg_palettes[0], block);
        sleep(1).await;

        let rect = Rect::from_vecs(tile_pos, FG_DIM);
        let fg_window = Window::create(
            fg,
            rect,
            resources.bg_palettes[0],
            block + BG_DIM.size() as u16,
        );
        fg_window.put_tilemap();
        sleep(1).await;

        let mon_bg_map = &resources.mon_slot_map;
        let rect = Rect::from_vecs(Vec2D::new(0, 0), BG_DIM);
        bg_window.copy_tilemap(tiles, &mon_bg_map.get(), rect);
        sleep(1).await;

        mgba_warn!("{:?} {:?}", fg_window, bg_window);

        bg_window.put_tilemap();
        bg_window.copy_to_vram();
        sleep(1).await;

        let mut entry = Entry {
            index,
            poke,
            sprite,
            item_sprite,
            tera_sprite,
            status_sprite,
            bg_window,
            fg_window,
        };
        entry.update_pos(tile_pos.tile_to_pixel());
        entry
    }
}

struct Menu<'a> {
    pokemons: *mut pokeemerald::Pokemon,

    style: Style,
    resources: &'a Resources,
    fg: BgHandle<'a>,

    entries: ArrayVec<Entry<'a>, 6>,
    focused_entry: u8,

    exit_callback: Box<dyn Fn()>,
}

#[derive(TryFrom, Clone, Copy)]
#[try_from(repr)]
#[repr(i32)]
enum PokeAction {
    Summary = 1,
    Switch = 2,
    GiveItem = 3,
    TakeItem = 4,
}

impl PokeAction {
    fn name(self) -> &'static Pkstr {
        match self {
            PokeAction::Summary => pkstr!(b"Summary"),
            PokeAction::Switch => pkstr!(b"Switch"),
            PokeAction::GiveItem => pkstr!(b"Give Item"),
            PokeAction::TakeItem => pkstr!(b"Take Item"),
        }
    }

    fn is_available(self, style: Style, poke: &Pokemon) -> bool {
        match self {
            PokeAction::Summary => true,
            PokeAction::Switch => style == Style::Party,
            PokeAction::GiveItem => style == Style::Party && !poke.is_egg(),
            PokeAction::TakeItem => style == Style::Party && poke.item().is_some(),
        }
    }
}

fn disjoint_borrow_mut<T>(indices: (usize, usize), slice: &mut [T]) -> (&mut T, &mut T) {
    if indices.0 == indices.1 {
        panic!()
    }
    if indices.0 >= slice.len() || indices.1 >= slice.len() {
        panic!()
    }
    unsafe {
        (
            &mut *slice.as_mut_ptr().add(indices.0),
            &mut *slice.as_mut_ptr().add(indices.1),
        )
    }
}

impl Menu<'_> {
    fn index_updown(&self, base: u8, delta: i8) -> u8 {
        let max = self.entries.len() as i8;
        let new = base as i8 + delta;
        let new = new.rem_euclid(6);
        let new = min(max - 1, new);
        new as u8
    }

    fn index_leftright(&self, base: u8, delta: i8) -> u8 {
        (base as i8 + delta).rem_euclid(self.entries.len() as i8) as u8
    }

    fn update_index(&self, index: u8) -> u8 {
        if Button::Right.pressed() {
            return self.index_leftright(index, 1);
        }
        if Button::Left.pressed() {
            return self.index_leftright(index, -1);
        }
        if Button::Up.pressed() || Button::Down.pressed() {
            return self.index_updown(index, -3);
        }
        index
    }

    fn change_focus(&mut self, new_index: u8) {
        use BackgroundStyle::*;
        self.entries[self.focused_entry as usize].update_bg(Unfocused, &self.resources.bg_palettes);
        self.entries[new_index as usize].update_bg(Focused, &self.resources.bg_palettes);
        self.focused_entry = new_index;
    }

    fn open_summary_screen(&mut self) {
        let focused_entry = self.focused_entry;
        let max = self.entries.len() as u8 - 1;
        let pokemons = self.pokemons;
        self.exit_callback = Box::new(move || unsafe {
            ShowPokemonSummaryScreen(
                PokemonSummaryScreenMode::SUMMARY_MODE_NORMAL as u8,
                pokemons.cast(),
                focused_entry,
                max,
                Some(return_from_party_callback),
            );
        });
    }

    fn change_hold_item(&mut self) {
        *SELECTED_POKE.borrow_mut() = self.focused_entry;
        self.exit_callback = Box::new(move || unsafe {
            GoToBagMenu(
                ITEMMENULOCATION_PARTY as u8,
                5,
                Some(return_from_give_hold_item_callback),
            );
        })
    }

    fn take_hold_item(&mut self) {
        let entry = &mut self.entries[self.focused_entry as usize];
        if let Some(item) = entry.poke.item() {
            unsafe {
                AddBagItem(item as u16, 1);
            }
            entry.poke.set_item(0);
            entry.item_sprite = None;
        }
    }

    fn change_focus_switch(&mut self, switch_index: u8, new_index: u8) {
        let previous = match switch_index == self.focused_entry {
            true => BackgroundStyle::SwitchUnfocused,
            false => BackgroundStyle::Unfocused,
        };
        let palettes = &self.resources.bg_palettes;
        self.entries[self.focused_entry as usize].update_bg(previous, palettes);
        self.entries[new_index as usize].update_bg(BackgroundStyle::SwitchFocused, palettes);
        self.focused_entry = new_index;
    }

    async fn choose_switch_mon(&mut self) -> Option<usize> {
        let switching_index = self.focused_entry;
        loop {
            sleep(1).await;
            if Button::B.pressed() {
                self.entries[switching_index as usize]
                    .update_bg(BackgroundStyle::Unfocused, &self.resources.bg_palettes);
                return None;
            }
            if Button::A.pressed() {
                return Some(switching_index.into());
            }

            let new_index = self.update_index(self.focused_entry);
            if new_index != self.focused_entry {
                self.change_focus_switch(switching_index, new_index);
            }
        }
    }

    async fn switch_mon(&mut self) {
        let focus = self.focused_entry as usize;
        let palettes = &self.resources.bg_palettes;
        self.entries[focus].update_bg(BackgroundStyle::SwitchFocused, palettes);
        if let Some(switch) = self.choose_switch_mon().await {
            if switch != self.focused_entry as usize {
                let indices = (self.focused_entry as usize, switch);
                let (a, b) = disjoint_borrow_mut(indices, &mut self.entries);
                a.switch(b, 20).await;
                a.print_info(self.resources);
                b.print_info(self.resources);
                a.fg_window.copy_to_vram();
                b.fg_window.copy_to_vram();
                self.entries[switch].update_bg(BackgroundStyle::Unfocused, palettes);
            }
        }
        let focus = self.focused_entry as usize;
        self.entries[focus].update_bg(BackgroundStyle::Focused, palettes);
    }

    async fn select_action(&mut self) -> Option<PokeAction> {
        let msg_box = load_msg_box_gfx(self.fg, 0x3B0, 14);
        let win = Window::create(self.fg, Rect::new(21, 11, 8, 8), msg_box.palette, 0x380);
        win.put_tilemap();
        win.copy_to_vram();

        let user_window = load_user_window_gfx(self.fg, 0x3D0, 15);
        win.draw_border(user_window);

        let entry = &self.entries[self.focused_entry as usize];

        let list_items: ArrayVec<ListMenuItem, 6> = [
            PokeAction::Summary,
            PokeAction::Switch,
            PokeAction::GiveItem,
            PokeAction::TakeItem,
        ]
        .into_iter()
        .filter(|act| act.is_available(self.style, &entry.poke))
        .map(|act| ListMenuItem {
            id: act as i32,
            name: act.name().as_ptr(),
        })
        .collect();

        let list = ListMenu::create(&win, &list_items, 8, 4, 0, [2, 1, 3], FONT_SMALL);
        let ret = match list.wait_for_result().await {
            Some(val) => val.try_into().ok(),
            None => None,
        };

        win.clear_with_border();
        win.copy_to_vram();

        drop(list);
        drop(win);

        if let Some(entry) = self.entries.get(5) {
            entry.fg_window.put_tilemap();
        }
        ret
    }

    async fn main_loop(&mut self) {
        loop {
            sleep(1).await;
            if Button::B.pressed() {
                break;
            }

            let new_index = self.update_index(self.focused_entry);
            if new_index != self.focused_entry {
                self.change_focus(new_index);
                continue;
            }

            if Button::A.pressed() {
                match self.select_action().await {
                    Some(PokeAction::Summary) => {
                        self.open_summary_screen();
                        break;
                    }
                    Some(PokeAction::GiveItem) => {
                        self.change_hold_item();
                        break;
                    }
                    Some(PokeAction::TakeItem) => {
                        self.take_hold_item();
                        continue;
                    }
                    Some(PokeAction::Switch) => {
                        self.switch_mon().await;
                        continue;
                    }
                    None => continue,
                }
            }
        }
    }
}

struct Resources {
    tileset: AllocBuf<TileBitmap4bpp>,
    bg_map: AllocBuf<Tile4bpp>,
    mon_slot_map: AllocBuf<TilePlain>,
    bg_palettes: [BgPalette; 6],

    hp_bar_map: AllocBuf<TilePlain>,

    tera_sheet: SpriteSheet<AllocBuf<TileBitmap4bpp>>,
    status_sheet: SpriteSheet<AllocBuf<TileBitmap4bpp>>,
    status_pal: ObjPalette,
}

async fn load_resources() -> Resources {
    let bg_palettes = load_bg_palettes(0, &PAL.load().get());
    sleep(1).await;

    let tileset = TILESET.load();
    sleep(1).await;

    let bg_map = SCROLL_BG_MAP.load();

    sleep(1).await;

    let mon_slot_map = MON_BG_MAP.load();
    sleep(1).await;

    let hp_bar_map = HP_MAP.load();

    let tera_sheet = SpriteSheet::load(TERA_SPRITE.load(), 15000, SPRITE_SIZE_16x16 as u8);
    sleep(1).await;

    let status_sheet = SpriteSheet::load(STATUS_SHEET.load(), 15001, SPRITE_SIZE_32x8 as u8);
    let status_pal = load_obj_palette(12, &STATUS_PAL.load().get());

    load_type_palettes();

    Resources {
        tileset,
        bg_map,
        mon_slot_map,
        bg_palettes,
        hp_bar_map,
        tera_sheet,
        status_sheet,
        status_pal,
    }
}

async fn summary_screen(
    back: MainCallback,
    style: Style,
    pokemons: *mut pokeemerald::Pokemon,
    count: usize,
    index: u8,
) {
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

    let resources = load_resources().await;

    let bg_map = Tilemap {
        map: 0,
        buffer: &resources.bg_map,
    };
    let tileset = Tileset {
        char_base: 1,
        offset: 0,
        tiles: &resources.tileset,
        palette: resources.bg_palettes[0],
    };

    let scroll_bg = Background::load(BackgroundIndex::Background3, 3, tileset, bg_map).await;
    let scroll_bg = scroll_bg.handle();
    scroll_bg.show();

    let empty_tilemap = AllocBuf::new(vec![0u8; resources.bg_map.size_bytes()].into_boxed_slice());
    let empty_tilemap = Tilemap {
        map: 2,
        buffer: empty_tilemap,
    };
    let fixed_bg = Background::load(BackgroundIndex::Background2, 2, tileset, empty_tilemap).await;
    let fixed_bg = fixed_bg.handle();
    fixed_bg.set_pos(0, 0);
    fixed_bg.show();

    let buffer_fg = AllocBuf::new(vec![0u8; resources.bg_map.size_bytes()].into_boxed_slice());
    let empty_tilemap = Tilemap {
        map: 4,
        buffer: &buffer_fg,
    };
    let fg = Background::load(BackgroundIndex::Background1, 1, tileset, empty_tilemap).await;
    let fg = fg.handle();
    fg.set_pos(0, 0);
    fg.show();
    fg.copy_tilemap_to_vram();

    let mut entries: ArrayVec<Entry, 6> = ArrayVec::new();
    for i in 0..min(6, count) {
        let poke = unsafe { Pokemon::from_ptr_and_index(pokemons, i) };
        entries.push(Entry::create(poke, &resources, fixed_bg, fg, i as u8).await);
    }

    for entry in entries.iter() {
        entry.print_info(&resources);
        sleep(1).await;
    }
    for (eindex, entry) in entries.iter().enumerate() {
        let style = match eindex as u8 {
            i if i == index => BackgroundStyle::Focused,
            _ => BackgroundStyle::Unfocused,
        };
        entry.update_bg(style, &resources.bg_palettes);
    }

    unsafe { SetVBlankCallback(Some(vblank_cb)) };
    fade_palette(PaletteMask::ALL, 0, 16, 0, 0).await;

    let mut menu = Menu {
        pokemons,
        style,
        resources: &resources,
        fg,
        entries,
        focused_entry: index,
        exit_callback: Box::new(move || unsafe { SetMainCallback2(back) }),
    };

    menu.main_loop().await;
    fade_palette(PaletteMask::ALL, 0, 0, 16, 0).await;
    (menu.exit_callback)();
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

        Window::clear_all();
        sleep(1).await;

        ResetBgsAndClearDma3BusyFlags(0);
    }
}

