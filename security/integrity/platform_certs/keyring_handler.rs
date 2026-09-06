//! Automatically rewritten from C Header to Rust Module
//! Source: security/integrity/platform_certs/keyring_handler.h
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
// Blacklist an X509 TBS hash.
//
extern "C" {
    pub fn blacklist_x509_tbs(source: *const c_char, data: *const c_void, len: usize);
}
//
// Blacklist the hash of an executable.
//
extern "C" {
    pub fn blacklist_binary(source: *const c_char, data: *const c_void, len: usize);
}
//
// Return the handler for particular signature list types found in the db.
//
extern "C" {
    pub fn get_handler_for_db(sig_type: *const efi_guid_t) -> efi_element_handler_t;
}
//
// Return the handler for particular signature list types found in the mok.
//
extern "C" {
    pub fn get_handler_for_mok(sig_type: *const efi_guid_t) -> efi_element_handler_t;
}
//
// Return the handler for particular signature list types for CA keys.
//
extern "C" {
    pub fn get_handler_for_ca_keys(sig_type: *const efi_guid_t) -> efi_element_handler_t;
}
//
// Return the handler for particular signature list types for code signing keys.
//
extern "C" {
    pub fn get_handler_for_code_signing_keys(sig_type: *const efi_guid_t) -> efi_element_handler_t;
}
//
// Return the handler for particular signature list types found in the dbx.
//
extern "C" {
    pub fn get_handler_for_dbx(sig_type: *const efi_guid_t) -> efi_element_handler_t;
}

