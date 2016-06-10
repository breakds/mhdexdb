extern crate rusqlite;

pub trait DexSqlite {
    fn statement() -> &'static str;
    fn new(&rusqlite::Row) -> Self;
}

// trait DexTableRow {
//     fn id(&self) -> i32;
//     fn dex_id(&self) -> i32;
//     fn name(&self) -> String;
// }

#[derive(Debug)]
pub struct Table<Row> {
    rows: Vec<Row>,
}

impl<Row: DexSqlite> Table<Row> {
    pub fn new(conn: &rusqlite::Connection) -> Table<Row> {
        let mut statement = conn.prepare(Row::statement()).unwrap();
        let rows_iter = statement.query_map(&[], |row| Row::new(row)).unwrap();
        let rows: Vec<Row> = rows_iter.filter_map(|item| item.ok()).collect();
        Table {
            rows: rows,
        }
    }
}

#[derive(Debug)]
pub enum Ref {
    Id(i32),
    DexId(i32),
}
