use super::{get_lock_file_path, lock_image_tag};
use crate::{
    docker::{self, options::Options},
    errors,
};
use clap::ArgMatches;
use docker::Env;
use errors::DockerRunErr;
use std::result::Result;
use std::{fs::File, io::Read, process::ExitStatus};

pub fn enable(args: &ArgMatches) -> Result<ExitStatus, DockerRunErr> {
    let (port, tag) = docker::mk_comm_opts(args);
    let mut f = File::open(get_lock_file_path("postgres"))
        .unwrap_or_else(|_| lock_image_tag("postgres", tag));
    let mut old_tag = String::new();
    f.read_to_string(&mut old_tag)?;
    let old_tag = old_tag;
    if args.is_present("force") || old_tag == tag {
        match docker::run(Options {
            name: "postgres".to_string(),
            envs: vec![Env::Str("POSTGRES_PASSWORD=pass".to_string())],
            volume: Some("postgres_data:/var/lib/postgresql/data".to_string()),
            image: format!("postgres:{}", tag),
            port: Some(port.to_string()),
        }) {
            Ok(exit_status) => {
                if exit_status.success() {
                    Ok(exit_status)
                } else {
                    Err(DockerRunErr::RunErr)
                }
            }
            Err(_) => Err(DockerRunErr::RunErr),
        }
    } else {
        Err(DockerRunErr::VersionConflict)
    }
}
