//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/s390/crypto/pkey_base.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright IBM Corp. 2024
//
// Pkey base: debug feature, defines and structs
// common to all pkey code.
//

//
// pkey debug feature
//

//
// common defines and common structs
//

// inside view of a generic protected key token
#[repr(C)]
#[derive(Copy, Clone)]
pub struct protkeytoken {
    pub /: *mut *mut u8 type; / 0x00 for PAES specific key tokens,
    pub res0: [u8; 3],
    pub /: *mut *mut u8 version; / should be 0x01 for protected key token,
    pub res1: [u8; 3],
    pub /: *mut *mut u32 keytype; / key type, one of the PKEY_KEYTYPE values,
    pub /: *mut *mut u32 len; / bytes actually stored in protkey[],
    pub /: *mut *mut u8 protkey[]; / the protected key blob,
    pub __packed: },
// inside view of a protected AES key token
#[repr(C)]
#[derive(Copy, Clone)]
pub struct protaeskeytoken {
    pub /: *mut *mut u8 type; / 0x00 for PAES specific key tokens,
    pub res0: [u8; 3],
    pub /: *mut *mut u8 version; / should be 0x01 for protected key token,
    pub res1: [u8; 3],
    pub /: *mut *mut u32 keytype; / key type, one of the PKEY_KEYTYPE values,
    pub /: *mut *mut u32 len; / bytes actually stored in protkey[],
    pub /: *mut *mut u8 protkey[MAXPROTKEYSIZE]; / the protected key blob,
    pub __packed: },
// inside view of a clear key token (type 0x00 version 0x02)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clearkeytoken {
    pub /: *mut *mut u8 type; / 0x00 for PAES specific key tokens,
    pub res0: [u8; 3],
    pub /: *mut *mut u8 version; / 0x02 for clear key token,
    pub res1: [u8; 3],
    pub /: *mut *mut *mut u32 keytype; / key type, one of the PKEY_KEYTYPE_ values,
    pub /: *mut *mut u32 len; / bytes actually stored in clearkey[],
    pub /: *mut *mut u8 clearkey[]; / clear key value,
    pub __packed: },
// helper function which translates the PKEY_KEYTYPE_AES_* to their keysize
    pub 16: return,
    pub 24: return,
    pub 32: return,
    pub 0: return,
// helper function which translates AES key bit size into PKEY_KEYTYPE_AES_*
    pub PKEY_KEYTYPE_AES_128: return,
    pub PKEY_KEYTYPE_AES_192: return,
    pub PKEY_KEYTYPE_AES_256: return,
    pub 0: return,
//
// helper function which translates the PKEY_KEYTYPE_
// to the protected key size minus the WK VP length
//
    pub 16: return,
    pub 24: return,
    pub 32: return,
    pub 32: return,
    pub 48: return,
    pub 80: return,
    pub 32: return,
    pub 54: return,
    pub 32: return,
    pub 64: return,
    pub 64: return,
    pub 128: return,
    pub 0: return,
//
// pkey_api.c:
//
    pub pkey_api_init(void): int __init,
    pub pkey_api_exit(void): void __exit,
//
// pkey_sysfs.c:
//
    pub pkey_attr_groups: [*const extern struct attribute_group; ],
//
// pkey handler registry
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pkey_handler {
    pub module: *mut module,
    pub name: *const c_char,
//
// is_supported_key() and is_supported_keytype() are called
// within an rcu_read_lock() scope and thus must not sleep!
//
    pub keylen): *const *const *const bool (is_supported_key)(u8 key, u32,
    pub pkey_key_type): *mut *mut bool (is_supported_keytype)(enum,
    pub xflags): u32,
    pub xflags): *mut *mut u32 protkeytype, u32,
    pub xflags): *mut *mut *mut *mut u8 keybuf, u32 keybuflen, u32 keyinfo, u32,
    pub xflags): *mut *mut *mut *mut u8 keybuf, u32 keybuflen, u32 keyinfo, u32,
    pub xflags): u32,
    pub xflags): u32,
    pub xflags): u32,
// used internal by pkey base
    pub list: list_head,
}

extern "C" {
    pub fn pkey_handler_register(handler: *mut pkey_handler) -> c_int;
}
extern "C" {
    pub fn pkey_handler_unregister(handler: *mut pkey_handler) -> c_int;
}
//
// invocation function for the registered pkey handlers
//
extern "C" {
    pub fn pkey_handler_put(handler: *const pkey_handler);
}
//
// Unconditional try to load all handler modules
//
extern "C" {
    pub fn pkey_handler_request_modules();
}
