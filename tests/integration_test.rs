use std::error::Error;

use cubipods::utils::bytes32::Bytes32;

mod common;

#[test]
fn is_args_given() -> Result<(), Box<dyn Error>> {
    let vm = common::setup(["cubipods", "--bytecode", "0x806020"])?;

    assert_eq!(vm.verbose, false);
    assert_eq!(vm.lexer.bytecode, "806020");

    let vm = common::setup(["cubipods", "--bytecode", "0x8060206020", "--verbose"])?;

    assert_eq!(vm.verbose, true);
    assert_eq!(vm.lexer.bytecode, "8060206020");

    Ok(())
}

#[test]
fn test_run_app() -> Result<(), Box<dyn Error>> {
    let mut vm = common::setup(["cubipods", "--bytecode", "0x60206040526002600155"])?;

    vm.run()?;

    assert_eq!(vm.stack.is_empty(), true);

    unsafe {
        let data = vm.memory.load_only("40".parse::<Bytes32>()?).to_string();
        assert_eq!(
            data,
            "0000000000000000000000000000000000000000000000000000000000000020"
        )
    }

    {
        let data = vm
            .storage
            .sload("01".parse::<Bytes32>()?)
            .unwrap()
            .to_string();
        assert_eq!(
            data,
            "0000000000000000000000000000000000000000000000000000000000000002"
        )
    }

    let mut vm = common::setup(["cubipods", "--bytecode", "0x6020602001"])?;

    vm.run()?;

    assert_eq!(vm.stack.peek().unwrap(), "40");

    Ok(())
}

#[test]
fn test_without_bytecode_returns_error() -> Result<(), Box<dyn Error>> {
    assert!(common::setup(["cubipods"]).is_err());

    Ok(())
}
