//! When "dev" feature is enabled, the Cli will use lightweight implementation
//! of common commands, similar to [`crate::command::dev::Command``].

use crate::command::common::Command;

pub mod prove;
pub mod public_params;
pub mod request;
pub mod run;
pub mod verify;

pub(crate) fn handle_command(cmd: Command) -> anyhow::Result<()> {
    match cmd {
        Command::Run(args) => run::handle_command(args),
        Command::Prove(args) => prove::handle_command(args),
        Command::Request(args) => request::handle_command(args),
        Command::PublicParams(args) => public_params::handle_command(args),
        Command::Verify(args) => verify::handle_command(args),

        _ => unimplemented!(),
    }
}
