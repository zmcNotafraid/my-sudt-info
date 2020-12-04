// Import from `core` instead of from `std` since we are in no-std mode
use core::result::Result;

// Import CKB syscalls and structures
// https://nervosnetwork.github.io/ckb-std/riscv64imac-unknown-none-elf/doc/ckb_std/index.html
use ckb_std::{
    high_level::{load_script, load_cell_lock_hash, QueryIter},
    ckb_types::{bytes::Bytes, prelude::*},
    ckb_constants::Source
};

use crate::error::Error;

pub fn main() -> Result<(), Error> {
    let script = load_script()?;
    let args: Bytes = script.args().unpack();

    check_input_exist_owner_lock(&args)?;

    return Ok(());
}

fn check_input_exist_owner_lock(args: &Bytes) -> Result<bool, Error> {
    let is_owner_mode = QueryIter::new(load_cell_lock_hash, Source::Input)
        .find(|lock_hash| args[..] == lock_hash[..]).is_some();
    if is_owner_mode {
        Ok(true)
    } else {
        Err(Error::OwnerLockScriptNotExist)
    }
}
