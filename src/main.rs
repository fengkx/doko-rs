use clap::Shell;
use errors::DockerRunErr;

mod cli;
pub mod docker;
pub mod errors;
mod services;

fn main() {
    let services = vec!["postgres", "mysql", "redis", "chrome"];
    let app = cli::build_cli(&services);
    let matches = &app.get_matches();
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
                    Err(DockerRunErr::IoErr(err)) => {
                        eprintln!("{}", err);
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
        ("completion", Some(scm)) => {
            let shell = scm.value_of("shell").unwrap().parse::<Shell>().unwrap();
            let mut stdout = std::io::stdout();
            cli::build_cli(&services).gen_completions_to(
                env!("CARGO_PKG_NAME"),
                shell,
                &mut stdout,
            );
        }
        _ => {}
    };
}
