use std::fmt;

use data::base::{Language, LangText, ByLanguage, DecodableWithContext};
use data::table::Table;
use rustc_serialize::{Decoder};

/* Sharpness Color */

#[derive(RustcDecodable)]
pub struct SharpnessColor {
    pub id: i32,
    pub color: String,
}

/* Weapon Column */

#[derive(RustcDecodable, Clone)]
pub struct WeaponColumn {
    pub name: String,
    pub numeric: bool,
    pub label: LangText,
}

impl fmt::Display for WeaponColumn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(writeln!(f, "WeaponColumn - {} {{", self.name));
        try!(writeln!(f, "  label: {}", self.label));
        try!(writeln!(f, "  numeric: {}", self.numeric));
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
    
    fn convert(raw: &RawWeaponType, context: &Table<WeaponColumn>) -> WeaponType {
        WeaponType {
            name: raw.name.clone(),
            columns: raw.columns.iter().map(|column_name| -> WeaponColumn {
                context.iter().find(|x| x.name == *column_name).unwrap().clone()
            }).collect::<Vec<WeaponColumn>>()
        }
    }
}

impl fmt::Display for WeaponType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(writeln!(f, "WeaponType - {} {{", self.name.get(Language::ENG)));
        try!(writeln!(f, "  name: {}", self.name));
        try!(write!(f, "  columns:"));
        for column in &self.columns {
            try!(write!(f, " {}", column.name));
        }
        try!(writeln!(f, ""));
        writeln!(f, "}}")
    }
}

/* Special Types */

#[derive(RustcDecodable)]
pub struct SpecialType {
    pub name: LangText,
    pub color: String,
}

/* Charge Blade and Switch Axe Phials */

#[derive(RustcDecodable)]
pub struct WeaponPhial {
    pub id: i32,
    pub name: LangText,
}
