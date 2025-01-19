//
// DO NOT MODIFY THIS FILE! It is auto-generated from src/data/battle_partners.party
//
// If you want to modify this file set COMPETITIVE_PARTY_SYNTAX to FALSE
// in include/config/general.h and remove this notice.
// Use sed -i '/^#line/d' 'src/data/battle_partners.h' to remove #line markers.
//

#line 1 "src/data/battle_partners.party"

#line 1
    [PARTNER_NONE] =
    {
#line 3
        .trainerClass = TRAINER_CLASS_PKMN_TRAINER_1,
#line 4
        .trainerPic = TRAINER_BACK_PIC_BRENDAN,
        .encounterMusic_gender = 
#line 6
            TRAINER_ENCOUNTER_MUSIC_MALE,
        .partySize = 0,
        .party = (const struct TrainerMon[])
        {
        },
    },
#line 8
    [PARTNER_STEVEN] =
    {
#line 9
        .trainerName = _("STEVEN"),
#line 10
        .trainerClass = TRAINER_CLASS_RIVAL,
#line 11
        .trainerPic = TRAINER_BACK_PIC_STEVEN,
        .encounterMusic_gender = 
#line 13
            TRAINER_ENCOUNTER_MUSIC_MALE,
        .partySize = 3,
        .party = (const struct TrainerMon[])
        {
            {
#line 15
            .species = SPECIES_METANG,
            .gender = TRAINER_MON_RANDOM_GENDER,
#line 19
            .ev = TRAINER_PARTY_EVS(0, 252, 252, 0, 6, 0),
#line 18
            .iv = TRAINER_PARTY_IVS(31, 31, 31, 31, 31, 31),
#line 17
            .lvl = 42,
#line 16
            .nature = NATURE_BRAVE,
            .dynamaxLevel = MAX_DYNAMAX_LEVEL,
            .moves = {
#line 20
                MOVE_LIGHT_SCREEN,
                MOVE_PSYCHIC,
                MOVE_REFLECT,
                MOVE_METAL_CLAW,
            },
            },
            {
#line 25
            .species = SPECIES_SKARMORY,
            .gender = TRAINER_MON_RANDOM_GENDER,
#line 29
            .ev = TRAINER_PARTY_EVS(252, 0, 0, 0, 6, 252),
#line 28
            .iv = TRAINER_PARTY_IVS(31, 31, 31, 31, 31, 31),
#line 27
            .lvl = 43,
#line 26
            .nature = NATURE_IMPISH,
            .dynamaxLevel = MAX_DYNAMAX_LEVEL,
            .moves = {
#line 30
                MOVE_TOXIC,
                MOVE_AERIAL_ACE,
                MOVE_PROTECT,
                MOVE_STEEL_WING,
            },
            },
            {
#line 35
            .species = SPECIES_AGGRON,
            .gender = TRAINER_MON_RANDOM_GENDER,
#line 39
            .ev = TRAINER_PARTY_EVS(0, 252, 0, 0, 252, 6),
#line 38
            .iv = TRAINER_PARTY_IVS(31, 31, 31, 31, 31, 31),
#line 37
            .lvl = 44,
#line 36
            .nature = NATURE_ADAMANT,
            .dynamaxLevel = MAX_DYNAMAX_LEVEL,
            .moves = {
#line 40
                MOVE_THUNDER,
                MOVE_PROTECT,
                MOVE_SOLAR_BEAM,
                MOVE_DRAGON_CLAW,
            },
            },
        },
    },
#line 45
    [PARTNER_PEPPER_NUSI_01] =
    {
#line 46
        .trainerName = _("STEVEN"),
#line 47
        .trainerPic = TRAINER_BACK_PIC_STEVEN,
        .encounterMusic_gender = 
#line 49
            TRAINER_ENCOUNTER_MUSIC_FEMALE,
#line 50
        .doubleBattle = FALSE,
#line 51
        .aiFlags = AI_FLAG_BASIC_TRAINER | AI_FLAG_ACE_POKEMON,
        .partySize = 1,
        .party = (const struct TrainerMon[])
        {
            {
#line 54
            .species = SPECIES_CAPSAKID,
            .gender = TRAINER_MON_RANDOM_GENDER,
#line 54
            .heldItem = ITEM_CHARTI_BERRY,
#line 58
            .ev = TRAINER_PARTY_EVS(252, 0, 252, 0, 0, 0),
#line 57
            .iv = TRAINER_PARTY_IVS(31, 31, 31, 31, 31, 31),
#line 56
            .ability = ABILITY_PRANKSTER,
#line 55
            .lvl = 15,
#line 59
            .nature = NATURE_BOLD,
            .dynamaxLevel = MAX_DYNAMAX_LEVEL,
            .moves = {
#line 60
                MOVE_FIRE_SPIN,
                MOVE_LEECH_SEED,
                MOVE_TORMENT,
                MOVE_PROTECT,
            },
            },
        },
    },
#line 66
    [PARTNER_PEPPER_NUSI_02] =
    {
#line 67
        .trainerName = _("STEVEN"),
#line 68
        .trainerPic = TRAINER_BACK_PIC_STEVEN,
        .encounterMusic_gender = 
#line 70
            TRAINER_ENCOUNTER_MUSIC_FEMALE,
#line 71
        .doubleBattle = FALSE,
#line 72
        .aiFlags = AI_FLAG_BASIC_TRAINER | AI_FLAG_ACE_POKEMON,
        .partySize = 1,
        .party = (const struct TrainerMon[])
        {
            {
#line 75
            .species = SPECIES_FLAPPLE,
            .gender = TRAINER_MON_RANDOM_GENDER,
#line 75
            .heldItem = ITEM_SITRUS_BERRY,
#line 79
            .ev = TRAINER_PARTY_EVS(252, 0, 252, 0, 0, 0),
#line 78
            .iv = TRAINER_PARTY_IVS(31, 31, 31, 31, 31, 31),
#line 77
            .ability = ABILITY_UNBURDEN,
#line 76
            .lvl = 33,
#line 80
            .nature = NATURE_BOLD,
            .dynamaxLevel = MAX_DYNAMAX_LEVEL,
            .moves = {
#line 81
                MOVE_INFESTATION,
                MOVE_ROOST,
                MOVE_SUBSTITUTE,
                MOVE_GRASSY_TERRAIN,
            },
            },
        },
    },
#line 87
    [PARTNER_PEPPER_NUSI_03] =
    {
#line 88
        .trainerName = _("STEVEN"),
#line 89
        .trainerPic = TRAINER_BACK_PIC_STEVEN,
        .encounterMusic_gender = 
#line 91
            TRAINER_ENCOUNTER_MUSIC_FEMALE,
#line 92
        .doubleBattle = FALSE,
#line 93
        .aiFlags = AI_FLAG_BASIC_TRAINER | AI_FLAG_ACE_POKEMON,
        .partySize = 1,
        .party = (const struct TrainerMon[])
        {
            {
#line 96
            .species = SPECIES_SHELLDER,
            .gender = TRAINER_MON_RANDOM_GENDER,
#line 96
            .heldItem = ITEM_ROCKY_HELMET,
#line 100
            .ev = TRAINER_PARTY_EVS(252, 0, 252, 0, 0, 0),
#line 99
            .iv = TRAINER_PARTY_IVS(31, 31, 31, 31, 31, 31),
#line 98
            .ability = ABILITY_SHELL_ARMOR,
#line 97
            .lvl = 22,
#line 101
            .nature = NATURE_IMPISH,
            .dynamaxLevel = MAX_DYNAMAX_LEVEL,
            .moves = {
#line 102
                MOVE_DIVE,
                MOVE_SPIKY_SHIELD,
                MOVE_FOLLOW_ME,
                MOVE_HYDRO_PUMP,
            },
            },
        },
    },
#line 108
    [PARTNER_PEPPER_NUSI_04] =
    {
#line 109
        .trainerName = _("STEVEN"),
#line 110
        .trainerPic = TRAINER_BACK_PIC_STEVEN,
        .encounterMusic_gender = 
#line 112
            TRAINER_ENCOUNTER_MUSIC_FEMALE,
#line 113
        .doubleBattle = FALSE,
#line 114
        .aiFlags = AI_FLAG_BASIC_TRAINER | AI_FLAG_ACE_POKEMON,
        .partySize = 1,
        .party = (const struct TrainerMon[])
        {
            {
#line 117
            .species = SPECIES_BRELOOM,
            .gender = TRAINER_MON_RANDOM_GENDER,
#line 117
            .heldItem = ITEM_TOXIC_ORB,
#line 121
            .ev = TRAINER_PARTY_EVS(252, 0, 252, 0, 0, 0),
#line 120
            .iv = TRAINER_PARTY_IVS(31, 31, 31, 31, 31, 31),
#line 119
            .ability = ABILITY_TOXIC_BOOST,
#line 118
            .lvl = 47,
#line 122
            .nature = NATURE_IMPISH,
            .dynamaxLevel = MAX_DYNAMAX_LEVEL,
            .moves = {
#line 123
                MOVE_DRAIN_PUNCH,
                MOVE_SEED_BOMB,
                MOVE_STRENGTH_SAP,
                MOVE_RAGE_POWDER,
            },
            },
        },
    },
#line 129
    [PARTNER_PEPPER_NUSI_05] =
    {
#line 130
        .trainerName = _("STEVEN"),
#line 131
        .trainerPic = TRAINER_BACK_PIC_STEVEN,
        .encounterMusic_gender = 
#line 133
            TRAINER_ENCOUNTER_MUSIC_FEMALE,
#line 134
        .doubleBattle = FALSE,
#line 135
        .aiFlags = AI_FLAG_BASIC_TRAINER | AI_FLAG_ACE_POKEMON,
        .partySize = 1,
        .party = (const struct TrainerMon[])
        {
            {
#line 138
            .species = SPECIES_GARGANACL,
            .gender = TRAINER_MON_RANDOM_GENDER,
#line 138
            .heldItem = ITEM_SITRUS_BERRY,
#line 142
            .ev = TRAINER_PARTY_EVS(252, 0, 0, 0, 0, 252),
#line 141
            .iv = TRAINER_PARTY_IVS(31, 31, 31, 31, 31, 31),
#line 140
            .ability = ABILITY_WATER_ABSORB,
#line 139
            .lvl = 58,
#line 143
            .nature = NATURE_CAREFUL,
            .dynamaxLevel = MAX_DYNAMAX_LEVEL,
            .moves = {
#line 144
                MOVE_ROCK_TOMB,
                MOVE_SHORE_UP,
                MOVE_RECOVER,
                MOVE_FOLLOW_ME,
            },
            },
        },
    },
#line 150
    [PARTNER_SISTER_ONITAIZI] =
    {
#line 151
        .trainerName = _("LEAF"),
#line 152
        .trainerPic = TRAINER_BACK_PIC_LEAF,
        .encounterMusic_gender = 
#line 154
            TRAINER_ENCOUNTER_MUSIC_FEMALE,
#line 155
        .doubleBattle = FALSE,
#line 156
        .aiFlags = AI_FLAG_BASIC_TRAINER | AI_FLAG_ACE_POKEMON,
        .partySize = 3,
        .party = (const struct TrainerMon[])
        {
            {
#line 159
            .species = SPECIES_NINETALES,
            .gender = TRAINER_MON_RANDOM_GENDER,
#line 159
            .heldItem = ITEM_FOCUS_SASH,
#line 163
            .ev = TRAINER_PARTY_EVS(0, 0, 0, 252, 252, 0),
#line 162
            .iv = TRAINER_PARTY_IVS(31, 31, 31, 31, 31, 31),
#line 161
            .ability = ABILITY_MAGIC_GUARD,
#line 160
            .lvl = 43,
#line 164
            .nature = NATURE_TIMID,
            .dynamaxLevel = MAX_DYNAMAX_LEVEL,
            .moves = {
#line 165
                MOVE_MYSTICAL_FIRE,
                MOVE_MISTY_TERRAIN,
                MOVE_DISABLE,
                MOVE_PROTECT,
            },
            },
            {
#line 170
            .species = SPECIES_MIENSHAO,
            .gender = TRAINER_MON_RANDOM_GENDER,
#line 170
            .heldItem = ITEM_EJECT_BUTTON,
#line 174
            .ev = TRAINER_PARTY_EVS(252, 252, 0, 0, 0, 0),
#line 173
            .iv = TRAINER_PARTY_IVS(31, 31, 31, 31, 31, 31),
#line 172
            .ability = ABILITY_TECHNICIAN,
#line 171
            .lvl = 43,
#line 175
            .nature = NATURE_ADAMANT,
            .dynamaxLevel = MAX_DYNAMAX_LEVEL,
            .moves = {
#line 176
                MOVE_DOUBLE_HIT,
                MOVE_PROTECT,
                MOVE_ENDURE,
                MOVE_FAKE_OUT,
            },
            },
            {
#line 181
            .species = SPECIES_SINISTCHA_UNREMARKABLE,
            .gender = TRAINER_MON_RANDOM_GENDER,
#line 181
            .heldItem = ITEM_LUM_BERRY,
#line 185
            .ev = TRAINER_PARTY_EVS(252, 0, 0, 0, 0, 252),
#line 184
            .iv = TRAINER_PARTY_IVS(31, 31, 31, 31, 31, 31),
#line 183
            .ability = ABILITY_WATER_BUBBLE,
#line 182
            .lvl = 43,
#line 186
            .nature = NATURE_CALM,
            .dynamaxLevel = MAX_DYNAMAX_LEVEL,
            .moves = {
#line 187
                MOVE_RAGE_POWDER,
                MOVE_LEECH_SEED,
                MOVE_STRENGTH_SAP,
                MOVE_LIFE_DEW,
            },
            },
        },
    },
