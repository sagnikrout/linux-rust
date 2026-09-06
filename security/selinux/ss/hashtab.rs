//! Automatically rewritten from C Header to Rust Module
//! Source: security/selinux/ss/hashtab.h
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
// A hash table (hashtab) maintains associations between
// key values and datum values.  The type of the key values
// and the type of the datum values is arbitrary.  The
// functions for hash computation and key comparison are
// provided by the creator of the table.
//
// Author : Stephen Smalley, <stephen.smalley.work@gmail.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hashtab_key_params {
    pub /: *const *const *const *const u32 (hash)(void key); / hash func,
    pub /: *const *const *const *const *const int (cmp)(void key1, void key2); / comparison func,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hashtab_node {
    pub key: *mut c_void,
    pub datum: *mut c_void,
    pub next: *mut hashtab_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hashtab {
    pub /: *mut *mut *mut *mut hashtab_node htable; / hash table,
    pub /: *mut *mut u32 size; / number of slots in hash table,
    pub /: *mut *mut u32 nel; / number of elements in hash table,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hashtab_info {
    pub slots_used: u32,
    pub max_chain_len: u32,
    pub chain2_len_sum: u64,
}

//
// Initializes a new hash table with the specified characteristics.
//
// Returns -ENOMEM if insufficient space is available or 0 otherwise.
//
extern "C" {
    pub fn hashtab_init(h: *mut hashtab, nel_hint: u32) -> c_int;
}
//
// Inserts the specified (key, datum) pair into the specified hash table.
//
// Returns -ENOMEM on memory allocation error,
// -EEXIST if there is already an entry with the same key,
// -EINVAL for general errors or
//
// Searches for the entry with the specified key in the hash table.
//
// Returns NULL if no entry has the specified key or
// the datum of the entry otherwise.
//
// Destroys the specified hash table.
//
extern "C" {
    pub fn hashtab_destroy(h: *mut hashtab);
}
//
// Applies the specified apply function to (key,datum,args)
// for each entry in the specified hash table.
//
// The order in which the function is applied to the entries
// is dependent upon the internal structure of the hash table.
//
// If apply returns a non-zero status, then hashtab_map will cease
// iterating through the hash table and will propagate the error
// return to its caller.
//

// Fill info with some hash table statistics
extern "C" {
    pub fn hashtab_stat(h: *mut hashtab, info: *mut hashtab_info);
}

