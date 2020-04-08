#[allow(unused)]
use tokio::prelude::*;
use {std::error::Error, tokio::runtime::Runtime};

mod sensor;
mod structs;
mod utils;

#[paw::main]
fn main(args: structs::Args) -> Result<(), Box<dyn Error>> {
    utils::privdrop();
    let mut rt = Runtime::new()?;
    rt.block_on(sensor::run_forever(&args))?;
    Ok(())
}
