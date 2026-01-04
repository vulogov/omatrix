extern crate log;
use crate::cmd::Cli;
use crate::cmd::omatrix_display_banner;

#[time_graph::instrument]
pub fn run(_: &Cli) {
    log::debug!("VERSION::run() reached");
    println!("{}", omatrix_display_banner::omatrix_banner());
}
