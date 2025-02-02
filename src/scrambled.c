#include "scrambled.h"
#include "caps.h"
#include "constants/species.h"
#include "daycare.h"
#include "pokedex.h"
#include "random.h"

const struct EggPool gEggPools[] = {
    [SCRAMBLED_EGG_STARTER] = {
        .name = _("Starter Egg"),
        .description = COMPOUND_STRING("Contains a Grass, Fire, or Water-type starter Pokémon."),
        .species = (const u16[]) {
            SPECIES_QUAXLY,
            SPECIES_SNIVY,
            SPECIES_CYNDAQUIL,
            SPECIES_TURTWIG,
            SPECIES_CHESPIN,
            SPECIES_SOBBLE,
            SPECIES_CHIMCHAR,
            SPECIES_CHARMANDER,
            SPECIES_TREECKO,
            SPECIES_FROAKIE,
            SPECIES_TEPIG,
            SPECIES_OSHAWOTT,
            SPECIES_MUDKIP,
            SPECIES_BULBASAUR,
            SPECIES_TOTODILE,
            SPECIES_POPPLIO,
            SPECIES_FENNEKIN,
            SPECIES_PIPLUP,
            SPECIES_FUECOCO,
            SPECIES_ROWLET,
            SPECIES_SPRIGATITO,
            SPECIES_SCORBUNNY,
            SPECIES_GROOKEY,
            SPECIES_SQUIRTLE,
            SPECIES_LITTEN,
            SPECIES_TORCHIC,
            SPECIES_CHIKORITA,
            SPECIES_NONE,
        },
    },
    [SCRAMBLED_EGG_NORMAL] = {
        .name = _("Everyday Egg"),
        .description = COMPOUND_STRING("Holds a Normal-type Pokémon."),
        .species = (const u16[]) {
            SPECIES_SMEARGLE,
            SPECIES_YUNGOOS,
            SPECIES_EEVEE,
            SPECIES_MEOWTH_ALOLA,
            SPECIES_MEOWTH,
            SPECIES_TEDDIURSA,
            SPECIES_MINCCINO,
            SPECIES_ROCKRUFF,
            SPECIES_MEOWTH_GALAR,
            SPECIES_LECHONK,
            SPECIES_AIPOM,
            SPECIES_SENTRET,
            SPECIES_MUNCHLAX,
            SPECIES_SKWOVET,
            SPECIES_NONE,
        },
    },
    [SCRAMBLED_EGG_BIRD] = {
        .name = _("Skybound Egg"),
        .description = COMPOUND_STRING("Holds a Flying-type Pokémon."),
        .species = (const u16[]) {
            SPECIES_ROOKIDEE,
            SPECIES_VULLABY,
            SPECIES_SWABLU,
            SPECIES_PIKIPEK,
            SPECIES_DUCKLETT,
            SPECIES_DODUO,
            SPECIES_RUFFLET,
            SPECIES_FLETCHLING,
            SPECIES_WINGULL,
            SPECIES_WATTREL,
            SPECIES_STARLY,
            SPECIES_HOOTHOOT,
            SPECIES_NONE,
        },
    },
    [SCRAMBLED_EGG_BUG] = {
        .name = _("Crawler Egg"),
        .description = COMPOUND_STRING("Holds a Bug-type Pokémon."),
        .species = (const u16[]) {
            SPECIES_TAROUNTULA,
            SPECIES_GRUBBIN,
            SPECIES_SPINARAK,
            SPECIES_COMBEE,
            SPECIES_SURSKIT,
            SPECIES_NYMBLE,
            SPECIES_JOLTIK,
            SPECIES_PINECO,
            SPECIES_KRICKETOT,
            SPECIES_VENONAT,
            SPECIES_SCATTERBUG,
            SPECIES_DEWPIDER,
            SPECIES_CUTIEFLY,
            SPECIES_YANMA,
            SPECIES_SEWADDLE,
            SPECIES_NONE,
        },
    },
    [SCRAMBLED_EGG_GRASS] = {
        .name = _("Verdant Egg"),
        .description = COMPOUND_STRING("Holds a Grass-type Pokémon."),
        .species = (const u16[]) {
            SPECIES_LOTAD,
            SPECIES_APPLIN,
            SPECIES_BELLSPROUT,
            SPECIES_FOONGUS,
            SPECIES_FOMANTIS,
            SPECIES_COTTONEE,
            SPECIES_PETILIL,
            SPECIES_HOPPIP,
            SPECIES_SHROOMISH,
            SPECIES_VOLTORB_HISUI,
            SPECIES_BOUNSWEET,
            SPECIES_EXEGGCUTE,
            SPECIES_SUNKERN,
            SPECIES_SEEDOT,
            SPECIES_ODDISH,
            SPECIES_TOEDSCOOL,
            SPECIES_SMOLIV,
            SPECIES_SKIDDO,
            SPECIES_NONE,
        },
    },
    [SCRAMBLED_EGG_CITY] = {
        .name = _("Urban Egg"),
        .description = COMPOUND_STRING("Holds a city-dwelling Pokémon."),
        .species = (const u16[]) {
            SPECIES_TOXEL,
            SPECIES_PAWNIARD,
            SPECIES_KOFFING,
            SPECIES_CROAGUNK,
            SPECIES_MURKROW,
            SPECIES_HOUNDOUR,
            SPECIES_ZORUA,
            SPECIES_POOCHYENA,
            SPECIES_MASCHIFF,
            SPECIES_BELDUM,
            SPECIES_STUNKY,
            SPECIES_SHINX,
            SPECIES_GRIMER,
            SPECIES_MAGNEMITE,
            SPECIES_GRIMER_ALOLA,
            SPECIES_VOLTORB,
            SPECIES_PAWMI,
            SPECIES_PICHU,
            SPECIES_SHROODLE,
            SPECIES_VAROOM,
            SPECIES_TIMBURR,
            SPECIES_SCRAGGY,
            SPECIES_ELEKID,
            SPECIES_GULPIN,
            SPECIES_NONE,
        },
    },
    [SCRAMBLED_EGG_DESERT] = {
        .name = _("Arid Egg"),
        .description = COMPOUND_STRING("Holds a desert-dwelling Pokémon."),
        .species = (const u16[]) {
            SPECIES_BLITZLE,
            SPECIES_LITLEO,
            SPECIES_TRAPINCH,
            SPECIES_CHARCADET,
            SPECIES_VULPIX,
            SPECIES_CAPSAKID,
            SPECIES_EKANS,
            SPECIES_SANDSHREW,
            SPECIES_LARVESTA,
            SPECIES_HIPPOPOTAS,
            SPECIES_BRAMBLIN,
            SPECIES_SLUGMA,
            SPECIES_CACNEA,
            SPECIES_SANDILE,
            SPECIES_GIBLE,
            SPECIES_MAGBY,
            SPECIES_SALANDIT,
            SPECIES_SILICOBRA,
            SPECIES_NONE,
        },
    },
    [SCRAMBLED_EGG_ICY] = {
        .name = _("Frosted Egg"),
        .description = COMPOUND_STRING("Holds an Ice-type Pokémon."),
        .species = (const u16[]) {
            SPECIES_VULPIX_ALOLA,
            SPECIES_SNOVER,
            SPECIES_BERGMITE,
            SPECIES_SNOM,
            SPECIES_SWINUB,
            SPECIES_SEEL,
            SPECIES_CETODDLE,
            SPECIES_FRIGIBAX,
            SPECIES_SNORUNT,
            SPECIES_SANDSHREW_ALOLA,
            SPECIES_GROWLITHE,
            SPECIES_DEERLING_WINTER,
            SPECIES_CUBCHOO,
            SPECIES_SLAKOTH,
            SPECIES_NONE,
        },
    },
    [SCRAMBLED_EGG_CAVE] = {
        .name = _("Rocky Egg"),
        .description = COMPOUND_STRING("Holds a cave-dwelling Pokémon."),
        .species = (const u16[]) {
            SPECIES_BONSLY,
            SPECIES_GOLETT,
            SPECIES_NACLI,
            SPECIES_PHANPY,
            SPECIES_DIGLETT_ALOLA,
            SPECIES_DIGLETT,
            SPECIES_GEODUDE_ALOLA,
            SPECIES_ROLYCOLY,
            SPECIES_GROWLITHE_HISUI,
            SPECIES_DRILBUR,
            SPECIES_CRANIDOS,
            SPECIES_GLIMMET,
            SPECIES_AXEW,
            SPECIES_DEINO,
            SPECIES_JANGMO_O,
            SPECIES_BAGON,
            SPECIES_MEDITITE,
            SPECIES_MIENFOO,
            SPECIES_TYROGUE,
            SPECIES_NOIBAT,
            SPECIES_SHIELDON,
            SPECIES_CUFANT,
            SPECIES_MAKUHITA,
            SPECIES_MANKEY,
            SPECIES_LARVITAR,
            SPECIES_GEODUDE,
            SPECIES_KUBFU,
            SPECIES_MUDBRAY,
            SPECIES_NONE,
        },
    },
    [SCRAMBLED_EGG_WATER] = {
        .name = _("Aquatic Egg"),
        .description = COMPOUND_STRING("Holds an aquatic Pokémon."),
        .species = (const u16[]) {
            SPECIES_TENTACOOL,
            SPECIES_MAREANIE,
            SPECIES_SHELLDER,
            SPECIES_FEEBAS,
            SPECIES_SLOWPOKE,
            SPECIES_BARBOACH,
            SPECIES_LUVDISC,
            SPECIES_CLAUNCHER,
            SPECIES_TYNAMO,
            SPECIES_HORSEA,
            SPECIES_SHELLOS,
            SPECIES_FINNEON,
            SPECIES_TADBULB,
            SPECIES_SKRELP,
            SPECIES_CORPHISH,
            SPECIES_SLOWPOKE,
            SPECIES_BASCULIN_WHITE_STRIPED,
            SPECIES_INKAY,
            SPECIES_WIGLETT,
            SPECIES_GOOMY,
            SPECIES_PSYDUCK,
            SPECIES_WOOPER_PALDEA,
            SPECIES_CHEWTLE,
            SPECIES_BUIZEL,
            SPECIES_WOOPER,
            SPECIES_DRATINI,
            SPECIES_POLIWAG,
            SPECIES_CHINCHOU,
            SPECIES_MAGIKARP,
            SPECIES_ARROKUDA,
            SPECIES_NONE,
        },
    },
    [SCRAMBLED_EGG_FAIRY] = {
        .name = _("Enchanted Egg"),
        .description = COMPOUND_STRING("Holds a Fairy or Psychic-type Pokémon."),
        .species = (const u16[]) {
            SPECIES_CLEFFA,
            SPECIES_SPOINK,
            SPECIES_RALTS,
            SPECIES_HATENNA,
            SPECIES_SNUBBULL,
            SPECIES_FLITTLE,
            SPECIES_GOTHITA,
            SPECIES_FLABEBE,
            SPECIES_CHINGLING,
            SPECIES_HAPPINY,
            SPECIES_COSMOG,
            SPECIES_IGGLYBUFF,
            SPECIES_TINKATINK,
            SPECIES_SOLOSIS,
            SPECIES_IMPIDIMP,
            SPECIES_FIDOUGH,
            SPECIES_ESPURR,
            SPECIES_DROWZEE,
            SPECIES_MAREEP,
            SPECIES_AZURILL,
            SPECIES_MILCERY,
            SPECIES_NONE,
        },
    },
    [SCRAMBLED_EGG_GHOST] = {
        .name = _("Phantom Egg"),
        .description = COMPOUND_STRING("Holds a Ghost-type Pokémon."),
        .species = (const u16[]) {
            SPECIES_DUSKULL,
            SPECIES_POLTCHAGEIST,
            SPECIES_SINISTEA,
            SPECIES_SHUPPET,
            SPECIES_SANDYGAST,
            SPECIES_GIMMIGHOUL_ROAMING,
            SPECIES_GREAVARD,
            SPECIES_ZORUA_HISUI,
            SPECIES_DRIFLOON,
            SPECIES_LITWICK,
            SPECIES_DREEPY,
            SPECIES_PHANTUMP,
            SPECIES_GASTLY,
            SPECIES_NONE,
        },
    },
    [SCRAMBLED_EGG_LV20] = {
        .name = _("Journey Egg"),
        .description = COMPOUND_STRING("Holds Pokémon that start a bit strong. Minimum Lv20"),
        .species = (const u16[]) {
            SPECIES_SNEASEL_HISUI,
            SPECIES_GLIGAR,
            SPECIES_FINIZEN,
            SPECIES_BRONZOR,
            SPECIES_ILLUMISE,
            SPECIES_COMFEY,
            SPECIES_SNEASEL,
            SPECIES_DUNSPARCE,
            SPECIES_TATSUGIRI,
            SPECIES_KOMALA,
            SPECIES_KLEFKI,
            SPECIES_RELLOR,
            SPECIES_PHIONE,
            SPECIES_DELIBIRD,
            SPECIES_GIMMIGHOUL,
            SPECIES_PORYGON,
            SPECIES_QWILFISH,
            SPECIES_QWILFISH_HISUI,
            SPECIES_MISDREAVUS,
            SPECIES_CRAMORANT,
            SPECIES_DITTO,
            SPECIES_SABLEYE,
            SPECIES_VOLBEAT,
            SPECIES_NOSEPASS,
            SPECIES_EISCUE,
            SPECIES_SQUAWKABILLY,
            SPECIES_MORPEKO,
            SPECIES_HAWLUCHA,
            SPECIES_CRABRAWLER,
            SPECIES_CARBINK,
            SPECIES_PACHIRISU,
            SPECIES_GIRAFARIG,
            SPECIES_DEDENNE,
            SPECIES_STANTLER,
            SPECIES_PLUSLE,
            SPECIES_RHYHORN,
            SPECIES_MINUN,
            SPECIES_BOMBIRDIER,
            SPECIES_FLAMIGO,
            SPECIES_ORICORIO_POM_POM,
            SPECIES_DURALUDON,
            SPECIES_NONE,
        },
    },
    [SCRAMBLED_EGG_LV30] = {
        .name = _("Explorer Egg"),
        .description = COMPOUND_STRING("Holds Pokémon that start quite strong. Minimum Lv30"),
        .species = (const u16[]) {
            SPECIES_FALINKS,
            SPECIES_TAUROS_PALDEA_COMBAT,
            SPECIES_SKARMORY,
            SPECIES_MINIOR_METEOR,
            SPECIES_SPIRITOMB,
            SPECIES_SEVIPER,
            SPECIES_ORTHWORM,
            SPECIES_ZANGOOSE,
            SPECIES_TAUROS,
            SPECIES_ORANGURU,
            SPECIES_TORKOAL,
            SPECIES_CYCLIZAR,
            SPECIES_BASCULIN,
            SPECIES_HERACROSS,
            SPECIES_PINCURCHIN,
            SPECIES_INDEEDEE,
            SPECIES_BASCULIN_BLUE_STRIPED,
            SPECIES_PASSIMIAN,
            SPECIES_KLAWF,
            SPECIES_SCYTHER,
            SPECIES_NONE,
        },
    },
    [SCRAMBLED_EGG_LV35] = {
        .name = _("Apex Egg"),
        .description = COMPOUND_STRING("Holds Pokémon that are immediately powerful. Minimum Lv35"),
        .species = (const u16[]) {
            SPECIES_KELDEO,
            SPECIES_STONJOURNER,
            SPECIES_FEZANDIPITI,
            SPECIES_LAPRAS,
            SPECIES_TERAPAGOS,
            SPECIES_PECHARUNT,
            SPECIES_MUNKIDORI,
            SPECIES_SHAYMIN,
            SPECIES_OGERPON,
            SPECIES_TROPIUS,
            SPECIES_HEATRAN,
            SPECIES_ROTOM,
            SPECIES_DONDOZO,
            SPECIES_OKIDOGI,
            SPECIES_NONE,
        },
    },
    [SCRAMBLED_EGG_LV40] = {
        .name = _("Mystic Egg"),
        .description = COMPOUND_STRING("Holds Legendary Pokémon. Minimum Lv40"),
        .species = (const u16[]) {
            SPECIES_SPECTRIER,
            SPECIES_GLASTRIER,
            SPECIES_MELOETTA,
            SPECIES_UXIE,
            SPECIES_AZELF,
            SPECIES_LATIOS,
            SPECIES_ZARUDE,
            SPECIES_JIRACHI,
            SPECIES_MESPRIT,
            SPECIES_LATIAS,
            SPECIES_NONE,
        },
    },
    [SCRAMBLED_EGG_LV45] = {
        .name = _("Legendary Egg"),
        .description = COMPOUND_STRING("Holds stronger Legendary Pokémon. Minimum Lv45"),
        .species = (const u16[]) {
            SPECIES_RAIKOU,
            SPECIES_WALKING_WAKE,
            SPECIES_ENTEI,
            SPECIES_ZAPDOS_GALAR,
            SPECIES_MOLTRES,
            SPECIES_ARTICUNO,
            SPECIES_SUICUNE,
            SPECIES_IRON_BOULDER,
            SPECIES_ARTICUNO_GALAR,
            SPECIES_VIRIZION,
            SPECIES_IRON_LEAVES,
            SPECIES_ZAPDOS,
            SPECIES_TERRAKION,
            SPECIES_GOUGING_FIRE,
            SPECIES_COBALION,
            SPECIES_IRON_CROWN,
            SPECIES_RAGING_BOLT,
            SPECIES_MOLTRES_GALAR,
            SPECIES_HOOPA,
            SPECIES_TING_LU,
            SPECIES_REGIROCK,
            SPECIES_TORNADUS,
            SPECIES_THUNDURUS,
            SPECIES_REGIDRAGO,
            SPECIES_REGISTEEL,
            SPECIES_REGIELEKI,
            SPECIES_REGICE,
            SPECIES_VOLCANION,
            SPECIES_CRESSELIA,
            SPECIES_LANDORUS,
            SPECIES_ENAMORUS,
            SPECIES_CHI_YU,
            SPECIES_WO_CHIEN,
            SPECIES_CHIEN_PAO,
            SPECIES_DARKRAI,
            SPECIES_NONE,
        },
    },
    [SCRAMBLED_EGG_LV50] = {
        .name = _("Titan Egg"),
        .description = COMPOUND_STRING("Holds the most powerful Legendary Pokémon. Minimum LV50"),
        .species = (const u16[]) {
            SPECIES_ETERNATUS,
            SPECIES_MEWTWO,
            SPECIES_DEOXYS,
            SPECIES_GIRATINA,
            SPECIES_REGIGIGAS,
            SPECIES_NECROZMA,
            SPECIES_KYUREM,
            SPECIES_NONE,
        },
    },
    [SCRAMBLED_EGG_RANDOM] = {
        .name = _("Mystery Egg"),
        .description = COMPOUND_STRING("Get a random Pokémon from all the available options."),
        .species = NULL
    }
};

u8 MaxEggPool()
{
    if (GetCurrentLevelCap() >= 50)
        return SCRAMBLED_EGG_LV50 + 1;
    if (GetCurrentLevelCap() >= 45)
        return SCRAMBLED_EGG_LV45 + 1;
    if (GetCurrentLevelCap() >= 40)
        return SCRAMBLED_EGG_LV40 + 1;
    if (GetCurrentLevelCap() >= 35)
        return SCRAMBLED_EGG_LV35 + 1;
    if (GetCurrentLevelCap() >= 30)
        return SCRAMBLED_EGG_LV30 + 1;
    if (GetCurrentLevelCap() >= 20)
        return SCRAMBLED_EGG_LV20 + 1;
    return SCRAMBLED_EGG_LV20;
}

static bool8 GotSpecies(u16 speciesId)
{
    u16 dexNo = SpeciesToNationalPokedexNum(speciesId);
    return GetSetPokedexFlag(dexNo, FLAG_GET_CAUGHT);
}

const u16 EGG_COUNT = 400;
static u16 GetRandomSpecies(u8 poolId)
{
    u16 choseableArray[EGG_COUNT];

    u16 added = 0;
    u16 pool = SCRAMBLED_EGG_STARTER;
    u16 specieIndex = 0;
    if (poolId != SCRAMBLED_EGG_RANDOM)
        pool = poolId;
    while (TRUE)
    {
        u16 species = gEggPools[pool].species[specieIndex];
        if (!GotSpecies(species))
        {
            DebugPrintfLevel(2, "ADDED %d", species);
            choseableArray[added] = species;
            added += 1;
        }
        specieIndex += 1;
        if (gEggPools[pool].species[specieIndex] == SPECIES_NONE)
        {
            if (poolId != SCRAMBLED_EGG_RANDOM)
                break;
            specieIndex = 0;
            pool += 1;
        }
        if (pool == MaxEggPool())
            break;
    }

    if (added == 0)
        return SPECIES_NONE;
    return choseableArray[Random() % added];
}

static const u16 sMiniorFormes[] = {
    SPECIES_MINIOR_METEOR_RED,
    SPECIES_MINIOR_ORANGE,
    SPECIES_MINIOR_YELLOW,
    SPECIES_MINIOR_GREEN,
    SPECIES_MINIOR_BLUE,
    SPECIES_MINIOR_INDIGO,
    SPECIES_MINIOR_VIOLET,
    SPECIES_NONE,
};

static const u16 sTaurosFormes[] = {
    SPECIES_TAUROS_PALDEA_COMBAT,
    SPECIES_TAUROS_PALDEA_BLAZE,
    SPECIES_TAUROS_PALDEA_AQUA,
    SPECIES_NONE,
};

static const struct
{
    u16 species;
    const u16 *formeList;
} sSpeciesFormes[] = {
    { .species = SPECIES_MINIOR, .formeList = sMiniorFormes },
    { .species = SPECIES_TAUROS_PALDEA_COMBAT, .formeList = sTaurosFormes },
};

void RollMonTeraType(struct Pokemon *mon, u16 species)
{
    while (TRUE)
    {
        u8 type = TYPE_NORMAL + Random() % 19;
        const u8 *speciesTypes = gSpeciesInfo[species].types;
        if (speciesTypes[0] != type && speciesTypes[1] != type && type != TYPE_MYSTERY)
        {
            SetMonData(mon, MON_DATA_TERA_TYPE, &type);
            break;
        }
    }
}

u8 GiveSpecies(u16 species)
{
    struct Pokemon mon;

    for (int i = 0; i < ARRAY_COUNT(sSpeciesFormes); ++i)
    {
        if (sSpeciesFormes[i].species != species)
            continue;
        u8 count = 0;
        while (sSpeciesFormes[i].formeList[count] != SPECIES_NONE)
            count += 1;

        u8 forme = Random() % count;
        species = sSpeciesFormes[i].formeList[forme];
        break;
    }

    CreateEgg(&mon, species, FALSE);
    u8 eggCycles = 0;
    SetMonData(&mon, MON_DATA_FRIENDSHIP, &eggCycles);
    u8 iv = 20;
    for (u8 i = 0; i < NUM_STATS; ++i)
        SetMonData(&mon, MON_DATA_HP_IV + i, &iv);
    RollMonTeraType(&mon, species);
    u16 dexNo = SpeciesToNationalPokedexNum(species);
    GetSetPokedexFlag(dexNo, FLAG_SET_CAUGHT);
    return GiveMonToPlayer(&mon);
}

u8 GiveScrambledEgg(u8 poolId)
{
    u16 species = GetRandomSpecies(poolId);
    if (species == SPECIES_NONE)
        return MON_CANT_GIVE;
    return GiveSpecies(species);
}
