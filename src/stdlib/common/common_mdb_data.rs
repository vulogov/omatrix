extern crate log;

use crate::stdlib::common::MATRIXDB;
use easy_error::{Error, bail};

const DATA_CREATE: &str = r#"
CREATE TABLE IF NOT EXISTS  Data (
    seq         INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp   FLOAT,
    id          STRING,
    data        BLOB
);
CREATE INDEX IF NOT EXISTS data_idx1 ON Data (timestamp);
CREATE INDEX IF NOT EXISTS data_idx2 ON Data (id);
"#;

const DATA_DROP_RAW_TABLE: &str = r#"
DROP TABLE IF EXISTS Data;
DROP INDEX IF EXISTS data_idx1;
DROP INDEX IF EXISTS data_idx2;
"#;

impl MATRIXDB {
    pub fn init_data_tables(&mut self) -> Result<(), Error> {
        let _ = match self.conn.execute_batch(DATA_CREATE) {
            Ok(_) => {}
            Err(err) => {
                bail!("Error creating Data tables structure: {}", err);
            }
        };
        Ok(())
    }

    pub fn drop_data_tables(&mut self) -> Result<(), Error> {
        let _ = match self.conn.execute_batch(DATA_DROP_RAW_TABLE) {
            Ok(_) => {}
            Err(err) => {
                bail!("Error dropping Data tables structure: {}", err);
            }
        };
        Ok(())
    }
    pub fn recreate_data_tables(&mut self) -> Result<(), Error> {
        match self.drop_data_tables() {
            Ok(_) => {}
            Err(err) => {
                bail!("Error dropping Data tables structure: {}", err);
            }
        };
        match self.init_data_tables() {
            Ok(_) => {}
            Err(err) => {
                bail!("Error creating Data tables structure: {}", err);
            }
        };
        Ok(())
    }
}
