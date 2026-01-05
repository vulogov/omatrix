extern crate log;

use crate::stdlib::common::MATRIXDB;
use easy_error::{Error, bail};

const META_DATA_CREATE: &str = r#"
CREATE TABLE IF NOT EXISTS  Metadata (
    name        STRING PRIMARY KEY,
    data        BLOB
);
"#;

const DATA_DROP_RAW_TABLE: &str = r#"
DROP TABLE IF EXISTS Metadata;
"#;

impl MATRIXDB {
    pub fn init_metadata_tables(&mut self) -> Result<(), Error> {
        let _ = match self.conn.execute_batch(META_DATA_CREATE) {
            Ok(_) => {}
            Err(err) => {
                bail!("Error creating Metadata tables structure: {}", err);
            }
        };
        Ok(())
    }

    pub fn drop_metadata_tables(&mut self) -> Result<(), Error> {
        let _ = match self.conn.execute_batch(DATA_DROP_RAW_TABLE) {
            Ok(_) => {}
            Err(err) => {
                bail!("Error dropping Metadata tables structure: {}", err);
            }
        };
        Ok(())
    }
    pub fn recreate_metadata_tables(&mut self) -> Result<(), Error> {
        match self.drop_metadata_tables() {
            Ok(_) => {}
            Err(err) => {
                bail!("Error dropping Metadata tables structure: {}", err);
            }
        };
        match self.init_metadata_tables() {
            Ok(_) => {}
            Err(err) => {
                bail!("Error creating Metadata tables structure: {}", err);
            }
        };
        Ok(())
    }
}
