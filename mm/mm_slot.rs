//! Automatically rewritten from C Header to Rust Module
//! Source: mm/mm_slot.h
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
// struct mm_slot - hash lookup from mm to mm_slot
// @hash: link to the mm_slots hash list
// @mm_node: link into the mm_slots list
// @mm: the mm that this information is valid for
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mm_slot {
    pub hash: hlist_node,
    pub mm_node: list_head,
    pub mm: *mut mm_struct,
}

extern "C" {
    pub fn kmem_cache_zalloc(_arg: cache, _arg: GFP_KERNEL) -> return;
}
//
// Note: mm_slot_lookup and mm_slot_insert cannot be converted to static inline
// functions because the hash helpers (hash_for_each_possible and hash_add) rely
// on the actual array argument 'hashtable' for sizeof() instead of pointers.
//

