use comfy_table::{Table, Row, Cell};   // cargo add comfy-table

pub trait TableRow {
    /// Column headers once per table.
    fn headers() -> Vec<&'static str>;

    /// One row of cells for `self`.
    fn to_row(&self) -> Vec<String> {
        todo!("This feature is in progress")
    }

    /// Zero *or more* rows.  Default = one row produced by `to_row`.
    fn to_rows(&self) -> Vec<Vec<String>> {
        vec![self.to_row()]
    }
}

impl<T: TableRow> TableRow for Vec<T> {
    fn headers() -> Vec<&'static str> {
        T::headers()
    }

    // Not used for collections; required by the trait.
    fn to_row(&self) -> Vec<String> {
        Vec::new()
    }

    // Flatten rows from each element.
    fn to_rows(&self) -> Vec<Vec<String>> {
        self.iter()
            .flat_map(|item| item.to_rows())
            .collect()
    }
}
