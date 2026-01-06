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
}
