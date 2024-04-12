use crate::extend::CharExt;
use super::{long_glyph::insert_long_glyph, schema::Candidate::*, Engine, Suggestion};

#[derive(Default, Clone)]
struct Sentence {
    output: String,
    groupping: Vec<usize>,
    score: usize,
    wc: u8,
}

impl Sentence {
    fn push_unique(&mut self, unique: &str, len: usize) {
        self.push_word(unique, len);
        self.score += len * 20;
    }

    fn push_exact(&mut self, exact: &str, len: usize) {
        self.push_word(exact, len);
        self.score += len * match len {
            1 => 10, // a, e and n can be very annoying
            2 => 29, // a unique prefix of length 3 is favored over an exact match of length 2 (so pim > pi'm)
            _ => 30, // use a 3 : 2 ratio by default
        };
    }

    fn push_word(&mut self, word: &str, len: usize) {
        if self.output.chars().last().map(|char|char.is_joiner()).unwrap_or(false) {
            *self.groupping.last_mut().unwrap() += len;
        } else {
            self.groupping.push(self.groupping.last().cloned().unwrap_or(0) + len);
        }
        self.output.push_str(word);
        self.wc += 1;
    }

    fn push_joiner(&mut self, joiner: char) {
        self.output.push(joiner);
        if let Some(last) = self.groupping.last_mut() {
            *last += 1;
        } else {
            self.groupping.push(1)
        }
    }
}
  
#[allow(unused)]
impl Engine {
    pub(super) fn suggest_sentence(&self, spelling: &str) -> Option<Suggestion>{
        let mut sents = self.suggest_sentences(spelling);
        let mut best_sent = None;
        let mut highest_score = 0;
        while !sents.is_empty() {
            let sent = sents.pop().unwrap();
            if sent.wc <= 1 {
                continue;
            }
            if sent.score > highest_score {
                highest_score = sent.score;
                best_sent = Some(sent);
            }
        }
        let Some(mut best_sent) = best_sent else {
            return None;
        };
        insert_long_glyph(&mut best_sent.output);
        Some(Suggestion{output:best_sent.output, groupping: best_sent.groupping})
    }
    
    fn suggest_sentences(&self, spelling: &str) -> Vec<Sentence> {
        let mut sent = Sentence::default();
        let mut sents = Vec::new();
        self.suggest_sentences_recursive(spelling, &mut sent, &mut sents);
        sents.push(sent);
        sents
    }

    fn suggest_sentences_recursive(
        &self, 
        spelling: &str, 
        sent: &mut Sentence, 
        sents: &mut Vec<Sentence>
    ) 
    {
        // push leading joiners into the sentence directly
        let mut spelling = spelling;
        for (i, byte) in spelling.as_bytes().iter().cloned().enumerate() {
            if let Some(joiner) = char::try_from(spelling.as_bytes()[i]).ok().and_then(|char|self.schema().puncts.get(&char)).cloned() {
                sent.push_joiner(joiner);
                continue;
            } else {
                spelling = &spelling[i..];
                break;
            }
        }
        // find the longest exact match and the longest unique match
        // however if the exact one is longer than the unique one, ignore the unique one.
        let mut exact = None;
        let mut exact_len = 0;
        let mut unique = None;
        let mut unique_len = 0;

        let mut found_unique = false;
        for len in (1..=spelling.len()).rev() {
            match self.schema().candis.get(&spelling[..len]) {
                Some(Exact(word, _)) => {
                    exact = Some(word.as_str());
                    exact_len = len;
                    break;
                }
                Some(Unique(word)) => {
                    if found_unique {
                        continue;
                    }
                    found_unique = true;
                    unique = Some(word.as_str());
                    unique_len = len;
                }
                _ => ()
            }
        }
        // clone if needed
        let mut extra_sent = if exact.is_some() && unique.is_some() {
            Some(sent.clone())
        } else {
            None
        };
        if let Some(exact) = exact {
            sent.push_exact(exact, exact_len);
            self.suggest_sentences_recursive(&spelling[exact_len..], sent, sents)
        }
        if let Some(unique) = unique {
            let sent = if extra_sent.is_some() {
                extra_sent.as_mut().unwrap()
            } else {
                sent
            };
            sent.push_unique(unique, unique_len);
            self.suggest_sentences_recursive(&spelling[unique_len..], sent, sents)
        }
        if let Some(extra_sent) = extra_sent {
            sents.push(extra_sent);
        }
    }
}

#[test]
fn repl() {
    use std::io::stdin;
    use super::{setup, engine};
    setup();
    let mut buf = String::new();
    loop {
        buf.clear();
        stdin().read_line(&mut buf).unwrap();
        let sugg = engine().suggest_sentence(&buf);
        if let Some(sugg) = sugg {
            println!("{}", sugg.output)
        } else {
            println!("No sentence")
        }
    }
}
#[test]
fn test() {
    use super::{setup, engine};
    fn assert_sent(spelling: &str, expected: &str) {
        let sent = engine().suggest_sentence(spelling).unwrap().output;
        let mut buf =  String::new();
        for word in expected.split(' ') {
            buf.push_str(&engine().suggest(word)[0].output)
        }
        assert_eq!(sent, buf)
    }
    setup();
    assert_sent("lilonsewi", "li lon sewi");
    assert_sent("pimaka", "pi ma");
    assert_sent("pimkule", "pimeja kule");
}