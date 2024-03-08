# MSBTools

[![Version 0.1](https://img.shields.io/badge/version-v0.1-red)](todo!)

A crate that allows manipulating Nintendo's proprietary MSBT format, with a library crate to parse those files.

# Usage

MSBTools has 4 different modes of operation:

## Extract:

This command converts an MSBT to TOML.

Usage:
`msbtool extract file.msbt`

This creates a "file.toml" file next to the executable, containing all the strings in the MSBT file ordered by internal index.

## Create:

This command converts a TOML to an MSBT file.

Usage:
`msbtool create file.toml`

This creates a "file.msbt" file next to the executable, containing all the strings in the TOML file.

## Diff:

This command creates a diff file between an MSBT file and multiple others.

Usage:
`msbtool diff original.msbt edited1.msbt edited2.msbt ...`

This creates a "original.msbd.txt" file next to the executable, containing the differences between the original file and all the edited files.

## Patch:

This command patches an MSBT file using a file formatted in the msbd format.

Usage:
`msbtool patch original.msbt diff.msbd.txt`

This creates a new MSBT file, named after the patch name found in the msbd file, with all the modifications specified by said msbd file.

# The MSBD format

MSBTool comes with its own diff format, made specifically for MSBT files. It is specified as:
```
[File Name]
[Patch name]
[SHA256 of the original file] (optional)

[+/-/~][label]
>[text]
>...

[+/-/~][label]
>[text]
>...

...
```

Each diff block being specified as:
- A state. Can be '+' for a new string, '-' for a deleted string, or '~' for an edited string.
- A label. Specifies the label of the string to add, delete or edit.
- A string. Specifies what the new string is. Deleted strings do not have this block.

# Control codes

Control codes have been escaped, for ease of use. For now, only one syntax is available, `[RawCmd Group.Type Argument1_Argument2_etc...]`. For instance, changing the text's colour would be `[RawCmd 0.3 RR_GG_BB_AA]`.

To close a control code, simply type `[/RawCmd Group.Type]`.

# Escape codes

For ease of use, certain characters have been escaped. To use them, type `[!Escape_code]`. For the 3DS's A button, for instance, you'd type `[!A_button_3DS]`.

The characters are as follow:

| Name| Code| Corresponds to|
|--------------|---------|--------------------------------------------------|
|A_button_3DS|0xE000|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe000.png"  width="25">|
|B_button_3DS|0xE001|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe001.png"  width="25">|
|X_button_3DS|0xE002|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe002.png"  width="25">|
|Y_button_3DS|0xE003|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe003.png"  width="25">|
|L_button_3DS|0xE004|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe004.png"  width="25">|
|R_button_3DS|0xE005|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe005.png"  width="25">|
|D_pad_3DS|0xE006|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe006.png"  width="25">|
|Clock_DS|0xE007|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe007.png"  width="25">|
|Happy_face_DS|0xE008|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe008.png"  width="25">|
|Angry_face_DS|0xE009|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe009.png"  width="25">|
|Sad_face_DS|0xE00A|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe00a.png"  width="25">|
|Expressionless_face_DS|0xE00B|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe00b.png"  width="25">|
|Sun_DS|0xE00C|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe00c.png"  width="25">|
|Cloud_DS|0xE00D|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe00d.png"  width="25">|
|Umbrella_DS|0xE00E|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe00e.png"  width="25">|
|Snowman_DS|0xE00F|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe00f.png"  width="25">|
|Exclamation_mark_DS|0xE010|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe010.png"  width="25">|
|Question_mark_DS|0xE011|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe011.png"  width="25">|
|Letter_DS|0xE012|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe012.png"  width="25">|
|Phone_DS|0xE013|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe013.png"  width="25">|
|Calibration_DS|0xE014|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe014.png"  width="25">|
|Spade_suit_DS|0xE015|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe015.png"  width="25">|
|Diamond_suit_DS|0xE016|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe016.png"  width="25">|
|Heart_DS|0xE017|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe017.png"  width="25">|
|Clubs_suit_DS|0xE018|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe018.png"  width="25">|
|Right_DS|0xE019|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe019.png"  width="25">|
|Left_DS|0xE01A|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe01a.png"  width="25">|
|Up_DS|0xE01B|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe01b.png"  width="25">|
|Down_DS|0xE01C|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe01c.png"  width="25">|
|Target_DS|0xE01D|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe01d.png"  width="25">|
|Camera_DS|0xE01E|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe01e.png"  width="25">|
|Unkown_1_DS|0xE01F|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe01f.png"  width="25">|
|Top_left_corner_border_DS|0xE020|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe020.png"  width="25">|
|Top_border_DS|0xE021|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe021.png"  width="25">|
|Top_right_corner_border_DS|0xE022|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe022.png"  width="25">|
|Right_border_DS|0xE023|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe023.png"  width="25">|
|Bottom_right_border_DS|0xE024|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe024.png"  width="25">|
|Bottom_border_DS|0xE025|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe025.png"  width="25">|
|Bottom_left_border_DS|0xE026|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe026.png"  width="25">|
|Left_border_DS|0xE027|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe027.png"  width="25">|
|Cross_DS|0xE028|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe028.png"  width="25">|
|Boxed_A_DS|0xE029|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe029.png"  width="25">|
|Boxed_B_DS|0xE02A|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe02a.png"  width="25">|
|Boxed_C_DS|0xE02B|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe02b.png"  width="25">|
|Boxed_D_DS|0xE02C|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe02c.png"  width="25">|
|Boxed_A_inverted_DS|0xE02D|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe02d.png"  width="25">|
|Boxed_M_inverted_DS|0xE02E|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe02e.png"  width="25">|
|Unknown_2_DS|0xE02F|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe02f.png"  width="25">|
|P_DS|0xE030|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe030.png"  width="25">|
|I_DS|0xE031|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe031.png"  width="25">|
|C_DS|0xE032|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe032.png"  width="25">|
|T_DS|0xE033|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe033.png"  width="25">|
|H_DS|0xE034|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe034.png"  width="25">|
|A_DS|0xE035|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe035.png"  width="25">|
|Unknown_3_DS|0xE036|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe036.png"  width="25">|
|Unknown_4_DS|0xE037|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe037.png"  width="25">|
|Unknown_5_DS|0xE038|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe038.png"  width="25">|
|Unknown_6_DS|0xE039|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe039.png"  width="25">|
|Unknown_7_DS|0xE03A|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe03a.png"  width="25">|
|Unknown_8_DS|0xE03B|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe03b.png"  width="25">|
|Unknown_9_DS|0xE03C|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe03c.png"  width="25">|
|Unknown_10_DS|0xE03D|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe03d.png"  width="25">|
|Inverted_boxed_cross_DS|0xE03E|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe03e.png"  width="25">|
|Inverted_boxed_cross_big_DS|0xE03F|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe03f.png"  width="25">|
|Circle_pad_3DS|0xE077|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe077.png"  width="25">|
|Power_button_3DS|0xE078|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe078.png"  width="25">|
|D_pad_up_3DS|0xE079|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe079.png"  width="25">|
|D_pad_down_3DS|0xE07A|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe07a.png"  width="25">|
|D_pad_left_3DS|0xE07B|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe07b.png"  width="25">|
|D_pad_right_3DS|0xE07C|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe07c.png"  width="25">|
|D_pad_up_down_3DS|0xE07D|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe07d.png"  width="25">|
|D_pad_left_right_3DS|0xE07E|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe07e.png"  width="25">|
|Camera_3DS|0xE01E|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe01e.png"  width="25">|
|Close_empty_3DS|0xE070|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe070.png"  width="25">|
|Close_full_3DS|0xE071|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe071.png"  width="25">|
|Back_3DS|0xE072|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe072.png"  width="25">|
|Home_3DS|0xE073|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe073.png"  width="25">|
|Steps_3DS|0xE074|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe074.png"  width="25">|
|Play_coin_3DS|0xE075|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe075.png"  width="25">|
|Video_3DS|0xE076|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe076.png"  width="25">|
|Power_Wii|0xE040|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe040.png"  width="25">|
|D_pad_Wii|0xE041|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe041.png"  width="25">|
|A_button_Wii|0xE042|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe042.png"  width="25">|
|B_button_Wii|0xE043|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe043.png"  width="25">|
|Home_Wii|0xE044|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe044.png"  width="25">|
|Plus_button_Wii|0xE045|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe045.png"  width="25">|
|Minus_button_Wii|0xE046|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe046.png"  width="25">|
|1_button_Wii|0xE047|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe047.png"  width="25">|
|2_button_Wii|0xE048|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe048.png"  width="25">|
|Nunchuk_stick_Wii|0xE049|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe049.png"  width="25">|
|C_button_Wii|0xE04A|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe04a.png"  width="25">|
|Z_button_Wii|0xE04B|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe04b.png"  width="25">|
|A_button_classic_Wii|0xE04C|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe04c.png"  width="25">|
|B_button_classic_Wii|0xE04D|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe04d.png"  width="25">|
|X_button_classic_Wii|0xE04E|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe04e.png"  width="25">|
|Y_button_classic_Wii|0xE04F|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe04f.png"  width="25">|
|L_stick_classic_Wii|0xE050|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe050.png"  width="25">|
|R_stick_classic_Wii|0xE051|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe051.png"  width="25">|
|L_button_classic_Wii|0xE052|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe052.png"  width="25">|
|R_button_classic_Wii|0xE053|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe053.png"  width="25">|
|ZL_button_classic_Wii|0xE054|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe054.png"  width="25">|
|ZR_button_classic_Wii|0xE055|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe055.png"  width="25">|
|Newline_Wii|0xE056|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe056.png"  width="25">|
|Space_Wii|0xE057|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe057.png"  width="25">|
|Hand_pointing_Wii|0xE058|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe058.png"  width="25">|
|Hand_pointing_1P_Wii|0xE059|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe059.png"  width="25">|
|Hand_pointing_2P_Wii|0xE05A|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe05a.png"  width="25">|
|Hand_pointing_3P_Wii|0xE05B|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe05b.png"  width="25">|
|Hand_pointing_4P_Wii|0xE05C|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe05c.png"  width="25">|
|Hand_closed_Wii|0xE05D|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe05d.png"  width="25">|
|Hand_closed_1P_Wii|0xE05E|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe05e.png"  width="25">|
|Hand_closed_2P_Wii|0xE05F|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe05f.png"  width="25">|
|Hand_closed_3P_Wii|0xE060|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe060.png"  width="25">|
|Hand_closed_4P_Wii|0xE061|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe061.png"  width="25">|
|Hand_opened_Wii|0xE062|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe062.png"  width="25">|
|Hand_opened_1P_Wii|0xE063|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe063.png"  width="25">|
|Hand_opened_2P_Wii|0xE064|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe064.png"  width="25">|
|Hand_opened_3P_Wii|0xE065|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe065.png"  width="25">|
|Hand_opened_4P_Wii|0xE066|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe066.png"  width="25">|
|Wii_Wii|0xE067|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe067.png"  width="25">|
|er_Wii|0xE068|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe068.png"  width="25">|
|re_Wii|0xE069|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe069.png"  width="25">|
|e_Wii|0xE06A|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe06a.png"  width="25">|
|Question_mark_Wii|0xE06B|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe06b.png"  width="25">|
|A_button_Switch|0xE0A0|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0a0.png"  width="25">|
|B_button_Switch|0xE0A1|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0a1.png"  width="25">|
|X_button_Switch|0xE0A2|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0a2.png"  width="25">|
|Y_button_Switch|0xE0A3|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0a3.png"  width="25">|
|L_button_Switch|0xE0A4|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0a4.png"  width="25">|
|R_button_Switch|0xE0A5|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0a5.png"  width="25">|
|ZL_button_Switch|0xE0A6|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0a6.png"  width="25">|
|ZR_button_Switch|0xE0A7|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0a7.png"  width="25">|
|SL_button_Switch|0xE0A8|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0a8.png"  width="25">|
|SR_button_Switch|0xE0A9|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0a9.png"  width="25">|
|D_buttons_Switch|0xE0AA|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0aa.png"  width="25">|
|Right_button_side_Switch|0xE0AB|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0ab.png"  width="25">|
|Down_button_side_Switch|0xE0AC|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0ac.png"  width="25">|
|Up_button_side_Switch|0xE0AD|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0ad.png"  width="25">|
|Left_button_side_Switch|0xE0AE|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0ae.png"  width="25">|
|D_button_up_Switch|0xE0AF|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0af.png"  width="25">|
|D_button_down_Switch|0xE0B0|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0b0.png"  width="25">|
|D_button_left_Switch|0xE0B1|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0b1.png"  width="25">|
|D_button_right_Switch|0xE0B2|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0b2.png"  width="25">|
|Vol_up_Switch|0xE0B3|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0b3.png"  width="25">|
|Vol_down_Switch|0xE0B4|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0b4.png"  width="25">|
|Plus_Switch|0xE0B5|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0b5.png"  width="25">|
|Minus_Switch|0xE0B6|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0b6.png"  width="25">|
|Power_Switch|0xE0B7|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0b7.png"  width="25">|
|Sleep_Switch|0xE0B8|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0b8.png"  width="25">|
|Home_Switch|0xE0B9|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0b9.png"  width="25">|
|Screenshot_Switch|0xE0BA|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0ba.png"  width="25">|
|Stick_Switch|0xE0C0|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0c0.png"  width="25">|
|Left_stick_Switch|0xE0C1|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0c1.png"  width="25">|
|Right_stick_Switch|0xE0C2|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0c2.png"  width="25">|
|Press_stick_Switch|0xE0C3|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0c3.png"  width="25">|
|Press_left_stick_Switch|0xE0C4|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0c4.png"  width="25">|
|Press_right_stick_Switch|0xE0C5|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0c5.png"  width="25">|
|Rotate_counter_clockwise_left_Switch|0xE0C6|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0c6.png"  width="25">|
|Rotate_counter_clockwise_right_Switch|0xE0C7|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0c7.png"  width="25">|
|Rotate_clockwise_left_Switch|0xE0C8|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0c8.png"  width="25">|
|Rotate_clockwise_right_Switch|0xE0C9|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0c9.png"  width="25">|
|D_pad_Switch|0xE0D0|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0d0.png"  width="25">|
|D_pad_up_Switch|0xE0D1|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0d1.png"  width="25">|
|D_pad_down_Switch|0xE0D2|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0d2.png"  width="25">|
|D_pad_left_Switch|0xE0D3|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0d3.png"  width="25">|
|D_pad_right_Switch|0xE0D4|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0d4.png"  width="25">|
|D_pad_up_down_Switch|0xE0D5|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0d5.png"  width="25">|
|D_pad_left_right_Switch|0xE0D6|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0d6.png"  width="25">|
|A_button_inverted_Switch|0xE0E0|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0e0.png"  width="25">|
|B_button_inverted_Switch|0xE0E1|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0e1.png"  width="25">|
|X_button_inverted_Switch|0xE0E2|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0e2.png"  width="25">|
|Y_button_inverted_Switch|0xE0E3|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0e3.png"  width="25">|
|L_button_inverted_Switch|0xE0E4|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0e4.png"  width="25">|
|R_button_inverted_Switch|0xE0E5|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0e5.png"  width="25">|
|ZL_button_inverted_Switch|0xE0E6|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0e6.png"  width="25">|
|ZR_button_inverted_Switch|0xE0E7|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0e7.png"  width="25">|
|SL_button_inverted_Switch|0xE0E8|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0e8.png"  width="25">|
|SR_button_inverted_Switch|0xE0E9|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0e9.png"  width="25">|
|D_buttons_inverted_Switch|0xE0EA|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0ea.png"  width="25">|
|D_button_up_inverted_Switch|0xE0EB|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0eb.png"  width="25">|
|D_button_down_inverted_Switch|0xE0EC|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0ec.png"  width="25">|
|D_button_left_inverted_Switch|0xE0ED|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0ed.png"  width="25">|
|D_button_right_inverted_Switch|0xE0EE|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0ee.png"  width="25">|
|Vol_up_inverted_Switch|0xE0EF|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0ef.png"  width="25">|
|Vol_down_inverted_Switch|0xE0F0|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0f0.png"  width="25">|
|Plus_inverted_Switch|0xE0F1|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0f1.png"  width="25">|
|Minus_inverted_Switch|0xE0F2|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0f2.png"  width="25">|
|Power_inverted_Switch|0xE0F3|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0f3.png"  width="25">|
|Home_inverted_Switch|0xE0F4|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0f4.png"  width="25">|
|Screenshot_inverted_Switch|0xE0F5|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe0f5.png"  width="25">|
|Stick_inverted_Switch|0xE100|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe100.png"  width="25">|
|Left_stick_inverted_Switch|0xE101|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe101.png"  width="25">|
|Right_stick_inverted_Switch|0xE102|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe102.png"  width="25">|
|Press_stick_inverted_Switch|0xE103|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe103.png"  width="25">|
|Press_left_stick_inverted_Switch|0xE104|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe104.png"  width="25">|
|Press_right_stick_inverted_Switch|0xE105|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe105.png"  width="25">|
|D_pad_inverted_Switch|0xE110|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe110.png"  width="25">|
|D_pad_up_inverted_Switch|0xE111|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe111.png"  width="25">|
|D_pad_down_inverted_Switch|0xE112|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe112.png"  width="25">|
|D_pad_left_inverted_Switch|0xE113|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe113.png"  width="25">|
|D_pad_right_inverted_Switch|0xE114|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe114.png"  width="25">|
|D_pad_up_down_inverted_Switch|0xE115|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe115.png"  width="25">|
|D_pad_left_right_inverted_Switch|0xE116|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe116.png"  width="25">|
|Handheld_controller_Switch|0xE121|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe121.png"  width="25">|
|Both_joycons_controller_Switch|0xE122|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe122.png"  width="25">|
|Left_joycon_controller_Switch|0xE123|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe123.png"  width="25">|
|Right_joycon_controller_Switch|0xE124|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe124.png"  width="25">|
|Left_joycon_with_minus_controller_Switch|0xE125|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe125.png"  width="25">|
|Right_joycon_with_plus_controller_Switch|0xE126|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe126.png"  width="25">|
|Joycon_side_controller_Switch|0xE127|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe127.png"  width="25">|
|Left_joycon_with_minus_side_controller_Switch|0xE128|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe128.png"  width="25">|
|Right_joycon_with_plus_side_controller_Switch|0xE129|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe129.png"  width="25">|
|Both_joycons_grip_controller_Switch|0xE12A|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe12a.png"  width="25">|
|No_joycons_grip_controller_Switch|0xE12B|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe12b.png"  width="25">|
|Pro_controller_Switch|0xE12C|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe12c.png"  width="25">|
|Brightness_Switch|0xE130|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe130.png"  width="25">|
|Friends_Switch|0xE131|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe131.png"  width="25">|
|News_Switch|0xE132|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe132.png"  width="25">|
|Eshop_Switch|0xE133|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe133.png"  width="25">|
|Gallery_Switch|0xE134|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe134.png"  width="25">|
|Apps_Switch|0xE135|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe135.png"  width="25">|
|Controllers_Switch|0xE136|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe136.png"  width="25">|
|Mail_Switch|0xE137|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe137.png"  width="25">|
|Phone_Switch|0xE138|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe138.png"  width="25">|
|PC_Switch|0xE139|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe139.png"  width="25">|
|TV_Switch|0xE13A|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe13a.png"  width="25">|
|Headphone_Switch|0xE13B|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe13b.png"  width="25">|
|Sound_Switch|0xE13C|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe13c.png"  width="25">|
|Warning_Switch|0xE140|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe140.png"  width="25">|
|Error_Switch|0xE141|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe141.png"  width="25">|
|Tip_Switch|0xE142|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe142.png"  width="25">|
|Up_Switch|0xE143|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe143.png"  width="25">|
|Down_Switch|0xE144|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe144.png"  width="25">|
|Left_Switch|0xE145|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe145.png"  width="25">|
|Right_Switch|0xE146|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe146.png"  width="25">|
|North_Switch|0xE147|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe147.png"  width="25">|
|South_Switch|0xE148|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe148.png"  width="25">|
|West_Switch|0xE149|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe149.png"  width="25">|
|East_Switch|0xE14A|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe14a.png"  width="25">|
|Yes_Switch|0xE14B|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe14b.png"  width="25">|
|No_Switch|0xE14C|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe14c.png"  width="25">|
|Warning_inverted_Switch|0xE150|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe150.png"  width="25">|
|Error_inverted_Switch|0xE151|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe151.png"  width="25">|
|Tip_inverted_Switch|0xE152|<img src="https://raw.githubusercontent.com/TheAlternateDoctor/nintendo_font_extra_characters/main/png/0xe152.png"  width="25">|