use std::{env, ffi::OsString, fs, path::PathBuf, sync::OnceLock};

use log::{debug, error};
use strum::EnumIter;
use windows::{
    Win32::{
        Foundation::{GetLastError, HINSTANCE},
        System::LibraryLoader::GetModuleFileNameA,
        UI::TextServices::HKL,
    },
    core::GUID,
};

use crate::{Error, Result, extend::ResultExt};

pub fn setup(dll_module: HINSTANCE) {
    DLL_MODULE.get_or_init(|| dll_module);
}

// global variables
static DLL_MODULE: OnceLock<HINSTANCE> = OnceLock::new();
pub fn dll_module() -> HINSTANCE {
    DLL_MODULE.get().copied().unwrap()
}

pub fn dll_path() -> Result<OsString> {
    let mut buf: Vec<u8> = vec![0; 512];
    unsafe { GetModuleFileNameA(dll_module(), &mut buf) };
    if buf[0] == 0 {
        let err = unsafe { GetLastError() };
        error!("Failed to find the dll path. {:?}", err);
        return Err(err.into());
    }
    let mut from = 0;
    let mut to = buf.len();
    while to != from + 1 {
        let i = (to + from) / 2;
        if buf[i] == 0 {
            to = i;
        } else {
            from = i;
        }
    }
    buf.truncate(to);
    let path = unsafe { OsString::from_encoded_bytes_unchecked(buf) };
    debug!("Found DLL in {}", path.to_string_lossy());
    Ok(path)
}

pub fn hkl() -> Result<HKL> {
    // TODO: save the result in memory
    let hkl = PathBuf::from(env::var("LOCALAPPDATA")?)
        .join(IME_NAME)
        .join("install.dat");
    let hkl = fs::read_to_string(hkl)?;
    let hkl = u32::from_str_radix(&hkl, 16).map_err(Error::InstallDatCorrupted)?;
    let hkl = HKL(hkl as isize);
    Ok(hkl)
}

pub fn hkl_or_us() -> HKL {
    hkl().log_err().unwrap_or(HKL(LanguageID::US as isize))
}

// registration stuff
pub const IME_NAME: &str = "Ajemi";
pub const IME_NAME_ASCII: &str = "Ajemi";
pub const IME_ID: GUID = GUID::from_u128(0xC93D3D59_2FAC_40E0_ABC6_A3658749E2FA);
pub const LANG_PROFILE_ID: GUID = GUID::from_u128(0xA411A7FC_A082_4B8A_8741_AA4A72613933);
pub const LANGBAR_ITEM_ID: GUID = GUID::from_u128(0x95288B2B_4D3B_4D4A_BF5B_9342E4F75E4D);
pub const DISPLAY_ATTR_ID: GUID = GUID::from_u128(0xE42647FB_4BF0_4570_9013_768487C5CAAE);
pub const LITE_TRAY_ICON_INDEX: u32 = 0;
pub const DARK_TRAY_ICON_INDEX: u32 = 1;
// customization
pub const CANDI_NUM: usize = 5;
pub const CANDI_INDEXES: [&str; CANDI_NUM] = ["1", "2", "3", "4", "5"];
pub const CANDI_INDEX_SUFFIX: &str = ". ";
pub const CANDI_INDEX_SUFFIX_MONO: &str = ".";
pub const PREEDIT_DELIMITER: &str = "'";
// included text
pub const DEFAULT_CONF: &str = include_str!("../res/conf.toml");
pub const SITELEN_DICT: &str = include_str!("../res/dict/sitelen.dict");
pub const EMOJI_DICT: &str = include_str!("../res/dict/emoji.dict");

// language IDs
#[derive(EnumIter)]
pub enum LanguageID {
    ADLaM = 0x00140C00,
    Albanian = 0x0000041C,
    Arabic101 = 0x00000401,
    Arabic102 = 0x00010401,
    Arabic102AZERTY = 0x00020401,
    ArmenianEasternLegacy = 0x0000042B,
    ArmenianPhonetic = 0x0002042B,
    ArmenianTypewriter = 0x0003042B,
    ArmenianWesternLegacy = 0x0001042B,
    AssameseINSCRIPT = 0x0000044D,
    AzerbaijaniStandard = 0x0001042C,
    AzerbaijaniCyrillic = 0x0000082C,
    AzerbaijaniLatin = 0x0000042C,
    Bangla = 0x00000445,
    BanglaINSCRIPT = 0x00020445,
    BanglaINSCRIPTLegacy = 0x00010445,
    Bashkir = 0x0000046D,
    Belarusian = 0x00000423,
    BelgianComma = 0x0001080C,
    BelgianPeriod = 0x00000813,
    BelgianFrench = 0x0000080C,
    BosnianCyrillic = 0x0000201A,
    Buginese = 0x000B0C00,
    Bulgarian = 0x00030402,
    BulgarianLatin = 0x00010402,
    BulgarianPhoneticTraditional = 0x00040402,
    BulgarianPhonetic = 0x00020402,
    BulgarianTypewriter = 0x00000402,
    CanadianFrench = 0x00001009,
    CanadianFrenchLegacy = 0x00000C0C,
    CanadianMultilingualStandard = 0x00011009,
    CentralAtlasTamazight = 0x0000085F,
    CentralKurdish = 0x00000492,
    CherokeeNation = 0x0000045C,
    CherokeePhonetic = 0x0001045C,
    ChineseSimplifiedUS = 0x00000804,
    ChineseSimplifiedSingaporeUS = 0x00001004,
    ChineseTraditionalUS = 0x00000404,
    ChineseTraditionalHongKongSARUS = 0x00000C04,
    ChineseTraditionalMacaoSARUS = 0x00001404,
    Czech = 0x00000405,
    CzechQWERTY = 0x00010405,
    CzechProgrammers = 0x00020405,
    Danish = 0x00000406,
    DevanagariINSCRIPT = 0x00000439,
    DivehiPhonetic = 0x00000465,
    DivehiTypewriter = 0x00010465,
    Dutch = 0x00000413,
    Dzongkha = 0x00000C51,
    EnglishIndia = 0x00004009,
    Estonian = 0x00000425,
    Faeroese = 0x00000438,
    Finnish = 0x0000040B,
    FinnishwithSami = 0x0001083B,
    French = 0x0000040C,
    Futhark = 0x00120C00,
    GeorgianErgonomic = 0x00020437,
    GeorgianLegacy = 0x00000437,
    GeorgianMES = 0x00030437,
    GeorgianOldAlphabets = 0x00040437,
    GeorgianQWERTY = 0x00010437,
    German = 0x00000407,
    GermanIBM = 0x00010407,
    Gothic = 0x000C0C00,
    Greek = 0x00000408,
    Greek220 = 0x00010408,
    Greek220Latin = 0x00030408,
    Greek319 = 0x00020408,
    Greek319Latin = 0x00040408,
    GreekLatin = 0x00050408,
    GreekPolytonic = 0x00060408,
    Greenlandic = 0x0000046F,
    Guarani = 0x00000474,
    Gujarati = 0x00000447,
    Hausa = 0x00000468,
    Hawaiian = 0x00000475,
    Hebrew = 0x0000040D,
    HebrewStandard = 0x0002040D,
    HindiTraditional = 0x00010439,
    Hungarian = 0x0000040E,
    Hungarian101key = 0x0001040E,
    Icelandic = 0x0000040F,
    Igbo = 0x00000470,
    InuktitutLatin = 0x0000085D,
    InuktitutNaqittaut = 0x0001045D,
    Irish = 0x00001809,
    Italian = 0x00000410,
    Italian142 = 0x00010410,
    Japanese = 0x00000411,
    Javanese = 0x00110C00,
    Kannada = 0x0000044B,
    Kazakh = 0x0000043F,
    Khmer = 0x00000453,
    KhmerNIDA = 0x00010453,
    Korean = 0x00000412,
    KyrgyzCyrillic = 0x00000440,
    Lao = 0x00000454,
    LatinAmerican = 0x0000080A,
    Latvian = 0x00000426,
    LatvianQWERTY = 0x00010426,
    LatvianStandard = 0x00020426,
    LisuBasic = 0x00070C00,
    LisuStandard = 0x00080C00,
    Lithuanian = 0x00010427,
    LithuanianIBM = 0x00000427,
    LithuanianStandard = 0x00020427,
    Luxembourgish = 0x0000046E,
    Macedonian = 0x0000042F,
    MacedonianStandard = 0x0001042F,
    Malayalam = 0x0000044C,
    Maltese47Key = 0x0000043A,
    Maltese48Key = 0x0001043A,
    Maori = 0x00000481,
    Marathi = 0x0000044E,
    MongolianMongolianScript = 0x00000850,
    MongolianCyrillic = 0x00000450,
    MyanmarPhoneticorder = 0x00010C00,
    MyanmarVisualorder = 0x00130C00,
    NZAotearoa = 0x00001409,
    Nepali = 0x00000461,
    NewTaiLue = 0x00020C00,
    Norwegian = 0x00000414,
    NorwegianwithSami = 0x0000043B,
    NKo = 0x00090C00,
    Odia = 0x00000448,
    Ogham = 0x00040C00,
    OlChiki = 0x000D0C00,
    OldItalic = 0x000F0C00,
    Osage = 0x00150C00,
    Osmanya = 0x000E0C00,
    PashtoAfghanistan = 0x00000463,
    Persian = 0x00000429,
    PersianStandard = 0x00050429,
    Phagspa = 0x000A0C00,
    Polish214 = 0x00010415,
    PolishProgrammers = 0x00000415,
    Portuguese = 0x00000816,
    PortugueseBrazilABNT = 0x00000416,
    PortugueseBrazilABNT2 = 0x00010416,
    Punjabi = 0x00000446,
    RomanianLegacy = 0x00000418,
    RomanianProgrammers = 0x00020418,
    RomanianStandard = 0x00010418,
    Russian = 0x00000419,
    RussianTypewriter = 0x00010419,
    RussianMnemonic = 0x00020419,
    Sakha = 0x00000485,
    SamiExtendedFinlandSweden = 0x0002083B,
    SamiExtendedNorway = 0x0001043B,
    ScottishGaelic = 0x00011809,
    SerbianCyrillic = 0x00000C1A,
    SerbianLatin = 0x0000081A,
    SesothosaLeboa = 0x0000046C,
    Setswana = 0x00000432,
    Sinhala = 0x0000045B,
    SinhalaWij9 = 0x0001045B,
    Slovak = 0x0000041B,
    SlovakQWERTY = 0x0001041B,
    Slovenian = 0x00000424,
    Sora = 0x00100C00,
    SorbianExtended = 0x0001042E,
    SorbianStandard = 0x0002042E,
    SorbianStandardLegacy = 0x0000042E,
    Spanish = 0x0000040A,
    SpanishVariation = 0x0001040A,
    Standard = 0x0000041A,
    Swedish = 0x0000041D,
    SwedishwithSami = 0x0000083B,
    SwissFrench = 0x0000100C,
    SwissGerman = 0x00000807,
    Syriac = 0x0000045A,
    SyriacPhonetic = 0x0001045A,
    TaiLe = 0x00030C00,
    Tajik = 0x00000428,
    Tamil = 0x00000449,
    Tamil99 = 0x00020449,
    TamilAnjal = 0x00030449,
    Tatar = 0x00010444,
    TatarLegacy = 0x00000444,
    Telugu = 0x0000044A,
    ThaiKedmanee = 0x0000041E,
    ThaiKedmaneenonShiftLock = 0x0002041E,
    ThaiPattachote = 0x0001041E,
    ThaiPattachotenonShiftLock = 0x0003041E,
    TibetanPRC = 0x00000451,
    TibetanPRCUpdated = 0x00010451,
    TifinaghBasic = 0x0000105F,
    TifinaghExtended = 0x0001105F,
    TraditionalMongolianStandard = 0x00010850,
    TurkishF = 0x0001041F,
    TurkishQ = 0x0000041F,
    Turkmen = 0x00000442,
    US = 0x00000409,
    USEnglishTableforIBMArabic238_L = 0x00050409,
    Ukrainian = 0x00000422,
    UkrainianEnhanced = 0x00020422,
    UnitedKingdom = 0x00000809,
    UnitedKingdomExtended = 0x00000452,
    UnitedStatesDvorak = 0x00010409,
    UnitedStatesDvorakforlefthand = 0x00030409,
    UnitedStatesDvorakforrighthand = 0x00040409,
    UnitedStatesInternational = 0x00020409,
    Urdu = 0x00000420,
    Uyghur = 0x00010480,
    UyghurLegacy = 0x00000480,
    UzbekCyrillic = 0x00000843,
    Vietnamese = 0x0000042A,
    Wolof = 0x00000488,
    Yoruba = 0x0000046A,
}
