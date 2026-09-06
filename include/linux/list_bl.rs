//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/list_bl.h
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
// Special version of lists, where head of the list has a lock in the lowest
// bit. This is useful for scalable hash tables without increasing memory
// footprint overhead.
//
// For modification operations, the 0 bit of hlist_bl_head->first
// pointer must be set.
//
// With some small modifications, this can easily be adapted to store several
// arbitrary bits (not just a single lock bit), if the need arises to store
// some fast and compact auxiliary data.
//

// Macro flag: #define LIST_BL_BUG_ON(x)

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hlist_bl_head {
    pub first: *mut hlist_bl_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hlist_bl_node {
    pub pprev: *mut *mut hlist_bl_node next,,
}

// pprev may be `first`, so be careful not to lose the lock bit
extern "C" {
    pub fn bit_spin_is_locked(_arg: 0, )b: *mut (unsigned long) -> return;
}
//
// hlist_bl_for_each_entry	- iterate over list of given type
// @tpos:	the type * to use as a loop cursor.
// @pos:	the &struct hlist_node to use as a loop cursor.
// @head:	the head for your list.
// @member:	the name of the hlist_node within the struct.
//

//
// hlist_bl_for_each_entry_safe - iterate over list of given type safe against removal of list entry
// @tpos:	the type * to use as a loop cursor.
// @pos:	the &struct hlist_node to use as a loop cursor.
// @n:		another &struct hlist_node to use as temporary storage
// @head:	the head for your list.
// @member:	the name of the hlist_node within the struct.
//

