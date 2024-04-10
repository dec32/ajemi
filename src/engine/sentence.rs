use super::{long_glyph::insert_long_glyph, schema::Candidate::*, Engine, Suggestion};

#[derive(Default, Clone)]
struct Sentence {
    output: String,
    groupping: Vec<usize>,
    score: usize,
    joining: bool,
    wc: usize,
}

impl Sentence {
    fn push_unique(&mut self, unique: &str, to: usize) {
        let len = to - self.groupping.last().unwrap_or(&0);
        self.output.push_str(unique);
        if self.joining {
            *self.groupping.last_mut().unwrap() = to;
            self.joining = false;
        } else {
            self.groupping.push(to);
        }
        self.score += len * 20;
        self.wc += 1;
    }

    fn push_exact(&mut self, exact: &str, to: usize) {
        let len = to - self.groupping.last().unwrap_or(&0);
        self.output.push_str(exact);
        if self.joining {
            *self.groupping.last_mut().unwrap() = to;
            self.joining = false;
        } else {
            self.groupping.push(to);
        }
        self.score += len * match len {
            1 => 10, // a, e and n can be very annoying
            2 => 29, // a unique prefix of length 3 is favored over an exact match of length 2 (so pim > pi'm)
            _ => 30, // use a 3 : 2 ratio by default
        };
        self.wc += 1;
    }

    fn push_joiner(&mut self, joiner: char) {
        self.output.push(joiner);
        self.joining = true;
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
        self.suggest_sentences_recursive(spelling, 0, &mut sent, &mut sents);
        sents.push(sent);
        sents
    }

    fn suggest_sentences_recursive(
        &self, 
        spelling: &str, 
        from: usize, 
        sent: &mut Sentence, 
        sents: &mut Vec<Sentence>
    ) 
    {
        if from >= spelling.len() {
            return;
        }
        // if any symbol (to be specific, joiners) appears, simple append it to the sentence
        if let Some(joiner) = char::try_from(spelling.as_bytes()[from]).ok().and_then(|char|self.schema().puncts.get(&char)).cloned() {
            sent.push_joiner(joiner);
            self.suggest_sentences_recursive(spelling, from + 1, sent, sents)
        }
        // find the longest exact match and the longest unique match
        // however if the exact one is longer than the unique one, ignore the unique one.
        let mut exact = None;
        let mut exact_to = 0;
        let mut unique = None;
        let mut unique_to = 0;

        let mut found_unique = false;
        for to in (from+1..=spelling.len()).rev() {
            match self.schema().candis.get(&spelling[from..to]) {
                Some(Exact(word, _)) => {
                    exact = Some(word.as_str());
                    exact_to = to;
                    break;
                }
                Some(Unique(word)) => {
                    if found_unique {
                        continue;
                    }
                    found_unique = true;
                    unique = Some(word.as_str());
                    unique_to = to;
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
            sent.push_exact(exact, exact_to);
            self.suggest_sentences_recursive(spelling, exact_to, sent, sents)
        }
        if let Some(unique) = unique {
            let sent = if extra_sent.is_some() {
                extra_sent.as_mut().unwrap()
            } else {
                sent
            };
            sent.push_unique(unique, unique_to);
            self.suggest_sentences_recursive(spelling, unique_to, sent, sents)
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