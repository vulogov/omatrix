extern crate log;
use shadow_rs::shadow;
shadow!(build);

pub mod setloglevel;

use clap::{Args, Parser, Subcommand};
use lazy_static::lazy_static;
use std::env;
use std::sync::{Mutex, RwLock};
use time_graph;

lazy_static! {
    pub static ref CLI: Mutex<Cli> = {
        let e: Mutex<Cli> = Mutex::new(Cli::parse());
        e
    };
}

fn do_panic() {
    log::debug!("Setting a global panic handler");
    better_panic::Settings::auto()
        .most_recent_first(false)
        .lineno_suffix(true)
        .verbosity(better_panic::Verbosity::Full)
        .install();
}

pub fn main() {
    let cli = Cli::parse();
    setloglevel::setloglevel(&cli);
    do_panic();
    let init_cli = CLI.lock().unwrap();
    log::debug!(
        "OMLOG tool version:{}, tag:{}, branch:{}, commit date: {}, commit author:{}({}), commit_id:{}. Build at {}",
        build::VERSION,
        build::TAG,
        build::BRANCH,
        build::COMMIT_DATE,
        build::COMMIT_AUTHOR,
        build::COMMIT_EMAIL,
        build::COMMIT_HASH,
        build::BUILD_TIME
    );
    log::debug!("Initialize global CLI");
    drop(init_cli);
    log::debug!("OMATRIX tool context initialized ...");

    if cli.profile {
        log::debug!("Enable JBUND profiler");
        time_graph::enable_data_collection(true);
    }

    if cli.profile {
        log::debug!("Generating JBUND profiler report");
        let graph = time_graph::get_full_graph();
        println!("{}", graph.as_table());
    }
}

#[derive(Parser, Clone, Debug)]
#[clap(name = "omatrix")]
#[clap(author = "Vladimir Ulogov <vladimir@ulogtov.us>")]
#[clap(version = env!("CARGO_PKG_VERSION"))]
#[clap(
    about = "OMATRIX - Observability Matrix Tool",
    long_about = "Matrix-based analytical tool for observability and telemetry"
)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[clap(long, action = clap::ArgAction::SetTrue, help="Execute internal profiler")]
    pub profile: bool,
}
