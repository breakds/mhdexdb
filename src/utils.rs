pub enum Language {
    ENG = 0,  // English
    CHS = 1,  // Chinese Simplified
    CHT = 2,  // Chinese Traditional
    JAP = 3,  // Japanese
    KOR = 4,  // Korean
}

pub type LangText = [String; 3];

pub trait ByLanguage<T> {
    fn new(String, String, String) -> Self;
    fn get(&self, Language) -> &T;
}

impl ByLanguage<String> for LangText {
    fn new(english: String, chinese_simplified: String,
           japanese: String) -> LangText {
        [english, chinese_simplified, japanese]
    }

    fn get(&self, language: Language) -> &String {
        &self[language as usize]
    }
}
