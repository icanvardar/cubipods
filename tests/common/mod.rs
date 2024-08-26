use std::{error::Error, ffi::OsString};

use clap::Parser;
use cubipods::{
    utils::cli::{AppBuilder, Args},
    vm::Vm,
};

pub fn setup<'a, I, T>(args: I) -> Result<Vm<'a>, Box<dyn Error>>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let args = Args::try_parse_from(args)?;
    let vm = Box::leak(Box::new(args)).build()?;

    Ok(vm)
}
