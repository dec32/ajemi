use std::{collections::HashMap, ffi::OsStr, cell::OnceCell, io::Read, any::Any, fmt::Debug};

use crate::extend::OsStrExt2;
use Suggestion::*;
use log::debug;

/// A private struct to store and query words.
struct Dict {
    // FIXME: the map can be extremly big (O(2^n)), a string pool may help
    suggestions: HashMap<Vec<u16>, Suggestion>
}

enum Suggestion {
    Exact(Vec<u16>),
    Unique(Vec<u16>),
    Duplicate,
}

impl Debug for Suggestion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Exact(arg0) => f.debug_tuple("Exact").field(&String::from_utf16_lossy(arg0)).finish(),
            Self::Unique(arg0) => f.debug_tuple("Unique").field(&String::from_utf16_lossy(arg0)).finish(),
            Self::Duplicate => write!(f, "Duplicate"),
        }
    }
}

impl Dict {
    fn new() -> Dict {
        Dict{suggestions: HashMap::new()}
    }
    fn insert(&mut self, spelling: &str, word: &str) {
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

    fn suggest(&self, letters: &[u16]) -> Vec<u16> {
        debug!("Suggest for (\"{}\")", String::from_utf16_lossy(letters));
        let mut from = 0;
        let mut to = letters.len();
        let mut result = Vec::new();

        while from < to {
            let slice = &letters[from..to];
            let suggestion = self.suggestions.get(slice);
            debug!("Found \"{:?}\" for \"{}\"", suggestion, String::from_utf16_lossy(slice));
            match suggestion {
                Some(Exact(word)) | Some(Unique(word)) => {
                    result.extend_from_slice(word);
                    from = to;
                    to = letters.len();
                },
                _ => {
                    to -= 1;
                }
            }
        }
        result
    }



}

//----------------------------------------------------------------------------
//
//  Static section.
//
//----------------------------------------------------------------------------

static mut DICT: OnceCell<Dict> = OnceCell::new();

pub fn setup() {
    let dict = unsafe{ 
        DICT.get_or_init(||Dict::new());
        DICT.get_mut().unwrap()
    };
    dict.insert("a", "󱤀");      
    dict.insert("akesi", "󱤁");  
    dict.insert("ala", "󱤂");    
    dict.insert("alasa", "󱤃");  
    dict.insert("ale", "󱤄");    
    dict.insert("anpa", "󱤅");   
    dict.insert("ante", "󱤆");   
    dict.insert("anu", "󱤇");    
    dict.insert("awen", "󱤈");   
    dict.insert("e", "󱤉");      
    dict.insert("en", "󱤊");     
    dict.insert("esun", "󱤋");   
    dict.insert("ijo", "󱤌");    
    dict.insert("ike", "󱤍");    
    dict.insert("ilo", "󱤎");    
    dict.insert("insa", "󱤏");   
    dict.insert("jaki", "󱤐");   
    dict.insert("jan", "󱤑");    
    dict.insert("jelo", "󱤒");   
    dict.insert("jo", "󱤓");     
    dict.insert("kala", "󱤔");   
    dict.insert("kalama", "󱤕"); 
    dict.insert("kama", "󱤖");   
    dict.insert("kasi", "󱤗");   
    dict.insert("ken", "󱤘");    
    dict.insert("kepeken", "󱤙");
    dict.insert("kili", "󱤚");   
    dict.insert("kiwen", "󱤛");
    dict.insert("ko", "󱤜");
    dict.insert("kon", "󱤝");
    dict.insert("kule", "󱤞");
    dict.insert("kulupu", "󱤟");
    dict.insert("kute", "󱤠");
    dict.insert("la", "󱤡");
    dict.insert("lape", "󱤢");
    dict.insert("laso", "󱤣");
    dict.insert("lawa", "󱤤");
    dict.insert("len", "󱤥");
    dict.insert("lete", "󱤦");
    dict.insert("li", "󱤧");
    dict.insert("lili", "󱤨");
    dict.insert("linja", "󱤩");
    dict.insert("lipu", "󱤪");
    dict.insert("loje", "󱤫");
    dict.insert("lon", "󱤬");
    dict.insert("luka", "󱤭");
    dict.insert("lukin", "󱤮");
    dict.insert("lupa", "󱤯");
    dict.insert("ma", "󱤰");
    dict.insert("mama", "󱤱");
    dict.insert("mani", "󱤲");
    dict.insert("meli", "󱤳");
    dict.insert("mi", "󱤴");
    dict.insert("mije", "󱤵");
    dict.insert("moku", "󱤶");
    dict.insert("moli", "󱤷");
    dict.insert("monsi", "󱤸");
    dict.insert("mu", "󱤹");
    dict.insert("mun", "󱤺");
    dict.insert("musi", "󱤻");
    dict.insert("mute", "󱤼");
    dict.insert("nanpa", "󱤽");
    dict.insert("nasa", "󱤾");
    dict.insert("nasin", "󱤿");
    dict.insert("nena", "󱥀");
    dict.insert("ni", "󱥁");
    dict.insert("nimi", "󱥂");
    dict.insert("noka", "󱥃");
    dict.insert("o", "󱥄");
    dict.insert("olin", "󱥅");
    dict.insert("ona", "󱥆");
    dict.insert("open", "󱥇");
    dict.insert("pakala", "󱥈");
    dict.insert("pali", "󱥉");
    dict.insert("palisa", "󱥊");
    dict.insert("pan", "󱥋");
    dict.insert("pana", "󱥌");
    dict.insert("pi", "󱥍");
    dict.insert("pilin", "󱥎");
    dict.insert("pimeja", "󱥏");
    dict.insert("pini", "󱥐");
    dict.insert("pipi", "󱥑");
    dict.insert("poka", "󱥒");
    dict.insert("poki", "󱥓");
    dict.insert("pona", "󱥔");
    dict.insert("pu", "󱥕");
    dict.insert("sama", "󱥖");
    dict.insert("seli", "󱥗");
    dict.insert("selo", "󱥘");
    dict.insert("seme", "󱥙");
    dict.insert("sewi", "󱥚");
    dict.insert("sijelo", "󱥛");
    dict.insert("sike", "󱥜");
    dict.insert("sin", "󱥝");
    dict.insert("sina", "󱥞");
    dict.insert("sinpin", "󱥟");
    dict.insert("sitelen", "󱥠");
    dict.insert("sona", "󱥡");
    dict.insert("soweli", "󱥢");
    dict.insert("suli", "󱥣");
    dict.insert("suno", "󱥤");
    dict.insert("supa", "󱥥");
    dict.insert("suwi", "󱥦");
    dict.insert("tan", "󱥧");
    dict.insert("taso", "󱥨");
    dict.insert("tawa", "󱥩");
    dict.insert("telo", "󱥪");
    dict.insert("tenpo", "󱥫");
    dict.insert("toki", "󱥬");
    dict.insert("tomo", "󱥭");
    dict.insert("tu", "󱥮");
    dict.insert("unpa", "󱥯");
    dict.insert("uta", "󱥰");
    dict.insert("utala", "󱥱");
    dict.insert("walo", "󱥲");
    dict.insert("wan", "󱥳");
    dict.insert("waso", "󱥴");
    dict.insert("wawa", "󱥵");
    dict.insert("weka", "󱥶");
    dict.insert("wile", "󱥷");
    dict.insert("namako", "󱥸");
    dict.insert("kin", "󱥹");
    dict.insert("oko", "󱥺");
    dict.insert("kipisi", "󱥻");
    dict.insert("leko", "󱥼");
    dict.insert("monsuta", "󱥽");
    dict.insert("tonsi", "󱥾");
    dict.insert("jasima", "󱥿");
    dict.insert("kijetesantakalu", "󱦀");
    dict.insert("soko", "󱦁");
    dict.insert("meso", "󱦂");
    dict.insert("epiku", "󱦃");
    dict.insert("kokosila", "󱦄");
    dict.insert("lanpan", "󱦅");
    dict.insert("n", "󱦆");
    dict.insert("misikeke", "󱦇");
    dict.insert("ku", "󱦈");
    dict.insert("pake", "󱦠");
    dict.insert("apeja", "󱦡");
    dict.insert("majuna", "󱦢");
    dict.insert("powe", "󱦣");
}


fn dict() -> &'static Dict {
    unsafe {DICT.get().unwrap()}
}

pub fn suggest(letters: &[u16]) -> Vec<u16> {
    dict().suggest(letters)
}


#[test]
fn i_dont_know_now_to_write_macros() {
    use std::fs::File;
    use std::path::Path;
    let mut file = File::open(Path::new("C:\\ajemi.dict.yaml")).unwrap();
    let mut text = String::with_capacity(2048);
    file.read_to_string(&mut text).unwrap();
    for line in text.lines() {
        let split: Vec<&str> = line.split('\t').collect();
        println!("dict.insert(\"{}\", \"{}\");", split[1], split[0]);
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

