//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/rhashtable-types.h
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
// Resizable, Scalable, Concurrent Hash Table
//
// Simple structures that might be needed in include
// files.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rhash_head {
    pub next: *mut rhash_head __rcu,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rhlist_head {
    pub rhead: rhash_head,
    pub next: *mut rhlist_head __rcu,
}

//
// struct rhashtable_compare_arg - Key for the function rhashtable_compare
// @ht: Hash table
// @key: Key to compare against
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rhashtable_compare_arg {
    pub ht: *mut rhashtable,
    pub key: *const c_void,
}

extern "C" {
    pub fn u32(data: *const *const rht_hashfn_t)(void, len: u32, seed: u32) -> typedef;
}
extern "C" {
    pub fn u32(data: *const *const rht_obj_hashfn_t)(void, len: u32, seed: u32) -> typedef;
}
//
// struct rhashtable_params - Hash table construction parameters
// @nelem_hint: Hint on number of elements, should be 75% of desired size
// @key_len: Length of key
// @key_offset: Offset of key in struct to be hashed
// @head_offset: Offset of rhash_head in struct to be hashed
// @max_size: Maximum size while expanding
// @min_size: Minimum size while shrinking
// @insecure_elasticity: Set to true to disable chain length checks
// @automatic_shrinking: Enable automatic shrinking of tables
// @hashfn: Hash function (default: jhash2 if !(key_len % 4), or jhash)
// @obj_hashfn: Function to hash object
// @obj_cmpfn: Function to compare key with object
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rhashtable_params {
    pub nelem_hint: u16,
    pub key_len: u16,
    pub key_offset: u16,
    pub head_offset: u16,
    pub max_size: c_uint,
    pub min_size: u16,
    pub insecure_elasticity: bool,
    pub automatic_shrinking: bool,
    pub hashfn: rht_hashfn_t,
    pub obj_hashfn: rht_obj_hashfn_t,
    pub obj_cmpfn: rht_obj_cmpfn_t,
}

//
// struct rhashtable - Hash table handle
// @tbl: Bucket table
// @key_len: Key length for hashfn
// @max_elems: Maximum number of elements in table
// @p: Configuration parameters
// @rhlist: True if this is an rhltable
// @run_work: Deferred worker to expand/shrink asynchronously
// @run_irq_work: Bounces the @run_work kick through hard IRQ context.
// @mutex: Mutex to protect current/future table swapping
// @lock: Spin lock to protect walker list
// @nelems: Number of elements in table
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rhashtable {
    pub tbl: *mut bucket_table __rcu,
    pub key_len: c_uint,
    pub max_elems: c_uint,
    pub p: rhashtable_params,
    pub rhlist: bool,
    pub run_work: work_struct,
    pub run_irq_work: irq_work,
    pub mutex: mutex,
    pub lock: spinlock_t,
    pub nelems: core::sync::atomic::AtomicI32,

    pub alloc_tag: *mut alloc_tag,

}

//
// struct rhltable - Hash table with duplicate objects in a list
// @ht: Underlying rhtable
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rhltable {
    pub ht: rhashtable,
}

//
// struct rhashtable_walker - Hash table walker
// @list: List entry on list of walkers
// @tbl: The table that we were walking over
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rhashtable_walker {
    pub list: list_head,
    pub tbl: *mut bucket_table,
}

//
// struct rhashtable_iter - Hash table iterator
// @ht: Table to iterate through
// @p: Current pointer
// @list: Current hash list pointer
// @walker: Associated rhashtable walker
// @slot: Current slot
// @skip: Number of entries to skip in slot
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rhashtable_iter {
    pub ht: *mut rhashtable,
    pub p: *mut rhash_head,
    pub list: *mut rhlist_head,
    pub walker: rhashtable_walker,
    pub slot: c_uint,
    pub skip: c_uint,
    pub end_of_table: bool,
}

