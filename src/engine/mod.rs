mod long_glyph;
mod sentence;
use std::{collections::{HashMap, HashSet}, cell::OnceCell};
use Candidate::*;

use crate::{engine::long_glyph::insert_long_glyph, CANDI_NUM};

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
        let mut suggs = Vec::with_capacity(CANDI_NUM);
        // Suggest a sentence
        if let Some(mut sugg) = self.suggest_sentence(spelling) {
            insert_long_glyph(&mut sugg.output);
            suggs.push(sugg);
        }
        // suggest single words
        let mut remains = CANDI_NUM - suggs.len();
        let mut exclude: HashSet<String> = HashSet::new();
        'outer_loop:
        for to in (1..=spelling.len()).rev() {
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
