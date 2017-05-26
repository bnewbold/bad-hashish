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

use clap::App;
use bad_hashish::Result;
use std::path::Path;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use flate2::read::GzDecoder;
use tar::Archive;
use crypto::digest::Digest;
use crypto::sha1::Sha1;


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
            //println!("It's a zip.");
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
                //println!("It's a tar inside");
                let mut a = Archive::new(reader);
                for inner in a.entries().unwrap() {
                    let mut inner = inner.unwrap();
                    // Only do actual files ("regular", not directories, fifo, etc)
                    if inner.header().entry_type() != tar::EntryType::Regular {
                        continue;
                    }
                    let mut hasher = Sha1::new();
                    let mut buf: [u8; 1*1024*1024] = [0; 1*1024*1024];
                    loop {
                        let got = inner.read(&mut buf[..])?;
                        if got <= 0 { break };
                        hasher.input(&buf[0..got]);
                    }
                    println!("{}  {}",
                             hasher.result_str(),
                             inner.header().path()?.to_str().unwrap());
                }
            }
        }
    }

    Ok(())
}

// At least for now...
quick_main!(run);
