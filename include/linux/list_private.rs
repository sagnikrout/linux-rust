//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/list_private.h
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
// Copyright (c) 2025, Google LLC.
// Pasha Tatashin <pasha.tatashin@soleen.com>
//
// DOC: Private List Primitives
//
// Provides a set of list primitives identical in function to those in
// ``<linux/list.h>``, but designed for cases where the embedded
// ``&struct list_head`` is private member.
//

//
// list_private_entry - get the struct for this entry
// @ptr:	the &struct list_head pointer.
// @type:	the type of the struct this is embedded in.
// @member:	the identifier passed to ACCESS_PRIVATE.
//

//
// list_private_first_entry - get the first element from a list
// @ptr:	the list head to take the element from.
// @type:	the type of the struct this is embedded in.
// @member:	the identifier passed to ACCESS_PRIVATE.
//

//
// list_private_last_entry - get the last element from a list
// @ptr:	the list head to take the element from.
// @type:	the type of the struct this is embedded in.
// @member:	the identifier passed to ACCESS_PRIVATE.
//

//
// list_private_next_entry - get the next element in list
// @pos:	the type * to cursor
// @member:	the name of the list_head within the struct.
//

//
// list_private_next_entry_circular - get the next element in list
// @pos:	the type * to cursor.
// @head:	the list head to take the element from.
// @member:	the name of the list_head within the struct.
//
// Wraparound if pos is the last element (return the first element).
// Note, that list is expected to be not empty.
//

//
// list_private_prev_entry - get the prev element in list
// @pos:	the type * to cursor
// @member:	the name of the list_head within the struct.
//

//
// list_private_prev_entry_circular - get the prev element in list
// @pos:	the type * to cursor.
// @head:	the list head to take the element from.
// @member:	the name of the list_head within the struct.
//
// Wraparound if pos is the first element (return the last element).
// Note, that list is expected to be not empty.
//

//
// list_private_entry_is_head - test if the entry points to the head of the list
// @pos:	the type * to cursor
// @head:	the head for your list.
// @member:	the name of the list_head within the struct.
//

//
// list_private_for_each_entry - iterate over list of given type
// @pos:	the type * to use as a loop cursor.
// @head:	the head for your list.
// @member:	the name of the list_head within the struct.
//

//
// list_private_for_each_entry_reverse - iterate backwards over list of given type.
// @pos:	the type * to use as a loop cursor.
// @head:	the head for your list.
// @member:	the name of the list_head within the struct.
//

//
// list_private_for_each_entry_continue - continue iteration over list of given type
// @pos:	the type * to use as a loop cursor.
// @head:	the head for your list.
// @member:	the name of the list_head within the struct.
//
// Continue to iterate over list of given type, continuing after
// the current position.
//

//
// list_private_for_each_entry_continue_reverse - iterate backwards from the given point
// @pos:	the type * to use as a loop cursor.
// @head:	the head for your list.
// @member:	the name of the list_head within the struct.
//
// Start to iterate over list of given type backwards, continuing after
// the current position.
//

//
// list_private_for_each_entry_from - iterate over list of given type from the current point
// @pos:	the type * to use as a loop cursor.
// @head:	the head for your list.
// @member:	the name of the list_head within the struct.
//
// Iterate over list of given type, continuing from current position.
//

//
// list_private_for_each_entry_from_reverse - iterate backwards over list of given type
// from the current point
// @pos:	the type * to use as a loop cursor.
// @head:	the head for your list.
// @member:	the name of the list_head within the struct.
//
// Iterate backwards over list of given type, continuing from current position.
//

//
// list_private_for_each_entry_safe - iterate over list of given type safe against removal of list entry
// @pos:	the type * to use as a loop cursor.
// @n:		another type * to use as temporary storage
// @head:	the head for your list.
// @member:	the name of the list_head within the struct.
//

//
// list_private_for_each_entry_safe_continue - continue list iteration safe against removal
// @pos:	the type * to use as a loop cursor.
// @n:		another type * to use as temporary storage
// @head:	the head for your list.
// @member:	the name of the list_head within the struct.
//
// Iterate over list of given type, continuing after current point,
// safe against removal of list entry.
//

//
// list_private_for_each_entry_safe_from - iterate over list from current point safe against removal
// @pos:	the type * to use as a loop cursor.
// @n:		another type * to use as temporary storage
// @head:	the head for your list.
// @member:	the name of the list_head within the struct.
//
// Iterate over list of given type from current point, safe against
// removal of list entry.
//

//
// list_private_for_each_entry_safe_reverse - iterate backwards over list safe against removal
// @pos:	the type * to use as a loop cursor.
// @n:		another type * to use as temporary storage
// @head:	the head for your list.
// @member:	the name of the list_head within the struct.
//
// Iterate backwards over list of given type, safe against removal
// of list entry.
//

//
// list_private_safe_reset_next - reset a stale list_for_each_entry_safe loop
// @pos:	the loop cursor used in the list_for_each_entry_safe loop
// @n:		temporary storage used in list_for_each_entry_safe
// @member:	the name of the list_head within the struct.
//
// list_safe_reset_next is not safe to use in general if the list may be
// modified concurrently (eg. the lock is dropped in the loop body). An
// exception to this is if the cursor element (pos) is pinned in the list,
// and list_safe_reset_next is called after re-taking the lock and before
// completing the current iteration of the loop body.
//

