#ifndef GUARD_SCRIPT_POKEMON_UTIL_H
#define GUARD_SCRIPT_POKEMON_UTIL_H

u32 ScriptGiveMon(u16, u8, u16);
u8 ScriptGiveEgg(u16);
void CreateScriptedWildMon(u16, u8, u16);
void CreateScriptedDoubleWildMon(u16, u8, u16, u16, u8, u16);
void ScriptSetMonMoveSlot(u8, u16, u8);
void ReducePlayerPartyToSelectedMons(void);
void HealPlayerParty(void);
void Script_GetChosenMonOffensiveEVs(void);
void Script_GetChosenMonDefensiveEVs(void);
void Script_GetChosenMonOffensiveIVs(void);
void Script_GetChosenMonDefensiveIVs(void);

void Script_SetNature(void);
void Script_SetTeraType(void);
void Script_ToggleShiny(void);
void Script_ToggleGender(void);
void Script_SetPokeball(void);
void Script_GetChosenMonLevel(void);
void Script_RemoveChosenMon(void);

#endif // GUARD_SCRIPT_POKEMON_UTIL_H
