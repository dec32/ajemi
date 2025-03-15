mod long_glyph;
mod sentence;
mod schema;
use std::collections::VecDeque;
use std::path::PathBuf;
use std::{env, fs};
use std::collections::HashSet;
use self::schema::Schema;
use self::schema::Candidate::*;
use crate::global::IME_NAME;
use crate::{Result, EMOJI_DICT, SITELEN_DICT};
use crate::{conf::CJK_SPACE, CANDI_NUM};

/// Suggestions from engine
#[derive(Default, Clone)]
pub struct Suggestion {
    pub output: String,
    pub groupping: Vec<usize>,
}

/// Engine. A struct to store and query words and punctuators
pub struct Engine {
    schemas: VecDeque<Schema>,
    squote_open: bool,
    dquote_open: bool,
}

impl Default for Engine {
    fn default() -> Engine {
        Engine {
            schemas: VecDeque::from([Schema::from(SITELEN_DICT), Schema::from(EMOJI_DICT)]),
            squote_open: false,
            dquote_open: false
        }
    }
}

impl Engine {
    pub fn build() -> Result<Engine> {
        let mut schemas = VecDeque::new();
        let mut default_schema = None;
        let path = PathBuf::from(env::var("APPDATA")?).join(IME_NAME).join("dict");
        fs::create_dir_all(&path)?;
        for entry in fs::read_dir(&path)? {
            let entry = entry?;
            let path = entry.path();
            let file_name = entry.file_name();
            let file_name = file_name.to_string_lossy();
            if path.is_dir() || !file_name.ends_with(".dict") {
                continue;
            }
            let schema = Schema::from(fs::read_to_string(path)?.as_str());
            if file_name == "sitelen.dict" {
                default_schema = Some(schema)
            } else {
                schemas.push_back(schema);
            }
        }
        if let Some(default_schema) = default_schema {
            schemas.push_front(default_schema);
        }
        if schemas.is_empty() {
            log::info!("No dictionary found. Creating default ones now.");
            let sitelen_path = path.as_path().join("sitelen.dict");
            let emoji_path = path.join("emoji.dict");
            fs::write(sitelen_path, SITELEN_DICT)?;
            fs::write(emoji_path, EMOJI_DICT)?;
            return Ok(Engine::default())
        }
        Ok(Engine { schemas, squote_open: false, dquote_open: false })
    }

    pub fn build_or_default() -> Engine {
        match Engine::build() {
            Ok(engine) => engine,
            Err(err) => {
                log::error!("Failed to build engine. Use default for fallback. {err:?}");
                Engine::default()
            }
        }
    }

    fn schema(&self) -> &Schema {
        self.schemas.front().unwrap()
    }

    pub fn next_schema(&mut self) {
        self.schemas.rotate_left(1);
        self.squote_open = false;
        self.dquote_open = false;
    }

    pub fn remap_punct(&mut self, punct: char) -> char {
        match punct {
            '\'' => {
                let remmaped = match self.squote_open {
                    false => self.schema().squote.0,
                    true =>  self.schema().squote.1
                };
                self.squote_open = !self.squote_open;
                remmaped
            }
            '"' => {
                let remmaped = match self.dquote_open {
                    false => self.schema().dquote.0,
                    true =>  self.schema().dquote.1
                };
                self.dquote_open = !self.dquote_open;
                remmaped
            }
            punct => self.schema().puncts
                .get(&punct)
                .cloned()
                .filter(|it| *it != '\u{3000}' || unsafe { !CJK_SPACE } )
                .unwrap_or(punct)
        }
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
        let mut exclude: HashSet<&str> = HashSet::new();
        'outer_loop:
        for to in (1..=spelling.len()).rev() {
            let slice = &spelling[0..to];
            let words: &mut dyn Iterator<Item = &String> = match self.schema().candis.get(slice) {
                Some(Exact(word, words)) => 
                    &mut Some(word).into_iter().chain(words.iter()),
                Some(Unique(word)) => 
                    &mut Some(word).into_iter(),
                Some(Duplicates(words)) => 
                    &mut words.iter(),
                None => {
                    continue;
                }
            };
            for word in words {
                let words: &mut dyn Iterator<Item = &String> = if let Some(alters) = self.schema().alters.get(word) {
                    &mut Some(word).into_iter().chain(alters.iter())
                } else {
                    &mut Some(word).into_iter()
                };
                for word in words {
                    if exclude.contains(word.as_str()) {
                        continue;
                    }
                    exclude.insert(word);
                    // append the trailing joiner(s) to the suggestion
                    let mut output = word.clone();
                    let mut to = to;
                    let bytes = spelling.as_bytes();
                    for i in to..spelling.len() {
                        if let Some(joiner) = char::try_from(bytes[i]).ok().and_then(|char|self.schema().puncts.get(&char)).cloned() {
                            output.push(joiner);
                            to += 1;
                        } else {
                            break;
                        }
                    }
                    suggs.push(Suggestion{ output, groupping: vec![to] });
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


#[test]
fn repl() {
    use std::io::stdin;
    let engine = Engine::build().unwrap();
    let mut buf = String::new();
    loop {
        buf.clear();
        stdin().read_line(&mut buf).unwrap();
        let suggs = engine.suggest(&buf);
        for sugg in suggs {
            println!("{}", sugg.output);
        }
        
    }
}
