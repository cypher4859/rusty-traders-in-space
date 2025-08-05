use comfy_table::{Table, Row, Cell};   // cargo add comfy-table
use owo_colors::OwoColorize;
use serde::Serialize;
use std::fmt::Debug;

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
