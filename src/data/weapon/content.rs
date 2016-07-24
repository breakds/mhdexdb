extern crate rusqlite;

use std::fmt;

use data::base::Language;
use data::base::LangText;
use data::base::ByLanguage;

use data::base::DexSqlite;
use data::base::Indexed;

// pub enum WeaponProperty {
//     Name = 0,
//     // Rare = 1,
//     // Affinity = 2,
//     Size = 1,
// }

// static WEAPON_PROPERTIES: [LangText; WeaponProperty::Size as usize] =
//     [LangText::new("Name".to_string(), "".to_string(), "".to_string())];


// pub struct WeaponType {
//     pub name: LangText,
//     pub columns: WeaponProperty,
// }

// static ref A = LangText::new("Name".to_string(), "".to_string(), "".to_string());

// pub struct WeaponColumn {
//     id: i32,
//     pub name: LangText,
// }

// pub struct WeaponType {
//     id: i32,
//     pub name: LangText,
// }

// pub struct WeaponColumn {
//     pub id: 
//     pub name: LangText,
// }

// pub struct WeaponType {
//     pub name: LangText,
//     pub columns: WeaponColumn,
// }

// impl trait DexJson {
//     fn new(&json::Object) -> DexJson {
        
//     }
// }

pub struct Weapon {
    // Identity properties
    id: i32,
    // pub type_id: i32,
    pub name: LangText,
    
    // Weapon Properties
    pub rare: i32,
    pub attack: i32,
    pub affinity: i32,
}

impl DexSqlite for Weapon {
    fn statement() -> &'static str {
        "SELECT DB_Wpn.Wpn_ID, \
         Wpn_Name_0, Wpn_Name_1, Wpn_Name_3, \
         Rare, \
         Atk, Affinity \
         FROM DB_Wpn \
         INNER JOIN ID_Wpn_Name on DB_Wpn.Wpn_ID = ID_Wpn_Name.Wpn_ID \
         ORDER BY DB_Wpn.Wpn_ID"
    }

    fn new(row: &rusqlite::Row) -> Weapon {
        Weapon {
            id: Weapon::to_id(row.get(0)),
            name: LangText::new(row.get(1), row.get(2), row.get(3)),
            rare: row.get(4),
            attack: row.get(5),
            affinity: {
                let affinity: f64 = row.get(6);
                (affinity * 100.0).round() as i32
            },
        }
    }
}

impl Indexed for Weapon {
    fn id(&self) -> i32 {
        self.id
    }

    fn dex_id(&self) -> i32 {
        self.id + 1
    }

    fn to_dex(id: i32) -> i32 {
        id + 1
    }

    fn to_id(dex_id: i32) -> i32 {
        dex_id - 1
    }
}

impl fmt::Display for Weapon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Weapon: {} {{", self.name.get(Language::ENG));
        writeln!(f, "  ID: {}", self.id);
        writeln!(f, "  Rare: {}", self.rare);
        writeln!(f, "  Attack: {}", self.attack);
        writeln!(f, "  Affinity: {}", self.affinity);
        writeln!(f, "}}")
    }
}
