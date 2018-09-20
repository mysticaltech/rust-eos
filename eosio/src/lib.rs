#![no_std]
#![feature(
    alloc,
    core_intrinsics,
    lang_items,
    alloc_error_handler,
    proc_macro_non_items
)]

extern crate alloc;
extern crate eosio_bytes;
extern crate eosio_derives;
extern crate eosio_macros;
extern crate eosio_sys;
extern crate eosio_types;
extern crate wee_alloc;

pub mod action;
pub mod db;
pub mod print;

pub mod types {
    pub use eosio_sys::ctypes::*;
    pub use eosio_types::*;
}

pub mod sys {
    pub use eosio_sys::*;
}

pub mod macros {
    pub use eosio_derives::*;
    pub use eosio_macros::*;
}

pub mod bytes {
    pub use eosio_bytes::*;
}

pub mod prelude {
    pub use super::action::*;
    pub use super::db::*;
    pub use super::macros::*;
    pub use super::print::*;
    pub use super::types::*;
    pub use eosio_bytes::*;
}

::eosio_macros::wee_alloc!();