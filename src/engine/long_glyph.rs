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

pub(super) fn insert_long_glyph(text: &mut String) {
    let mut output = String::new();
    let mut open = false;
    let mut general_question = None;
    for ch in text.chars() {
        // insert reverse long glyph for ala and remember the question
        if ch == ALA {
            let Some(mut prev) = output.pop() else {
                output.push(ch);
                continue;
            };
            // ala only closes long glyphs that just begin to form structure like "ken ala ken"
            if prev == START_OF_LONG_GLYGH {
                open = false;
                prev = output.pop().unwrap();
            }
            if open {
                output.push(prev);
                output.push(ch);
            } else {
                general_question = Some(prev);
                output.push(START_OF_REVERSE_LONG_GLYGH);
                output.push(prev);
                output.push(END_OF_LONG_REVERSE_GLYPH);
                output.push(ch);
            } 
        // to see if ch is being asked. if so, insert long glyph
        } else if let Some(gp) = general_question {
            general_question = None;
            if ch == gp {
                output.push(START_OF_LONG_GLYGH);
                output.push(ch);
                output.push(END_OF_LONG_GLYPH);
            } else {
                // in this case ala is used for denying but not questioning
                output.push(ch);
            }
        // no question, insert ch then open long glyph if needed
        } else if matches!(ch, AWEN|KEN|KEPEKEN|LON|PI|TAWA) {
            // close previous long glyph if needed
            if open {
                let prev = output.pop().unwrap();
                if prev != START_OF_LONG_GLYGH {
                    output.push(prev);
                    output.push(END_OF_LONG_GLYPH);
                }
            }
            output.push(ch);
            output.push(START_OF_LONG_GLYGH);
            open = true;
        } else {
            output.push(ch);
        }
    }
    if open {
        let prev = output.pop().unwrap();
        if prev != START_OF_LONG_GLYGH {
            output.push(prev);
            output.push(END_OF_LONG_GLYPH);
        }
    }
    if text.len() != output.len() {
        text.clear();
        text.push_str(&output);
    }
}