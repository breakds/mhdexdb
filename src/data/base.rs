extern crate rusqlite;

use std::fmt;
use rustc_serialize::{Decodable, Decoder};

/* ---------- Data Object and I/O traits ---------- */

pub trait DexSqlite {
    fn statement() -> &'static str;
    fn new(&rusqlite::Row) -> Self;
}

pub trait DecodableWithContext {
    type Raw: Decodable;
    type Context;

    fn convert(raw: &raw, context: &Context);
}

pub trait Indexed {
    fn id(&self) -> i32;
    fn dex_id(&self) -> i32;
    fn to_dex(i32) -> i32;
    fn to_id(i32) -> i32;
}

#[derive(Debug)]
pub enum Ref {
    Id(i32),
    DexId(i32),
}

/* ---------- Struct: Language ---------- */

pub enum Language {
    ENG = 0,  // English
    CHS = 1,  // Chinese Simplified
    CHT = 2,  // Chinese Traditional
    JAP = 3,  // Japanese
    KOR = 4,  // Korean
    Size = 5,
}

pub trait ByLanguage<T> {
    fn get(&self, Language) -> &T;
}

/* ---------- Struct: LangText ---------- */

#[derive(Debug)]
pub struct LangText {
    storage: [String; Language::Size as usize],
}

impl LangText {
    pub fn new(english: &str, chinese_simplified: &str,
           japanese: &str) -> LangText {
        LangText {
            storage: [english.to_owned(),
                      chinese_simplified.to_owned(),
                      "".to_owned(),
                      japanese.to_owned(),
                      "".to_owned()]
        }
    }
}

impl ByLanguage<String> for LangText {
    fn get(&self, language: Language) -> &String {
        &self.storage[language as usize]
    }
}

impl Decodable for LangText {
    fn decode<D: Decoder>(d: &mut D) -> Result<LangText, D::Error> {
        // For read_struct and read_struct_field on Json decoder, the
        // second field is actually *NOT* used. Correct values are
        // provided for them.
        //
        // For read_struct, the struct name parameter is *NOT* used on
        // Json decoder.
        d.read_struct("LangText", 3, |d| {
            let english: String = try!(
                d.read_struct_field("ENG", Language::ENG as usize,
                                    |d| { d.read_str() }));
            let chinese_simplified: String = try!(
                d.read_struct_field("CHS", Language::CHS as usize,
                                    |d| { d.read_str() }));
            let japanese: String = try!(
                d.read_struct_field("JAP", Language::JAP as usize,
                                    |d| { d.read_str() }));
            Ok(LangText::new(&english, &chinese_simplified, &japanese))
        })
    }
}

impl fmt::Display for LangText {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}[{}, {}]",
               self.get(Language::ENG),
               self.get(Language::CHS),
               self.get(Language::JAP))
    }
}
