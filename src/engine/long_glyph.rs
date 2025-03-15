use crate::conf;

const ALA: char = '󱤂';
const AWEN: char = '󱤈';
const KEN: char = '󱤘';
const KEPEKEN: char = '󱤙';
const LON: char =  '󱤬';
const PI: char = '󱥍';
const TAWA: char = '󱥩';
const LA: char = '󱤡';
const KAMA: char = '󱤖';


const START_OF_LONG_GLYGH: char = '󱦗';
const END_OF_LONG_GLYPH: char = '󱦘';
const START_OF_REVERSE_LONG_GLYGH: char = '󱦚';
const END_OF_REVERSE_LONG_GLYPH: char = '󱦛';

pub(super) fn insert_long_glyph(text: &mut String) {
    let mut output = String::new();
    let mut open = false;
    let mut general_question = None;
    for ch in text.chars() {
        // insert reverse long glyph for ala and remember the question
        if ext_as_ala(ch) {
            let Some(mut prev) = output.pop() else {
                output.push(ch);
                continue;
            };
            if prev == END_OF_LONG_GLYPH || prev == ALA {
                output.push(prev);
                output.push(ch);
                continue; 
            }
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
                output.push(END_OF_REVERSE_LONG_GLYPH);
                output.push(ch);
            } 
        // to see if ch is being asked. if so, insert long glyph
        } else if general_question.is_some() && ch == general_question.unwrap() {
            general_question = None;
            output.push(START_OF_LONG_GLYGH);
            output.push(ch);
            output.push(END_OF_LONG_GLYPH);
        // no question, insert ch then open long glyph if needed
        } else if ext_left(ch) {
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
        } else if ext_right(ch) {
            if open {
                output.push(ch);
                continue;
            }
            // get the non-underscored part out
            let mut temp = String::new();
            loop {
                let Some(prev) = output.pop() else {
                    break;
                };
                // todo 
                // in theory END_OF_LONG_GLYPH cound suggest the pattern "x ala x"
                // it needs to be handled separately (the long glyph for x ala x will be canceled)
                if ext_right(prev) || prev == END_OF_LONG_GLYPH {
                    output.push(prev);
                    break;
                } else if ext_as_ala(prev) {
                    temp.push(prev);
                    let (a, b, c) = (output.pop(), output.pop(), output.pop());
                    match (a, b, c) {
                        (Some(END_OF_REVERSE_LONG_GLYPH), Some(prev), Some(START_OF_REVERSE_LONG_GLYGH)) => {
                            temp.push(prev)
                        },
                        _ => {
                            c.map(|it|output.push(it));
                            b.map(|it|output.push(it));
                            a.map(|it|output.push(it));
                            break;
                        }
                    }
                } else {
                    temp.push(prev)
                }
            }
            if temp.is_empty() {
                output.push(ch);
                continue;
            }
            output.push(START_OF_REVERSE_LONG_GLYGH);
            loop {
                let Some(t) = temp.pop() else {
                    break;
                };
                output.push(t);
            }
            output.push(END_OF_REVERSE_LONG_GLYPH);
            output.push(ch);
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


fn ext_as_ala(ch: char) -> bool {
    ch == ALA && conf::get().behavior.long_glyph
}


fn ext_left(ch: char) -> bool {
    match ch {
        PI => conf::get().behavior.long_pi,
        AWEN|KEN|KEPEKEN|LON|TAWA => conf::get().behavior.long_glyph,
        _ => false
    }
}

#[allow(unused)]
fn ext_right(ch: char) -> bool {
    match ch {
        // KAMA is disabled for now because i don't want to handle "tenpo kama la"
        LA => conf::get().behavior.long_glyph,
        KAMA => false,
        _ => false
    }
}


