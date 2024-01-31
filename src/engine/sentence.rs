use super::{Engine, Suggestion, Candidate::*};

#[derive(Default, Clone)]
struct Sentence {
    output: String,
    groupping: Vec<usize>,
    exact_len: usize,
    unique_len: usize,
}

#[allow(unused)]
impl Engine {
    pub(super) fn suggest_sentence(&self, spelling: &str) -> Option<Suggestion>{
        let mut sents = self.suggest_sentences(spelling);
        let mut best_sent = None;
        let mut max_weight = 0.0;
        while !sents.is_empty() {
            let sent = sents.pop().unwrap();
            if sent.unique_len == 0 {
                best_sent = Some(sent);
                break;
            }
            let weight = sent.exact_len as f64 / sent.unique_len as f64;
            if weight > max_weight {
                max_weight = weight;
                best_sent = Some(sent);
            }
        }
        best_sent
            .filter(|s|!s.output.is_empty())
            .map(|s|Suggestion{output:s.output, groupping: s.groupping})
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
        // find the longest exact match and the longest unique match
        // however if the exact one is longer than the unique one, neglect latter.
        let mut exact = None;
        let mut exact_to = 0;
        let mut unique = None;
        let mut unique_to = 0;

        let mut found_unique = false;
        for to in (from+1..=spelling.len()).rev() {
            match self.candidates.get(&spelling[from..to]) {
                Some(Exact(word, _)) => {
                    exact = Some(word);
                    exact_to = to;
                    break;
                }
                Some(Unique(word)) => {
                    if found_unique {
                        continue;
                    }
                    found_unique = true;
                    unique = Some(word);
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
            sent.output.push_str(&exact);
            sent.groupping.push(exact_to);
            sent.exact_len += exact_to - from;
            self.suggest_sentences_recursive(spelling, exact_to, sent, sents)
        }
        if let Some(unique) = unique {
            let sent = if extra_sent.is_some() {
                extra_sent.as_mut().unwrap()
            } else {
                sent
            };
            sent.output.push_str(&unique);
            sent.groupping.push(unique_to);
            sent.unique_len += unique_to - from;
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
        let sents = engine().suggest_sentence(&buf);
        for sent in sents.iter() {
            println!("{}", sent.output)
        }
    }
}