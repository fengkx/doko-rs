use std::{
    io,
    process::{Command, ExitStatus, Stdio},
};

pub mod options {

    #[derive(Debug)]
    pub enum Env {
        Kv { key: String, value: String },
        Str(String),
    }

    #[derive(Debug)]
    pub struct Options {
        pub name: String,
        pub port: Option<String>,
        pub image: String,
        pub envs: Vec<Env>,
        pub volume: Option<String>,
    }

    // impl<'a> From<&ArgMatches<'a>> for Options {
    //     fn from(m: &ArgMatches) -> Self {
    //         let port = m.value_of("port").unwrap();
    //         let tag = m.value_of("image-tag").unwrap();
    //         Self {
    //             name: String::new(),
    //             port: Some(port.to_string()),
    //             image: String::new(),
    //             envs: vec![],
    //             volume: None,
    //         }
    //     }
    // }
}

use clap::ArgMatches;
pub use options::{Env, Options};

pub fn mk_comm_opts<'a>(args: &'a ArgMatches) -> (&'a str, &'a str) {
    let port = args.value_of("port").unwrap();
    let tag = args.value_of("image_tag").unwrap();
    (port, tag)
}

pub fn run(opt: Options) -> io::Result<ExitStatus> {
    let args = ["run", "--name", &opt.name, "--rm", "-d"];
    let mut command = Command::new("docker");
    command.args(&args);
    if let Some(port) = &opt.port {
        let port = if port.contains(':') {
            String::from(port)
        } else {
            format!("{0}:{0}", port)
        };
        command.arg("-p").arg(&port);
    }
    if let Some(volume) = &opt.volume {
        command.arg("-v").arg(volume);
    }
    for env in opt.envs {
        let env_str = match env {
            Env::Str(s) => s,
            Env::Kv { key, value } => format!("{}={}", key, value),
        };
        command.arg("-e").arg(env_str);
    }
    command.arg(&opt.image);
    command.stdout(Stdio::null()).status()
}

pub fn stop(name: &str) -> io::Result<ExitStatus> {
    Command::new("docker")
        .arg("stop")
        .arg(name)
        .stdout(Stdio::null())
        .status()
}
