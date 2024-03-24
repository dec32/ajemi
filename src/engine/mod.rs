mod long_glyph;
mod sentence;
mod schema;
use std::{cell::OnceCell, collections::HashSet};
use self::schema::Schema;
use self::schema::Candidate::*;
use crate::{conf::CJK_SPACE, CANDI_NUM};

/// Suggestions from engine
#[derive(Default, Clone)]
pub struct Suggestion {
    pub output: String,
    pub groupping: Vec<usize>,
}

/// Engine. A struct to store and query words and punctuators
pub struct Engine {
    schemas: [Schema;2],
    schema_index: usize,
}

impl Engine {
    fn new() -> Engine {
        Engine {
            schemas: [schema::sitelen(), schema::emoji()],
            schema_index: 0
        }
    }

    fn schema(&self) -> &Schema {
        &self.schemas[self.schema_index]
    }

    pub fn next_schema(&mut self) {
        self.schema_index = (self.schema_index + 1) % self.schemas.len()
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
        // suggest a sentence
        if let Some(sugg) = self.suggest_sentence(spelling) {
            suggs.push(sugg);
        }
        // suggest single words
        let mut remains = CANDI_NUM - suggs.len();
        let mut exclude: HashSet<String> = HashSet::new();
        'outer_loop:
        for to in (1..=spelling.len()).rev() {
            let slice = &spelling[0..to];
            let words: Box<dyn Iterator<Item = &String>> = match self.schema().candis.get(slice) {
                Some(Exact(word, words)) => 
                    Box::new(Some(word).into_iter().chain(words.iter())),
                Some(Unique(word)) => 
                    Box::new(Some(word).into_iter()),
                Some(Duplicates(words)) => 
                    Box::new(words.iter()),
                None => {
                    continue;
                }
            };
            for word in words {
                let words: Box<dyn Iterator<Item = &String>> = if let Some(alters) = self.schema().alters.get(word) {
                    Box::new(Some(word).into_iter().chain(alters.iter()))
                } else {
                    Box::new(Some(word).into_iter())
                };
                for word in words {
                    if exclude.contains(word) {
                        continue;
                    }
                    suggs.push(Suggestion{ output: word.clone(), groupping: vec![to] });
                    exclude.insert(word.clone());
                    remains -= 1;
                    if remains <= 0 {
                        break 'outer_loop;
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
