extern crate log;

use crate::stdlib::common::MATRIXDB;
use duckdb::Connection;
use easy_error::{Error, bail};

impl MATRIXDB {
    pub fn new(path: Option<String>) -> Result<MATRIXDB, Error> {
        match path {
            Some(path) => {
                let conn = match Connection::open(&path) {
                    Ok(conn) => conn,
                    Err(err) => bail!("Failed to open database: {}", err),
                };
                Ok(MATRIXDB {
                    path: Some(path),
                    conn,
                })
            }
            None => {
                let conn = match Connection::open_in_memory() {
                    Ok(conn) => conn,
                    Err(err) => bail!("Failed to open in-memory database: {}", err),
                };
                Ok(MATRIXDB { path: None, conn })
            }
        }
    }
    pub fn create_table_structure(&mut self) -> Result<(), Error> {
        let _ = match self.init_metadata_tables() {
            Ok(_) => {}
            Err(err) => {
                bail!("{}", err);
            }
        };
        Ok(())
    }

    pub fn recreate_table_structure(&mut self) -> Result<(), Error> {
        let _ = match self.recreate_metadata_tables() {
            Ok(_) => {}
            Err(err) => {
                bail!("{}", err);
            }
        };
        Ok(())
    }
}
