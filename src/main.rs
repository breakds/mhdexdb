extern crate dexdb;
extern crate rusqlite;


use dexdb::table::DexSqlite;
use dexdb::table::Table;
use dexdb::data::weapon::Weapon;

use rusqlite::Connection;
use rusqlite::Statement;

fn get_names(conn: &Connection) -> rusqlite::Result<Vec<String>> {
    let mut stmt = try!(conn.prepare("SELECT Wpn_Name_0 FROM ID_Wpn_Name"));
    let rows = try!(stmt.query_map(&[], |row| row.get(0)));

    let result: Vec<String> = rows.filter_map(|item| {
        item.ok()
    }).collect();

    Ok(result)

    // println!("{:?}", result);

    // let mut names = Vec::new();
    // // for name_result in rows {
    // //     names.push(try!(name_result));
    // // }

    // Ok(names)
}

fn main() {
    let conn = Connection::open_with_flags("/home/breakds/dataset/mhx/mhx.db",
                                           rusqlite::SQLITE_OPEN_READ_ONLY).unwrap();

    // let names = get_names(&conn).unwrap();

    // println!("{}", names[1]);

    let weapons: Table<Weapon> = Table::<Weapon>::new(&conn);

    println!("{:?}", weapons);
}
