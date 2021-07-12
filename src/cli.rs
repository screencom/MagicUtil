use clap::{Arg, App, SubCommand, ArgMatches, crate_authors};

/// Matches the CLI arguments and returns an object containing the values.
pub fn match_cli_arguments() -> ArgMatches<'static> {
    App::new("MagicUtil")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Released under the MIT license.\n\nUseful utilities on a Samsung MagicINFO server for sysadmin tasks.")
        .author(crate_authors!("\n"))
        .subcommand(SubCommand::with_name("system")
            .about("Query system properties used in MagicINFO")
            .subcommand(SubCommand::with_name("hwunique")
                .about("Prints the hardware unique calculated from properties of this system")
                .arg(Arg::with_name("json")
                    .help("Setting this value returns the property values as json")
                    .long("json")
                ))
            .subcommand(SubCommand::with_name("macaddress")
                .about("Prints the system's MAC address")
                .arg(Arg::with_name("json")
                    .help("Setting this value returns the property values as json")
                    .long("json")
                ))
            .subcommand(SubCommand::with_name("boardid")
                .about("Prints the system's board ID")
                .arg(Arg::with_name("json")
                    .help("Setting this value returns the property values as json")
                    .long("json")
                ))
            .subcommand(SubCommand::with_name("ipaddress")
                .about("Prints the system's local ipaddress")
                .arg(Arg::with_name("json")
                    .help("Setting this value returns the property values as json")
                    .long("json")
                ))
        )
        .subcommand(SubCommand::with_name("info")
            .about("Utilities based on retrieving information from the system")
            .subcommand(SubCommand::with_name("magicinfo")
                .about("Returns MagicINFO related information")
                .arg(Arg::with_name("json")
                    .help("Setting this value returns the property values as json")
                    .long("json")
                ))
            .subcommand(SubCommand::with_name("database")
                .about("Returns information about the database")
                .arg(Arg::with_name("json")
                    .help("Setting this value returns the property values as json")
                    .long("json")
                ))
            .subcommand(SubCommand::with_name("service")
                .about("Returns information about MagicINFO service")
                .arg(Arg::with_name("json")
                    .help("Setting this value returns the property values as json")
                    .long("json")
                ))
            .subcommand(SubCommand::with_name("system")
                .about("Returns information about the system MagicINFO runs on")
                .arg(Arg::with_name("json")
                    .help("Setting this value returns the property values as json")
                    .long("json")
                ))
            .subcommand(SubCommand::with_name("all")
                .about("Returns all usefull information")
                .arg(Arg::with_name("json")
                    .help("Setting this value returns the property values as json")
                    .long("json")
                ))
        )
        .subcommand(SubCommand::with_name("service")
            .about("Utilities based on the MagicINFO Windows service")
            .subcommand(SubCommand::with_name("status")
                .about("Returns the current service status")
                .arg(Arg::with_name("json")
                    .help("Setting this value returns the value as json")
                    .long("json")
                ))
            .subcommand(SubCommand::with_name("start")
                .about("Stats the MagicINFO service")
                .arg(Arg::with_name("available")
                    .help("Waits until the HTTP service is available")
                    .long("available")
                )
                .arg(Arg::with_name("silent")
                    .help("Disables output to stdout")
                    .long("silent")
                )
            )
            .subcommand(SubCommand::with_name("stop")
                .about("Stops the MagicINFO service")
                .arg(Arg::with_name("silent")
                    .help("Disables output to stdout")
                    .long("silent")
                )
            )
            .subcommand(SubCommand::with_name("restart")
                .about("Restarts the MagicINFO service")
                .arg(Arg::with_name("available")
                    .help("Waits until the HTTP service is available")
                    .long("available")
                )
                .arg(Arg::with_name("silent")
                    .help("Disables output to stdout")
                    .long("silent")
                )
            )
            .subcommand(SubCommand::with_name("available")
                .about("Checks if the MagicINFO web interface is available")
                .arg(Arg::with_name("json")
                    .help("Setting this value returns the property values as json")
                    .long("json")
                )
            )
        )
        .subcommand(SubCommand::with_name("config")
            .about("Get, set, replace or remove properties from MagicINFO's main config.properties file")
            .subcommand(SubCommand::with_name("get")
                .about("Returns one or more configuration properties")
                .arg(Arg::with_name("PROPERTY")
                    .multiple(true)
                    .required(true)
                    .takes_value(true)
                )
                .arg(Arg::with_name("json")
                    .help("Setting this value returns the property values as json")
                    .long("json")
                )
                .arg(Arg::with_name("decrypt")
                    .help("Setting this flag will try to decrypt the encrypted values")
                    .long("decrypt")
                )
            )
            .subcommand(SubCommand::with_name("set")
                .about("Sets a configuration property")
                .arg(Arg::with_name("KEY")
                    .required(true)
                    .takes_value(true)
                )
                .arg(Arg::with_name("VALUE")
                    .required(true)
                    .takes_value(true)
                )
                .arg(Arg::with_name("encrypt")
                    .help("Setting this flag will encrypt the value before writing it to the config.properties file")
                    .long("encrypt")
                )
            )
            .subcommand(SubCommand::with_name("remove")
                .about("Removes a configuration property")
                .arg(Arg::with_name("KEY")
                    .required(true)
                    .takes_value(true)
                )
            )
            .subcommand(SubCommand::with_name("replace")
                .about("Changes a configuration property by replacing part of the existing value")
                .arg(Arg::with_name("KEY")
                    .required(true)
                    .takes_value(true)
                )
                .arg(Arg::with_name("SEARCH")
                    .required(true)
                    .takes_value(true)
                )
                .arg(Arg::with_name("REPLACE")
                    .required(true)
                    .takes_value(true)
                )
            )
        )
        .subcommand(SubCommand::with_name("open")
            .about("Tries to open the given file")
            .arg(Arg::with_name("FILE")
                .required(true)
                .takes_value(true)
            )
        )
        .subcommand(SubCommand::with_name("tail")
            .about("Tries to tail and follow the given file")
            .arg(Arg::with_name("FILE")
                .required(true)
                .takes_value(true)
            )
        )
        .subcommand(SubCommand::with_name("bcrypt")
            .about("Utilities based on MagicINFO's bcrypt hashing algorithm used to store password")
            .subcommand(SubCommand::with_name("hash")
                .about("Hashes the given plaintext with the bcrypt algorithm")
                .arg(Arg::with_name("PLAINTEXT")
                    .required(true)
                    .takes_value(true)
                )
            )
        )
        .get_matches()
}