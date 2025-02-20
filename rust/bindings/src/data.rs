use core::ffi::c_void;

use crate::charmap::ArrayPkstr;
use crate::pokeemerald::{self, *};

pub struct Pokemon {
    ptr: *mut pokeemerald::Pokemon,
}

impl Pokemon {
    pub unsafe fn from_ptr_and_index(ptr: *mut pokeemerald::Pokemon, index: usize) -> Self {
        Pokemon {
            ptr: unsafe { ptr.add(index) },
        }
    }

    pub fn as_ptr(&self) -> *mut pokeemerald::Pokemon {
        self.ptr
    }
    pub fn get_player_party(index: u8) -> Option<Pokemon> {
        unsafe {
            if index >= gPlayerPartyCount {
                return None;
            }
            let party_ptr = &raw mut gPlayerParty[0];
            Some(Pokemon {
                ptr: party_ptr.add(index as usize),
            })
        }
    }
    pub fn get_mon_data(&self, data: u32) -> u32 {
        unsafe { GetMonData2(self.ptr, data as i32) }
    }

    pub unsafe fn set_mon_data(&self, data: u32, ptr: *const c_void) {
        unsafe { SetMonData(self.ptr, data as i32, ptr) };
    }

    pub fn level(&self) -> u8 {
        self.get_mon_data(MON_DATA_LEVEL) as u8
    }

    pub fn hp(&self) -> u16 {
        self.get_mon_data(MON_DATA_HP) as u16
    }

    pub fn max_hp(&self) -> u16 {
        self.get_mon_data(MON_DATA_MAX_HP) as u16
    }

    pub fn name(&self) -> ArrayPkstr<13> {
        let mut slice = [0xFF; 13];
        unsafe { GetMonData3(self.ptr, MON_DATA_NICKNAME as _, slice.as_mut_ptr()) };
        unsafe { ArrayPkstr::from_slice(&slice) }
    }

    pub fn tera_type(&self) -> u16 {
        self.get_mon_data(MON_DATA_TERA_TYPE) as u16
    }

    pub fn species(&self) -> u16 {
        self.get_mon_data(MON_DATA_SPECIES_OR_EGG) as _
    }

    pub fn is_egg(&self) -> bool {
        self.get_mon_data(MON_DATA_IS_EGG) != 0
    }
    pub fn personality(&self) -> u32 {
        self.get_mon_data(MON_DATA_PERSONALITY)
    }
    pub fn shiny(&self) -> bool {
        self.get_mon_data(MON_DATA_IS_SHINY) != 0
    }
    pub fn item(&self) -> Option<usize> {
        match self.get_mon_data(MON_DATA_HELD_ITEM) {
            0 => None,
            n => Some(n as usize),
        }
    }
    pub fn set_item(&self, item: u16) {
        unsafe {
            self.set_mon_data(MON_DATA_HELD_ITEM, (&raw const item).cast());
        }
    }

    pub fn status(&self) -> u8 {
        if self.hp() == 0 {
            return 7; // AILMENT_FNT
        }
        unsafe { GetAilmentFromStatus(self.get_mon_data(MON_DATA_STATUS)) }
    }

    pub fn swap(this: &mut Self, other: &mut Self) {
        unsafe {
            core::ptr::swap(this.ptr, other.ptr);
        }
    }
}

pub fn get_species(species: usize) -> &'static SpeciesInfo {
    unsafe { gSpeciesInfo.as_ptr().add(species).as_ref().unwrap() }
}

pub fn get_item(index: usize) -> &'static Item {
    unsafe { gItemsInfo.as_ptr().add(index).as_ref().unwrap() }
}
