use std::error::Error;

use clap::Parser;

use crate::vm::Vm;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    bytecode: String,

    #[arg(short, long)]
    verbose: bool,
}

pub trait AppBuilder {
    fn get_args(&self) -> &Args;

    fn build(&self) -> Result<Vm, Box<dyn Error>> {
        Vm::new(&self.get_args().bytecode)
    }
}

impl AppBuilder for Args {
    fn get_args(&self) -> &Args {
        self
    }
}
