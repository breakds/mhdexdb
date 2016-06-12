extern crate rusqlite;

use utils::LangText;
use utils::ByLanguage;

use table::DexSqlite;
use table::IndexedRow;

#[derive(Debug)]
pub struct Weapon {
    // Base properties
    id: i32,
    name: LangText,
    
    // Weapon Properties
    rare: i32,
    // attack: i32,
    // affinity: i32,
}

impl DexSqlite for Weapon {
    fn statement() -> &'static str {
        "SELECT DB_Wpn.Wpn_ID, \
         Wpn_Name_0, Wpn_Name_1, Wpn_Name_3, \
         Rare \
         FROM DB_Wpn \
         INNER JOIN ID_Wpn_Name on DB_Wpn.Wpn_ID = ID_Wpn_Name.Wpn_ID \
         ORDER BY DB_Wpn.Wpn_ID"
    }

    fn new(row: &rusqlite::Row) -> Weapon {
        Weapon {
            id: Weapon::to_id(row.get(0)),
            name: LangText::new(row.get(1), row.get(2), row.get(3)),
            rare: row.get(4),
        }
    }
}

impl IndexedRow for Weapon {
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
