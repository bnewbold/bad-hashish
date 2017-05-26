// bad-hashish: a tool for recursively, remotely multi-hashing files
// Copyright (C) 2017  Bryan Newbold <bnewbold@robocracy.org>
// GPLv3

extern crate bad_hashish;

#[macro_use]
extern crate error_chain;

extern crate clap;
extern crate tree_magic;
extern crate flate2;
extern crate tar;
extern crate crypto;
extern crate zip;

use clap::App;
use bad_hashish::Result;
use std::path::Path;
use std::io::{Read, BufReader};
use std::io::prelude::*;
use std::fs::File;
use flate2::read::GzDecoder;
use tar::Archive;
use crypto::digest::Digest;
use crypto::sha1::Sha1;
use crypto::sha2::Sha256;
use crypto::blake2b::Blake2b;
use zip::read::ZipArchive;

fn hash_stream<T: Read>(mut stream: T, name: &str, total_size: u64) -> Result<()> {

    let mut hash_sha1 = Sha1::new();
    let mut hash_sha256 = Sha256::new();
    let mut hash_blake2b = Blake2b::new(32);
    let mut buf: [u8; 1*1024*1024] = [0; 1*1024*1024];
    loop {
        let got = stream.read(&mut buf[..])?;
        if got <= 0 { break };
        hash_sha1.input(&buf[0..got]);
        //hash_sha256.input(&buf[0..got]);
        //hash_blake2b.input(&buf[0..got]);
    }
    /*
    println!("{} {} {} {} {}",
        hash_sha1.result_str(),
        hash_sha256.result_str(),
        hash_blake2b.result_str(),
        total_size,
        name);
    */
    println!("{}  {}",
        hash_sha1.result_str(),
        name);
    Ok(())
}

fn do_tar<T: Read>(stream: T) -> Result<()> {
    //println!("It's a tar inside");
    let mut a = Archive::new(stream);
    for inner in a.entries().unwrap() {
        let mut inner = inner.unwrap();
        let (inner_name, inner_size) = {
            let header = inner.header();
            // Only do actual files ("regular", not directories, fifo, etc)
            if header.entry_type() != tar::EntryType::Regular {
                continue;
            }
            let inner_name = header.path()?.to_string_lossy().to_string();
            let inner_size = header.entry_size()?;
            (inner_name, inner_size)
        };
        hash_stream(&mut inner, &inner_name, inner_size)?;
    }
    Ok(())
}

fn run() -> Result<()> {

    let matches = App::new("bad-hashish")
        .version(env!("CARGO_PKG_VERSION"))
        .about("a tool for recursively, remotely multi-hashing files")
        .arg_from_usage("<FILE>... 'files to hash from'")
        .get_matches();

    for f in matches.values_of("FILE").unwrap() {
        let path: &Path = Path::new(f);
        println!("{} ({})", f, tree_magic::from_filepath(path));

        if tree_magic::match_filepath("application/zip", path) {
            let f = File::open(path)?;
            let mut zip = ZipArchive::new(f)?;

            for i in 0..zip.len() {
                let mut inner = zip.by_index(i).unwrap();
                if (0o40000u32 & inner.unix_mode().unwrap()) != 0 {
                    // This is a directory
                    continue
                }
                let (name, size) = (inner.name().to_string(), inner.size());
                hash_stream(&mut inner, &name, size)?;
            }

        } else if tree_magic::match_filepath("application/x-tar", path) {
            let f = File::open(path)?;
            do_tar(f)?;
        } else if tree_magic::match_filepath("application/gzip", path) {
            //println!("It's gzip.");
            let f = File::open(path)?;
            let gz = GzDecoder::new(f)?;

            let mut reader = BufReader::with_capacity(4*1024*1024, gz);
            let is_tar: bool = {
                let buf = reader.fill_buf()?;
                //println!("Inside is: {}", tree_magic::from_u8(&buf));
                tree_magic::match_u8("application/x-tar", &buf)
            };

            if is_tar {
                do_tar(reader)?;
            }
        } else if path.is_dir() {
            continue
        } else {
            let mut f = File::open(path)?;
            let (name, size) = (&path.to_string_lossy().to_string(), f.metadata()?.len());
            hash_stream(&mut f, name, size)?;
        }
    }

    Ok(())
}

// At least for now...
quick_main!(run);
