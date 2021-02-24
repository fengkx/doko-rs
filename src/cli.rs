use clap::{App, AppSettings, Arg, Shell, SubCommand};
pub fn build_cli(services: &[&str]) -> App<'static, 'static> {
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
    App::new("doko")
        .version("v0.1")
        .about("A docker-based development dependency manager")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("completion")
                .about("generate shell completion")
                .setting(AppSettings::DisableHelpSubcommand)
                .setting(AppSettings::DisableVersion)
                .arg(
                    Arg::with_name("shell")
                        .short("s")
                        .long("shell")
                        .help("shell name")
                        .takes_value(true)
                        .possible_values(&Shell::variants())
                        .required(true),
                ),
        )
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
                .about("disable services")
                .setting(AppSettings::DisableVersion)
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommands(services.iter().map(|s| SubCommand::with_name(s))),
        )
}
