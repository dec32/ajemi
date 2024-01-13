use std::{collections::HashMap, ffi::OsStr};

use crate::extend::OsStrExt2;
use Suggestion::*;

type Chars = Vec<u16>;
pub struct Dict {
    // FIXME: the map can be extremly big, a string pool may help
    suggestions: HashMap<Chars, Suggestion>
}

#[derive(Debug)]
enum Suggestion {
    Exact(Chars),
    Unique(Chars),
    Duplicate,
}

impl Dict {
    pub fn new() -> Dict {
        Dict{suggestions: HashMap::new()}
    }
    pub fn insert(&mut self, spelling: &str, word: &str) {
        let spelling = OsStr::new(spelling).wchars();
        let word = OsStr::new(word).wchars();

        self.suggestions.insert(spelling.clone(), Exact(word.clone()));
        for len in 1..spelling.len() {
            let prefix = &spelling[0..len];
            match self.suggestions.get(prefix) {
                None => 
                    self.suggestions.insert(prefix.to_vec(), Unique(word.clone())),
                Some(Unique(_)) | Some(Duplicate) => 
                    self.suggestions.insert(prefix.to_vec(), Duplicate),
                Some(Exact(_)) 
                    => None,
            };
        }
    }

    pub fn suggest(&self, letters: &Chars) -> Chars {
        let mut from = 0;
        let mut to = letters.len();
        let mut result = Chars::new();

        // 0. assume the letters are milukinekijenasalol
        let mut only_exact = true;
        loop {
            let slice = &letters[from..to];
            let suggestion = self.suggestions.get(slice);
            match (suggestion, only_exact) {
                // 1. find all continuous exact matches
                // [mi][lukin][e]kijenasalol

                // 4. back to finding exact matches
                // [mi][lukin][e](kije)[nasa]lol
                (Some(Exact(word)), true) => {
                    result.extend_from_slice(word);
                    from = to;
                    to = letters.len();
                },
                // 3. try find **one** unique prefix
                // [mi][lukin][e](kije)nasalol
                (Some(Unique(word)), false) => {
                    result.extend_from_slice(word);
                    from = to;
                    to = letters.len();
                    only_exact = true;
                },
                _ => {
                    to -= 1;
                }
            }
            if from >= to {
                // 2. failed to find next exact match
                // 5. failed to find next exact match
                if only_exact {
                    to = letters.len();
                    only_exact = false
                }
                // 6. failed to find even one unique prefix
                else {
                    break
                }
            }
        }
        result
    }
}


pub fn suggest(letters: &[u16]) -> Vec<u16> {
    if letters == OsStr::new("ike").wchars() {
        OsStr::new("㭗").wchars()
    } else {
        Vec::with_capacity(0)
    }
}

#[test]
pub fn test() {
    let mut dict = Dict::new();
    dict.insert("mi", "我");
    dict.insert("lukin", "看");
    dict.insert("e", "兮");
    dict.insert("kijetesantakalu", "狸");
    dict.insert("nasa", "怪");
    dict.insert("lape", "眠");
    for (letters, suggestion) in &dict.suggestions {
        let letters = String::from_utf16_lossy(&letters);
        let suggestion = match suggestion {
            Exact(word) => format!("Exact({})", String::from_utf16_lossy(&word)),
            Unique(word) => format!("Unique({})", String::from_utf16_lossy(&word)),
            Duplicate => String::from("Unique")
        };
        println!("{letters}={suggestion}");
    }

    let suggestion = dict.suggest(&OsStr::new("milukinekijenasalol").wchars());
    let suggestion = String::from_utf16_lossy(&suggestion);
    println!("{suggestion}")
    
}