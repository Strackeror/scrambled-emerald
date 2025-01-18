#ifdef __INTELLISENSE__
#include "data.h"
static struct Trainer trainers[] = {
#endif

    [TRAINER_OGERPON] = {
        .trainerName = _("Ogerpon"),
        .trainerPic = TRAINER_PIC_OGERPON,
        .trainerClass = TRAINER_CLASS_OGERPON,
        .encounterMusic_gender = TRAINER_ENCOUNTER_MUSIC_SUSPICIOUS,
        .aiFlags = AI_FLAG_BASIC_TRAINER | AI_FLAG_SEQUENCE_SWITCHING,
        .partySize = 4,
        .party = (const struct TrainerMon[]) {
            {
                .species = SPECIES_OGERPON_HEARTHFLAME,
                .lvl = 50,
                .ability = ABILITY_DEFIANT,
                .heldItem = ITEM_HEARTHFLAME_MASK,
                .teraType = TYPE_FIRE,
                .nature = NATURE_JOLLY,
                .iv = TRAINER_PARTY_IVS(31, 31, 31, 31, 31, 31),
                .ev = TRAINER_PARTY_EVS(252, 0, 0, 252, 0, 0),
                .moves = {
                    MOVE_IVY_CUDGEL,
                    MOVE_HORN_LEECH,
                    MOVE_STOMPING_TANTRUM,
                    MOVE_SPIRIT_BREAK,
                },
            },
            {
                .species = SPECIES_OGERPON_WELLSPRING,
                .lvl = 50,
                .ability = ABILITY_DEFIANT,
                .heldItem = ITEM_WELLSPRING_MASK,
                .teraType = TYPE_WATER,
                .nature = NATURE_JOLLY,
                .iv = TRAINER_PARTY_IVS(31, 31, 31, 31, 31, 31),
                .ev = TRAINER_PARTY_EVS(252, 0, 0, 252, 0, 0),
                .moves = {
                    MOVE_IVY_CUDGEL,
                    MOVE_HORN_LEECH,
                    MOVE_ICE_HAMMER,
                    MOVE_SWEET_KISS,
                },
            },
            {
                .species = SPECIES_OGERPON_CORNERSTONE,
                .lvl = 50,
                .ability = ABILITY_DEFIANT,
                .heldItem = ITEM_CORNERSTONE_MASK,
                .teraType = TYPE_ROCK,
                .nature = NATURE_JOLLY,
                .iv = TRAINER_PARTY_IVS(31, 31, 31, 31, 31, 31),
                .ev = TRAINER_PARTY_EVS(252, 0, 0, 252, 0, 0),
                .moves = {
                    MOVE_IVY_CUDGEL,
                    MOVE_HORN_LEECH,
                    MOVE_BODY_PRESS,
                    MOVE_STOMPING_TANTRUM,
                },
            },
            {
                .species = SPECIES_OGERPON_TEAL,
                .lvl = 50,
                .ability = ABILITY_DEFIANT,
                .heldItem = ITEM_NONE,
                .teraType = TYPE_GRASS,
                .nature = NATURE_JOLLY,
                .iv = TRAINER_PARTY_IVS(31, 31, 31, 31, 31, 31),
                .ev = TRAINER_PARTY_EVS(252, 0, 0, 252, 0, 0),
                .moves = {
                    MOVE_THRASH,
                    MOVE_HORN_LEECH,
                    MOVE_TAKE_DOWN,
                    MOVE_ZEN_HEADBUTT,
                },
            },

        },
    },

#ifdef __INTELLISENSE__
};
#endif
