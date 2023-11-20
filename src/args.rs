
use std::{
    path::PathBuf as Path,
    process,
};
use getopts::Occur;
use args::{ Args as MArgs, ArgsError as ArgsError };

const PROGRAM_NAME: &'static str = "wordzip";
const PROGRAM_DESC: &'static str = "Usage: wordzip [mode(-c/-d)] -i [input-file] -o [output-file]";

use crate::Result;

#[derive(Debug, PartialEq)]
pub struct Args {
    mode: Mode,
    r#if: Path,
    r#of: Path,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Mode {
    Comp,
    Decomp,
}

impl Args {

    #[inline]
    pub fn parse<T>(input: &Vec<T>) -> Result<Args>
        where
            T: ToString + AsRef<std::ffi::OsStr> + PartialEq,
        {
        let mut args = MArgs::new(PROGRAM_NAME, PROGRAM_DESC);
        args.flag("h", "help", "Show this help");

        args.flag("c", "compress", "Compress the given file");
        args.flag("d", "decompress", "Decompress the given file");

        args.option("i", "input-file", "Specifies input file", "<path>", Occur::Optional, None);
        args.option("o", "output-file", "Specifies output file", "<path>", Occur::Optional, None);

        args.flag("f", "force", "Override output file if it's exists");

        args.parse(input)?;

        if  args.value_of("help")?
            // input.len() < 2
        {
            println!("{}", args.usage());
            process::exit(0);
        }

        let mut mode = Mode::Comp;
        if args.value_of("decompress")? && args.value_of("compress")? {
            return Err(Box::new(ArgsError::new("operation", "Only one mode can be selected!")));
        } else if args.value_of("compress")? {
            mode = Mode::Comp
        } else if args.value_of("decompress")? {
            mode = Mode::Decomp
        }


        let input_file = args.value_of("input-file");
        let output_file = args.value_of("output-file");


        Ok(
        Args {
            mode,
            r#if: Path::from(Self::if_path_parse(input_file.into())?),
            r#of: Path::from(Self::of_path_parse(output_file.into(), args.value_of("force")?)?),
        })

    }

    #[inline]
    fn if_path_parse(s: Option<std::result::Result<String, ArgsError>>) -> Result<Path> {
        
        if let Some(p) = s {
            if let Ok(p) = p {
                let path = Path::from(p.to_string());
                if path.is_file() {
                    return Ok(path)
                }
            }
        } 
        Err(Box::new(ArgsError::new("path", "Specify correct file to compress!")))
    }

    #[inline]
    fn of_path_parse(s: Option<std::result::Result<String, ArgsError>>, r#override: bool) -> Result<Path> {
        let mut path = Path::new();
        if let Some(p) = s {
            if let Ok(p) = p {

                path = Path::from(p.to_string());
                if (path.is_file() || path.is_dir()) && !r#override {
                    return Err(Box::new(ArgsError::new("path",
                                "File with same name already exists!\n\
                                Specify other file or use `-f` for override existing file.")))
                }
            }
        }
        Ok(path)
    }

    #[inline]
    pub fn r#if(&self) -> &Path {
        &self.r#if
    }

    #[inline]
    pub fn r#of(&self) -> &Path {
        &self.r#of
    }

    #[inline]
    pub fn mode(&self) -> &Mode {
        &self.mode
    }
}

#[test]
fn parse_cases() {

    const OK: &'static str = "src/tests/ok.txt";
    const PERMISSION_DENIED: &'static str = "src/tests/permission_denied.txt";

    let ok =                vec!["-i", OK];
    let if_not_exists =     vec!["-i", "asldfasdhfjklashfljkas.adsa"];
    let permission_denied = vec!["-i", PERMISSION_DENIED];

    let of_not_exits =      vec!["-i", OK, "-o", "asdfhasdhfljkasfhj.asd"];
    let of_exits =          vec!["-i", OK, "-o", PERMISSION_DENIED];
    let of_exits_override = vec!["-i", OK, "-o", PERMISSION_DENIED, "-f"];

    let two_modes_together = vec!["-c", "-d"];

    // Ok
    assert!(Args::parse(&ok).is_ok());
    assert!(Args::parse(&permission_denied).is_ok());
    assert!(Args::parse(&of_not_exits).is_ok());
    assert!(Args::parse(&of_exits_override).is_ok());

    // Err
    assert!(Args::parse(&if_not_exists).is_err());
    assert!(Args::parse(&of_exits).is_err());
    assert!(Args::parse(&two_modes_together).is_err());

    // General test
    let good_args = vec!["-d", "-i", OK, "-o", "new.txt"];

    assert_eq!(
        Args::parse(&good_args).unwrap(),
        Args {
            mode: Mode::Decomp,
            r#if: Path::from(OK),
            r#of: Path::from("new.txt")
        });

}
