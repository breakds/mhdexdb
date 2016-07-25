use std::fmt;

use data::base::{Language, LangText, ByLanguage};
use rustc_serialize::{Decodable, Decoder};

#[derive(RustcDecodable)]
pub struct WeaponColumn {
    pub name: String,
    pub label: LangText,
}

impl fmt::Display for WeaponColumn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "WeaponColumn: {} {{", self.name);
        writeln!(f, "  label: {}", self.label);
        writeln!(f, "}}")
    }
}
