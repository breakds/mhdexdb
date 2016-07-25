extern crate dexdb;
extern crate rusqlite;
extern crate rustc_serialize;

use dexdb::data::base::{DexSqlite, LangText};
use dexdb::data::table::Table;
use dexdb::data::dataset::DataSet;
use dexdb::data::weapon::Weapon;
use dexdb::data::weapon::WeaponColumn;

use rusqlite::Connection;

use rustc_serialize::json;

fn main() {
    let conn = Connection::open_with_flags("/home/breakds/dataset/mhx/mhx.db",
                                           rusqlite::SQLITE_OPEN_READ_ONLY).unwrap();

    // let names = get_names(&conn).unwrap();

    // println!("{}", names[1]);

    let weapons: Table<Weapon> = Table::<Weapon>::new(&conn);

    println!("{}", weapons.iter().find(|&weapon| weapon.affinity > 0).unwrap());

    // println!("{}", WeaponColumn::new(&Json::from_str("{\"name\": \"haha\", \"label\": {\"ENG\": \"haha\", \"JAP\": \"jap\", \"CHS\": \"hehe\"}}").unwrap()));

    let weapon_columns: Table<WeaponColumn> = Table::<WeaponColumn>::from_json(
        "/home/breakds/pf/projects/mhdexdb/data/metadata/weapon_columns.json");
    println!("{}", weapon_columns);

    let dataset: DataSet = DataSet::new();
    println!("{}", dataset.weapon_columns);
}
