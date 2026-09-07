#![allow(dead_code)]

use anyhow::{Result, anyhow};
use flate2::read::GzDecoder;
use humansize::{DECIMAL, format_size};
use std::{
    io::{Cursor, Read},
    path::{Path, PathBuf},
    str::FromStr,
};
use tar::Archive;
use zip::{ZipArchive, read::root_dir_common_filter};

use clap::Parser;
use reqwest::Url;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// url to download
    url: Url,

    /// output file path, omit for cwd
    path: Option<PathBuf>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let (filename, ftype) = parse_url(&cli.url)?;

    eprintln!("downloading from url: {}", cli.url);

    let resp = reqwest::blocking::get(cli.url)?.bytes()?;

    eprintln!("downloaded: {}", format_size(resp.len(), DECIMAL));

    let path = if let Some(p) = cli.path {
        p.join(filename)
    } else {
        filename
    };

    // pick proper decompression
    match ftype {
        FileType::TarZ => {
            eprintln!("decompressing gzip");
            handle_tar(GzDecoder::new(&resp[..]), path)?;
        }
        FileType::Tar => handle_tar(&resp[..], path)?,
        FileType::Zip => handle_zip(&resp[..], path)?,
    };

    eprintln!("done!");
    Ok(())
}

#[derive(Debug)]
enum FileType {
    Tar,
    TarZ,
    Zip,
}

fn parse_url(url: &Url) -> Result<(PathBuf, FileType)> {
    // get last segment of path
    let path_seg = url
        .path_segments()
        .and_then(|mut iter| iter.next_back())
        .ok_or_else(|| anyhow!("couldnt extract file path from: {}", url))?;

    // split last segment into filename and type
    let (filename, ext) = path_seg
        .split_once('.')
        .map(|(f, e)| {
            (
                PathBuf::from_str(f).expect("this should always be valid"),
                e,
            )
        })
        .ok_or_else(|| anyhow!("couldnt split filename: {}", path_seg))?;

    match ext {
        "tgz" | "tar.gz" => Ok((filename, FileType::TarZ)),
        "tar" => Ok((filename, FileType::Tar)),
        "zip" => Ok((filename, FileType::Zip)),
        _ => Err(anyhow!("unsupported filetype")),
    }
}

fn handle_tar(reader: impl Read, path: impl AsRef<Path>) -> Result<()> {
    eprintln!("unwrapping tarball");

    Archive::new(reader).unpack(path).map_err(Into::into)
}

fn handle_zip(reader: impl Read + AsRef<[u8]>, path: impl AsRef<Path>) -> Result<()> {
    eprintln!("unzipping file");
    let cursor = Cursor::new(reader);

    ZipArchive::new(cursor)?
        .extract_unwrapped_root_dir(path, root_dir_common_filter)
        .map_err(Into::into)
}
