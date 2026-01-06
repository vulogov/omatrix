extern crate log;
use easy_error::{Error, bail};
use std::sync::Mutex;

use crate::stdlib::common::{MATRIXDB, OMDB};

impl OMDB {
    pub fn new(path: Option<String>) -> Result<Self, Error> {
        let mdb = match MATRIXDB::new(path.clone()) {
            Ok(mdb) => mdb,
            Err(err) => {
                bail!("{}", err);
            }
        };
        Ok(OMDB {
            path,
            mdb: Mutex::new(mdb),
        })
    }

    pub fn reinitialize(&self) -> Result<(), Error> {
        let mut mdb = match self.mdb.lock() {
            Ok(mdb) => mdb,
            Err(err) => {
                bail!("{}", err);
            }
        };
        match mdb.recreate_table_structure() {
            Ok(_) => {
                log::debug!("Database has been reinitialized");
            }
            Err(err) => {
                drop(mdb);
                bail!("{}", err);
            }
        }
        drop(mdb);
        Ok(())
    }
}
