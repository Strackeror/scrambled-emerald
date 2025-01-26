#ifndef SCRAMBLED_H
#define SCRAMBLED_H

#include "global.h"

enum EggType
{
    SCRAMBLED_EGG_NONE,
    SCRAMBLED_EGG_STARTER,
    SCRAMBLED_EGG_NORMAL,
    SCRAMBLED_EGG_BIRD,
    SCRAMBLED_EGG_BUG,
    SCRAMBLED_EGG_GRASS,
    SCRAMBLED_EGG_CITY,
    SCRAMBLED_EGG_DESERT,
    SCRAMBLED_EGG_ICY,
    SCRAMBLED_EGG_CAVE,
    SCRAMBLED_EGG_WATER,
    SCRAMBLED_EGG_FAIRY,
    SCRAMBLED_EGG_GHOST,
    SCRAMBLED_EGG_LV20,
    SCRAMBLED_EGG_LV30,
    SCRAMBLED_EGG_LV35,
    SCRAMBLED_EGG_LV40,
    SCRAMBLED_EGG_LV45,
    SCRAMBLED_EGG_LV50,
    SCRAMBLED_EGG_RANDOM,
};

struct EggPool
{
    const u16 *species;
    const u8 name[16];
    const u8* description;
};

extern const struct EggPool gEggPools[];

u8 MaxEggPool();
u8 GiveScrambledEgg(u8 poolId);
u8 GiveSpecies(u16 species);

#endif // SCRAMBLED_H
