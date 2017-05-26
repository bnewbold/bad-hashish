// bad-hashish: a tool for recursively, remotely multi-hashing files
// Copyright (C) 2017  Bryan Newbold <bnewbold@robocracy.org>
// GPLv3

#[macro_use]
extern crate log;

#[macro_use]
extern crate error_chain;


mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain! {
        foreign_links {
            Io(::std::io::Error);
        }
    }
}
pub use errors::*;

pub fn bhash() -> Result<()> {


    Ok(())
}
