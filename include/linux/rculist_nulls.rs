//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/rculist_nulls.h
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
// RCU-protected list version
//

//
// hlist_nulls_del_init_rcu - deletes entry from hash list with re-initialization
// @n: the element to delete from the hash list.
//
// Note: hlist_nulls_unhashed() on the node return true after this. It is
// useful for RCU based read lockfree traversal if the writer side
// must know if the list entry is still hashed or already unhashed.
//
// In particular, it means that we can not poison the forward pointers
// that may still be used for walking the hash list and we can only
// zero the pprev pointer so list_unhashed() will return true after
// this.
//
// The caller must take whatever precautions are necessary (such as
// holding appropriate locks) to avoid racing with another
// list-mutation primitive, such as hlist_nulls_add_head_rcu() or
// hlist_nulls_del_rcu(), running on this same list.  However, it is
// perfectly legal to run concurrently with the _rcu list-traversal
// primitives, such as hlist_nulls_for_each_entry_rcu().
//
// hlist_nulls_first_rcu - returns the first element of the hash list.
// @head: the head of the list.
//

//
// hlist_nulls_next_rcu - returns the element of the list after @node.
// @node: element of the list.
//

//
// hlist_nulls_pprev_rcu - returns the dereferenced pprev of @node.
// @node: element of the list.
//

//
// hlist_nulls_del_rcu - deletes entry from hash list without re-initialization
// @n: the element to delete from the hash list.
//
// Note: hlist_nulls_unhashed() on entry does not return true after this,
// the entry is in an undefined state. It is useful for RCU based
// lockfree traversal.
//
// In particular, it means that we can not poison the forward
// pointers that may still be used for walking the hash list.
//
// The caller must take whatever precautions are necessary
// (such as holding appropriate locks) to avoid racing
// with another list-mutation primitive, such as hlist_nulls_add_head_rcu()
// or hlist_nulls_del_rcu(), running on this same list.
// However, it is perfectly legal to run concurrently with
// the _rcu list-traversal primitives, such as
// hlist_nulls_for_each_entry().
//
// hlist_nulls_add_head_rcu
// @n: the element to add to the hash list.
// @h: the list to add to.
//
// Description:
// Adds the specified element to the specified hlist_nulls,
// while permitting racing traversals.
//
// The caller must take whatever precautions are necessary
// (such as holding appropriate locks) to avoid racing
// with another list-mutation primitive, such as hlist_nulls_add_head_rcu()
// or hlist_nulls_del_rcu(), running on this same list.
// However, it is perfectly legal to run concurrently with
// the _rcu list-traversal primitives, such as
// hlist_nulls_for_each_entry_rcu(), used to prevent memory-consistency
// problems on Alpha CPUs.  Regardless of the type of CPU, the
// list-traversal primitive must be guarded by rcu_read_lock().
//
// hlist_nulls_add_tail_rcu
// @n: the element to add to the hash list.
// @h: the list to add to.
//
// Description:
// Adds the specified element to the specified hlist_nulls,
// while permitting racing traversals.
//
// The caller must take whatever precautions are necessary
// (such as holding appropriate locks) to avoid racing
// with another list-mutation primitive, such as hlist_nulls_add_head_rcu()
// or hlist_nulls_del_rcu(), running on this same list.
// However, it is perfectly legal to run concurrently with
// the _rcu list-traversal primitives, such as
// hlist_nulls_for_each_entry_rcu(), used to prevent memory-consistency
// problems on Alpha CPUs.  Regardless of the type of CPU, the
// list-traversal primitive must be guarded by rcu_read_lock().
//
// Note: write side code, so rcu accessors are not needed.
// after that hlist_nulls_del will work
//
// hlist_nulls_replace_rcu - replace an old entry by a new one
// @old: the element to be replaced
// @new: the new element to insert
//
// Description:
// Replace the old entry with the new one in a RCU-protected hlist_nulls, while
// permitting racing traversals.
//
// The caller must take whatever precautions are necessary (such as holding
// appropriate locks) to avoid racing with another list-mutation primitive, such
// as hlist_nulls_add_head_rcu() or hlist_nulls_del_rcu(), running on this same
// list.  However, it is perfectly legal to run concurrently with the _rcu
// list-traversal primitives, such as hlist_nulls_for_each_entry_rcu().
//
// hlist_nulls_replace_init_rcu - replace an old entry by a new one and
// initialize the old
// @old: the element to be replaced
// @new: the new element to insert
//
// Description:
// Replace the old entry with the new one in a RCU-protected hlist_nulls, while
// permitting racing traversals, and reinitialize the old entry.
//
// Note: @old must be hashed.
//
// The caller must take whatever precautions are necessary (such as holding
// appropriate locks) to avoid racing with another list-mutation primitive, such
// as hlist_nulls_add_head_rcu() or hlist_nulls_del_rcu(), running on this same
// list. However, it is perfectly legal to run concurrently with the _rcu
// list-traversal primitives, such as hlist_nulls_for_each_entry_rcu().
//
// hlist_nulls_for_each_entry_rcu - iterate over rcu list of given type
// @tpos:	the type * to use as a loop cursor.
// @pos:	the &struct hlist_nulls_node to use as a loop cursor.
// @head:	the head of the list.
// @member:	the name of the hlist_nulls_node within the struct.
//
// The barrier() is needed to make sure compiler doesn't cache first element [1],
// as this loop can be restarted [2]
// [1] Documentation/memory-barriers.txt around line 1533
// [2] Documentation/RCU/rculist_nulls.rst around line 146
//

//
// hlist_nulls_for_each_entry_safe -
// iterate over list of given type safe against removal of list entry
// @tpos:	the type * to use as a loop cursor.
// @pos:	the &struct hlist_nulls_node to use as a loop cursor.
// @head:	the head of the list.
// @member:	the name of the hlist_nulls_node within the struct.
//

