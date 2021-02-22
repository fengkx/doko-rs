use clap::ArgMatches;

use crate::docker::{self, options::Options};
use std::{io::Result, process::ExitStatus};
pub fn enable(args: &ArgMatches) -> Result<ExitStatus> {
    let (port, tag) = docker::mk_comm_opts(args);

    docker::run(Options {
        name: "redis".to_string(),
        envs: vec![],
        volume: Some("redis_data:/data".to_string()),
        image: format!("redis:{}", tag),
        port: Some(port.to_string()),
    })
}
