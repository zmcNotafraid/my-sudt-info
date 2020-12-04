// Import from `core` instead of from `std` since we are in no-std mode
use core::result::Result;

// Import CKB syscalls and structures
// https://nervosnetwork.github.io/ckb-std/riscv64imac-unknown-none-elf/doc/ckb_std/index.html
use ckb_std::{
    high_level::{load_script, load_cell_type_hash, load_cell_lock_hash, QueryIter, load_cell_type},
    ckb_types::{bytes::Bytes, prelude::*},
    ckb_constants::Source
};

use crate::error::Error;

pub fn main() -> Result<(), Error> {
    let script = load_script()?;
    let args: Bytes = script.args().unpack();

    let sudt_index = match check_sudt_type_hash_equal_sudt_info_args(&args) {
        Ok(sudt_index) => sudt_index,
        Err(err) => return Err(err)
    };
    check_input_exist_owner_lock(sudt_index)?;

    return Ok(());
}

fn check_sudt_type_hash_equal_sudt_info_args(args: &Bytes) -> Result<usize, Error> {
    let sudt_index = QueryIter::new(load_cell_type_hash, Source::Output)
        .position(|type_hash| args[..] == type_hash.unwrap()[..]);
    match sudt_index {
        Some(sudt_index) => Ok(sudt_index),
        None => Err(Error::InfoTypeArgsNotMatch)
    }
}

fn check_input_exist_owner_lock(sudt_index: usize) -> Result<bool, Error> {
    let sudt_type_script = match load_cell_type(sudt_index, Source::Output) {
        Ok(cell_type) => cell_type.unwrap(),
        Err(err) => return Err(err.into())
    };
    let input_owner_lock = QueryIter::new(load_cell_lock_hash, Source::Input)
        .find(|lock_hash| sudt_type_script.args().raw_data()[..] == lock_hash[..]);

    match input_owner_lock {
        Some(_) => Ok(true),
        None => Err(Error::OwnerLockScriptNotMatch)
    }
}
