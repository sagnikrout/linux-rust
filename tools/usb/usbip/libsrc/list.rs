//! Automatically rewritten from C Header to Rust Module
//! Source: tools/usb/usbip/libsrc/list.h
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
// Stripped down implementation of linked list taken
// from the Linux Kernel.
//
// Simple doubly linked list implementation.
//
// Some of the internal functions ("__xxx") are useful when
// manipulating whole lists rather than single entries, as
// sometimes we already know the next/prev entries and we can
// generate better code by using them directly rather than
// using the generic single-entry routines.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct list_head {
    pub prev: *mut *mut list_head next,,
}

//
// Insert a new entry between two known consecutive entries.
//
// This is only for internal list manipulation where we know
// the prev/next entries already!
//
// list_add - add a new entry
// @new: new entry to be added
// @head: list head to add it after
//
// Insert a new entry after the specified head.
// This is good for implementing stacks.
//
// Delete a list entry by making the prev/next entries
// point to each other.
//
// This is only for internal list manipulation where we know
// the prev/next entries already!
//
pub const POISON_POINTER_DELTA: c_int = 0;

//
// list_del - deletes entry from list.
// @entry: the element to delete from the list.
// Note: list_empty() on entry does not return true after this, the entry is
// in an undefined state.
//
// list_entry - get the struct for this entry
// @ptr:	the &struct list_head pointer.
// @type:	the type of the struct this is embedded in.
// @member:	the name of the list_head within the struct.
//

//
// list_for_each	-	iterate over a list
// @pos:	the &struct list_head to use as a loop cursor.
// @head:	the head for your list.
//

//
// list_for_each_safe - iterate over a list safe against removal of list entry
// @pos:	the &struct list_head to use as a loop cursor.
// @n:		another &struct list_head to use as temporary storage
// @head:	the head for your list.
//

//
// container_of - cast a member of a structure out to the containing structure
// @ptr:	the pointer to the member.
// @type:	the type of the container struct this is embedded in.
// @member:	the name of the member within the struct.
//

