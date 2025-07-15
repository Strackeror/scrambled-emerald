//
// DO NOT MODIFY THIS FILE! It is auto-generated from src/data/battle_partners.party
//
// If you want to modify this file set COMPETITIVE_PARTY_SYNTAX to FALSE
// in include/config/general.h and remove this notice.
// Use sed -i '/^#line/d' 'src/data/battle_partners.h' to remove #line markers.
//

#line 1 "src/data/battle_partners.party"

#line 1
    [DIFFICULTY_NORMAL][PARTNER_NONE] =
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
    [DIFFICULTY_NORMAL][PARTNER_STEVEN] =
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
    [DIFFICULTY_NORMAL][PARTNER_PEPPER_NUSI_01] =
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
            .species = SPECIES_SHELLDER,
            .gender = TRAINER_MON_RANDOM_GENDER,
#line 54
            .heldItem = ITEM_LEFTOVERS,
#line 58
            .ev = TRAINER_PARTY_EVS(252, 0, 252, 0, 0, 0),
#line 57
            .iv = TRAINER_PARTY_IVS(31, 31, 31, 31, 31, 31),
#line 56
            .ability = ABILITY_SHELL_ARMOR,
#line 55
            .lvl = 15,
#line 59
            .nature = NATURE_BOLD,
            .dynamaxLevel = MAX_DYNAMAX_LEVEL,
            .moves = {
#line 60
                MOVE_AQUA_JET,
                MOVE_AQUA_RING,
                MOVE_FOLLOW_ME,
                MOVE_SPIKY_SHIELD,
            },
            },
        },
    },
#line 66
    [DIFFICULTY_NORMAL][PARTNER_PEPPER_NUSI_02] =
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
            .species = SPECIES_TOEDSCOOL,
            .gender = TRAINER_MON_RANDOM_GENDER,
#line 75
            .heldItem = ITEM_FOCUS_SASH,
#line 79
            .ev = TRAINER_PARTY_EVS(252, 0, 252, 0, 0, 0),
#line 78
            .iv = TRAINER_PARTY_IVS(31, 31, 31, 31, 31, 31),
#line 77
            .ability = ABILITY_POISON_HEAL,
#line 76
            .lvl = 33,
#line 80
            .nature = NATURE_BOLD,
            .dynamaxLevel = MAX_DYNAMAX_LEVEL,
            .moves = {
#line 81
                MOVE_MUD_SHOT,
                MOVE_NONE,
                MOVE_NONE,
                MOVE_NONE,
            },
            },
        },
    },
#line 87
    [DIFFICULTY_NORMAL][PARTNER_PEPPER_NUSI_03] =
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
            .species = SPECIES_NACLI,
            .gender = TRAINER_MON_RANDOM_GENDER,
#line 96
            .heldItem = ITEM_ROCKY_HELMET,
#line 100
            .ev = TRAINER_PARTY_EVS(252, 0, 252, 0, 0, 0),
#line 99
            .iv = TRAINER_PARTY_IVS(31, 31, 31, 31, 31, 31),
#line 98
            .ability = ABILITY_ROUGH_SKIN,
#line 97
            .lvl = 22,
#line 101
            .nature = NATURE_IMPISH,
            .dynamaxLevel = MAX_DYNAMAX_LEVEL,
            .moves = {
#line 102
                MOVE_PROTECT,
                MOVE_SUBSTITUTE,
                MOVE_FOLLOW_ME,
                MOVE_SHORE_UP,
            },
            },
        },
    },
#line 108
    [DIFFICULTY_NORMAL][PARTNER_PEPPER_NUSI_04] =
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
            .species = SPECIES_SCOVILLAIN,
            .gender = TRAINER_MON_RANDOM_GENDER,
#line 117
            .heldItem = ITEM_TOXIC_ORB,
#line 121
            .ev = TRAINER_PARTY_EVS(0, 252, 0, 252, 0, 0),
#line 120
            .iv = TRAINER_PARTY_IVS(31, 31, 31, 31, 31, 31),
#line 119
            .ability = ABILITY_CHLOROPHYLL,
#line 118
            .lvl = 47,
#line 122
            .nature = NATURE_ADAMANT,
            .dynamaxLevel = MAX_DYNAMAX_LEVEL,
            .moves = {
#line 123
                MOVE_THUNDER_FANG,
                MOVE_THUNDER_FANG,
                MOVE_THUNDER_FANG,
                MOVE_THUNDER_FANG,
            },
            },
        },
    },
#line 129
    [DIFFICULTY_NORMAL][PARTNER_PEPPER_NUSI_05] =
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
            .species = SPECIES_GREEDENT,
            .gender = TRAINER_MON_RANDOM_GENDER,
#line 138
            .heldItem = ITEM_SITRUS_BERRY,
#line 142
            .ev = TRAINER_PARTY_EVS(252, 0, 0, 0, 0, 252),
#line 141
            .iv = TRAINER_PARTY_IVS(31, 31, 31, 31, 31, 31),
#line 140
            .ability = ABILITY_UNAWARE,
#line 139
            .lvl = 58,
#line 143
            .nature = NATURE_CAREFUL,
            .dynamaxLevel = MAX_DYNAMAX_LEVEL,
            .moves = {
#line 144
                MOVE_PROTECT,
                MOVE_SLACK_OFF,
                MOVE_STUFF_CHEEKS,
                MOVE_FOLLOW_ME,
            },
            },
        },
    },
#line 150
    [DIFFICULTY_NORMAL][PARTNER_DONDOGIRI_REVERSE] =
    {
#line 151
        .trainerName = _("ARVEN"),
#line 152
        .trainerPic = TRAINER_BACK_PIC_STEVEN,
        .encounterMusic_gender =
#line 154
            TRAINER_ENCOUNTER_MUSIC_MALE,
#line 155
        .doubleBattle = FALSE,
#line 156
        .aiFlags = AI_FLAG_BASIC_TRAINER | AI_FLAG_ACE_POKEMON,
        .partySize = 1,
        .party = (const struct TrainerMon[])
        {
            {
#line 158
            .species = SPECIES_GREEDENT,
            .gender = TRAINER_MON_RANDOM_GENDER,
#line 158
            .heldItem = ITEM_SITRUS_BERRY,
#line 162
            .ev = TRAINER_PARTY_EVS(252, 0, 0, 0, 0, 252),
#line 161
            .iv = TRAINER_PARTY_IVS(31, 31, 31, 31, 31, 31),
#line 160
            .ability = ABILITY_RIPEN,
#line 159
            .lvl = 15,
#line 163
            .nature = NATURE_CAREFUL,
            .dynamaxLevel = MAX_DYNAMAX_LEVEL,
            .moves = {
#line 164
                MOVE_PROTECT,
                MOVE_NONE,
                MOVE_NONE,
                MOVE_NONE,
            },
            },
        },
    },
#line 170
    [DIFFICULTY_NORMAL][PARTNER_GREAT_TREADS_REVERSE] =
    {
#line 171
        .trainerName = _("ARVEN"),
#line 172
        .trainerPic = TRAINER_BACK_PIC_STEVEN,
        .encounterMusic_gender =
#line 174
            TRAINER_ENCOUNTER_MUSIC_MALE,
#line 175
        .doubleBattle = FALSE,
#line 176
        .aiFlags = AI_FLAG_BASIC_TRAINER | AI_FLAG_ACE_POKEMON,
        .partySize = 1,
        .party = (const struct TrainerMon[])
        {
            {
#line 178
            .species = SPECIES_SCOVILLAIN,
            .gender = TRAINER_MON_RANDOM_GENDER,
#line 178
            .heldItem = ITEM_TOXIC_ORB,
#line 182
            .ev = TRAINER_PARTY_EVS(0, 252, 0, 252, 0, 0),
#line 181
            .iv = TRAINER_PARTY_IVS(31, 31, 31, 31, 31, 31),
#line 180
            .ability = ABILITY_CHLOROPHYLL,
#line 179
            .lvl = 24,
#line 183
            .nature = NATURE_ADAMANT,
            .dynamaxLevel = MAX_DYNAMAX_LEVEL,
            .moves = {
#line 184
                MOVE_THUNDER_FANG,
                MOVE_NONE,
                MOVE_NONE,
                MOVE_NONE,
            },
            },
        },
    },
#line 190
    [DIFFICULTY_NORMAL][PARTNER_ORTHWORM_REVERSE] =
    {
#line 191
        .trainerName = _("ARVEN"),
#line 192
        .trainerPic = TRAINER_BACK_PIC_STEVEN,
        .encounterMusic_gender =
#line 194
            TRAINER_ENCOUNTER_MUSIC_MALE,
#line 195
        .doubleBattle = FALSE,
#line 196
        .aiFlags = AI_FLAG_BASIC_TRAINER | AI_FLAG_ACE_POKEMON,
        .partySize = 1,
        .party = (const struct TrainerMon[])
        {
            {
#line 198
            .species = SPECIES_TOEDSCOOL,
            .gender = TRAINER_MON_RANDOM_GENDER,
#line 198
            .heldItem = ITEM_FOCUS_SASH,
#line 202
            .ev = TRAINER_PARTY_EVS(252, 0, 252, 0, 0, 0),
#line 201
            .iv = TRAINER_PARTY_IVS(31, 31, 31, 31, 31, 31),
#line 200
            .ability = ABILITY_POISON_HEAL,
#line 199
            .lvl = 38,
#line 203
            .nature = NATURE_BOLD,
            .dynamaxLevel = MAX_DYNAMAX_LEVEL,
            .moves = {
#line 204
                MOVE_MUD_SHOT,
                MOVE_NONE,
                MOVE_NONE,
                MOVE_NONE,
            },
            },
        },
    },
#line 210
    [DIFFICULTY_NORMAL][PARTNER_BOMBIRDIER_REVERSE] =
    {
#line 211
        .trainerName = _("ARVEN"),
#line 212
        .trainerPic = TRAINER_BACK_PIC_STEVEN,
        .encounterMusic_gender =
#line 214
            TRAINER_ENCOUNTER_MUSIC_MALE,
#line 215
        .doubleBattle = FALSE,
#line 216
        .aiFlags = AI_FLAG_BASIC_TRAINER | AI_FLAG_ACE_POKEMON,
        .partySize = 1,
        .party = (const struct TrainerMon[])
        {
            {
#line 218
            .species = SPECIES_NACLI,
            .gender = TRAINER_MON_RANDOM_GENDER,
#line 218
            .heldItem = ITEM_FOCUS_SASH,
#line 222
            .ev = TRAINER_PARTY_EVS(252, 0, 252, 0, 0, 0),
#line 221
            .iv = TRAINER_PARTY_IVS(31, 31, 31, 31, 31, 31),
#line 220
            .ability = ABILITY_WATER_ABSORB,
#line 219
            .lvl = 54,
#line 223
            .nature = NATURE_IMPISH,
            .dynamaxLevel = MAX_DYNAMAX_LEVEL,
            .moves = {
#line 224
                MOVE_SWAGGER,
                MOVE_NONE,
                MOVE_NONE,
                MOVE_NONE,
            },
            },
        },
    },
#line 230
    [DIFFICULTY_NORMAL][PARTNER_KLAWF_REVERSE] =
    {
#line 231
        .trainerName = _("ARVEN"),
#line 232
        .trainerPic = TRAINER_BACK_PIC_STEVEN,
        .encounterMusic_gender =
#line 234
            TRAINER_ENCOUNTER_MUSIC_MALE,
#line 235
        .doubleBattle = FALSE,
#line 236
        .aiFlags = AI_FLAG_BASIC_TRAINER | AI_FLAG_ACE_POKEMON,
        .partySize = 1,
        .party = (const struct TrainerMon[])
        {
            {
#line 238
            .species = SPECIES_SHELLDER,
            .gender = TRAINER_MON_RANDOM_GENDER,
#line 238
            .heldItem = ITEM_LEFTOVERS,
#line 242
            .ev = TRAINER_PARTY_EVS(252, 0, 252, 0, 0, 0),
#line 241
            .iv = TRAINER_PARTY_IVS(31, 31, 31, 31, 31, 31),
#line 240
            .ability = ABILITY_SHELL_ARMOR,
#line 239
            .lvl = 58,
#line 243
            .nature = NATURE_IMPISH,
            .dynamaxLevel = MAX_DYNAMAX_LEVEL,
            .moves = {
#line 244
                MOVE_SUBSTITUTE,
                MOVE_AQUA_RING,
                MOVE_NONE,
                MOVE_NONE,
            },
            },
        },
    },
#line 250
    [DIFFICULTY_NORMAL][PARTNER_SISTER_ONITAIZI] =
    {
#line 251
        .trainerName = _("LEAF"),
#line 252
        .trainerPic = TRAINER_BACK_PIC_LEAF,
        .encounterMusic_gender =
#line 254
            TRAINER_ENCOUNTER_MUSIC_FEMALE,
#line 255
        .doubleBattle = FALSE,
#line 256
        .aiFlags = AI_FLAG_BASIC_TRAINER | AI_FLAG_ACE_POKEMON,
        .partySize = 1,
        .party = (const struct TrainerMon[])
        {
            {
#line 259
            .species = SPECIES_NINETALES,
            .gender = TRAINER_MON_RANDOM_GENDER,
#line 259
            .heldItem = ITEM_FOCUS_SASH,
#line 263
            .ev = TRAINER_PARTY_EVS(252, 0, 0, 252, 0, 0),
#line 262
            .iv = TRAINER_PARTY_IVS(31, 31, 31, 31, 31, 31),
#line 261
            .ability = ABILITY_MAGIC_GUARD,
#line 260
            .lvl = 43,
#line 264
            .nature = NATURE_TIMID,
            .dynamaxLevel = MAX_DYNAMAX_LEVEL,
            .moves = {
#line 265
                MOVE_WISH,
                MOVE_MISTY_TERRAIN,
                MOVE_ENDURE,
                MOVE_PROTECT,
            },
            },
        },
    },
