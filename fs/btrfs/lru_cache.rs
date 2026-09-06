//! Automatically rewritten from C Header to Rust Module
//! Source: fs/btrfs/lru_cache.h
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
// A cache entry. This is meant to be embedded in a structure of a user of
// this module. Similar to how struct list_head and struct rb_node are used.
//
// Note: it should be embedded as the first element in a struct (offset 0), and
// this module assumes it was allocated with kmalloc(), so it calls kfree() when
// it needs to free an entry.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_lru_cache_entry {
    pub lru_list: list_head,
    pub key: u64,
//
// Optional generation associated to a key. Use 0 if not needed/used.
// Entries with the same key and different generations are stored in a
// linked list, so use this only for cases where there's a small number
// of different generations.
//
    pub gen: u64,
//
// The maple tree uses unsigned long type for the keys, which is 32 bits
// on 32 bits systems, and 64 bits on 64 bits systems. So if we want to
// use something like inode numbers as keys, which are always a u64, we
// have to deal with this in a special way - we store the key in the
// entry itself, as a u64, and the values inserted into the maple tree
// are linked lists of entries - so in case we are on a 64 bits system,
// that list always has a single entry, while on 32 bits systems it
// may have more than one, with each entry having the same value for
// their lower 32 bits of the u64 key.
//
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_lru_cache {
    pub lru_list: list_head,
    pub entries: maple_tree,
// Number of entries stored in the cache.
    pub size: c_uint,
// Maximum number of entries the cache can have.
    pub max_size: c_uint,
}

extern "C" {
    pub fn btrfs_lru_cache_init(cache: *mut btrfs_lru_cache, max_size: c_uint);
}
extern "C" {
    pub fn btrfs_lru_cache_clear(cache: *mut btrfs_lru_cache);
}
