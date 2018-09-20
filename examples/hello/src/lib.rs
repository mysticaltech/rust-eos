#![no_std]
#![feature(proc_macro_non_items)]

extern crate eosio;

use eosio::prelude::*;

#[eosio_action]
fn hi(name: Name) {
    print!("Hello, ", name);
}

eosio_abi!(hi);