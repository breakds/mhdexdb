use data::weapon::{WeaponColumn, WeaponType, Weapon};

use data::table::Table;

pub struct DataSet {
    // Metadata
    pub weapon_columns: Table<WeaponColumn>,
    pub weapon_types: Table<WeaponType>,

    // Content data
    pub weapons: Table<Weapon>,
}

impl DataSet {
    pub fn new(metadata_directory: &str) -> DataSet {

        let weapon_columns = Table::<WeaponColumn>::from_json(
            &DataSet::metadata_path(metadata_directory, "weapon_columns.json"));

        let weapon_types = Table::<WeaponType>::from_json_context(
            &DataSet::metadata_path(metadata_directory, "weapon_types.json"),
            &weapon_columns);

        DataSet {
            weapon_columns: weapon_columns,
            weapon_types: weapon_types,
            weapons: Table::<Weapon>::empty(),
        }
    }

    fn metadata_path(metadata_directory: &str, json_file: &str) -> String {
        metadata_directory.to_string() + "/" + json_file
    }
}
