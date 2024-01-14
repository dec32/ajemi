use std::{collections::HashMap, ffi::OsStr, cell::OnceCell};

use crate::extend::OsStrExt2;
use log::debug;

/// A private struct to store and query words.
struct Dict {
    suggestions: HashMap<Vec<u16>, Vec<u16>>,
}

impl Dict {
    fn from_entries(entries: Vec<(&str, &str)>) -> Dict {
        use Candidate::*;
        enum Candidate {
            Exact(Vec<u16>),
            Unique(Vec<u16>),
            Duplicate,
        }
        let mut candidates = HashMap::new();
        for entry in entries {
            let spelling = OsStr::new(entry.0).wchars();
            let word = OsStr::new(entry.1).wchars();
            candidates.insert(spelling.clone(), Exact(word.clone()));
            for len in 1..spelling.len() {
                let prefix = &spelling[0..len];
                match candidates.get(prefix) {
                    None => 
                        candidates.insert(prefix.to_vec(), Unique(word.clone())),
                    Some(Unique(_)) | Some(Duplicate) => 
                        candidates.insert(prefix.to_vec(), Duplicate),
                    Some(Exact(_)) 
                        => None,
                };
            }
        }
        // by doing so, we lost the info about whether an ascii sequence is an exact spelling
        // or a unique prefix. i tried keeping the info, but then the suggest method would 
        // randomly stop recognizing prefixes.
        // haven't figured out why and possbilly will never do. i don't have time
        let mut suggestions = HashMap::new();
        for (prefix_or_spelling, candidate) in candidates {
            match candidate {
                Exact(word) | Unique(word) => 
                    suggestions.insert(prefix_or_spelling, word),
                Duplicate =>
                    None
            };
        }
        Dict{suggestions}
    }

    fn suggest(&self, letters: &[u16]) -> Vec<u16> {
        debug!("Suggest for (\"{}\")", String::from_utf16_lossy(letters));
        let mut from = 0;
        let mut to = letters.len();
        let mut result = Vec::new();
        while from < to {
            let slice = &letters[from..to];
            let suggestion = self.suggestions.get(slice);
            // to match `Some(Exact(word)) | Some(Unique(word))` will cause the issue mentioned above
            if let Some(word) = suggestion {
                result.extend_from_slice(word);
                from = to;
                to = letters.len();
            } else {
                to -= 1;
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
    unsafe { 
        DICT.get_or_init(||Dict::from_entries(vec![
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
        ]));
    };
}

fn dict() -> &'static Dict {
    unsafe {DICT.get().unwrap()}
}

pub fn suggest(letters: &[u16]) -> Vec<u16> {
    dict().suggest(letters)
    // dict()._suggest(letters)
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
    use std::ffi::OsString;
    setup();
    let mut buf = String::new();
    loop {
        buf.clear();
        stdin().read_line(&mut buf).unwrap();
        let wchars = OsString::from(&buf).wchars();
        let suggestion = String::from_utf16_lossy(&suggest(&wchars));
        println!("{}", suggestion);
    }

}

#[test]
pub fn test_dict() {
    let dict = Dict::from_entries(vec![
        ("mi", "我"),
        ("lukin", "看"),
        ("e", "兮"),
        ("kijetesantakalu", "狸"),
        ("nasa", "怪"),
        ("lape", "眠"),
    ]);

    for (letters, suggestion) in &dict.suggestions {
        let letters = String::from_utf16_lossy(&letters);
        println!("{letters}={:?}", suggestion);
    }

    let suggestion = dict.suggest(&OsStr::new("milukinekijenasalol").wchars());
    let suggestion = String::from_utf16_lossy(&suggestion);
    println!("{suggestion}")  
}

