extern crate rusqlite;

use std::slice;
use data::base::{DexSqlite, Indexed};

#[derive(Debug)]
pub struct Table<Row> {
    rows: Vec<Row>,
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

    pub fn iter(&self) -> slice::Iter<Row> {
        self.rows.iter()
    }
}

impl<Row> Table<Row> {
    pub fn get(&self, id: i32) -> &Row {
        &self.rows[id as usize]
    }
}
