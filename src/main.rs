
use wordzip::{
    args::Args,
    *
};

use std::env::args;
use std::fs::File;

fn main() -> Result<()> {
    let args = args().into_iter().collect::<Vec<_>>();

    let f = fs::Files::open(Args::parse(&args)?)?;
    
    proc::proc(f)?;

    Ok(())
}
