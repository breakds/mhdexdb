extern crate rusqlite;

use table::DexSqlite;
use table::IndexedRow;

#[derive(Debug)]
pub struct Weapon {
    // Base properties
    id: i32,
    name: String,
    
    // Weapon Properties
    // rare: i32,
    // attack: i32,
    // affinity: i32,
}

impl DexSqlite for Weapon {
    fn statement() -> &'static str {
        "SELECT Wpn_ID, Wpn_Name_0 FROM ID_Wpn_Name"
    }

    fn new(row: &rusqlite::Row) -> Weapon {
        Weapon {
            id: Weapon::to_id(row.get(0)),
            name: row.get(1),
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
