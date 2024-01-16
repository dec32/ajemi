use std::{collections::HashMap, cell::OnceCell};


/// A private struct to store and query words, punctuators
#[derive(Default)]
struct Engine {
    // todo the values are actually pretty small (1 ~ 2 chars) thus cheap to copy
    // try implement a stringlet type that implments the Copy trait, not sure if thats possible
    dict: HashMap<String, String>,
    puncts: HashMap<char, String>,
}

impl Engine {
    fn new() -> Engine{
        Default::default()
    }

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

    fn insert_punt(&mut self, punct: char, remapped: &str) {
        self.puncts.insert(punct, String::from(remapped));
    }

    fn suggest(&self, letters: &str) -> String {
        let mut from = 0;
        let mut to = letters.len();
        let mut result = String::new();
        while from < to {
            let slice = &letters[from..to];
            let suggestion = self.dict.get(slice);
            // to match `Some(Exact(word)) | Some(Unique(word))` will cause the issue mentioned above
            if let Some(word) = suggestion {
                result += word;
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

static mut ENGINE: OnceCell<Engine> = OnceCell::new();

pub fn setup() {
    let engine = unsafe { 
        ENGINE.get_or_init(Engine::new);
        ENGINE.get_mut().unwrap()
    };
    engine.insert_punt('[', "󱦐");
    engine.insert_punt(']', "󱦑");
    engine.insert_punt('+', "󱦕");
    engine.insert_punt('-', "󱦖");
    engine.insert_punt('(', "󱦗");
    engine.insert_punt(')', "󱦘");
    engine.insert_punt('{', "󱦚");
    engine.insert_punt('}', "󱦛");
    engine.insert_punt('.', "󱦜");
    engine.insert_punt(':', "󱦝");
    engine.insert_punt('<', "「");
    engine.insert_punt('>', "」");
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

fn engine() -> &'static Engine {
    unsafe {ENGINE.get().unwrap()}
}

pub fn suggest(letters: &str) -> String {
    engine().suggest(letters)
}

pub fn remap_punct(punct: char) -> String {
    // FIXME wow this String alloc is so unneccessary
    engine().puncts.get(&punct)
        .map(String::clone)
        .unwrap_or_else(||{
            let mut buf = String::with_capacity(1);
            buf.push(punct);
            buf
        })
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
        let suggestion = &suggest(&buf);
        println!("{}", suggestion);
    }

}

#[test]
pub fn test_dict() {
    let mut engine = Engine::new();
    engine.load_dict(vec![
        ("mi", "我"),
        ("lukin", "看"),
        ("e", "兮"),
        ("kijetesantakalu", "狸"),
        ("nasa", "怪"),
        ("lape", "眠"),
    ]);

    for (letters, suggestion) in &engine.dict {
        println!("{letters}={:?}", suggestion);
    }

    let suggestion = engine.suggest("milukinekijenasalol");
    println!("{suggestion}")  
}

