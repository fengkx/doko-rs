use clap::{App, AppSettings, Arg, SubCommand};
use errors::DockerRunErr;

pub mod docker;
pub mod errors;
mod services;

fn main() {
    let services = ["postgres", "mysql", "redis", "chrome"];
    let image_tag_arg = Arg::with_name("image_tag")
        .short("t")
        .long("tag")
        .help("docker image tag");
    let port_arg = Arg::with_name("port")
        .short("p")
        .long("port")
        .help("port forwarding");
    let volume_arg = Arg::with_name("volume")
        .short("v")
        .long("volume")
        .help("volume path on host");
    let force_arg = Arg::with_name("force")
        .long("force")
        .help("force to run ignoring old image_tag")
        .takes_value(false);
    let matches = App::new("doko")
        .version("v0.1")
        .about("A docker-based development dependency manager")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("enable")
                .setting(AppSettings::NoBinaryName)
                .setting(AppSettings::ArgRequiredElseHelp)
                .setting(AppSettings::DisableVersion)
                .setting(AppSettings::DisableHelpSubcommand)
                .about("enable service")
                .subcommand(
                    SubCommand::with_name("postgres")
                        .about("PostgreSQL docker service")
                        .arg(image_tag_arg.clone().default_value("12-alpine"))
                        .arg(port_arg.clone().default_value("5432"))
                        .arg(volume_arg.clone().default_value("postgres_data"))
                        .arg(force_arg.clone()),
                )
                .subcommand(
                    SubCommand::with_name("mysql")
                        .about("MySQL docker service")
                        .arg(image_tag_arg.clone().default_value("8"))
                        .arg(port_arg.clone().default_value("3306"))
                        .arg(volume_arg.clone().default_value("mysql_data")),
                )
                .subcommand(
                    SubCommand::with_name("redis")
                        .about("Redis docker service")
                        .arg(image_tag_arg.clone().default_value("6-alpine"))
                        .arg(port_arg.clone().default_value("6379"))
                        .arg(volume_arg.clone().default_value("redis_data")),
                )
                .subcommand(
                    SubCommand::with_name("chrome")
                        .about("Headless Chrome docker service")
                        .arg(image_tag_arg.clone().default_value("latest"))
                        .arg(port_arg.clone().default_value("6343")),
                ),
        )
        .subcommand(
            SubCommand::with_name("disable")
                .setting(AppSettings::DisableVersion)
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommands(services.iter().map(|s| SubCommand::with_name(s))),
        )
        .get_matches();
    match matches.subcommand() {
        ("enable", Some(scm)) => match scm.subcommand() {
            ("postgres", Some(sscm)) => {
                match services::postgres::enable(sscm) {
                    Ok(_) => {
                        println!(
                            "PostgreSQL is listening on port {}",
                            sscm.value_of("port").unwrap()
                        );
                        services::lock_image_tag("postgres", sscm.value_of("image_tag").unwrap());
                    }
                    Err(DockerRunErr::VersionConflict) => {
                        eprintln!("postgres tag is different from the old one, You should clean the volumes first");
                        eprintln!("use --force to override the old image tag")
                    }
                    Err(DockerRunErr::RunErr) => {}
                };
            }
            ("mysql", Some(sscm)) => {
                services::mysql::enable(sscm).unwrap();
                println!(
                    "MySQL is listening on port {}",
                    sscm.value_of("port").unwrap()
                );
            }
            ("redis", Some(sscm)) => {
                services::redis::enable(sscm).unwrap();
                println!(
                    "Redis is listening on port {}",
                    sscm.value_of("port").unwrap()
                );
            }
            ("chrome", Some(sscm)) => {
                services::chrome::enable(sscm).unwrap();
                println!(
                    "Chrome is listening on port {}",
                    sscm.value_of("port").unwrap()
                );
            }
            _ => {}
        },
        ("disable", Some(scm)) => {
            if let Some(srv) = scm.subcommand_name() {
                if services.contains(&srv) {
                    docker::stop(srv).unwrap();
                } else {
                    println!("avaliable services: {}", services.join(", "))
                }
            }
        }
        _ => {}
    };
}
