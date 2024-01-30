use crate::extend::StringExt;
use LongGlyph::*;


const ALA: char = '󱤂';     // frequently used and has good font support
const PI: char = '󱥍';      // frequently used and has good font support
const ANU: char = '󱤇';     // lack font support.
const TAWA: char = '󱥩';    // less frequently used
const KEPEKEN: char = '󱤙'; // less frequently used
const AWEN: char = '󱤈';    // less frequently used
const LON: char =  '󱤬';    // less frequently used
const TAN: char = '󱥧';     // less frequently used and lack font support
const KEN: char = '󱤘';     // less frequently used and "ken ala ken" make things a lot harder

const START_OF_LONG_GLYGH: char = '󱦗';
const END_OF_LONG_GLYPH: char = '󱦘';
const START_OF_REVERSE_LONG_GLYGH: char = '󱦚';
const END_OF_LONG_REVERSE_GLYPH: char = '󱦛';

#[allow(unused)]
enum LongGlyph { Leftwards, Ala, Anu }

pub fn process_long_glyph(text: &mut String) {
    let mut long_glyph = None;
    let mut chars = Vec::with_capacity(text.len() * 2);
    let mut index = 0;
    for (i, ch) in text.chars().enumerate() {
        chars.push(ch);
        if long_glyph.is_some() {
            continue;
        }
        let lg = match ch {
            PI => Some(Leftwards),
            ALA => Some(Ala),
            ANU|TAWA|KEPEKEN|TAN|KEN|AWEN|LON => None, // disabled for now
            _ => None
        };
        if lg.is_some() {
            long_glyph = lg;
            index = i;
        }
    }
    if long_glyph.is_none() {
        return;
    }
    match long_glyph.unwrap() {
        Leftwards => {
            if index == chars.len() - 1 {
                return;
            }
            text.clear();
            text.push_chars(&chars[0..=index]);
            text.push(START_OF_LONG_GLYGH);
            text.push_chars(&chars[index + 1..]);
            text.push(END_OF_LONG_GLYPH);
        }
        Anu => {
            if chars.len() <= 1 || index == 0{
                return;
            }
            text.clear();
            text.push_chars(&chars[0..index - 1]);
            if index >= 1 {
                text.push(START_OF_REVERSE_LONG_GLYGH);
                text.push(chars[index - 1]);
                text.push(END_OF_LONG_REVERSE_GLYPH);
                text.push(chars[index]);
                if index < chars.len() - 1 {
                    text.push(START_OF_LONG_GLYGH);
                    text.push(chars[index + 1]);
                    text.push(END_OF_LONG_GLYPH);  
                }
            }
            if index < chars.len() -2 {
                text.push_chars(&chars[index + 2..]);
            }
        }
        Ala => {
            if chars.len() <= 1 || index == 0{
                return;
            }
            text.clear();
            text.push_chars(&chars[0..index - 1]);
            if index >= 1 {
                text.push(START_OF_REVERSE_LONG_GLYGH);
                text.push(chars[index - 1]);
                text.push(END_OF_LONG_REVERSE_GLYPH);
                text.push(chars[index]);
                if index < chars.len() - 1 {
                    if chars[index + 1] == chars[index - 1] {
                        text.push(START_OF_LONG_GLYGH);
                        text.push(chars[index + 1]);
                        text.push(END_OF_LONG_GLYPH);
                    } else {
                        text.push(chars[index + 1]);
                    }
                }
            }
            if index < chars.len() - 2 {
                text.push_chars(&chars[index + 2..]);
            }
        }
    }
}