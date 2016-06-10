extern crate rusqlite;

use table::DexSqlite;

#[derive(Debug)]
pub struct Weapon {
    name: String,
}

impl DexSqlite for Weapon {
    fn statement() -> &'static str {
        "SELECT Wpn_Name_0 FROM ID_Wpn_Name"
    }

    fn new(row: &rusqlite::Row) -> Weapon {
        Weapon {
            name: row.get(0),
        }
    }
}
