extern crate rusqlite;

pub trait DexSqlite {
    fn statement() -> &'static str;
    fn new(&rusqlite::Row) -> Self;
}

pub trait IndexedRow {
    fn id(&self) -> i32;
    fn dex_id(&self) -> i32;
    fn to_dex(i32) -> i32;
    fn to_id(i32) -> i32;
}

#[derive(Debug)]
pub struct Table<Row> {
    rows: Vec<Row>,
}

impl<Row: DexSqlite + IndexedRow> Table<Row> {
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

impl<Row> Table<Row> {
    pub fn get(&self, id: i32) -> &Row {
        &self.rows[id as usize]
    }
}

#[derive(Debug)]
pub enum Ref {
    Id(i32),
    DexId(i32),
}
