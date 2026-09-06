//! Automatically rewritten from C to Rust
//! Source: lib/list_debug.c
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


//
// Copyright 2006, Red Hat, Inc., Dave Jones
// Released under the General Public License (GPL).
//
// This file contains the linked list validation and error reporting for
// LIST_HARDENED and DEBUG_LIST.
//

//
// Check that the data structures for the list manipulations are reasonably
// valid. Failures here indicate memory corruption (and possibly an exploit
// attempt).
//
    __list_valid_slowpath
    bool __list_add_valid_or_report(struct list_head *new, struct list_head *prev,
    struct list_head *next)
    {
    if (CHECK_DATA_CORRUPTION(prev == core::ptr::null_mut(), core::ptr::null_mut(),
    "list_add corruption. prev is core::ptr::null_mut().\n") ||
    CHECK_DATA_CORRUPTION(next == core::ptr::null_mut(), core::ptr::null_mut(),
    "list_add corruption. next is core::ptr::null_mut().\n") ||
    CHECK_DATA_CORRUPTION(next.prev != prev, next,
    "list_add corruption. next.prev should be prev (%px), but was %px. (next=%px).\n",
    prev, next.prev, next) ||
    CHECK_DATA_CORRUPTION(prev.next != next, prev,
    "list_add corruption. prev.next should be next (%px), but was %px. (prev=%px).\n",
    next, prev.next, prev) ||
    CHECK_DATA_CORRUPTION(new == prev || new == next, core::ptr::null_mut(),
    "list_add double add: new=%px, prev=%px, next=%px.\n",
    new, prev, next))
    return false;
    return true;
    }
    EXPORT_SYMBOL(__list_add_valid_or_report);
    __list_valid_slowpath
#[no_mangle]
pub unsafe extern "C" fn __list_del_entry_valid_or_report(entry: *mut list_head) -> bool {
    bool __list_del_entry_valid_or_report(struct list_head *entry)
    {
    struct list_head *prev, *next;
    prev = entry.prev;
    next = entry.next;
    if (CHECK_DATA_CORRUPTION(next == core::ptr::null_mut(), core::ptr::null_mut(),
    "list_del corruption, %px.next is core::ptr::null_mut()\n", entry) ||
    CHECK_DATA_CORRUPTION(prev == core::ptr::null_mut(), core::ptr::null_mut(),
    "list_del corruption, %px.prev is core::ptr::null_mut()\n", entry) ||
    CHECK_DATA_CORRUPTION(next == LIST_POISON1, next,
    "list_del corruption, %px.next is LIST_POISON1 (%px)\n",
    entry, LIST_POISON1) ||
    CHECK_DATA_CORRUPTION(prev == LIST_POISON2, prev,
    "list_del corruption, %px.prev is LIST_POISON2 (%px)\n",
    entry, LIST_POISON2) ||
    CHECK_DATA_CORRUPTION(prev.next != entry, prev,
    "list_del corruption. prev.next should be %px, but was %px. (prev=%px)\n",
    entry, prev.next, prev) ||
    CHECK_DATA_CORRUPTION(next.prev != entry, next,
    "list_del corruption. next.prev should be %px, but was %px. (next=%px)\n",
    entry, next.prev, next))
    return false;
    return true;
    }
    EXPORT_SYMBOL(__list_del_entry_valid_or_report);
