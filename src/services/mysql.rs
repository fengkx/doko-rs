use clap::ArgMatches;
use docker::Env;

use crate::docker::{self, options::Options};
use std::{io::Result, process::ExitStatus};
pub fn enable(args: &ArgMatches) -> Result<ExitStatus> {
    let (port, tag) = docker::mk_comm_opts(args);

    docker::run(Options {
        name: "mysql".to_string(),
        envs: vec![Env::Str("MYSQL_ROOT_PASSWORD=pass".to_string())],
        volume: Some("mysql_data:/var/lib/mysql".to_string()),
        image: format!("mysql:{}", tag),
        port: Some(port.to_string()),
    })
}
