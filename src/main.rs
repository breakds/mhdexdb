extern crate dexdb;
extern crate rusqlite;


use dexdb::table::DexSqlite;
use dexdb::table::Table;
use dexdb::data::weapon::Weapon;

use rusqlite::Connection;
use rusqlite::Statement;

fn main() {
    let conn = Connection::open_with_flags("/home/breakds/dataset/mhx/mhx.db",
                                           rusqlite::SQLITE_OPEN_READ_ONLY).unwrap();

    // let names = get_names(&conn).unwrap();

    // println!("{}", names[1]);

    let weapons: Table<Weapon> = Table::<Weapon>::new(&conn);

    println!("{:?}", weapons.get(2));
}
