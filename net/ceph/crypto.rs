//! Automatically rewritten from C Header to Rust Module
//! Source: net/ceph/crypto.h
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

pub const CEPH_MAX_KEY_LEN: c_int = 32;
pub const CEPH_MAX_CON_SECRET_LEN: c_int = 64;
//
// cryptographic secret
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_crypto_key {
    pub type: c_int,
    pub created: ceph_timespec,
    pub len: c_int,
    pub key: *mut c_void,
    pub aes_tfm: *mut crypto_sync_skcipher,
    pub hmac_key: hmac_sha256_key,
    pub krb5_type: *const krb5_enctype,
    pub krb5_tfms: [*mut crypto_aead; 3],
}

extern "C" {
    pub fn ceph_crypto_key_decode(key: *mut ceph_crypto_key, p: *mut c_void, end: *mut c_void) -> c_int;
}
extern "C" {
    pub fn ceph_crypto_key_unarmor(key: *mut ceph_crypto_key, in: *const c_char) -> c_int;
}
extern "C" {
    pub fn ceph_crypto_key_destroy(key: *mut ceph_crypto_key);
}
// crypto.c
extern "C" {
    pub fn ceph_crypt_data_offset(key: *const ceph_crypto_key) -> c_int;
}
extern "C" {
    pub fn ceph_crypt_buflen(key: *const ceph_crypto_key, data_len: c_int) -> c_int;
}
extern "C" {
    pub fn ceph_crypto_init() -> c_int;
}
extern "C" {
    pub fn ceph_crypto_shutdown();
}
// armor.c
extern "C" {
    pub fn ceph_armor(dst: *mut c_char, src: *const c_char, end: *const c_char) -> c_int;
}
extern "C" {
    pub fn ceph_unarmor(dst: *mut c_char, src: *const c_char, end: *const c_char) -> c_int;
}
