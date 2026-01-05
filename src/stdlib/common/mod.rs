extern crate log;

use duckdb::Connection;
use easy_error::Error;

pub mod common_mdb;
pub mod common_mdb_metadata;

pub struct MATRIXDB {
    pub path: Option<String>,
    pub conn: Connection,
}
