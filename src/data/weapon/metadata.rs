use std::fmt;

use data::base::{Language, LangText, ByLanguage, DecodableWithContext};
use data::table::Table;
use rustc_serialize::{Decodable, Decoder};

/* Weapon Column */

#[derive(RustcDecodable, Clone)]
pub struct WeaponColumn {
    pub name: String,
    pub numeric: bool,
    pub label: LangText,
}

impl fmt::Display for WeaponColumn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "WeaponColumn - {} {{", self.name);
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
        writeln!(f, "WeaponType - {} {{", self.name.get(Language::ENG));
        writeln!(f, "  name: {}", self.name);
        write!(f, "  columns:");
        for column in &self.columns {
            write!(f, " {}", column.name);
        }
        writeln!(f, "");
        writeln!(f, "}}")
    }
}





