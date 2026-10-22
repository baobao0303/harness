pub mod args;
pub mod errors;
pub mod formatters;
pub mod handlers;
pub mod stubs;

pub use args::{Cli, Command};
pub use errors::*;
pub use formatters::*;
pub use handlers::*;
pub use stubs::*;

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;
    use crate::interface::args::SuperviseAction;
    use clap::CommandFactory;

    #[test]
    fn cli_definition_is_valid() {
        Cli::command().debug_assert();
    }

    #[test]
    fn story_help_documents_proof_command_shape() {
        let mut command = Cli::command();
        let story = command.find_subcommand_mut("story").unwrap();

        let update_help = story
            .find_subcommand_mut("update")
            .unwrap()
            .render_long_help()
            .to_string();
        assert!(update_help.contains("--unit <0|1>"));
        assert!(update_help.contains("--integration <0|1>"));
        assert!(update_help.contains("Proof flags use numeric booleans"));

        let verify_help = story
            .find_subcommand_mut("verify")
            .unwrap()
            .render_long_help()
            .to_string();
        assert!(verify_help.contains("story verify only accepts the story id"));
        assert!(verify_help.contains("Configure proof with story add/update --verify"));
    }

    #[test]
    fn parses_supervise_subcommand() {
        let cli = Cli::parse_from(["harness", "supervise", "evaluate", "--text", "Fix typo in docs"]);
        match cli.command {
            Command::Supervise(args) => match args.action {
                SuperviseAction::Evaluate(eval_args) => {
                    assert_eq!(eval_args.text.as_deref(), Some("Fix typo in docs"));
                }
                _ => panic!("Expected Evaluate action"),
            },
            _ => panic!("Expected Supervise command"),
        }
    }

    #[test]
    fn parses_learn_status_subcommand() {
        let cli = Cli::parse_from(["harness", "learn", "status", "--domain", "frontend"]);
        match cli.command {
            Command::Learn(args) => match args.action {
                crate::interface::args::LearnAction::Status { domain } => {
                    assert_eq!(domain.as_deref(), Some("frontend"));
                }
                _ => panic!("Expected Status action"),
            },
            _ => panic!("Expected Learn command"),
        }
    }

    #[test]
    fn command_help_documents_lane_values_and_version() {
        let mut command = Cli::command();
        assert!(command.render_long_help().to_string().contains("--version"));

        let intake_help = command
            .find_subcommand_mut("intake")
            .unwrap()
            .render_long_help()
            .to_string();
        assert!(intake_help.contains("--lane <tiny|normal|high-risk>"));
        assert!(intake_help.contains("Use tiny instead of low"));

        let story_add_help = command
            .find_subcommand_mut("story")
            .unwrap()
            .find_subcommand_mut("add")
            .unwrap()
            .render_long_help()
            .to_string();
        assert!(story_add_help.contains("--lane <tiny|normal|high-risk>"));

        let backlog_add_help = command
            .find_subcommand_mut("backlog")
            .unwrap()
            .find_subcommand_mut("add")
            .unwrap()
            .render_long_help()
            .to_string();
        assert!(backlog_add_help.contains("--risk <tiny|normal|high-risk>"));
        assert!(backlog_add_help.contains("Accepted lanes"));

        let matrix_help = command
            .find_subcommand_mut("query")
            .unwrap()
            .find_subcommand_mut("matrix")
            .unwrap()
            .render_long_help()
            .to_string();
        assert!(matrix_help.contains("--numeric"));
    }
}
