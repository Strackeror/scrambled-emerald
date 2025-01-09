Common_EventScript_SetupRivalGfxId::
	checkplayergender
	setvar VAR_OBJ_GFX_ID_0, OBJ_EVENT_GFX_RIVAL_MAY_NORMAL
	return

Common_EventScript_SetupRivalOnBikeGfxId::
	checkplayergender
	setvar VAR_OBJ_GFX_ID_3, OBJ_EVENT_GFX_RIVAL_MAY_MACH_BIKE
	return

@ Unused
Common_EventScript_SetupRivalGfxIdSameGender::
	checkplayergender
	goto_if_eq VAR_RESULT, MALE, EventScript_SetupRivalGfxIdMale2
	goto_if_eq VAR_RESULT, FEMALE, EventScript_SetupRivalGfxIdFemale2
	end

EventScript_SetupRivalGfxIdMale2::
	setvar VAR_OBJ_GFX_ID_0, OBJ_EVENT_GFX_RIVAL_BRENDAN_NORMAL
	return

EventScript_SetupRivalGfxIdFemale2::
	setvar VAR_OBJ_GFX_ID_0, OBJ_EVENT_GFX_RIVAL_MAY_NORMAL
	return
