mod long_glyph;
use std::{collections::{HashMap, HashSet}, cell::OnceCell};
use Candidate::*;

use crate::{engine::long_glyph::process_long_glyph, CANDIDATE_NUM};

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

// Suggestion containing extra infos for sentence predicting
#[derive(Default, Clone)]
struct Sentence {
    output: String,
    groupping: Vec<usize>,
    exact_count: u8,
    unique_count: u8
}

/// Engine. A struct to store and query words and punctuators
#[derive(Default)]
pub struct Engine {
    // todo use SmallString
    candidates: HashMap<String, Candidate>,
    puncts: HashMap<char, char>,
}

impl Engine {
    fn new() -> Engine{
        Default::default()
    }
    fn load_dict(&mut self, entries: Vec<(&str, &str)>) {
        let mut candidates = HashMap::new();
        for (spelling, word) in entries {
            // store exact spellings -> words
            candidates.insert(spelling.to_string(), Exact(word.to_string(), Vec::new()));
            // store prefixes -> words
            for len in 1..spelling.len() {
                let prefix = &spelling[0..len];
                match candidates.get_mut(prefix) {
                    None => {
                        candidates.insert(prefix.to_string(), Unique(word.to_string()));
                    },
                    Some(Unique(unique)) => {
                        let mut duplicates = Vec::new();
                        duplicates.push(unique.clone());
                        duplicates.push(word.to_string());
                        candidates.insert(prefix.to_string(), Duplicates(duplicates));
                    },
                    Some(Duplicates(duplicates)) | Some(Exact(_, duplicates)) => {
                        duplicates.push(word.to_string());
                    }
                }
            }
        }
        self.candidates = candidates;
    }

    fn insert_punt(&mut self, punct: char, remapped: char) {
        self.puncts.insert(punct, remapped);
    }

    pub fn remap_punct(&self, punct: char) -> char {
        self.puncts.get(&punct).cloned().unwrap_or(punct)
    }

    pub fn suggest(&self, spelling: &str) -> Vec<Suggestion> {
        if !spelling.is_ascii() {
            return Vec::new(); 
        }
        let mut suggs = Vec::with_capacity(CANDIDATE_NUM);
        // Suggest a sentence
        let mut sugg = Suggestion::default();
        let mut from = 0;
        let mut to = spelling.len();
        while from < to {
            let slice = &spelling[from..to];
            let candiate = self.candidates.get(slice);
            match candiate {
                Some(Exact(word, _)) | Some(Unique(word)) => {
                    sugg.groupping.push(to);
                    sugg.output.push_str(word);
                    from = to;
                    to = spelling.len();
                },
                _ => {
                    to -= 1;
                }
            }
        }
        if !sugg.output.is_empty() {
            process_long_glyph(&mut sugg.output);
            suggs.push(sugg);
        }
        // suggest single words
        let mut remains = CANDIDATE_NUM - suggs.len();
        let mut exclude: HashSet<String> = suggs.iter()
            .filter(|it|it.output.chars().count() == 1)
            .map(|it|it.output.clone())
            .collect();
        'outer_loop:
        for to in (1..spelling.len()).rev() {
            let slice = &spelling[0..to];
            let empty_vec = Vec::new();
            let (word, words) = match self.candidates.get(slice) {
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
            }
        }
        suggs
    }
}


// something experimental
impl Engine {
    #[allow(dead_code)]
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
            sent.exact_count += 1;
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
            sent.unique_count += 1;
            self.suggest_sentences_recursive(spelling, unique_to, sent, sents)
        }
        if let Some(extra_sent) = extra_sent {
            sents.push(extra_sent);
        }
    }
}


//----------------------------------------------------------------------------
//
//  Static section.
//
//----------------------------------------------------------------------------

static mut ENGINE: OnceCell<Engine> = OnceCell::new();

pub fn engine() -> &'static Engine {
    // todo the returned reference is mutable one
    unsafe {ENGINE.get().unwrap()}
}

pub fn setup() {
    let engine = unsafe { 
        ENGINE.get_or_init(Engine::new);
        ENGINE.get_mut().unwrap()
    };
    engine.insert_punt('[', '󱦐');
    engine.insert_punt(']', '󱦑');
    engine.insert_punt('^', '󱦕');
    engine.insert_punt('*', '󱦖');
    engine.insert_punt('(', '󱦗');
    engine.insert_punt(')', '󱦘');
    engine.insert_punt('{', '󱦚');
    engine.insert_punt('}', '󱦛');
    engine.insert_punt('.', '󱦜');
    engine.insert_punt(':', '󱦝');
    engine.insert_punt('<', '「');
    engine.insert_punt('>', '」');
    engine.insert_punt('-', '\u{200D}'); // zero-width joiner

    engine.load_dict(vec![
        ("a", "󱤀"),      
        ("akesi", "󱤁"),  
        ("ala", "󱤂"),    
        ("alasa", "󱤃"),  
        ("ale", "󱤄"),    
        ("anpa", "󱤅"),   
        ("ante", "󱤆"),   
        ("anu", "󱤇"),    
        ("awen", "󱤈"),   
        ("e", "󱤉"),      
        ("en", "󱤊"),     
        ("esun", "󱤋"),   
        ("ijo", "󱤌"),    
        ("ike", "󱤍"),    
        ("ilo", "󱤎"),    
        ("insa", "󱤏"),   
        ("jaki", "󱤐"),   
        ("jan", "󱤑"),    
        ("jelo", "󱤒"),   
        ("jo", "󱤓"),     
        ("kala", "󱤔"),   
        ("kalama", "󱤕"), 
        ("kama", "󱤖"),   
        ("kasi", "󱤗"),   
        ("ken", "󱤘"),    
        ("kepeken", "󱤙"),
        ("kili", "󱤚"),   
        ("kiwen", "󱤛"),  
        ("ko", "󱤜"),
        ("kon", "󱤝"),
        ("kule", "󱤞"),
        ("kulupu", "󱤟"),
        ("kute", "󱤠"),
        ("la", "󱤡"),
        ("lape", "󱤢"),
        ("laso", "󱤣"),
        ("lawa", "󱤤"),
        ("len", "󱤥"),
        ("lete", "󱤦"),
        ("li", "󱤧"),
        ("lili", "󱤨"),
        ("linja", "󱤩"),
        ("lipu", "󱤪"),
        ("loje", "󱤫"),
        ("lon", "󱤬"),
        ("luka", "󱤭"),
        ("lukin", "󱤮"),
        ("lupa", "󱤯"),
        ("ma", "󱤰"),
        ("mama", "󱤱"),
        ("mani", "󱤲"),
        ("meli", "󱤳"),
        ("mi", "󱤴"),
        ("mije", "󱤵"),
        ("moku", "󱤶"),
        ("moli", "󱤷"),
        ("monsi", "󱤸"),
        ("mu", "󱤹"),
        ("mun", "󱤺"),
        ("musi", "󱤻"),
        ("mute", "󱤼"),
        ("nanpa", "󱤽"),
        ("nasa", "󱤾"),
        ("nasin", "󱤿"),
        ("nena", "󱥀"),
        ("ni", "󱥁"),
        ("nimi", "󱥂"),
        ("noka", "󱥃"),
        ("o", "󱥄"),
        ("olin", "󱥅"),
        ("ona", "󱥆"),
        ("open", "󱥇"),
        ("pakala", "󱥈"),
        ("pali", "󱥉"),
        ("palisa", "󱥊"),
        ("pan", "󱥋"),
        ("pana", "󱥌"),
        ("pi", "󱥍"),
        ("pilin", "󱥎"),
        ("pimeja", "󱥏"),
        ("pini", "󱥐"),
        ("pipi", "󱥑"),
        ("poka", "󱥒"),
        ("poki", "󱥓"),
        ("pona", "󱥔"),
        ("pu", "󱥕"),
        ("sama", "󱥖"),
        ("seli", "󱥗"),
        ("selo", "󱥘"),
        ("seme", "󱥙"),
        ("sewi", "󱥚"),
        ("sijelo", "󱥛"),
        ("sike", "󱥜"),
        ("sin", "󱥝"),
        ("sina", "󱥞"),
        ("sinpin", "󱥟"),
        ("sitelen", "󱥠"),
        ("sona", "󱥡"),
        ("soweli", "󱥢"),
        ("suli", "󱥣"),
        ("suno", "󱥤"),
        ("supa", "󱥥"),
        ("suwi", "󱥦"),
        ("tan", "󱥧"),
        ("taso", "󱥨"),
        ("tawa", "󱥩"),
        ("telo", "󱥪"),
        ("tenpo", "󱥫"),
        ("toki", "󱥬"),
        ("tomo", "󱥭"),
        ("tu", "󱥮"),
        ("unpa", "󱥯"),
        ("uta", "󱥰"),
        ("utala", "󱥱"),
        ("walo", "󱥲"),
        ("wan", "󱥳"),
        ("waso", "󱥴"),
        ("wawa", "󱥵"),
        ("weka", "󱥶"),
        ("wile", "󱥷"),
        ("namako", "󱥸"),
        ("kin", "󱥹"),
        ("oko", "󱥺"),
        ("kipisi", "󱥻"),
        ("leko", "󱥼"),
        ("monsuta", "󱥽"),
        ("tonsi", "󱥾"),
        ("jasima", "󱥿"),
        ("kijetesantakalu", "󱦀"),
        ("soko", "󱦁"),
        ("meso", "󱦂"),
        ("epiku", "󱦃"),
        ("kokosila", "󱦄"),
        ("lanpan", "󱦅"),
        ("n", "󱦆"),
        ("misikeke", "󱦇"),
        ("ku", "󱦈"),
        ("pake", "󱦠"),
        ("apeja", "󱦡"),
        ("majuna", "󱦢"),
        ("powe", "󱦣"),
    ]);
}

#[test]
fn i_dont_know_now_to_write_macros() {
    use std::fs::File;
    use std::path::Path;
    use std::io::Read;
    let mut file = File::open(Path::new("C:\\ajemi.dict.yaml")).unwrap();
    let mut text = String::with_capacity(2048);
    file.read_to_string(&mut text).unwrap();
    for line in text.lines() {
        let split: Vec<&str> = line.split('\t').collect();
        println!("(\"{}\", \"{}\"),", split[1], split[0]);
    }
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


#[test]
fn repl_sentence() {
    use std::io::stdin;
    setup();
    let mut buf = String::new();
    loop {
        buf.clear();
        stdin().read_line(&mut buf).unwrap();
        let sents = engine().suggest_sentences(&buf);
        for sent in sents {
            println!("{}", sent.output)
        }
    }
}



