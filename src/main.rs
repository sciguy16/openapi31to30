use color_eyre::{
    eyre::{bail, OptionExt},
    Result,
};
use std::{ffi::OsString, path::PathBuf};

const USAGE: &str = "Usage: --input INPUT_FILE --output OUTPUT_FILE";

#[derive(Debug, PartialEq, Eq)]
struct Args {
    input: PathBuf,
    output: PathBuf,
}

impl Args {
    pub fn parse(mut args: impl Iterator<Item = OsString>) -> Result<Self> {
        let _bin_name = args.next().ok_or_eyre(USAGE)?;
        let input_arg = args.next().ok_or_eyre(USAGE)?;
        if input_arg != "--input" {
            bail!(USAGE)
        }
        let input = args.next().ok_or_eyre(USAGE)?.into();
        let output_arg = args.next().ok_or_eyre(USAGE)?;
        if output_arg != "--output" {
            bail!(USAGE)
        }
        let output = args.next().ok_or_eyre(USAGE)?.into();

        Ok(Self { input, output })
    }
}

fn main() -> color_eyre::Result<()> {
    let args = Args::parse(std::env::args_os())?;
    let input = std::fs::read_to_string(args.input)?;
    let output = openapi31to30::convert(&input)?;
    std::fs::write(&args.output, output.as_str())?;
    println!("Converted schema written to {}", args.output.display());
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn argparse() {
        assert_eq!(
            Args::parse(
                ["./test", "--input", "input.yaml", "--output", "output.yaml"]
                    .into_iter()
                    .map(OsString::from)
            )
            .unwrap(),
            Args {
                input: "input.yaml".into(),
                output: "output.yaml".into()
            }
        );

        let cases: &[&[&str]] = &[
            &[],
            &["./test"],
            &["./test", "--input", "a", "--output"],
            &["./test", "--input", "a", "--input", "a"],
            &["./test", "--output", "a", "--input", "a"],
        ];
        for invalid in cases {
            assert_eq!(
                Args::parse(invalid.iter().map(OsString::from))
                    .unwrap_err()
                    .to_string(),
                USAGE
            );
        }
    }
}
