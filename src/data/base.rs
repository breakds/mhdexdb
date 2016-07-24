extern crate rusqlite;
extern crate rustc_serialize;

use std::fmt;
use self::rustc_serialize::json;

pub trait DexSqlite {
    fn statement() -> &'static str;
    fn new(&rusqlite::Row) -> Self;
}

pub trait DexJson {
    fn new(&json::Object) -> Self;
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

pub enum Language {
    ENG = 0,  // English
    CHS = 1,  // Chinese Simplified
    CHT = 2,  // Chinese Traditional
    JAP = 3,  // Japanese
    KOR = 4,  // Korean
    Size = 5,
}

pub type LangText = [String; Language::Size as usize];

pub trait ByLanguage<T> {
    fn new(String, String, String) -> Self;
    fn get(&self, Language) -> &T;
}

impl ByLanguage<String> for LangText {
    fn new(english: String, chinese_simplified: String,
           japanese: String) -> LangText {
        [english, chinese_simplified, japanese, "".to_string(), "".to_string()]
    }

    fn get(&self, language: Language) -> &String {
        &self[language as usize]
    }
}
