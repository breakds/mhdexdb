extern crate dexdb;
extern crate mio;
extern crate rusqlite;
extern crate rustc_serialize;

use dexdb::data::base::{DexSqlite, LangText};
use dexdb::data::table::Table;
use dexdb::data::dataset::DataSet;
use dexdb::data::weapon::{WeaponColumn, WeaponType, Weapon};
use dexdb::server::DexDataServer;

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

    let dataset: DataSet = DataSet::new("/home/breakds/pf/projects/mhdexdb/data/metadata");
    println!("{:?}", dataset.switch_axe_phials);

    // -------------------- Setting up the server. --------------------
    let mut server = DexDataServer::new_simple("127.0.0.1:12345");
    server.run(1024);
}
