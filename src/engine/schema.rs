use std::collections::HashMap;

use Candidate::*;
use log::error;

/// To expain why a certain spelling is mapped to certain word(s)
#[derive(Debug)]
pub enum Candidate {
    /// The spelling is an exact spelling of a certain word.
    /// Meanwhile it can also be a prefix of other words.
    /// For example, `"li"` is `Exact("li", ["lili", "linja", "lipu"])`.
    Exact(String, Vec<String>),
    /// The spelling is unique prefix for a certain word. No other words starts with it.
    /// For example, `"kije"` is `Unique("kijetesantakalu")`.
    Unique(String),
    /// The spelling is not an exact spelling or a unique prefix.
    /// For example, `"an"` is `Duplicates(["anpa", "ante", "anu"])`.
    Duplicates(Vec<String>),
}

/// Stores the dictionary and the remapped punctuators.
/// The dicitonary is indexed in a special way.
#[derive(Debug)]
pub struct Schema {
    pub candis: HashMap<String, Candidate>,
    pub alters: HashMap<String, Vec<String>>,
    pub puncts: HashMap<char, char>,
    pub squote: (char, char),
    pub dquote: (char, char),
}

//----------------------------------------------------------------------------
//
//  Load schemas from files.
//
//----------------------------------------------------------------------------

#[derive(Clone, Copy)]
enum Atom<'a> {
    Text(&'a str),
    Punct(char),
}

impl<'a> From<&'a str> for Atom<'a> {
    fn from(str: &'a str) -> Atom<'a> {
        use Atom::*;
        let mut chars = str.chars();
        let first_ch = chars.nth(0).unwrap();
        if !first_ch.is_alphanumeric() && chars.nth(1).is_none() {
            Punct(first_ch)
        } else if first_ch == '#' {
            match u32::from_str_radix(&str[1..], 16)
                .ok()
                .and_then(char::from_u32)
            {
                Some(punct) => Punct(punct),
                None => Text(str),
            }
        } else if str == "space" {
            Punct(' ')
        } else {
            Text(str)
        }
    }
}

impl<'a> ToString for Atom<'a> {
    fn to_string(&self) -> String {
        use Atom::*;
        match self {
            Text(text) => text.to_string(),
            Punct(punct) => {
                let mut string = String::with_capacity(4);
                string.push(*punct);
                string
            }
        }
    }
}

impl From<&str> for Schema {
    fn from(value: &str) -> Schema {
        use Atom::*;
        let mut candis = HashMap::new();
        let mut alters = HashMap::new();
        let mut puncts = HashMap::new();
        let mut squote = ('\'', '\'');
        let mut dquote = ('"', '"');

        let mut atoms = Vec::new();
        for list in value.lines() {
            if list.is_empty() || list.starts_with("//") {
                continue;
            }
            atoms.clear();
            atoms.extend(
                list.split(char::is_whitespace)
                    .filter(|str| !str.is_empty())
                    .map(Atom::from),
            );
            match atoms[..] {
                [Punct('\''), Punct(open), Punct(close)] => {
                    squote = (open, close);
                }
                [Punct('"'), Punct(open), Punct(close)] => {
                    dquote = (open, close);
                }
                [Punct('\''), Punct(dumb)] => {
                    squote = (dumb, dumb);
                }
                [Punct('"'), Punct(dumb)] => {
                    dquote = (dumb, dumb);
                }
                [Punct(punct), Punct(remapped)] => {
                    puncts.insert(punct, remapped);
                }
                [Text(spelling), word, ..] => {
                    // store exact spelling -> word
                    candis.insert(spelling.to_string(), Exact(word.to_string(), Vec::new()));
                    // store prefixes -> word
                    for len in 1..spelling.len() {
                        let prefix = &spelling[0..len];
                        match candis.get_mut(prefix) {
                            None => {
                                candis.insert(prefix.to_string(), Unique(word.to_string()));
                            }
                            Some(Unique(unique)) => {
                                let mut duplicates = Vec::new();
                                duplicates.push(unique.clone());
                                duplicates.push(word.to_string());
                                candis.insert(prefix.to_string(), Duplicates(duplicates));
                            }
                            Some(Duplicates(duplicates)) | Some(Exact(_, duplicates)) => {
                                duplicates.push(word.to_string());
                            }
                        }
                    }
                    // store word -> alternatives
                    let word = word.to_string();
                    for alter in atoms.iter().skip(2) {
                        match alters.get_mut(&word) {
                            None => {
                                alters.insert(word.clone(), vec![alter.to_string()]);
                            }
                            Some(alters) => {
                                alters.push(alter.to_string());
                            }
                        }
                    }
                }
                _ => {
                    error!("Unrecogniable pattern: {list}");
                }
            }
        }
        Schema {
            candis,
            alters,
            puncts,
            squote,
            dquote,
        }
    }
}

#[test]
fn test() {
    test_schema(crate::SITELEN_DICT);
    test_schema(crate::EMOJI_DICT);
}

#[allow(unused)]
fn test_schema(str: &str) {
    let start = std::time::Instant::now();
    let schema = Schema::from(str);
    let elapsed = std::time::Instant::now() - start;
    println!("Elapsed: {:?}", elapsed);
    println!("{:?}", schema);
    println!()
}
