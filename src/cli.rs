use std::path::Path;
use clap::ArgMatches;
use crate::encode::GronsfeldEncoded;
use std::fs;
use crate::decode::GronsfeldDecoded;
use crate::cli::error::{DecodingError, EncodingError};


pub mod error;

pub enum Args<'a> {
    Encode(Conf<'a>),
    Decode(Conf<'a>)
}

pub struct Conf<'a> {
    inp: &'a Path,
    out: &'a Path,
    key: &'a Path,
}

pub fn double_perm_encode(cfg: Conf) -> Result<(), EncodingError> {
    let input = fs::read(cfg.inp)?;
    let key = fs::read(cfg.key)?;

    let encoded =
        GronsfeldEncoded::new(&input, &key)?;

    let output: Vec<u8> = encoded.into_iter().collect();
    fs::write(cfg.out, output)?;

    Ok(())
}

pub fn double_perm_decode(cfg: Conf) -> Result<(), DecodingError> {
    let input = fs::read(cfg.inp)?;
    let key = fs::read(cfg.key)?;

    let encoded = GronsfeldDecoded::new(&input, &key)?;

    let output: Vec<u8> = encoded.into_iter().collect();
    fs::write(cfg.out, output)?;

    Ok(())
}

pub fn parse_config(matches: &ArgMatches) -> Option<Args> {
    let encode_matches =  matches.subcommand_matches("encode");
    if encode_matches.is_some() {
        let inp: &str = matches.value_of("input")?;
        let out: &str = matches.value_of("output")?;
        let key: &str = matches.value_of("key")?;

        return Some(Args::Encode(Conf {
            inp: Path::new(inp), out: Path::new(out),
            key: Path::new(key)
        }))
    }

    let decode_matches = matches.subcommand_matches("decode");
    if decode_matches.is_some() {
        let inp: &str = matches.value_of("input")?;
        let out: &str = matches.value_of("output")?;
        let key: &str = matches.value_of("key")?;

        return Some(Args::Decode(Conf {
            inp: Path::new(inp), out: Path::new(out),
            key: Path::new(key)
        }))
    }

    None
}