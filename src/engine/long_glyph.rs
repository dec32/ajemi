const ALA: char = '󱤂';
const AWEN: char = '󱤈';
const KEN: char = '󱤘';
const KEPEKEN: char = '󱤙';
const LON: char =  '󱤬';
const PI: char = '󱥍';
const TAWA: char = '󱥩';

const START_OF_LONG_GLYGH: char = '󱦗';
const END_OF_LONG_GLYPH: char = '󱦘';
const START_OF_REVERSE_LONG_GLYGH: char = '󱦚';
const END_OF_LONG_REVERSE_GLYPH: char = '󱦛';

pub fn process_long_glyph(text: &mut String) {
    let mut output = String::new();
    let mut open = false;
    let mut general_question = None;
    for ch in text.chars() {
        if ch == ALA {
            let Some(mut prev) = output.pop() else {
                output.push(ch);
                continue;
            };
            if prev == START_OF_LONG_GLYGH {
                open = false;
                prev = output.pop().unwrap();
            }
            if open {
                output.push(END_OF_LONG_GLYPH);
            }
            general_question = Some(prev);
            output.push(START_OF_REVERSE_LONG_GLYGH);
            output.push(prev);
            output.push(END_OF_LONG_REVERSE_GLYPH);
            output.push(ch);
        } else {
            if let Some(gp) = general_question {
                if ch == gp {
                    output.push(START_OF_LONG_GLYGH);
                    output.push(ch);
                    output.push(END_OF_LONG_GLYPH);
                } else {
                    general_question = None;
                    output.push(ch);
                }
                continue;
            } else {
                output.push(ch);
                if matches!(ch, AWEN|KEN|KEPEKEN|LON|PI|TAWA) && !open {
                    output.push(START_OF_LONG_GLYGH);
                    open = true;
                }
            }
        }
    }
    if open {
        let prev = output.pop().unwrap();
        if prev != START_OF_LONG_GLYGH {
            output.push(prev);
            output.push(END_OF_LONG_GLYPH);
        }
    }
    text.clear();
    text.push_str(&output);
}