//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/dm-vdo/priority-table.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright 2023 Red Hat
//

//
// A priority_table is a simple implementation of a priority queue for entries with priorities that
// are small non-negative integer values. It implements the obvious priority queue operations of
// enqueuing an entry and dequeuing an entry with the maximum priority. It also supports removing
// an arbitrary entry. The priority of an entry already in the table can be changed by removing it
// and re-enqueuing it with a different priority. All operations have O(1) complexity.
//
// The links for the table entries must be embedded in the entries themselves. Lists are used to
// link entries in the table and no wrapper type is declared, so an existing list entry in an
// object can also be used to queue it in a priority_table, assuming the field is not used for
// anything else while so queued.
//
// The table is implemented as an array of queues (circular lists) indexed by priority, along with
// a hint for which queues are non-empty. Steven Skiena calls a very similar structure a "bounded
// height priority queue", but given the resemblance to a hash table, "priority table" seems both
// shorter and more apt, if somewhat novel.
//
extern "C" {
    pub fn vdo_free_priority_table(table: *mut priority_table);
}
extern "C" {
    pub fn vdo_reset_priority_table(table: *mut priority_table);
}
extern "C" {
    pub fn vdo_priority_table_dequeue(table: *mut priority_table) -> *mut list_head  __must_check;
}
extern "C" {
    pub fn vdo_priority_table_remove(table: *mut priority_table, entry: *mut list_head);
}
extern "C" {
    pub fn vdo_is_priority_table_empty(table: *mut priority_table) -> bool __must_check;
}
