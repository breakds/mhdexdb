use std::fmt;

use data::base::{Language, LangText, ByLanguage};
use rustc_serialize::{Decodable, Decoder};

/* Weapon Column */

#[derive(RustcDecodable)]
pub struct WeaponColumn {
    pub name: String,
    pub numeric: bool,
    pub label: LangText,
}

impl fmt::Display for WeaponColumn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "WeaponColumn: {} {{", self.name);
        writeln!(f, "  label: {}", self.label);
        writeln!(f, "  numeric: {}", self.numeric);
        writeln!(f, "}}")
    }
}

/* Weapon Type */
#[derive(RustcDecodable)]
pub struct RawWeaponType {
    pub name: LangText,
    pub columns: Vec<String>,
}

pub struct WeaponType {
    pub name: LangText,
    pub columns: Vec<WeaponColumn>,
}

impl DecodableWithContext for WeaponType {
    type Raw = RawWeaponType;
    type Context = Table<WeaponColumn>;
    
    fn convert(raw: &RawWeaponType, context: &Table<WeaponColumn>) {
        let mut columns = 
        WeaponType {
            name: raw.name,
            columnes: context.
        }
    }
}





