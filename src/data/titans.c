#include "constants/abilities.h"
#include "constants/species.h"
#include "constants/trainers.h"

#include "global.h"
#include "data.h"

static const u32 MAX_IVS = TRAINER_PARTY_IVS(31, 31, 31, 31, 31, 31);
static const struct TrainerMon KLAWF = {
    .species = SPECIES_KLAWF,
    .lvl = 15,
    .ability = ABILITY_ANGER_SHELL,
    .heldItem = ITEM_CLEAR_AMULET,
    .nature = NATURE_ADAMANT,
    .iv = MAX_IVS,
    .ev = TRAINER_PARTY_EVS(252, 252, 0, 0, 0, 0),
    .moves = {
        MOVE_SMACK_DOWN,
        MOVE_LEECH_LIFE,
        MOVE_RAZOR_SHELL,
        MOVE_STOMPING_TANTRUM,
    }
};
static const struct TrainerMon BOMBIRDIER = {
    .species = SPECIES_BOMBIRDIER,
    .lvl = 22,
    .ability = ABILITY_TECHNICIAN,
    .heldItem = ITEM_ASSAULT_VEST,
    .nature = NATURE_ADAMANT,
    .iv = MAX_IVS,
    .ev = TRAINER_PARTY_EVS(252, 252, 0, 0, 0, 0),
    .moves = {
        MOVE_LASH_OUT,
        MOVE_PLUCK,
        MOVE_SMART_STRIKE,
        MOVE_SMACK_DOWN,
    }
};

static const struct TrainerMon ORTHWORM = {
    .species = SPECIES_ORTHWORM,
    .lvl = 33,
    .ability = ABILITY_EARTH_EATER,
    .heldItem = ITEM_MENTAL_HERB,
    .nature = NATURE_CAREFUL,
    .iv = MAX_IVS,
    .ev = TRAINER_PARTY_EVS(252, 0, 0, 0, 0, 252),
    .moves = {
        MOVE_IRON_TAIL,
        MOVE_DRILL_RUN,
        MOVE_STONE_EDGE,
        MOVE_COIL,
    }
};

static const struct TrainerMon GREAT_TUSK = {
    .species = SPECIES_GREAT_TUSK,
    .lvl = 47,
    .ability = ABILITY_PRISM_ARMOR,
    .heldItem = ITEM_ASSAULT_VEST,
    .nature = NATURE_ADAMANT,
    .iv = MAX_IVS,
    .ev = TRAINER_PARTY_EVS(0, 252, 0, 0, 0, 252),
    .moves = {
        MOVE_BEHEMOTH_BASH,
        MOVE_PRECIPICE_BLADES,
        MOVE_CRUSH_CLAW,
        MOVE_TEMPER_FLARE,
    }
};

static const struct TrainerMon IRON_TREADS = {
    .species = SPECIES_IRON_TREADS,
    .lvl = 47,
    .ability = ABILITY_FILTER,
    .heldItem = ITEM_ASSAULT_VEST,
    .nature = NATURE_ADAMANT,
    .iv = MAX_IVS,
    .ev = TRAINER_PARTY_EVS(252, 252, 0, 0, 0, 0),
    .moves = {
        MOVE_BEHEMOTH_BLADE,
        MOVE_PRECIPICE_BLADES,
        MOVE_CRUSH_CLAW,
        MOVE_ZING_ZAP,
    }
};

static const struct TrainerMon DONDOZO = {
    .species = SPECIES_DONDOZO,
    .lvl = 58,
    .ability = ABILITY_RIPEN,
    .heldItem = ITEM_APICOT_BERRY,
    .nature = NATURE_ADAMANT,
    .iv = MAX_IVS,
    .ev = TRAINER_PARTY_EVS(252, 252, 0, 0, 0, 0),
    .moves = {
        MOVE_OUTRAGE,
        MOVE_GUNK_SHOT,
        MOVE_EARTHQUAKE,
        MOVE_STUFF_CHEEKS,
    }
};

static const struct TrainerMon TATSUGIRI = {
    .species = SPECIES_TATSUGIRI,
    .lvl = 58,
    .ability = ABILITY_CONTRARY,
    .heldItem = ITEM_ASSAULT_VEST,
    .nature = NATURE_BOLD,
    .iv = MAX_IVS,
    .ev = TRAINER_PARTY_EVS(252, 0, 252, 0, 0, 0),
    .moves = {
        MOVE_HYDRO_CANNON,
        MOVE_DRACO_METEOR,
    }
};

static const struct TrainerMon OKIDOGI = {
    .species = SPECIES_OKIDOGI,
    .lvl = 43,
    .ability = ABILITY_TOXIC_CHAIN,
    .heldItem = ITEM_BLACK_SLUDGE,
    .nature = NATURE_ADAMANT,
    .iv = MAX_IVS,
    .ev = TRAINER_PARTY_EVS(0, 252, 0, 0, 0, 252),
    .moves = {
        MOVE_DYNAMIC_PUNCH,
        MOVE_POISON_FANG,
        MOVE_PAYBACK,
        MOVE_MEGA_PUNCH,
    }
};

static const struct TrainerMon MUNKIDORI = {
    .species = SPECIES_MUNKIDORI,
    .lvl = 43,
    .ability = ABILITY_TOXIC_CHAIN,
    .heldItem = ITEM_BLACK_SLUDGE,
    .nature = NATURE_MODEST,
    .iv = MAX_IVS,
    .ev = TRAINER_PARTY_EVS(0, 0, 252, 0, 252, 0),
    .moves = {
        MOVE_PSYSTRIKE,
        MOVE_MALIGNANT_CHAIN,
        MOVE_FOCUS_BLAST,
        MOVE_DARK_PULSE,
    }
};

static const struct TrainerMon FEZANDIPITI = {
    .species = SPECIES_FEZANDIPITI,
    .lvl = 43,
    .ability = ABILITY_TOXIC_CHAIN,
    .heldItem = ITEM_BLACK_SLUDGE,
    .nature = NATURE_MODEST,
    .iv = MAX_IVS,
    .ev = TRAINER_PARTY_EVS(252, 0, 0, 0, 252, 0),
    .moves = {
        MOVE_ALLURING_VOICE,
        MOVE_VENOSHOCK,
        MOVE_PSYCHIC,
        MOVE_HEAT_WAVE,
    }
};

const struct Titan gTitans[]
    = {
          [1] = {
              .titan = KLAWF,
              .type = TITAN_TYPE_FLEE_10,
          },
          [2] = {
              .titan = KLAWF,
              .healthPercent = 200,
              .partnerId = PARTNER_PEPPER_NUSI_01,
          },
          [3] = {
              .titan = BOMBIRDIER,
              .type = TITAN_TYPE_FLEE_10,
          },
          [4] = {
              .titan = BOMBIRDIER,
              .healthPercent = 200,
              .partnerId = PARTNER_PEPPER_NUSI_03,
          },
          [5] = {
              .titan = ORTHWORM,
              .type = TITAN_TYPE_FLEE_10,
          },
          [6] = {
              .titan = ORTHWORM,
              .healthPercent = 200,
              .partnerId = PARTNER_PEPPER_NUSI_02,
          },
          [7] = {
              .titan = GREAT_TUSK,
              .type = TITAN_TYPE_FLEE_10,
          },
          [8] = {
              .titan = GREAT_TUSK,
              .healthPercent = 200,
              .partnerId = PARTNER_PEPPER_NUSI_04,
          },
          [9] = {
              .titan = IRON_TREADS,
              .type = TITAN_TYPE_FLEE_10,
          },
          [10] = {
              .titan = IRON_TREADS,
              .healthPercent = 200,
              .partnerId = PARTNER_PEPPER_NUSI_04,
          },
          [11] = {
              .titan = DONDOZO,
              .type = TITAN_TYPE_FLEE_10,
          },
          [12] = {
              .titan = TATSUGIRI,
              .healthPercent = 225,
              .partnerId = PARTNER_PEPPER_NUSI_05,
          },
          [13] = {
              .titan = OKIDOGI,
              .healthPercent = 400,
              .partnerId = PARTNER_SISTER_ONITAIZI,
              .type = TITAN_TYPE_BOOST_DEF,
          },
          [14] = {
              .titan = MUNKIDORI,
              .healthPercent = 400,
              .partnerId = PARTNER_SISTER_ONITAIZI,
              .type = TITAN_TYPE_BOOST_SPDEF,
          },
          [15] = {
              .titan = FEZANDIPITI,
              .healthPercent = 400,
              .partnerId = PARTNER_SISTER_ONITAIZI,
              .type = TITAN_TYPE_BOOST_SPEED,
          },
      };
