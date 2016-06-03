trait TableRow {
    fn id(&self) -> i32;
    fn dex_id(&self) -> i32;
    fn name(&self) -> String;
}

trait RowIndexer {
    
}

struct Table<Row> {
    rows: Vec<Row>,
}

#[derive(Debug)]
pub enum Ref {
    Id(i32),
    DexId(i32),
}
