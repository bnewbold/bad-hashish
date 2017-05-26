// bad-hashish: a tool for recursively, remotely multi-hashing files
// Copyright (C) 2017  Bryan Newbold <bnewbold@robocracy.org>
// GPLv3

#[macro_use]
extern crate log;

#[macro_use]
extern crate error_chain;

extern crate zip;


mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain! {
        foreign_links {
            Io(::std::io::Error);
            Zip(::zip::result::ZipError);
        }
    }
}
pub use errors::*;

pub fn bhash() -> Result<()> {


    Ok(())
}
