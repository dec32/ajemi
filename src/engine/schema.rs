use std::collections::HashMap;
use Candidate::*;

/// To expain why a certain spelling is mapped to certain word(s)
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
    Duplicates(Vec<String>)
}

/// Stores the dictionary and the remapped punctuators.
/// The dicitonary is indexed in a special way.
pub struct Schema {
    pub candis: HashMap<String, Candidate>,
    pub alters: HashMap<String, Vec<String>>,
    pub puncts: HashMap<char, char>,
}

impl Schema {
    fn new<const D:usize, const P:usize>(dict: [(&str, &str);D], punctuators: [(char, char);P]) -> Schema {
        let mut candis = HashMap::new();
        let mut alters = HashMap::new();
        for (spelling, word) in dict {
            // store exact spellings -> words
            if let Some(Exact(exact, _)) = candis.get(spelling) {
                match alters.get_mut(exact) {
                    None => { 
                        alters.insert(exact.to_string(), vec![word.to_string()]); 
                    }
                    Some(alters) => { 
                        alters.push(word.to_string()); 
                    }
                }
                continue;
            } else {
                candis.insert(spelling.to_string(), Exact(word.to_string(), Vec::new()));
            }
            // store prefixes -> words
            for len in 1..spelling.len() {
                let prefix = &spelling[0..len];
                match candis.get_mut(prefix) {
                    None => {
                        candis.insert(prefix.to_string(), Unique(word.to_string()));
                    },
                    Some(Unique(unique)) => {
                        let mut duplicates = Vec::new();
                        duplicates.push(unique.clone());
                        duplicates.push(word.to_string());
                        candis.insert(prefix.to_string(), Duplicates(duplicates));
                    },
                    Some(Duplicates(duplicates)) | Some(Exact(_, duplicates)) => {
                        duplicates.push(word.to_string());
                    }
                }
            }
        }
    
        let mut puncts = HashMap::new();
        for (punct, remapped) in punctuators {
            puncts.insert(punct, remapped);
        }
    
        Schema {candis, alters, puncts}
    }
}


pub fn sitelen() -> Schema {
    Schema::new([
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
    ],[
        ('[', '󱦐'),
        (']', '󱦑'),
        ('^', '󱦕'),
        ('*', '󱦖'),
        ('(', '󱦗'),
        (')', '󱦘'),
        ('{', '󱦚'),
        ('}', '󱦛'),
        ('.', '󱦜'),
        (':', '󱦝'),
        ('<', '「'),
        ('>', '」'),
        ('-', '\u{200D}'), // ZWJ
        (' ', '\u{3000}'), // CJK space
    ])
}

pub fn emoji() -> Schema {
    Schema::new([
        ("a", "🅰️"),
        ("akesi", "🦎"),
        ("akesi", "🐸"),
        ("ala", "❌"),
        ("alasa", "🏹"),
        ("ale", "🌌"),
        ("anpa", "🧎"),
        ("anpa", "🙇"),
        ("ante", "🔀"),
        ("anu", "🤷"),
        ("awen", "⚓"),
        ("e", "⏩"),
        ("en", "🤝"),
        ("esun", "🛒"),
        ("ijo", "🐚"),
        ("ike", "😔"),
        ("ike", "👎"),
        ("ilo", "🔦"),
        ("insa", "🗳️"),
        ("jaki", "💩"),
        ("jan", "🧑"),
        ("jelo", "🍋"),
        ("jo", "👜"),
        ("kala", "🐟"),
        ("kala", "🐙"),
        ("kalama", "👏"),
        ("kama", "🛬"),
        ("kasi", "🌱"),
        ("ken", "💪"),
        ("kepeken", "✍️"),
        ("kili", "🍎"),
        ("kiwen", "💎"),
        ("ko", "🍦"),
        ("kon", "💨"),
        ("kule", "🌈"),
        ("kulupu", "👥"),
        ("kute", "👂"),
        ("la", "ℹ️"),
        ("la", "💁"),
        ("lape", "😴"),
        ("laso", "☘️"),
        ("lawa", "👑"),
        ("len", "🧣"),
        ("lete", "❄️"),
        ("li", "▶️"),
        ("lili", "🐁"),
        ("linja", "🧶"),
        ("lipu", "🍁"),
        ("loje", "👅"),
        ("lon", "⏺️"),
        ("lon", "✅"),
        ("lon", "🫴"),
        ("luka", "🖐️"),
        ("lukin", "👀"),
        ("lupa", "🚪"),
        ("ma", "🏝️"),
        ("mama", "🍼"),
        ("mani", "🐮"),
        ("meli", "👩"),
        ("meli", "🚺"),
        ("mi", "👇"),
        ("mi", "🅿️"),
        ("mije", "👨"),
        ("mije", "🚹"),
        ("moku", "🍜"),
        ("moli", "😵"),
        ("monsi", "🍑"),
        ("mu", "🐽"),
        ("mun", "🌙"),
        ("musi", "🎭"),
        ("mute", "👐"),
        ("nanpa", "#️⃣"),
        ("nasa", "🌀"),
        ("nasin", "🛤️"),
        ("nena", "🗻"),
        ("ni", "⬇️"),
        ("ni", "⬆️"),
        ("ni", "⬅️"),
        ("ni", "➡️"),
        ("nimi", "📛"),
        ("noka", "🦵"),
        ("o", "🅾️"),
        ("olin", "💕"),
        ("ona", "👈"),
        ("ona", "♋️"),
        ("open", "🎬"),
        ("pakala", "💥"),
        ("pali", "🏗️"),
        ("palisa", "📏"),
        ("pan", "🍞"),
        ("pana", "🙌"),
        ("pi", "📎"),
        ("pilin", "❤️"),
        ("pimeja", "🎱"),
        ("pini", "🏁"),
        ("pini", "🛑"),
        ("pipi", "🐛"),
        ("poka", "👯"),
        ("poki", "📦"),
        ("pona", "😌"),
        ("pona", "👍"),
        ("pu", "🧘"),
        ("sama", "⚖️"),
        ("seli", "🔥"),
        ("selo", "🍌"),
        ("seme", "❓"),
        ("sewi", "☁️"),
        ("sijelo", "🧍"),
        ("sike", "⭕"),
        ("sin", "✨"),
        ("sina", "👆"),
        ("sina", "6️⃣"),
        ("sinpin", "🗿"),
        ("sitelen", "🎨"),
        ("sitelen", "🖼️"),
        ("sona", "🧠"),
        ("soweli", "🦔"),
        ("suli", "🐘"),
        ("suno", "☀️"),
        ("supa", "🛏️"),
        ("suwi", "🍬"),
        ("tan", "↩️"),
        ("taso", "🚦"),
        ("taso", "🚥"),
        ("tawa", "🛫"),
        ("telo", "💧"),
        ("tenpo", "🕒"),
        ("toki", "💬"),
        ("tomo", "🏠"),
        ("tu", "⏸️"),
        ("unpa", "🍆"),
        ("uta", "👄"),
        ("utala", "⚔️"),
        ("utala", "🆚"),
        ("walo", "🐑"),
        ("wan", "1️⃣"),
        ("waso", "🐦"),
        ("wawa", "⚡"),
        ("weka", "🆑"),
        ("wile", "🙏"),
        ("wile", "🧲"),
        ("epiku", "😁"),
        ("jasima", "🪞"),
        ("jasima", "🪩"),
        ("kijetesantakalu", "🦡"),
        ("kijetesantakalu", "🦝"),
        ("kin", "*️⃣"),
        ("kipisi", "✂️"),
        ("kokosila", "🐊"),
        ("ku", "🔬"),
        ("lanpan", "🤳"),
        ("leko", "🧱"),
        ("meso", "😑"),
        ("misikeke", "💊"),
        ("monsuta", "👻"),
        ("n", "🆖"),
        ("namako", "🌶️"),
        ("oko", "👁️"),
        ("soko", "🍄"),
        ("tonsi", "⚧️"),
        ("majuna", "🪷"),
        ("majuna", "💾"),
        ("majuna", "🧓"),
        ("su", "🧙"),
        ("su", "🧵"),
    ], [
        ('[', '\u{1F58C}'),
        (']', '\u{1F58C}'),
    ])
}


