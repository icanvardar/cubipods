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
        let args = self.get_args();
        Vm::new(&args.bytecode, args.verbose)
    }
}

impl AppBuilder for Args {
    fn get_args(&self) -> &Args {
        self
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::OsString;

    use super::*;

    #[test]
    fn it_initializes_args() -> Result<(), Box<dyn Error>> {
        let args = get_mock_args(&["cubipods", "--bytecode", "0x600160026003610101"])?;

        let mut vm = args.build()?;

        vm.run()?;

        assert_eq!(vm.stack.is_empty(), false);
        assert_eq!(vm.stack.length, 4);
        assert_eq!(vm.stack.peek().unwrap(), "0101");
        assert_eq!(args.verbose, false);

        Ok(())
    }

    // NOTE: helper function to create a mock args instance
    fn get_mock_args<I, T>(itr: I) -> Result<Args, Box<dyn Error>>
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        return Ok(Args::try_parse_from(itr)?);
    }
}
