use std::error::Error;

use clap::Parser;
use cubipods::utils::cli::{AppBuilder, Args};

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let mut vm = args.build()?;

    vm.run()?;

    if vm.verbose {
        vm.history.summarize();
        vm.history.analyze(&vm);
    }

    Ok(())
}
