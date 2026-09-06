//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireguard/peerlookup.h
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
// Copyright (C) 2015-2019 Jason A. Donenfeld <Jason@zx2c4.com>. All Rights Reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pubkey_hashtable {
// TODO: move to rhashtable
    pub 11): DECLARE_HASHTABLE(hashtable,,
    pub key: siphash_key_t,
    pub lock: mutex,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct index_hashtable {
// TODO: move to rhashtable
    pub 13): DECLARE_HASHTABLE(hashtable,,
    pub lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum index_hashtable_type {
    INDEX_HASHTABLE_HANDSHAKE = 1U << 0,
    INDEX_HASHTABLE_KEYPAIR = 1U << 1
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct index_hashtable_entry {
    pub peer: *mut wg_peer,
    pub index_hash: hlist_node,
    pub type: index_hashtable_type,
    pub index: __le32,
}
