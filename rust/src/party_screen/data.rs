use crate::pokeemerald::{self, *};

pub struct Pokemon {
    ptr: *mut pokeemerald::Pokemon,
}

impl Pokemon {
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

    pub fn species(&self) -> u16 {
        self.get_mon_data(MON_DATA_SPECIES) as _
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
}

pub fn get_species(species: usize) -> &'static SpeciesInfo {
    unsafe { gSpeciesInfo.as_ptr().add(species).as_ref().unwrap() }
}

pub fn get_item(index: usize) -> &'static Item {
    unsafe { gItemsInfo.as_ptr().add(index).as_ref().unwrap() }
}
