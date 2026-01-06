extern crate log;

use duckdb::Connection;
use std::sync::Mutex;

pub mod common_mdb;
pub mod common_mdb_catalog;
pub mod common_mdb_data;
pub mod common_mdb_metadata;
pub mod common_omdb;

pub struct MATRIXDB {
    pub path: Option<String>,
    pub conn: Connection,
}

pub struct OMDB {
    pub path: Option<String>,
    pub mdb: Mutex<MATRIXDB>,
}
