use std::{collections::HashMap, fmt::Display};

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
    /// non-alphanumeric character
    Nanch(char),
    /// everthing else
    Text(&'a str),
}

impl<'a> From<&'a str> for Atom<'a> {
    fn from(str: &'a str) -> Atom<'a> {
        use Atom::*;
        let mut chars = str.chars();
        let first_ch = chars.next().unwrap();
        if !first_ch.is_alphanumeric() && chars.next().is_none() {
            Nanch(first_ch)
        } else if str.starts_with("U+") {
            match u32::from_str_radix(&str[2..], 16)
                .ok()
                .and_then(char::from_u32)
            {
                Some(nanch) => Nanch(nanch),
                None => Text(str),
            }
        } else {
            Text(str)
        }
    }
}

impl Display for Atom<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Atom::*;
        match self {
            Text(text) => write!(f, "{text}"),
            Nanch(punct) => write!(f, "{punct}"),
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
            if list.is_empty() || list.starts_with("#") {
                continue;
            }
            atoms.clear();
            atoms.extend(list.split_whitespace().map(Atom::from));
            match atoms[..] {
                [Nanch('\''), Nanch(open), Nanch(close)] => {
                    squote = (open, close);
                }
                [Nanch('"'), Nanch(open), Nanch(close)] => {
                    dquote = (open, close);
                }
                [Nanch('\''), Nanch(dumb)] => {
                    squote = (dumb, dumb);
                }
                [Nanch('"'), Nanch(dumb)] => {
                    dquote = (dumb, dumb);
                }
                [Nanch(punct), Nanch(remapped)] => {
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
                                let duplicates = vec![unique.clone(), word.to_string()];
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
