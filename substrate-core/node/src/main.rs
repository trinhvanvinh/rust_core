mod benchmarking;
mod chain_spec;
mod cli;
mod command;
mod service;

fn main() -> sc_cli::Result<()> {
    command::run()
}
