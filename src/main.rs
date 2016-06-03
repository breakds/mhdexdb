extern crate dexdb;
extern crate rusqlite;

use dexdb::data::quest;

use rusqlite::Connection;
use rusqlite::Statement;

fn get_names(conn: &Connection) -> rusqlite::Result<Vec<String>> {
    let mut stmt = try!(conn.prepare("SELECT Wpn_Name_0 FROM ID_Wpn_Name"));
    let rows = try!(stmt.query_map(&[], |row| row.get(0)));
    
    let mut names = Vec::new();
    for name_result in rows {
        names.push(try!(name_result));
    }

    Ok(names)
}

fn main() {
    let conn = Connection::open_with_flags("/home/breakds/dataset/mhx/mhx.db",
                                           rusqlite::SQLITE_OPEN_READ_ONLY).unwrap();

    let names = get_names(&conn).unwrap();

    println!("{}", names[1]);
}
