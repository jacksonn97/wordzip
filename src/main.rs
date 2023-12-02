use wordzip::{
    args::{Args, Mode},
    proc::{Do, Unzip, Zip},
    Result,
};

use std::env::args;
use std::{fs::File, io};

fn main() -> Result<()> {
    let args = args().into_iter().collect::<Vec<_>>();

    let args = Args::parse(&args)?;

    run(args)?;

    Ok(())
}

fn run(a: Args) -> Result<()> {
    let f = File::open(a.input_file)?;
    let s = io::read_to_string(f)?;

    let to_save = match a.mode {
        Mode::Zip => Do::Zip(Zip::from(s)).proc(),
        Mode::Unzip => Do::Unzip(Unzip::from(s)).proc(),
    };
    to_save?.save(a.output_file)?;

    Ok(())
}
