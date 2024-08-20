use std::error::Error;

use cubipods::vm::Vm;

fn main() -> Result<(), Box<dyn Error>> {
    let mut vm = Vm::new("0xffff")?;

    vm.run()?;

    Ok(())
}
