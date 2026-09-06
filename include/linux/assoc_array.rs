//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/assoc_array.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
// Generic associative array implementation.
//
// See Documentation/core-api/assoc_array.rst for information.
//
// Copyright (C) 2013 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

//
// Generic associative array.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct assoc_array {
    pub /: *mut *mut *mut assoc_array_ptr root; / The node at the root of the tree,
    pub nr_leaves_on_tree: c_ulong,
}

//
// Operations on objects and index keys for use by array manipulation routines.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct assoc_array_ops {
// Method to get a chunk of an index key from caller-supplied data
    pub level): *const *const *const unsigned long (get_key_chunk)(void index_key, int,
// Method to get a piece of an object's index key
    pub level): *const *const *const unsigned long (get_object_key_chunk)(void object, int,
// Is this the object we're looking for?
    pub index_key): *const *const *const bool (compare_object)(void object, void,
// How different is an object from an index key, to a bit position in
// their keys? (or -1 if they're the same)
//
    pub index_key): *const *const *const int (diff_objects)(void object, void,
// Method to free an object.
    pub object): *mut *mut void (free_object)(void,
}

//
// Access and manipulation functions.
//
extern "C" {
    pub fn assoc_array_apply_edit(edit: *mut assoc_array_edit);
}
extern "C" {
    pub fn assoc_array_cancel_edit(edit: *mut assoc_array_edit);
}

