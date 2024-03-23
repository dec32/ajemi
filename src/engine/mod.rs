mod long_glyph;
mod sentence;
mod schema;
use std::{cell::OnceCell, collections::HashSet};
use Candidate::*;

use crate::{conf::CJK_SPACE, CANDI_NUM};

use self::schema::Schema;

/// To expain why a certain spelling is mapped to certain word(s)
enum Candidate {
    /// The spelling is an exact spelling of a certain word.
    /// Meanwhile it can also be a prefix of other words.
    /// For example, `"li"` is `Exact("li", ["lili", "linja", "lipu"])`.
    Exact(String, Vec<String>),
    /// The spelling is unique prefix for a certain word. No other words starts with it.
    /// For example, `"kije"` is `Unique("kijetesantakalu")`.
    Unique(String),
    /// The spelling is not an exact spelling or a unique prefix.
    /// For example, `"an"` is `Duplicates(["anpa", "ante", "anu"])`.
    Duplicates(Vec<String>)
}

/// Suggestions from engine
#[derive(Default, Clone)]
pub struct Suggestion {
    pub output: String,
    pub groupping: Vec<usize>,
}

/// Engine. A struct to store and query words and punctuators
pub struct Engine {
    schemas: [Schema;2],
    index: usize,
}

impl Engine {
    fn new() -> Engine {
        Engine {
            schemas: [schema::sitelen(), schema::emoji()],
            index: 0
        }
    }

    fn schema(&self) -> &Schema {
        &self.schemas[self.index]
    }

    pub fn next_schema(&mut self) {
        self.index = (self.index + 1) % self.schemas.len()
    }

    pub fn remap_punct(&self, punct: char) -> char {
        self.schema().puncts
            .get(&punct)
            .cloned()
            .filter(|it| *it != '\u{3000}' || unsafe { !CJK_SPACE } )
            .unwrap_or(punct)
    }

    pub fn suggest(&self, spelling: &str) -> Vec<Suggestion> {
        if !spelling.is_ascii() {
            return Vec::new(); 
        }
        let mut suggs = Vec::with_capacity(CANDI_NUM);
        // Suggest a sentence
        if let Some(sugg) = self.suggest_sentence(spelling) {
            suggs.push(sugg);
        }
        // suggest single words
        let mut remains = CANDI_NUM - suggs.len();
        let mut exclude: HashSet<String> = HashSet::new();
        'outer_loop:
        for to in (1..=spelling.len()).rev() {
            let slice = &spelling[0..to];
            let empty_vec = Vec::new();
            let (word, words) = match self.schema().candis.get(slice) {
                Some(Exact(word, words)) => 
                    (Some(word), words),
                Some(Unique(word)) => 
                    (Some(word), &empty_vec),
                Some(Duplicates(words)) => 
                    (None, words),
                None => {
                    continue;
                }
            };
            for w in word.into_iter().chain(words) {
                if exclude.contains(w) {
                    continue;
                }
                suggs.push(Suggestion{ output: w.clone(), groupping: vec![to] });
                exclude.insert(w.clone());
                remains -= 1;
                if remains <= 0 {
                    break 'outer_loop;
                }


                // alternative glyphs. this iteration part is like hell
                if let Some(alters) = self.schema().alters.get(w) {
                    for w in alters {
                        if exclude.contains(w) {
                            continue;
                        }
                        suggs.push(Suggestion{ output: w.clone(), groupping: vec![to] });
                        exclude.insert(w.clone());
                        remains -= 1;
                        if remains <= 0 {
                            break 'outer_loop;
                        }
                    }
                }
            }
        }
        suggs
    }
}


//----------------------------------------------------------------------------
//
//  Static section.
//
//----------------------------------------------------------------------------

static mut ENGINE: OnceCell<Engine> = OnceCell::new();

pub fn engine() -> &'static mut Engine {
    unsafe {ENGINE.get_mut().unwrap()}
}

pub fn setup() {
    unsafe { 
        ENGINE.get_or_init(Engine::new);
    };
}

#[test]
fn repl() {
    use std::io::stdin;
    setup();
    let mut buf = String::new();
    loop {
        buf.clear();
        stdin().read_line(&mut buf).unwrap();
        let suggs = engine().suggest(&buf);
        for sugg in suggs {
            println!("{}", sugg.output);
        }
        
    }
}
