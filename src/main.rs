mod system;
mod config;
mod cli;
mod open;

fn main() {
    let matches = cli::match_cli_arguments();
    if let Some(submatches) = matches.subcommand_matches("system") {
        return system::handle_system_command(submatches);
    }

    if let Some(submatches) = matches.subcommand_matches("config") {
        return config::handle_config_command(submatches);
    }

    if let Some(submatches) = matches.subcommand_matches("open") {
        return open::handle_open_command(submatches);
    }

    if let Some(submatches) = matches.subcommand_matches("tail") {
        return open::handle_tail_command(submatches);
    }
}
