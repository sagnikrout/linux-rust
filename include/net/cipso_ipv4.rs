//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/cipso_ipv4.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// CIPSO - Commercial IP Security Option
//
// This is an implementation of the CIPSO 2.2 protocol as specified in
// draft-ietf-cipso-ipsecurity-01.txt with additional tag types as found in
// FIPS-188, copies of both documents can be found in the Documentation
// directory.  While CIPSO never became a full IETF RFC standard many vendors
// have chosen to adopt the protocol and over the years it has become a
// de-facto standard for labeled networking.
//
// Author: Paul Moore <paul@paul-moore.com>
//
// (c) Copyright Hewlett-Packard Development Company, L.P., 2006
//

// known doi values
pub const CIPSO_V4_DOI_UNKNOWN: c_uint = 0x00000000;
// standard tag types
pub const CIPSO_V4_TAG_INVALID: c_int = 0;
pub const CIPSO_V4_TAG_RBITMAP: c_int = 1;
pub const CIPSO_V4_TAG_ENUM: c_int = 2;
pub const CIPSO_V4_TAG_RANGE: c_int = 5;
pub const CIPSO_V4_TAG_PBITMAP: c_int = 6;
pub const CIPSO_V4_TAG_FREEFORM: c_int = 7;
// non-standard tag types (tags > 127)
pub const CIPSO_V4_TAG_LOCAL: c_int = 128;
// doi mapping types
pub const CIPSO_V4_MAP_UNKNOWN: c_int = 0;
pub const CIPSO_V4_MAP_TRANS: c_int = 1;
pub const CIPSO_V4_MAP_PASS: c_int = 2;
pub const CIPSO_V4_MAP_LOCAL: c_int = 3;
// limits
pub const CIPSO_V4_MAX_REM_LVLS: c_int = 255;
pub const CIPSO_V4_INV_LVL: c_uint = 0x80000000;

pub const CIPSO_V4_MAX_REM_CATS: c_int = 65534;
pub const CIPSO_V4_INV_CAT: c_uint = 0x80000000;

//
// CIPSO DOI definitions
//
// DOI definition struct
pub const CIPSO_V4_TAG_MAXCNT: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cipso_v4_doi {
    pub doi: u32,
    pub type: u32,
    pub std: *mut cipso_v4_std_map_tbl,
    pub map: },
    pub tags: [u8; CIPSO_V4_TAG_MAXCNT],
    pub refcount: refcount_t,
    pub list: list_head,
    pub rcu: rcu_head,
}

// Standard CIPSO mapping table
// NOTE: the highest order bit (i.e. 0x80000000) is an 'invalid' flag, if the
// bit is set then consider that value as unspecified, meaning the
// mapping for that particular level/category is invalid
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cipso_v4_std_map_tbl {
    pub cipso: *mut u32,
    pub local: *mut u32,
    pub cipso_size: u32,
    pub local_size: u32,
    pub lvl: },
    pub cipso: *mut u32,
    pub local: *mut u32,
    pub cipso_size: u32,
    pub local_size: u32,
    pub cat: },
}

//
// Sysctl Variables
//

//
// DOI List Functions
//

extern "C" {
    pub fn cipso_v4_doi_free(doi_def: *mut cipso_v4_doi);
}
extern "C" {
    pub fn cipso_v4_doi_remove(doi: u32, audit_info: *mut netlbl_audit) -> c_int;
}
extern "C" {
    pub fn cipso_v4_doi_putdef(doi_def: *mut cipso_v4_doi);
}

//
// Label Mapping Cache Functions
//

extern "C" {
    pub fn cipso_v4_cache_invalidate();
}

//
// Protocol Handling Functions
//

extern "C" {
    pub fn cipso_v4_error(skb: *mut sk_buff, error: c_int, gateway: u32);
}
extern "C" {
    pub fn cipso_v4_sock_delattr(sk: *mut sock);
}
extern "C" {
    pub fn cipso_v4_sock_getattr(sk: *mut sock, secattr: *mut netlbl_lsm_secattr) -> c_int;
}
extern "C" {
    pub fn cipso_v4_req_delattr(req: *mut request_sock);
}
extern "C" {
    pub fn cipso_v4_skbuff_delattr(skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn cipso_v4_validate(skb: *const sk_buff, option: *mut c_uchar) -> c_int;
}

// option = opt + err_offset;

