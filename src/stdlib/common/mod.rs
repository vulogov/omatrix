extern crate log;

use duckdb::Connection;

pub mod common_mdb;
pub mod common_mdb_catalog;
pub mod common_mdb_data;
pub mod common_mdb_metadata;

pub struct MATRIXDB {
    pub path: Option<String>,
    pub conn: Connection,
}
