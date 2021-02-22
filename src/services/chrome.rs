use clap::ArgMatches;

use crate::docker::{self, options::Options};
use std::{io::Result, process::ExitStatus};
pub fn enable(args: &ArgMatches) -> Result<ExitStatus> {
    let (port, tag) = docker::mk_comm_opts(args);
    docker::run(Options {
        name: "chrome".to_string(),
        envs: vec![],
        volume: None,
        image: format!("browserless/chrome:{}", tag),
        port: Some(port.to_string()),
    })
}
