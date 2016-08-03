use data::weapon::{SharpnessColor, WeaponColumn, WeaponType, Weapon,
                   SpecialType, WeaponPhial};

use data::table::Table;

pub struct DataSet {
    // Metadata
    pub sharpness_colors: Table<SharpnessColor>,
    pub weapon_columns: Table<WeaponColumn>,
    pub weapon_types: Table<WeaponType>,
    pub special_types: Table<SpecialType>,
    pub charge_blade_phials: Table<WeaponPhial>,
    pub switch_axe_phials: Table<WeaponPhial>,

    // Content data
    pub weapons: Table<Weapon>,
}

impl DataSet {
    pub fn new(metadata_directory: &str) -> DataSet {

        let sharpness_colors = Table::<SharpnessColor>::from_json(
            &DataSet::metadata_path(metadata_directory, "sharpness_colors.json"));

        let weapon_columns = Table::<WeaponColumn>::from_json(
            &DataSet::metadata_path(metadata_directory, "weapon_columns.json"));

        let weapon_types = Table::<WeaponType>::from_json_context(
            &DataSet::metadata_path(metadata_directory, "weapon_types.json"),
            &weapon_columns);

        let special_types = Table::<SpecialType>::from_json(
            &DataSet::metadata_path(metadata_directory, "special_types.json"));

        let charge_blade_phials = Table::<WeaponPhial>::from_json(
            &DataSet::metadata_path(metadata_directory, "charge_blade_phials.json"));

        let switch_axe_phials = Table::<WeaponPhial>::from_json(
            &DataSet::metadata_path(metadata_directory, "switch_axe_phials.json"));
        
        DataSet {
            sharpness_colors: sharpness_colors,
            weapon_columns: weapon_columns,
            weapon_types: weapon_types,
            special_types: special_types,
            charge_blade_phials: charge_blade_phials,
            switch_axe_phials: switch_axe_phials,
            
            weapons: Table::<Weapon>::empty(),
        }
    }

    fn metadata_path(metadata_directory: &str, json_file: &str) -> String {
        metadata_directory.to_string() + "/" + json_file
    }
}
