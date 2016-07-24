extern crate dexdb;
extern crate rusqlite;


use dexdb::data::base::DexSqlite;
use dexdb::data::table::Table;
use dexdb::data::weapon::Weapon;

use rusqlite::Connection;

fn main() {
    let conn = Connection::open_with_flags("/home/breakds/dataset/mhx/mhx.db",
                                           rusqlite::SQLITE_OPEN_READ_ONLY).unwrap();

    // let names = get_names(&conn).unwrap();

    // println!("{}", names[1]);

    let weapons: Table<Weapon> = Table::<Weapon>::new(&conn);

    println!("{}", weapons.iter().find(|&weapon| weapon.affinity > 0).unwrap());
}
