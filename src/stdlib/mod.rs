extern crate log;

use crate::cmd::CLI;
use crate::stdlib::common::OMDB;
use lazy_static::lazy_static;
use std::sync::RwLock;

pub mod api;
pub mod common;

lazy_static! {
    pub static ref DB: RwLock<OMDB> = {
        let cli = match CLI.lock() {
            Ok(cli) => cli,
            Err(e) => panic!("Unable to lock CLI: {}", e),
        };
        let db_path = match &cli.store_path {
            Some(path) => format!("{}{}", path, "/omatrix.container"),
            None => panic!("No store path specified"),
        };
        let db = match OMDB::new(Some(db_path.to_string())) {
            Ok(db) => RwLock::new(db),
            Err(e) => panic!("Unable to open database: {}", e),
        };
        log::debug!("OMATRIX database initialized in: {}", db_path.clone());
        db
    };
}
