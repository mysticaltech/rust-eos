#![feature(proc_macro_non_items)]

extern crate eosio_macros;

use eosio_macros::{n, s};

#[test]
fn test_n() {
    assert_eq!(n!(test), 14_605_613_396_213_628_928 as u64);
    assert_eq!(n!(1234), 614_248_767_926_829_056 as u64);
    assert_eq!(n!(123451234512), 614_251_535_012_020_768 as u64);
}

#[test]
fn test_s() {
    assert_eq!(s!(0, TGFT), 361956332544);
    assert_eq!(s!(4, EOS), 1397703940);
    assert_eq!(s!(0, EOS), 1397703936);
    assert_eq!(s!(1, EDNA), 280485971201);
}