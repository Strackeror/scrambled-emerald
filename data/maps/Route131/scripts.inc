.set LOCALID_DONDOZO, 9

Route131_MapScripts::
	map_script MAP_SCRIPT_ON_TRANSITION, Route131_OnTransition
	.byte 0

Route131_OnTransition:
	call_if_ge VAR_SOOTOPOLIS_CITY_STATE, 4, Route131_EventScript_CheckSetAbnormalWeather
	call Route131_EventScript_SetLayout
	end

Route131_EventScript_SetLayout::
	setmaplayoutindex LAYOUT_ROUTE131_SKY_PILLAR
	return

Route131_EventScript_CheckSetAbnormalWeather::
	call_if_set FLAG_SYS_WEATHER_CTRL, Common_EventScript_SetAbnormalWeather
	return

Route131_EventScript_Richard::
	trainerbattle_single TRAINER_RICHARD, Route131_Text_RichardIntro, Route131_Text_RichardDefeat
	msgbox Route131_Text_RichardPostBattle, MSGBOX_AUTOCLOSE
	end

Route131_EventScript_Herman::
	trainerbattle_single TRAINER_HERMAN, Route131_Text_HermanIntro, Route131_Text_HermanDefeat
	msgbox Route131_Text_HermanPostBattle, MSGBOX_AUTOCLOSE
	end

Route131_EventScript_Susie::
	trainerbattle_single TRAINER_SUSIE, Route131_Text_SusieIntro, Route131_Text_SusieDefeat
	msgbox Route131_Text_SusiePostBattle, MSGBOX_AUTOCLOSE
	end

Route131_EventScript_Kara::
	trainerbattle_single TRAINER_KARA, Route131_Text_KaraIntro, Route131_Text_KaraDefeat
	msgbox Route131_Text_KaraPostBattle, MSGBOX_AUTOCLOSE
	end

Route131_EventScript_Reli::
	trainerbattle_double TRAINER_RELI_AND_IAN, Route131_Text_ReliIntro, Route131_Text_ReliDefeat, Route131_Text_ReliNotEnoughMons
	msgbox Route131_Text_ReliPostBattle, MSGBOX_AUTOCLOSE
	end

Route131_EventScript_Ian::
	trainerbattle_double TRAINER_RELI_AND_IAN, Route131_Text_IanIntro, Route131_Text_IanDefeat, Route131_Text_IanNotEnoughMons
	msgbox Route131_Text_IanPostBattle, MSGBOX_AUTOCLOSE
	end

Route131_EventScript_Talia::
	trainerbattle_single TRAINER_TALIA, Route131_Text_TaliaIntro, Route131_Text_TaliaDefeat
	msgbox Route131_Text_TaliaPostBattle, MSGBOX_AUTOCLOSE
	end

Route131_EventScript_Kevin::
	trainerbattle_single TRAINER_KEVIN, Route131_Text_KevinIntro, Route131_Text_KevinDefeat
	msgbox Route131_Text_KevinPostBattle, MSGBOX_AUTOCLOSE
	end

Route131_EventScript_TitanDondozo::
	goto_if_eq VAR_PATH, 2, Route131_EventScript_TitanDondozoReverse
	setvar VAR_TEMP_F, 58
	buffernumberstring STR_VAR_2, VAR_TEMP_F
	goto_if_lt VAR_LEVEL_CAP, VAR_TEMP_F, Common_EventScript_CheckIfLevelCapTooLow
	lock
	faceplayer
	msgbox Route131_Text_Titan, MSGBOX_DEFAULT
	titanbattle 11
	waitstate
	msgbox Route131_Text_Titan, MSGBOX_DEFAULT
	titanbattle 12
	waitstate
	fadescreen FADE_TO_BLACK
	removeobject LOCALID_DONDOZO
	setflag FLAG_TITAN_DONDOZO
	fadescreen FADE_FROM_BLACK
	call Common_EventScript_AddMoneyScrambledTitanFights
	setvar VAR_LEVEL_CAP 60
	call Common_EventScript_NewLevelCapTextAndFanfare
	release
	end

Route131_EventScript_TitanDondozoReverse::
	setvar VAR_TEMP_F, 15
	buffernumberstring STR_VAR_2, VAR_TEMP_F
	goto_if_lt VAR_LEVEL_CAP, VAR_TEMP_F, Common_EventScript_CheckIfLevelCapTooLow
	lock
	faceplayer
	msgbox Route131_Text_Titan, MSGBOX_DEFAULT
	titanbattle 16
	waitstate
	msgbox Route131_Text_Titan, MSGBOX_DEFAULT
	titanbattle 17
	waitstate
	fadescreen FADE_TO_BLACK
	removeobject LOCALID_DONDOZO
	setflag FLAG_TITAN_DONDOZO
	fadescreen FADE_FROM_BLACK
	call Common_EventScript_AddMoneyScrambledTitanFights
	setvar VAR_LEVEL_CAP 18
	call Common_EventScript_NewLevelCapTextAndFanfare
	release
	end

Route131_Text_Titan::
	.string "...$"
