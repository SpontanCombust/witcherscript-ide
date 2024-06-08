/* 
 * Warning: this script is a part of native content directory ditributed with WIDE.
 * It includes types accessible from scripts that are not however explicitly declared anywhere.
 * Their reconstructed definition may not be accurate.
 */


// My own guess as to why these are not declared directly in WS is because enums can't be imported like structs or classes can.


// engine/components.ws

enum EWoundTypeFlags
{
	WTF_None		= 0,
	WTF_Cut			= 1, //FLAG( 0 ),
	WTF_Explosion	= 2, //FLAG( 1 ),
	WTF_Frost		= 4, //FLAG( 2 ),
	WTF_All			= 7, //WTF_Cut | WTF_Explosion | WTF_Frost,
}



// engine/gameplayEffectsComponent.ws

enum EEntityGameplayEffectFlags {
	EGEF_FocusModeHighlight,
	EGEF_CatViewHiglight
}



// engine/inputKeys.ws

enum EInputKey
{
	IK_None=0,
	IK_LeftMouse=1,
	IK_RightMouse=2,
	IK_MiddleMouse=3,
	IK_Unknown04=4,
	IK_Unknown05=5,
	IK_Unknown06=6,
	IK_Unknown07=7,
	IK_Backspace=8,
	IK_Tab=9,
	IK_Unknown0A=10,
	IK_Unknown0B=11,
	IK_Unknown0C=12,
	IK_Enter=13,
	IK_Unknown0E=14,
	IK_Unknown0F=15,
	IK_Shift=16,
	IK_Ctrl=17,
	IK_Alt=18,
	IK_Pause=19,
	IK_CapsLock=20,
	IK_Unknown15=21,
	IK_Unknown16=22,
	IK_Unknown17=23,
	IK_Unknown18=24,
	IK_Unknown19=25,
	IK_Unknown1A=26,
	IK_Escape=27,
	IK_Unknown1C=28,
	IK_Unknown1D=29,
	IK_Unknown1E=30,
	IK_Unknown1F=31,
	IK_Space=32,
	IK_PageUp=33,
	IK_PageDown=34,
	IK_End=35,
	IK_Home=36,
	IK_Left=37,
	IK_Up=38,
	IK_Right=39,
	IK_Down=40,
	IK_Select=41,
	IK_Print=42,
	IK_Execute=43,
	IK_PrintScrn=44,
	IK_Insert=45,
	IK_Delete=46,
	IK_Help=47,
	IK_0=48,
	IK_1=49,
	IK_2=50,
	IK_3=51,
	IK_4=52,
	IK_5=53,
	IK_6=54,
	IK_7=55,
	IK_8=56,
	IK_9=57,
	IK_Unknown3A=58,
	IK_Unknown3B=59,
	IK_Unknown3C=60,
	IK_Unknown3D=61,
	IK_Unknown3E=62,
	IK_Unknown3F=63,
	IK_Unknown40=64,
	IK_A=65,
	IK_B=66,
	IK_C=67,
	IK_D=68,
	IK_E=69,
	IK_F=70,
	IK_G=71,
	IK_H=72,
	IK_I=73,
	IK_J=74,
	IK_K=75,
	IK_L=76,
	IK_M=77,
	IK_N=78,
	IK_O=79,
	IK_P=80,
	IK_Q=81,
	IK_R=82,
	IK_S=83,
	IK_T=84,
	IK_U=85,
	IK_V=86,
	IK_W=87,
	IK_X=88,
	IK_Y=89,
	IK_Z=90,
	IK_Unknown5B=91,
	IK_Unknown5C=92,
	IK_Unknown5D=93,
	IK_Unknown5E=94,
	IK_Unknown5F=95,
	IK_NumPad0=96,
	IK_NumPad1=97,
	IK_NumPad2=98,
	IK_NumPad3=99,
	IK_NumPad4=100,
	IK_NumPad5=101,
	IK_NumPad6=102,
	IK_NumPad7=103,
	IK_NumPad8=104,
	IK_NumPad9=105,
	IK_NumStar=106,
	IK_NumPlus=107,
	IK_Separator=108,
	IK_NumMinus=109,
	IK_NumPeriod=110,
	IK_NumSlash=111,
	IK_F1=112,
	IK_F2=113,
	IK_F3=114,
	IK_F4=115,
	IK_F5=116,
	IK_F6=117,
	IK_F7=118,
	IK_F8=119,
	IK_F9=120,
	IK_F10=121,
	IK_F11=122,
	IK_F12=123,
	IK_F13=124,
	IK_F14=125,
	IK_F15=126,
	IK_F16=127,
	IK_F17=128,
	IK_F18=129,
	IK_F19=130,
	IK_F20=131,
	IK_F21=132,
	IK_F22=133,
	IK_F23=134,
	IK_F24=135,
	IK_Pad_A_CROSS=136,
	IK_Pad_B_CIRCLE=137,
	IK_Pad_X_SQUARE=138,
	IK_Pad_Y_TRIANGLE=139,
	IK_Pad_Start=140,
	IK_Pad_Back_Select=141,
	IK_Pad_DigitUp=142,
	IK_Pad_DigitDown=143,
	IK_Pad_DigitLeft=144,
	IK_Pad_DigitRight=145,
	IK_Pad_LeftThumb=146,		// L3	
	IK_Pad_RightThumb=147,		// R3
	IK_Pad_LeftShoulder=148,	// L1
	IK_Pad_RightShoulder=149,	// R1
	IK_Pad_LeftTrigger=150,		// Axis	L2
	IK_Pad_RightTrigger=151,	// Axis	R2
	IK_Pad_LeftAxisX=152,		// Axis	ANALOG LEFT
	IK_Pad_LeftAxisY=153,		// Axis
	IK_Pad_RightAxisX=154,		// Axis	ANALOG RIGHT
	IK_Pad_RightAxisY=155,		// Axis
	IK_NumLock=156,
	IK_ScrollLock=157,
	IK_Unknown9E=158,
	IK_Unknown9F=159,
	IK_LShift=160,
	IK_RShift=161,
	IK_LControl=162,
	IK_RControl=163,
	IK_UnknownA4=164,
	IK_UnknownA5=165,
	IK_UnknownA6=166,
	IK_UnknownA7=167,
	IK_UnknownA8=168,
	IK_UnknownA9=169,
	IK_UnknownAA=170,
	IK_UnknownAB=171,
	IK_UnknownAC=172,
	IK_UnknownAD=173,
	IK_UnknownAE=174,
	IK_UnknownAF=175,
	IK_UnknownB0=176,
	IK_UnknownB1=177,
	IK_UnknownB2=178,
	IK_UnknownB3=179,
	IK_UnknownB4=180,
	IK_UnknownB5=181,
	IK_UnknownB6=182,
	IK_UnknownB7=183,
	IK_UnknownB8=184,
	IK_Unicode=185,
	IK_Semicolon=186,
	IK_Equals=187,
	IK_Comma=188,
	IK_Minus=189,
	IK_Period=190,
	IK_Slash=191,
	IK_Tilde=192,
	IK_Mouse4=193,
	IK_Mouse5=194,
	IK_Mouse6=195,
	IK_Mouse7=196,
	IK_Mouse8=197,
	IK_UnknownC6=198,
	IK_UnknownC7=199,
	IK_Joy1=200,
	IK_Joy2=201,
	IK_Joy3=202,
	IK_Joy4=203,
	IK_Joy5=204,
	IK_Joy6=205,
	IK_Joy7=206,
	IK_Joy8=207,
	IK_Joy9=208,
	IK_Joy10=209,
	IK_Joy11=210,
	IK_Joy12=211,
	IK_Joy13=212,
	IK_Joy14=213,
	IK_Joy15=214,
	IK_Joy16=215,
	IK_UnknownD8=216,
	IK_UnknownD9=217,
	IK_UnknownDA=218,
	IK_LeftBracket=219,
	IK_Backslash=220,
	IK_RightBracket=221,
	IK_SingleQuote=222,
	IK_UnknownDF=223,
	IK_UnknownE0=224,
	IK_UnknownE1=225,
	IK_UnknownE2=226,
	IK_UnknownE3=227,
	IK_MouseX=228,
	IK_MouseY=229,
	IK_MouseZ=230,
	IK_MouseW=231,
	IK_JoyU=232,
	IK_JoyV=233,
	IK_JoySlider1=234,
	IK_JoySlider2=235,
	IK_MouseWheelUp=236,
	IK_MouseWheelDown=237,
	IK_UnknownEE=238,
	IK_UnknownEF=239,
	IK_JoyX=240,
	IK_JoyY=241,
	IK_JoyZ=242,
	IK_JoyR=243,
	IK_UnknownF4=244,
	IK_UnknownF5=245,
	IK_Attn=246,
	IK_ClearSel=247,
	IK_ExSel=248,
	IK_ErEof=249,
	IK_Play=250,
	IK_Zoom=251,
	IK_NoName=252,
	IK_UnknownFD=253,
	IK_UnknownFE=254,
	IK_PS4_OPTIONS=255,
	IK_PS4_TOUCH_PRESS=256,
	IK_Last=257,
	IK_Count=258,

	IK_Pad_First	= 136, // IK_Pad_A_CROSS
	IK_Pad_Last		= 155, // IK_Pad_RightAxisY
}



// engine/physicsTraceManager.ws

enum EBatchQueryQueryFlag
{
	EQQF_IMPACT,
	EQQF_NORMAL,
	EQQF_DISTANCE,
	EQQF_TOUCHING_HIT,
	EQQF_BLOCKING_HIT,
	EQQF_NO_INITIAL_OVERLAP,
	EQQF_PRECISE_SWEEP,
	EQQF_MESH_BOTH_SIDES
}

enum EBatchQueryState
{
	BQS_NotFound,
	BQS_NotReady,
	BQS_Processed
}



// engine/showFlags.ws

enum EShowFlags
{
	SHOW_Meshes,
	SHOW_AI,
	SHOW_HierarchicalGrid,
	SHOW_BeamTree,
	SHOW_WalkMesh,
	SHOW_Occluders,
	SHOW_Shadows,
	SHOW_Lights,
	SHOW_Terrain,
	SHOW_PostProcess,
	SHOW_Selection,
	SHOW_Grid,
	SHOW_Sprites,
	SHOW_Particles,
	SHOW_Wireframe,
	SHOW_WalkSurfaces,
	SHOW_Areas,
	SHOW_Logic,
	SHOW_Log,
	SHOW_Profilers,
	SHOW_Pathfinding,
	SHOW_Stickers,
	SHOW_Bboxes,
	SHOW_Exploration,
	SHOW_Behavior,
	SHOW_Emissive,
	SHOW_Spawnset,
	SHOW_Collision,
	SHOW_GUI,
	SHOW_VisualDebug,
	SHOW_Decals,
	SHOW_Lighting,
	SHOW_TSLighting,
	SHOW_Refraction
}



// game/actorsStorage.ws

enum EScriptQueryFlags {
	FLAG_ExcludePlayer		= 1,  	 // FLAG( 0 ),
	FLAG_OnlyActors			= 2,  	 // FLAG( 1 ),
	FLAG_OnlyAliveActors	= 4,  	 // FLAG( 2 ),
	FLAG_WindEmitters		= 8,  	 // FLAG( 3 ),
	FLAG_Vehicles			= 16, 	 // FLAG( 4 ),
	FLAG_ExcludeTarget		= 32, 	 // FLAG( 5 ),
	FLAG_Attitude_Neutral	= 64, 	 // FLAG( 6 ),		towards actor specfied as 'target' param
	FLAG_Attitude_Friendly	= 128, 	 // FLAG( 7 ),		towards actor specfied as 'target' param
	FLAG_Attitude_Hostile	= 256, 	 // FLAG( 8 ),		towards actor specfied as 'target' param
	FLAG_ZDiff_3			= 512, 	 // FLAG( 9 ),
	FLAG_ZDiff_5			= 1024,  // FLAG( 10 ),
	FLAG_ZDiff_Range		= 2048,  // FLAG( 11 ),
	FLAG_PathLibTest		= 4096,  // FLAG( 12 ),
	FLAG_NotVehicles		= 8192,  // FLAG( 13 ),
	FLAG_TestLineOfSight	= 16384, // FLAG( 14 ),		broken: with 5 enemies with clear line of sight it finds 0
}



// game/characterStats.ws

enum EBaseCharacterStats
{
	BCS_Vitality,
	BCS_Essence,
	BCS_Stamina,
	BCS_Toxicity,
	BCS_Focus,
	BCS_Morale,
	BCS_Air,
	BCS_Panic,			// default panic val is now 100
	BCS_PanicStatic,	// Used when reducing BCS_Panic. BCS_Panic can't go below BCS_PanicStatic value
	BCS_SwimmingStamina,
	BCS_Undefined
}

enum ECharacterDefenseStats
{
	CDS_None,
	CDS_PhysicalRes,
	CDS_BleedingRes,
	CDS_PoisonRes,
	CDS_FireRes,
	CDS_FrostRes,
	CDS_ShockRes,
	CDS_ForceRes,
	CDS_FreezeRes,	// #B deprecated
	CDS_WillRes,
	CDS_BurningRes,
	CDS_SlashingRes,
	CDS_PiercingRes,
	CDS_BludgeoningRes,
	CDS_RendingRes,
	CDS_ElementalRes,
	CDS_DoTBurningDamageRes,
	CDS_DoTPoisonDamageRes,
	CDS_DoTBleedingDamageRes
}



// game/types.ws

enum EAnimationEventType
{
	AET_Tick,
	AET_DurationStart,
	AET_DurationStartInTheMiddle,
	AET_DurationEnd,
	AET_Duration,
}

enum EDifficultyMode
{
	EDM_NotSet,
	EDM_Easy,
	EDM_Medium,
	EDM_Hard,
	EDM_Hardcore
}

enum EVisibilityTest
{
	VT_None,
	VT_LineOfSight,
	VT_RangeAndLineOfSight,
}

enum EAIPriority
{
	AIP_Lowest,
	AIP_Low,
	AIP_Normal,
	AIP_High,
	AIP_Highest,
	AIP_BlockingScene,
	AIP_Cutscene,
	AIP_Combat,
	AIP_Custom,
	AIP_Minigame,
	AIP_Audience,
	AIP_Unconscious,
}

enum EFocusModeVisibility
{
	FMV_None,
	FMV_Interactive,
	FMV_Clue
}

enum EAIAttitude
{
	AIA_Neutral,
	AIA_Friendly,
	AIA_Hostile
}

enum ENPCGroupType
{
	ENGT_Enemy,
	ENGT_Commoner,
	ENGT_Quest,
	ENGT_Guard
}

enum EAsyncCheckResult
{
	ASR_InProgress,
	ASR_ReadyTrue,
	ASR_ReadyFalse,
	ASR_Failed
}
