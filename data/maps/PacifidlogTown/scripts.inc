.set LOCALID_KIERAN4, 4
.set LOCALID_CARMINE3, 5

PacifidlogTown_MapScripts::
	map_script MAP_SCRIPT_ON_TRANSITION, PacifidlogTown_OnTransition
	map_script MAP_SCRIPT_ON_RESUME, PacifidlogTown_OnResume
	.byte 0

PacifidlogTown_OnTransition:
	setflag FLAG_VISITED_PACIFIDLOG_TOWN
	call_if_set FLAG_BADGE04_GET, PacifidlogTown_EventScript_AddKieran4
	call_if_set FLAG_BADGE06_GET, PacifidlogTown_EventScript_RemoveKieran4AddCarmine3
	end

PacifidlogTown_EventScript_AddKieran4::
	setobjectxyperm LOCALID_KIERAN4, 1, 14
	setobjectmovementtype LOCALID_KIERAN4, MOVEMENT_TYPE_LOOK_AROUND
	return

PacifidlogTown_EventScript_RemoveKieran4AddCarmine3::
	setobjectxyperm LOCALID_KIERAN4, 5, -1
	setobjectxyperm LOCALID_CARMINE3, 2, 14
	setobjectmovementtype LOCALID_CARMINE3, MOVEMENT_TYPE_LOOK_AROUND
	return

PacifidlogTown_OnResume:
	setstepcallback STEP_CB_PACIFIDLOG_BRIDGE
	end

PacifidlogTown_EventScript_NinjaBoy::
	msgbox PacifidlogTown_Text_NeatHousesOnWater, MSGBOX_NPC
	end

PacifidlogTown_EventScript_Girl::
	msgbox PacifidlogTown_Text_FastRunningCurrent, MSGBOX_NPC
	end

PacifidlogTown_EventScript_Fisherman::
	msgbox PacifidlogTown_Text_SkyPillarTooScary, MSGBOX_NPC
	end

PacifidlogTown_EventScript_TownSign::
	msgbox PacifidlogTown_Text_TownSign, MSGBOX_SIGN
	end

PacifidlogTown_EventScript_Kieran4::
	setvar VAR_TEMP_F, 40
	buffernumberstring STR_VAR_2, VAR_TEMP_F
	goto_if_lt VAR_LEVEL_CAP, VAR_TEMP_F, Common_EventScript_CheckIfLevelCapTooLow
	trainerbattle_single TRAINER_BROTHER_01_04, PacifidlogTown_Text_Kieran4Intro, PacifidlogTown_Text_Kieran4Defeat, PacifidlogTown_EventScript_Kieran4_Defeated
	msgbox PacifidlogTown_Text_Kieran4PostBattle, MSGBOX_AUTOCLOSE
	end

PacifidlogTown_EventScript_Kieran4_Defeated::
	setvar VAR_LEVEL_CAP 42
	call Common_EventScript_NewLevelCapTextAndFanfare
	return

PacifidlogTown_EventScript_Carmine3::
	setvar VAR_TEMP_F, 50
	buffernumberstring STR_VAR_2, VAR_TEMP_F
	goto_if_lt VAR_LEVEL_CAP, VAR_TEMP_F, Common_EventScript_CheckIfLevelCapTooLow
	trainerbattle_single TRAINER_SISTER_01_03, PacifidlogTown_Text_Carmine3Intro, PacifidlogTown_Text_Carmine3Defeat, PacifidlogTown_EventScript_Carmine3_Defeated
	msgbox PacifidlogTown_Text_Carmine3PostBattle, MSGBOX_DEFAULT
	release
	end

PacifidlogTown_EventScript_Carmine3_Defeated::
	setvar VAR_LEVEL_CAP 52
	call Common_EventScript_NewLevelCapTextAndFanfare
	return

PacifidlogTown_Text_Carmine3Intro:
	.string "Stop! Have a good look at my precious\n"
	.string "POKéMON!$"

PacifidlogTown_Text_Carmine3Defeat:
	.string "Oh, how dare you!\n"
	.string "Don't take it so seriously!$"

PacifidlogTown_Text_Carmine3PostBattle:
	.string "Maybe I'll go catch more POKéMON at\n"
	.string "the SAFARI.$"


PacifidlogTown_Text_Kieran4Intro:
	.string "What do you call a wild man up in the\n"
	.string "mountains? A mountain man, right?\p"
	.string "So why don't they call a POKéMON in\n"
	.string "the mountains a mountain POKéMON?$"

PacifidlogTown_Text_Kieran4Defeat:
	.string "My POKéMON…\n"
	.string "Ran out of power…$"

PacifidlogTown_Text_Kieran4PostBattle:
	.string "They halted development here to\n"
	.string "protect POKéMON, right?\l"
	.string "There's a feel-good story!$"

PacifidlogTown_Text_FastRunningCurrent:
	.string "The sea between PACIFIDLOG and\n"
	.string "SLATEPORT has a fast-running tide.\p"
	.string "If you decide to SURF, you could end\n"
	.string "up swept away somewhere else.$"

PacifidlogTown_Text_NeatHousesOnWater:
	.string "See, isn't it neat?\n"
	.string "These houses are on water!\p"
	.string "I was born here!$"

PacifidlogTown_Text_SkyPillarTooScary:
	.string "The SKY PILLAR?\p"
	.string "…Oh, you must mean that tall, tall\n"
	.string "tower a little further out.\p"
	.string "If you asked me, I wouldn't climb it.\n"
	.string "It's too scary to get up that high.\p"
	.string "Life at sea level in PACIFIDLOG,\n"
	.string "that suits me fine.$"

PacifidlogTown_Text_TownSign:
	.string "PACIFIDLOG TOWN\p"
	.string "“Where the morning sun smiles upon\n"
	.string "the waters.”$"

