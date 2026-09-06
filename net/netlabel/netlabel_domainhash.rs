//! Automatically rewritten from C Header to Rust Module
//! Source: net/netlabel/netlabel_domainhash.h
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
// NetLabel Domain Hash Table
//
// This file manages the domain hash table that NetLabel uses to determine
// which network labeling protocol to use for a given domain.  The NetLabel
// system manages static and dynamic label mappings for network protocols such
// as CIPSO and RIPSO.
//
// Author: Paul Moore <paul@paul-moore.com>
//
// (c) Copyright Hewlett-Packard Development Company, L.P., 2006, 2008
//

// Domain hash table size
// XXX - currently this number is an uneducated guess
pub const NETLBL_DOMHSH_BITSIZE: c_int = 7;
// Domain mapping definition structures
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netlbl_domaddr_map {
    pub list4: list_head,
    pub list6: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netlbl_dommap_def {
    pub type: u32,
    pub addrsel: *mut netlbl_domaddr_map,
    pub cipso: *mut cipso_v4_doi,
    pub calipso: *mut calipso_doi,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netlbl_domaddr4_map {
    pub def: netlbl_dommap_def,
    pub list: netlbl_af4list,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netlbl_domaddr6_map {
    pub def: netlbl_dommap_def,
    pub list: netlbl_af6list,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netlbl_dom_map {
    pub domain: *mut c_char,
    pub def: netlbl_dommap_def,
    pub family: u16,
    pub valid: u32,
    pub list: list_head,
    pub rcu: rcu_head,
}

// init function
extern "C" {
    pub fn netlbl_domhsh_init(size: u32) -> c_int;
}
// Manipulate the domain hash table
extern "C" {
    pub fn netlbl_domhsh_remove_default(family: u16, audit_info: *mut netlbl_audit) -> c_int;
}

