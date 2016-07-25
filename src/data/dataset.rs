use data::weapon::WeaponColumn;
use data::weapon::Weapon;

use data::table::Table;

pub struct DataSet {
    // Metadata
    pub weapon_columns: Table<WeaponColumn>,
    // Content data
    pub weapons: Table<Weapon>,
}

impl DataSet {
    pub fn new() -> DataSet {
        DataSet {
            weapon_columns: Table::<WeaponColumn>::empty(),
            weapons: Table::<Weapon>::empty(),
        }
    }
}
