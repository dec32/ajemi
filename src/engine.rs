use std::{collections::HashMap, cell::OnceCell};

use log::debug;


/// A struct to store and query words and punctuators
#[derive(Default)]
pub struct Engine {
    // todo the values are actually pretty small (1 ~ 2 chars) thus cheap to copy
    // try implement a stringlet type that implments the Copy trait, not sure if thats possible
    dict: HashMap<String, String>,
    puncts: HashMap<char, char>,
}

impl Engine {
    fn new() -> Engine{
        Default::default()
    }

    /// Map all spellings and unique prefixes to their correspond words. 
    /// 
    /// A prefix being "unique" means in the whole lexicon there's only one word that start with it.
    ///  
    /// For example, "kije" is a unique prefix for "kijetesantakalu" because there's no other word
    /// that starts with "kije".
    fn load_dict(&mut self, entries: Vec<(&str, &str)>) {
        use Candidate::*;
        enum Candidate {
            Exact(String),
            Unique(String),
            Duplicate,
        }
        let mut candidates = HashMap::new();
        for (spelling, word) in entries {
            candidates.insert(spelling.to_string(), Exact(word.to_string()));
            for len in 1..spelling.len() {
                let prefix = &spelling[0..len];
                match candidates.get(prefix) {
                    None => 
                        candidates.insert(prefix.to_string(), Unique(word.to_string())),
                    Some(Unique(_)) | Some(Duplicate) => 
                        candidates.insert(prefix.to_string(), Duplicate),
                    Some(Exact(_)) 
                        => None,
                };
            }
        }
        // by doing so, we lost the info about whether an ascii sequence is an exact spelling
        // or a unique prefix. i tried keeping the info, but then the suggest method would 
        // randomly stop recognizing prefixes.
        // haven't figured out why and possbilly will never do. i don't have time
        let mut dict = HashMap::new();
        for (prefix_or_spelling, candidate) in candidates {
            match candidate {
                Exact(word) | Unique(word) => 
                    dict.insert(prefix_or_spelling, word),
                Duplicate =>
                    None
            };
        }
        self.dict = dict;
    }

    fn insert_punt(&mut self, punct: char, remapped: char) {
        self.puncts.insert(punct, remapped);
    }

    // I hate this kind of out parameters but otherwise the allocations can be crazy.
    pub fn suggest(&self, spelling: &str, groupping: &mut Vec<usize>, output: &mut String){
        groupping.clear();
        output.clear();
        let mut from = 0;
        let mut to = spelling.len();
        while from < to {
            let slice = &spelling[from..to];
            // to match `Some(Exact(word)) | Some(Unique(word))` will cause the issue mentioned above
            if let Some(word) = self.dict.get(slice) {
                groupping.push(to);
                output.push_str(word);
                from = to;
                to = spelling.len();
            } else {
                to -= 1;
            }
        }
    }

    pub fn remap_punct(&self, punct: char) -> char {
        // FIXME wow this String alloc is so unneccessary
        let remapped = self.puncts.get(&punct);
        debug!("The remapped punct for '{punct}' is '{:?}'", remapped);
        if remapped.is_some() {
            return remapped.unwrap().clone();
        } else {
            return punct;
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
    engine.insert_punt('+', '󱦕');
    engine.insert_punt('-', '󱦖');
    engine.insert_punt('(', '󱦗');
    engine.insert_punt(')', '󱦘');
    engine.insert_punt('{', '󱦚');
    engine.insert_punt('}', '󱦛');
    engine.insert_punt('.', '󱦜');
    engine.insert_punt(':', '󱦝');
    engine.insert_punt('<', '「');
    engine.insert_punt('>', '」');
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

