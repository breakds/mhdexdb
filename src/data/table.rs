extern crate rusqlite;

use std::fmt;
use std::io;
use std::io::prelude::*;
use std::slice;
use std::fs::File;
use data::base::{DexSqlite, Indexed};
use rustc_serialize::Decodable;
use rustc_serialize::json;

#[derive(Debug)]
pub struct Table<Row> {
    rows: Vec<Row>,
}

impl<Row> Table<Row> {
    pub fn empty() -> Table<Row> {
        Table {
            rows: vec![],
        }
    }
}

impl<Row: DexSqlite + Indexed> Table<Row> {
    pub fn new(conn: &rusqlite::Connection) -> Table<Row> {
        let mut statement = conn.prepare(Row::statement()).unwrap();
        let rows_iter = statement.query_map(&[], |row| Row::new(row)).unwrap();
        let rows: Vec<Row> = rows_iter.filter_map(
            |item| match item {
                Ok(row) => if row.id() >= 0 {
                    Some(row)
                } else {
                    None
                },
                _ => None,
            }).collect();
        Table {
            rows: rows,
        }
    }
}

impl <Row: Decodable> Table<Row> {
    pub fn from_json(path: &str) -> Table<Row> {
        let mut file = File::open(path).unwrap();
        let mut text = String::new();
        file.read_to_string(&mut text);
        
        let rows: Vec<Row> = json::decode(&text).unwrap();

        Table {
            rows: rows,
        }
    }
}

impl<Row> Table<Row> {
    pub fn get(&self, id: i32) -> &Row {
        &self.rows[id as usize]
    }

    pub fn iter(&self) -> slice::Iter<Row> {
        self.rows.iter()
    }
}

impl<Row: fmt::Display> fmt::Display for Table<Row> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.rows {
            writeln!(f, "{}", &row);
        }
        writeln!(f, "---")
    }
}
