extern crate log;

use crate::stdlib::common::MATRIXDB;
use easy_error::{Error, bail};

const CATALOG_DATA_CREATE: &str = r#"
CREATE TABLE IF NOT EXISTS  Sources (
    id          STRING PRIMARY KEY,
    name        STRING,
);
CREATE TABLE IF NOT EXISTS  Keys (
    id          STRING PRIMARY KEY,
    name        STRING,
    source      STRING,
);
CREATE INDEX IF NOT EXISTS catalog_idx1 ON Sources (name);
CREATE INDEX IF NOT EXISTS catalog_idx2 ON Keys (name);
CREATE INDEX IF NOT EXISTS catalog_idx3 ON Keys (source,name);
"#;

const CATALOG_DROP_RAW_TABLE: &str = r#"
DROP TABLE IF EXISTS Sources;
DROP TABLE IF EXISTS Keys;
DROP INDEX IF EXISTS catalog_idx1;
DROP INDEX IF EXISTS catalog_idx2;
DROP INDEX IF EXISTS catalog_idx3;
"#;

impl MATRIXDB {
    pub fn init_catalog_tables(&mut self) -> Result<(), Error> {
        let _ = match self.conn.execute_batch(CATALOG_DATA_CREATE) {
            Ok(_) => {}
            Err(err) => {
                bail!("Error creating Catalog tables structure: {}", err);
            }
        };
        Ok(())
    }

    pub fn drop_catalog_tables(&mut self) -> Result<(), Error> {
        let _ = match self.conn.execute_batch(CATALOG_DROP_RAW_TABLE) {
            Ok(_) => {}
            Err(err) => {
                bail!("Error dropping Catalog tables structure: {}", err);
            }
        };
        Ok(())
    }
    pub fn recreate_catalog_tables(&mut self) -> Result<(), Error> {
        match self.drop_catalog_tables() {
            Ok(_) => {}
            Err(err) => {
                bail!("Error dropping Catalog tables structure: {}", err);
            }
        };
        match self.init_catalog_tables() {
            Ok(_) => {}
            Err(err) => {
                bail!("Error creating Catalog tables structure: {}", err);
            }
        };
        Ok(())
    }
}
