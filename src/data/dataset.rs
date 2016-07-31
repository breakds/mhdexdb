use data::weapon::WeaponColumn;
use data::weapon::Weapon;

use data::table::Table;

use rustc_serialize::Decodable;

pub struct DataSet {
    // Metadata
    pub weapon_columns: Table<WeaponColumn>,
    // Content data
    pub weapons: Table<Weapon>,
}

impl DataSet {
    pub fn new(metadata_directory: &str) -> DataSet {

        let weapon_columns = Table::<WeaponColumn>::from_json(
            &DataSet::metadata_path(metadata_directory, "weapon_columns.json"));
        DataSet {
            weapon_columns: weapon_columns,
            weapons: Table::<Weapon>::empty(),
        }
    }

    fn metadata_path(metadata_directory: &str, json_file: &str) -> String {
        metadata_directory.to_string() + "/" + json_file
    }
}
