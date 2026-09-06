//! Automatically rewritten from C Header to Rust Module
//! Source: include/crypto/internal/des.h
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


// SPDX-License-Identifier: GPL-2.0
//
// DES & Triple DES EDE key verification helpers
//

//
// crypto_des_verify_key - Check whether a DES key is weak
// @tfm: the crypto algo
// @key: the key buffer
//
// Returns -EINVAL if the key is weak and the crypto TFM does not permit weak
// keys. Otherwise, 0 is returned.
//
// It is the job of the caller to ensure that the size of the key equals
// DES_KEY_SIZE.
//
// RFC2451:
//
// For DES-EDE3, there is no known need to reject weak or
// complementation keys.  Any weakness is obviated by the use of
// multiple keys.
//
// However, if the first two or last two independent 64-bit keys are
// equal (k1 == k2 or k2 == k3), then the DES3 operation is simply the
// same as DES.  Implementers MUST reject keys that exhibit this
// property.
//
// crypto_des3_ede_verify_key - Check whether a DES3-EDE key is weak
// @tfm: the crypto algo
// @key: the key buffer
//
// Returns -EINVAL if the key is weak and the crypto TFM does not permit weak
// keys or when running in FIPS mode. Otherwise, 0 is returned. Note that some
// keys are rejected in FIPS mode even if weak keys are permitted by the TFM
// flags.
//
// It is the job of the caller to ensure that the size of the key equals
// DES3_EDE_KEY_SIZE.
//
extern "C" {
    pub fn crypto_des_verify_key(_arg: crypto_skcipher_tfm(tfm), _arg: key) -> return;
}
extern "C" {
    pub fn crypto_des3_ede_verify_key(_arg: crypto_skcipher_tfm(tfm), _arg: key) -> return;
}
extern "C" {
    pub fn crypto_des_verify_key(_arg: crypto_aead_tfm(tfm), _arg: key) -> return;
}
extern "C" {
    pub fn crypto_des3_ede_verify_key(_arg: crypto_aead_tfm(tfm), _arg: key) -> return;
}
