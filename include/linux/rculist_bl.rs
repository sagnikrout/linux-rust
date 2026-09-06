//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/rculist_bl.h
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
// RCU-protected bl list version. See include/linux/list_bl.h.
//

// return the first ptr or next element in an RCU protected list

//
// hlist_bl_del_rcu - deletes entry from hash list without re-initialization
// @n: the element to delete from the hash list.
//
// Note: hlist_bl_unhashed() on entry does not return true after this,
// the entry is in an undefined state. It is useful for RCU based
// lockfree traversal.
//
// In particular, it means that we can not poison the forward
// pointers that may still be used for walking the hash list.
//
// The caller must take whatever precautions are necessary
// (such as holding appropriate locks) to avoid racing
// with another list-mutation primitive, such as hlist_bl_add_head_rcu()
// or hlist_bl_del_rcu(), running on this same list.
// However, it is perfectly legal to run concurrently with
// the _rcu list-traversal primitives, such as
// hlist_bl_for_each_entry().
//
// hlist_bl_add_head_rcu
// @n: the element to add to the hash list.
// @h: the list to add to.
//
// Description:
// Adds the specified element to the specified hlist_bl,
// while permitting racing traversals.
//
// The caller must take whatever precautions are necessary
// (such as holding appropriate locks) to avoid racing
// with another list-mutation primitive, such as hlist_bl_add_head_rcu()
// or hlist_bl_del_rcu(), running on this same list.
// However, it is perfectly legal to run concurrently with
// the _rcu list-traversal primitives, such as
// hlist_bl_for_each_entry_rcu(), used to prevent memory-consistency
// problems on Alpha CPUs.  Regardless of the type of CPU, the
// list-traversal primitive must be guarded by rcu_read_lock().
//
// don't need hlist_bl_first_rcu* because we're under lock
// need _rcu because we can have concurrent lock free readers
//
// hlist_bl_for_each_entry_rcu - iterate over rcu list of given type
// @tpos:	the type * to use as a loop cursor.
// @pos:	the &struct hlist_bl_node to use as a loop cursor.
// @head:	the head for your list.
// @member:	the name of the hlist_bl_node within the struct.
//

//
// hlist_bl_for_each_entry_continue_rcu - continue iteration over list of given
// type
// @tpos:	the type * to use as a loop cursor.
// @pos:	the &struct hlist_bl_node to use as a loop cursor.
// @member:	the name of the hlist_bl_node within the struct.
//
// Continue to iterate over list of given type, continuing after
// the current position which must have been in the list when the RCU read
// lock was taken.
// This would typically require either that you obtained the node from a
// previous walk of the list in the same RCU read-side critical section, or
// that you held some sort of non-RCU reference (such as a reference count)
// to keep the node alive *and* in the list.
//

