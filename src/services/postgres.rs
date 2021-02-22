use clap::ArgMatches;
use docker::Env;

use crate::docker::{self, options::Options};
use std::{io::Result, process::ExitStatus};
pub fn enable(args: &ArgMatches) -> Result<ExitStatus> {
    let (port, tag) = docker::mk_comm_opts(args);
    docker::run(Options {
        name: "postgres".to_string(),
        envs: vec![Env::Str("POSTGRES_PASSWORD=pass".to_string())],
        volume: Some("postgres_data:/var/lib/postgresql/data".to_string()),
        image: format!("postgres:{}", tag),
        port: Some(port.to_string()),
    })
}
