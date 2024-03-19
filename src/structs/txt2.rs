use std::collections::VecDeque;
use std::io::{Read, Seek, SeekFrom};
use bytestream::{ByteOrder, StreamReader};
use regex::Regex;
use crate::error::{Error, Result};
use crate::msbt::MSBTString;
use crate::control_codes::{convert_control_code, convert_control_code_binary, convert_control_code_close, convert_control_code_close_binary};

#[derive(Debug, Clone)]
pub struct TXT2{
    _magic: Vec::<u8>,
    _section_size: u32,
    _string_amount: u32,
    pub offsets: Vec<u32>,
    pub strings: Vec<Vec<u8>>
}

const ESCAPE_CODES_3DS:[(&str, u16);23] = [
    ("A_button_3DS", 0xE000),
    ("B_button_3DS", 0xE001),
    ("X_button_3DS", 0xE002),
    ("Y_button_3DS", 0xE003),
    ("L_button_3DS", 0xE004),
    ("R_button_3DS", 0xE005),
    ("D_pad_3DS", 0xE006),
    ("Circle_pad_3DS", 0xE077),
    ("Power_button_3DS", 0xE078),
    ("D_pad_up_3DS", 0xE079),
    ("D_pad_down_3DS", 0xE07A),
    ("D_pad_left_3DS", 0xE07B),
    ("D_pad_right_3DS", 0xE07C),
    ("D_pad_up_down_3DS", 0xE07D),
    ("D_pad_left_right_3DS", 0xE07E),
    ("Camera_3DS", 0xE01E),
    ("Close_empty_3DS", 0xE070),
    ("Close_full_3DS", 0xE071),
    ("Back_3DS", 0xE072),
    ("Home_3DS", 0xE073),
    ("Steps_3DS", 0xE074),
    ("Play_coin_3DS", 0xE075),
    ("Video_3DS", 0xE076),
];

const ESCAPE_CODES_WII:[(&str, u16);44] = [
    ("Power_Wii", 0xE040),
    ("D_pad_Wii", 0xE041),
    ("A_button_Wii", 0xE042),
    ("B_button_Wii", 0xE043),
    ("Home_Wii", 0xE044),
    ("Plus_button_Wii", 0xE045),
    ("Minus_button_Wii", 0xE046),
    ("1_button_Wii", 0xE047),
    ("2_button_Wii", 0xE048),
    ("Nunchuk_stick_Wii", 0xE049),
    ("C_button_Wii", 0xE04A),
    ("Z_button_Wii", 0xE04B),
    ("A_button_classic_Wii", 0xE04C),
    ("B_button_classic_Wii", 0xE04D),
    ("X_button_classic_Wii", 0xE04E),
    ("Y_button_classic_Wii", 0xE04F),
    ("L_stick_classic_Wii", 0xE050),
    ("R_stick_classic_Wii", 0xE051),
    ("L_button_classic_Wii", 0xE052),
    ("R_button_classic_Wii", 0xE053),
    ("ZL_button_classic_Wii", 0xE054),
    ("ZR_button_classic_Wii", 0xE055),
    ("Newline_Wii", 0xE056),
    ("Space_Wii", 0xE057),
    ("Hand_pointing_Wii", 0xE058),
    ("Hand_pointing_1P_Wii", 0xE059),
    ("Hand_pointing_2P_Wii", 0xE05A),
    ("Hand_pointing_3P_Wii", 0xE05B),
    ("Hand_pointing_4P_Wii", 0xE05C),
    ("Hand_closed_Wii", 0xE05D),
    ("Hand_closed_1P_Wii", 0xE05E),
    ("Hand_closed_2P_Wii", 0xE05F),
    ("Hand_closed_3P_Wii", 0xE060),
    ("Hand_closed_4P_Wii", 0xE061),
    ("Hand_opened_Wii", 0xE062),
    ("Hand_opened_1P_Wii", 0xE063),
    ("Hand_opened_2P_Wii", 0xE064),
    ("Hand_opened_3P_Wii", 0xE065),
    ("Hand_opened_4P_Wii", 0xE066),
    ("Wii_Wii", 0xE067),
    ("er_Wii", 0xE068),
    ("re_Wii", 0xE069),
    ("e_Wii", 0xE06A),
    ("Question_mark_Wii", 0xE06B),
];

const ESCAPE_CODES_SWITCH:[(&str, u16);120] = [
    ("A_button_Switch", 0xE0A0),
    ("B_button_Switch", 0xE0A1),
    ("X_button_Switch", 0xE0A2),
    ("Y_button_Switch", 0xE0A3),
    ("L_button_Switch", 0xE0A4),
    ("R_button_Switch", 0xE0A5),
    ("ZL_button_Switch", 0xE0A6),
    ("ZR_button_Switch", 0xE0A7),
    ("SL_button_Switch", 0xE0A8),
    ("SR_button_Switch", 0xE0A9),
    ("D_buttons_Switch", 0xE0AA),
    ("Right_button_side_Switch", 0xE0AB),
    ("Down_button_side_Switch", 0xE0AC),
    ("Up_button_side_Switch", 0xE0AD),
    ("Left_button_side_Switch", 0xE0AE),
    ("D_button_up_Switch", 0xE0AF),
    ("D_button_down_Switch", 0xE0B0),
    ("D_button_left_Switch", 0xE0B1),
    ("D_button_right_Switch", 0xE0B2),
    ("Vol_up_Switch", 0xE0B3),
    ("Vol_down_Switch", 0xE0B4),
    ("Plus_Switch", 0xE0B5),
    ("Minus_Switch", 0xE0B6),
    ("Power_Switch", 0xE0B7),
    ("Sleep_Switch", 0xE0B8),
    ("Home_Switch", 0xE0B9),
    ("Screenshot_Switch", 0xE0BA),

    ("Stick_Switch", 0xE0C0),
    ("Left_stick_Switch", 0xE0C1),
    ("Right_stick_Switch", 0xE0C2),
    ("Press_stick_Switch", 0xE0C3),
    ("Press_left_stick_Switch", 0xE0C4),
    ("Press_right_stick_Switch", 0xE0C5),
    ("Rotate_counter_clockwise_left_Switch", 0xE0C6),
    ("Rotate_counter_clockwise_right_Switch", 0xE0C7),
    ("Rotate_clockwise_left_Switch", 0xE0C8),
    ("Rotate_clockwise_right_Switch", 0xE0C9),

    ("D_pad_Switch", 0xE0D0),
    ("D_pad_up_Switch", 0xE0D1),
    ("D_pad_down_Switch", 0xE0D2),
    ("D_pad_left_Switch", 0xE0D3),
    ("D_pad_right_Switch", 0xE0D4),
    ("D_pad_up_down_Switch", 0xE0D5),
    ("D_pad_left_right_Switch", 0xE0D6),

    ("A_button_inverted_Switch", 0xE0E0),
    ("B_button_inverted_Switch", 0xE0E1),
    ("X_button_inverted_Switch", 0xE0E2),
    ("Y_button_inverted_Switch", 0xE0E3),
    ("L_button_inverted_Switch", 0xE0E4),
    ("R_button_inverted_Switch", 0xE0E5),
    ("ZL_button_inverted_Switch", 0xE0E6),
    ("ZR_button_inverted_Switch", 0xE0E7),
    ("SL_button_inverted_Switch", 0xE0E8),
    ("SR_button_inverted_Switch", 0xE0E9),
    ("D_buttons_inverted_Switch", 0xE0EA),
    ("D_button_up_inverted_Switch", 0xE0EB),
    ("D_button_down_inverted_Switch", 0xE0EC),
    ("D_button_left_inverted_Switch", 0xE0ED),
    ("D_button_right_inverted_Switch", 0xE0EE),
    ("Vol_up_inverted_Switch", 0xE0EF),
    ("Vol_down_inverted_Switch", 0xE0F0),
    ("Plus_inverted_Switch", 0xE0F1),
    ("Minus_inverted_Switch", 0xE0F2),
    ("Power_inverted_Switch", 0xE0F3),
    ("Home_inverted_Switch", 0xE0F4),
    ("Screenshot_inverted_Switch", 0xE0F5),

    ("Stick_inverted_Switch", 0xE100),
    ("Left_stick_inverted_Switch", 0xE101),
    ("Right_stick_inverted_Switch", 0xE102),
    ("Press_stick_inverted_Switch", 0xE103),
    ("Press_left_stick_inverted_Switch", 0xE104),
    ("Press_right_stick_inverted_Switch", 0xE105),

    ("D_pad_inverted_Switch", 0xE110),
    ("D_pad_up_inverted_Switch", 0xE111),
    ("D_pad_down_inverted_Switch", 0xE112),
    ("D_pad_left_inverted_Switch", 0xE113),
    ("D_pad_right_inverted_Switch", 0xE114),
    ("D_pad_up_down_inverted_Switch", 0xE115),
    ("D_pad_left_right_inverted_Switch", 0xE116),

    ("Handheld_controller_Switch", 0xE121),
    ("Both_joycons_controller_Switch", 0xE122),
    ("Left_joycon_controller_Switch", 0xE123),
    ("Right_joycon_controller_Switch", 0xE124),
    ("Left_joycon_with_minus_controller_Switch", 0xE125),
    ("Right_joycon_with_plus_controller_Switch", 0xE126),
    ("Joycon_side_controller_Switch", 0xE127),
    ("Left_joycon_with_minus_side_controller_Switch", 0xE128),
    ("Right_joycon_with_plus_side_controller_Switch", 0xE129),
    ("Both_joycons_grip_controller_Switch", 0xE12A),
    ("No_joycons_grip_controller_Switch", 0xE12B),
    ("Pro_controller_Switch", 0xE12C),

    ("Brightness_Switch", 0xE130),
    ("Friends_Switch", 0xE131),
    ("News_Switch", 0xE132),
    ("Eshop_Switch", 0xE133),
    ("Gallery_Switch", 0xE134),
    ("Apps_Switch", 0xE135),
    ("Controllers_Switch", 0xE136),
    ("Mail_Switch", 0xE137),
    ("Phone_Switch", 0xE138),
    ("PC_Switch", 0xE139),
    ("TV_Switch", 0xE13A),
    ("Headphone_Switch", 0xE13B),
    ("Sound_Switch", 0xE13C),
    
    ("Warning_Switch", 0xE140),
    ("Error_Switch", 0xE141),
    ("Tip_Switch", 0xE142),
    ("Up_Switch", 0xE143),
    ("Down_Switch", 0xE144),
    ("Left_Switch", 0xE145),
    ("Right_Switch", 0xE146),
    ("North_Switch", 0xE147),
    ("South_Switch", 0xE148),
    ("West_Switch", 0xE149),
    ("East_Switch", 0xE14A),
    ("Yes_Switch", 0xE14B),
    ("No_Switch", 0xE14C),
    
    ("Warning_inverted_Switch", 0xE150),
    ("Error_inverted_Switch", 0xE151),
    ("Tip_inverted_Switch", 0xE152),
];

const ESCAPE_CODES_DS:[(&str, u16);57] = [
    ("Clock_DS", 0xE007),
    ("Happy_face_DS", 0xE008),
    ("Angry_face_DS", 0xE009),
    ("Sad_face_DS", 0xE00A),
    ("Expressionless_face_DS", 0xE00B),
    ("Sun_DS", 0xE00C),
    ("Cloud_DS", 0xE00D),
    ("Umbrella_DS", 0xE00E),
    ("Snowman_DS", 0xE00F),
    ("Exclamation_mark_DS", 0xE010),
    ("Question_mark_DS", 0xE011),
    ("Letter_DS", 0xE012),
    ("Phone_DS", 0xE013),
    ("Calibration_DS", 0xE014),
    ("Spade_suit_DS", 0xE015),
    ("Diamond_suit_DS", 0xE016),
    ("Heart_DS", 0xE017),
    ("Clubs_suit_DS", 0xE018),
    ("Right_DS", 0xE019),
    ("Left_DS", 0xE01A),
    ("Up_DS", 0xE01B),
    ("Down_DS", 0xE01C),
    ("Target_DS", 0xE01D),
    ("Camera_DS", 0xE01E),
    ("Unkown_1_DS", 0xE01F),
    ("Top_left_corner_border_DS", 0xE020),
    ("Top_border_DS", 0xE021),
    ("Top_right_corner_border_DS", 0xE022),
    ("Right_border_DS", 0xE023),
    ("Bottom_right_border_DS", 0xE024),
    ("Bottom_border_DS", 0xE025),
    ("Bottom_left_border_DS", 0xE026),
    ("Left_border_DS", 0xE027),
    ("Cross_DS", 0xE028),
    ("Boxed_A_DS", 0xE029),
    ("Boxed_B_DS", 0xE02A),
    ("Boxed_C_DS", 0xE02B),
    ("Boxed_D_DS", 0xE02C),
    ("Boxed_A_inverted_DS", 0xE02D),
    ("Boxed_M_inverted_DS", 0xE02E),
    ("Unknown_2_DS", 0xE02F),
    ("P_DS", 0xE030),
    ("I_DS", 0xE031),
    ("C_DS", 0xE032),
    ("T_DS", 0xE033),
    ("H_DS", 0xE034),
    ("A_DS", 0xE035),
    ("Unknown_3_DS", 0xE036),
    ("Unknown_4_DS", 0xE037),
    ("Unknown_5_DS", 0xE038),
    ("Unknown_6_DS", 0xE039),
    ("Unknown_7_DS", 0xE03A),
    ("Unknown_8_DS", 0xE03B),
    ("Unknown_9_DS", 0xE03C),
    ("Unknown_10_DS", 0xE03D),
    ("Inverted_boxed_cross_DS", 0xE03E),
    ("Inverted_boxed_cross_big_DS", 0xE03F),
];

impl TXT2{
    pub fn read_from<R: Read + Seek>(buffer: &mut R, order: bytestream::ByteOrder) -> Result<TXT2> {
        println!("Extracting strings...");
        let block_start = buffer.stream_position()?;
        let mut magic = vec![0u8;4];
        buffer.read_exact(&mut magic)?;
        if magic != b"TXT2" {
            return Err(Error::MalformedFile)
        }
        let section_size = u32::read_from(buffer, order)?;
        buffer.seek(SeekFrom::Current(8))?;
        let string_amount = u32::read_from(buffer, order)?;
        let mut offsets = Vec::<u32>::new();
        for _i in 0..string_amount {
            offsets.push(u32::read_from(buffer, order)?);
        }
        let strings = Self::get_strings(buffer, order, offsets.clone(), block_start+0x10)?;
        println!("Extracted strings.");
        Ok(TXT2{
            _magic: magic,
            _section_size: section_size,
            _string_amount: string_amount,
            offsets,
            strings,
        })
    }

    fn get_strings<R:Read + Seek>(buffer: &mut R, order: bytestream::ByteOrder, offsets: Vec<u32>, start_pos: u64) -> Result<Vec<Vec<u8>>> {
        let mut strings = Vec::<Vec<u8>>::new();
        let mut start_offset = offsets[0];
        for offset in offsets{
            if offset != start_offset {
                // println!("{:#x}:{:#x}", start_pos,offset);
                buffer.seek(SeekFrom::Start(start_pos+start_offset as u64))?;
                let mut string = Vec::<u8>::new();
                for _i in 0..(offset-start_offset){
                    string.push(u8::read_from(buffer, order)?);
                }
                strings.push(string);
                start_offset = offset;
            }
        }
        buffer.seek(SeekFrom::Start(start_pos+start_offset as u64))?;
        let mut string = Vec::<u8>::new();
        buffer.read_to_end(&mut string)?;
        let mut padding = string.last().copied().unwrap();
        while padding != 0x00 {
            string.truncate(string.len()-1);
            padding = string.last().copied().unwrap();
        }
        strings.push(string);
        Ok(strings)
    }

    pub fn write_binary(msbt_strings: Vec<MSBTString>, order: bytestream::ByteOrder) -> Result<Vec<u8>> {
        println!("Formatting strings...");
        let mut result = Vec::<u8>::new();
        let mut offsets = Vec::<u32>::new();
        let mut strings = Vec::<Vec<u8>>::new();
        let mut new_strings = msbt_strings.clone();
        new_strings.sort_by(|a, b| a.index.cmp(&b.index));
        //First offset
        let mut last_offset = msbt_strings.len() as u32*4+4;
        let mut section_size = 4_u32; //amount of strings
        offsets.push(last_offset);
        for string in new_strings{
            last_offset += string.string.len() as u32;
            section_size += string.string.len() as u32+4;
            strings.push(string.string);
            offsets.push(last_offset);
        }
        offsets.truncate(offsets.len()-1);
        //binary tiem
        result.append(&mut b"TXT2".to_vec());
        match order {
            ByteOrder::BigEndian => {
                result.append(&mut section_size.to_be_bytes().to_vec());
                result.append(&mut vec![0,0,0,0,0,0,0,0]);
                result.append(&mut (msbt_strings.len() as u32).to_be_bytes().to_vec());
                for offset in offsets{
                    result.append(&mut offset.to_be_bytes().to_vec());
                }
                for string in strings{
                    result.append(&mut string.clone());
                }
            }
            ByteOrder::LittleEndian => {
                result.append(&mut section_size.to_le_bytes().to_vec());
                result.append(&mut vec![0,0,0,0,0,0,0,0]);
                result.append(&mut (msbt_strings.len() as u32).to_le_bytes().to_vec());
                for offset in offsets{
                    result.append(&mut offset.to_le_bytes().to_vec());
                }
                for string in strings{
                    result.append(&mut string.clone());
                }
            }
        }
        let padding = 16 - result.len() %16;
        for _i in 0..padding{
            result.push(0xD0);
        }

        println!("Formated strings.");
        Ok(result)
    }

    // Control code format: [CMD groupe.type raw_as_XX] i.e. \[RawCmd 0.3 E4_00_00_FF] for red colour
    pub fn parse_binary(string: Vec<u8>, order: bytestream::ByteOrder) -> String{
        let mut result = String::new();
        let mut revert_string:VecDeque<u8> = string.into_iter().collect();
        while !revert_string.is_empty() {
            let char_temp = [revert_string.pop_front().unwrap(), revert_string.pop_front().unwrap()];
            let char= read_char(char_temp, order);
            if char == 0x0E{ //Start of control code!
                result.push_str(&convert_control_code_binary(&mut revert_string, order));
            } else if char == 0x0F{ // End of control code!
                result.push_str(&convert_control_code_close_binary(&mut revert_string, order));
            } else {
                result.push_str(&Self::search_escape_code(char));
            }
        }
        result
    }


    fn search_escape_code(char: u16) -> String {
        if char >= 0xE000 {
            let result = ESCAPE_CODES_3DS.into_iter().find(|&x| x.1 == char);
            if result.is_some(){
                return format!("[!{}]",result.unwrap().0);
            }
            let result = ESCAPE_CODES_SWITCH.into_iter().find(|&x| x.1 == char);
            if result.is_some(){
                return format!("[!{}]",result.unwrap().0);
            }
            let result = ESCAPE_CODES_WII.into_iter().find(|&x| x.1 == char);
            if result.is_some(){
                return format!("[!{}]",result.unwrap().0);
            }
            let result = ESCAPE_CODES_DS.into_iter().find(|&x| x.1 == char);
            if result.is_some(){
                return format!("[!{}]",result.unwrap().0);
            }
        }
        std::char::from_u32(char as u32).unwrap().to_string()
    }

    fn convert_escape_code(code: &str, order: bytestream::ByteOrder) -> Vec<u8> {
        let mut bare_code = code.to_string();
        bare_code.pop();
        bare_code.remove(0);
        bare_code.remove(0);
        if bare_code.contains("3DS") {
            let result = ESCAPE_CODES_3DS.into_iter().find(|&x| x.0 == bare_code);
            if let Some(result) = result{
                match order{
                    ByteOrder::BigEndian => return result.1.to_be_bytes().to_vec(),
                    ByteOrder::LittleEndian => return result.1.to_le_bytes().to_vec(),
                }
            }
        }
        if bare_code.contains("Switch") {
            let result = ESCAPE_CODES_SWITCH.into_iter().find(|&x| x.0 == bare_code);
            if let Some(result) = result{
                match order{
                    ByteOrder::BigEndian => return result.1.to_be_bytes().to_vec(),
                    ByteOrder::LittleEndian => return result.1.to_le_bytes().to_vec(),
                }
            }
        }
        if bare_code.contains("Wii") {
            let result = ESCAPE_CODES_WII.into_iter().find(|&x| x.0 == bare_code);
            if let Some(result) = result{
                match order{
                    ByteOrder::BigEndian => return result.1.to_be_bytes().to_vec(),
                    ByteOrder::LittleEndian => return result.1.to_le_bytes().to_vec(),
                }
            }
        }
        if bare_code.contains("DS") {
            let result = ESCAPE_CODES_DS.into_iter().find(|&x| x.0 == bare_code);
            if let Some(result) = result{
                match order{
                    ByteOrder::BigEndian => return result.1.to_be_bytes().to_vec(),
                    ByteOrder::LittleEndian => return result.1.to_le_bytes().to_vec(),
                }
            }
        }
        let mut raw_bytes = Vec::<u8>::new();
        for char in code.chars(){
            raw_bytes.append(&mut convert_char(char, order));
        }
        raw_bytes
    }

    pub fn parse_string(string: &str, order: bytestream::ByteOrder) -> Result<Vec<u8>>{
        // println!("Parsing \"{}\"", string);
        let mut result = Vec::<u8>::new();
        let escape_regex = Regex::new(r"(\[![0-9a-zA-Z_]+\])").unwrap();
        let control_regex = Regex::new(r"(\[[A-Za-z]+ [0-9]{1,2}\.[0-9]{1,2}[ 0-9A-F_]*])").unwrap();
        let control_close_regex = Regex::new(r"(\[\/[A-Za-z]+ [0-9]{1,2}\.[0-9]{1,2}])").unwrap();
        let mut codes = Vec::<(usize, Vec<u8>)>::new();
        for code_match in control_regex.find_iter(string) {
            codes.push((code_match.start(), convert_control_code(code_match.as_str(), order)));
        }
        for code_match in escape_regex.find_iter(string) {
            codes.push((code_match.start(), Self::convert_escape_code(code_match.as_str(), order)));
        }
        for code_match in control_close_regex.find_iter(string) {
            codes.push((code_match.start(), convert_control_code_close(code_match.as_str(), order)));
        }

        codes.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mut pos = 0;
        let mut char_array: VecDeque<char> = VecDeque::from_iter(string.chars());
        for code in codes{
            for _i in pos..code.0{
                result.append(&mut convert_char(char_array.pop_front().unwrap(), order));
            }
            pos = code.0;
            result.append(&mut code.1.clone());
            let mut char = '[';
            while char != ']' {
                char = char_array.pop_front().unwrap();
                pos+=1;
            }
        }
        for _i in 0..char_array.len(){
            result.append(&mut convert_char(char_array.pop_front().unwrap(), order));
        }
        Ok(result)
    }
}

pub fn read_char(char_temp: [u8;2], order: bytestream::ByteOrder) -> u16{
    match order {
        ByteOrder::BigEndian => u16::from_be_bytes(char_temp),
        ByteOrder::LittleEndian => u16::from_le_bytes(char_temp),
    }
}

pub fn convert_char(char: char, order:bytestream::ByteOrder) -> Vec<u8> {
    let mut result = Vec::<u8>::new();
    let mut char_utf16 = [0; 1];
    char.encode_utf16(&mut char_utf16);
    match order{
        ByteOrder::LittleEndian => result.append(&mut char_utf16.into_iter().flat_map(|c| c.to_le_bytes()).collect()),
        ByteOrder::BigEndian => result.append(&mut char_utf16.into_iter().flat_map(|c| c.to_be_bytes()).collect()),
    }
    result
}