//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/list_nulls.h
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
// Special version of lists, where end of list is not a NULL pointer,
// but a 'nulls' marker, which can have many different values.
// (up to 2^31 different values guaranteed on all platforms)
//
// In the standard hlist, termination of a list is the NULL pointer.
// In this special 'nulls' variant, we use the fact that objects stored in
// a list are aligned on a word (4 or 8 bytes alignment).
// We therefore use the last significant bit of 'ptr' :
// Set to 1 : This is a 'nulls' end-of-list marker (ptr >> 1)
// Set to 0 : This is a pointer to some object (ptr)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hlist_nulls_head {
    pub first: *mut hlist_nulls_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hlist_nulls_node {
    pub pprev: *mut *mut hlist_nulls_node next,,
}

//
// ptr_is_a_nulls - Test if a ptr is a nulls
// @ptr: ptr to be tested
//
// get_nulls_value - Get the 'nulls' value of the end of chain
// @ptr: end of chain
//
// Should be called only if is_a_nulls(ptr);
//
// hlist_nulls_unhashed - Has node been removed and reinitialized?
// @h: Node to be checked
//
// Not that not all removal functions will leave a node in unhashed state.
// For example, hlist_del_init_rcu() leaves the node in unhashed state,
// but hlist_nulls_del() does not.
//
// hlist_nulls_unhashed_lockless - Has node been removed and reinitialized?
// @h: Node to be checked
//
// Not that not all removal functions will leave a node in unhashed state.
// For example, hlist_del_init_rcu() leaves the node in unhashed state,
// but hlist_nulls_del() does not.  Unlike hlist_nulls_unhashed(), this
// function may be used locklessly.
//
extern "C" {
    pub fn is_a_nulls(_arg: READ_ONCE(h->first)) -> return;
}
//
// hlist_nulls_for_each_entry	- iterate over list of given type
// @tpos:	the type * to use as a loop cursor.
// @pos:	the &struct hlist_node to use as a loop cursor.
// @head:	the head for your list.
// @member:	the name of the hlist_node within the struct.
//

//
// hlist_nulls_for_each_entry_from - iterate over a hlist continuing from current point
// @tpos:	the type * to use as a loop cursor.
// @pos:	the &struct hlist_node to use as a loop cursor.
// @member:	the name of the hlist_node within the struct.
//

