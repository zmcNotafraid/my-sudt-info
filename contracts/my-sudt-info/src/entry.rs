// Import from `core` instead of from `std` since we are in no-std mode
use core::result::Result;

// Import CKB syscalls and structures
// https://nervosnetwork.github.io/ckb-std/riscv64imac-unknown-none-elf/doc/ckb_std/index.html
use ckb_std::{
    high_level::{load_script, load_cell_type_hash, load_cell_lock_hash, QueryIter, load_cell},
    ckb_types::{bytes::Bytes, prelude::*},
    ckb_constants::Source
};

use crate::blake2b::blake2b_256;
use crate::error::Error;

pub fn main() -> Result<(), Error> {
    let script = load_script()?;
    let args: Bytes = script.args().unpack();

    check_sudt_type_hash_equal_sudt_info_args(&args)?;

    check_input_exist_owner_lock(&args)?;

    return Ok(());
}

fn check_sudt_type_hash_equal_sudt_info_args(args: &Bytes) -> Result<bool, Error> {
    let is_equal  = QueryIter::new(load_cell_type_hash, Source::Output)
        .find(|type_hash| args[..] == type_hash.unwrap()[..]).is_some();
    if is_equal {
        Ok(is_equal)
    } else {
        Err(Error::InfoTypeArgsNotMatch)
    }
}

fn check_input_exist_owner_lock(args: &Bytes) -> Result<bool, Error> {
    let sudt_cell = QueryIter::new(load_cell, Source::Output)
        .find(|cell| {
            if cell.type_().to_opt().is_some() {
                args[..] == blake2b_256(cell.type_().to_opt().unwrap().as_slice())
            } else { false }}).unwrap();
    let owner_lock_exist = QueryIter::new(load_cell_lock_hash, Source::Input)
        .find(|lock_hash| sudt_cell.type_().to_opt().unwrap().args().raw_data()[..] == lock_hash[..]).is_some();

    if owner_lock_exist {
        Ok(owner_lock_exist)
    } else {
        Err(Error::OwnerLockScriptNotMatch)
    }
}
