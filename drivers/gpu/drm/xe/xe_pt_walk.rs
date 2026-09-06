//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_pt_walk.h
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
// Copyright © 2022 Intel Corporation
//

//
// struct xe_ptw - base class for driver pagetable subclassing.
// @children: Pointer to an array of children if any.
// @staging: Pointer to an array of staging if any.
//
// Drivers could subclass this, and if it's a page-directory, typically
// embed an array of xe_ptw pointers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_ptw {
    pub children: *mut xe_ptw,
    pub staging: *mut xe_ptw,
}

//
// struct xe_pt_walk - Embeddable struct for walk parameters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_pt_walk {
// @ops: The walk ops used for the pagewalk
    pub ops: *const xe_pt_walk_ops,
//
// @shifts: Array of page-table entry shifts used for the
// different levels, starting out with the leaf level 0
// page-shift as the first entry. It's legal for this pointer to be
// changed during the walk.
//
    pub shifts: *const u64,
// @max_level: Highest populated level in @shifts
    pub max_level: c_uint,
//
// @shared_pt_mode: Whether to skip all entries that are private
// to the address range and called only for entries that are
// shared with other address ranges. Such entries are referred to
// as shared pagetables.
//
    pub shared_pt_mode: bool,
// @staging: Walk staging PT structure
    pub staging: bool,
}

//
// typedef xe_pt_entry_fn - gpu page-table-walk callback-function
// @parent: The parent page table.
// @offset: The offset (number of entries) into the page table.
// @level: The level of @parent.
// @addr: The virtual address.
// @next: The virtual address for the next call, or end address.
// @child: Pointer to pointer to child page-table at this @offset. The
// function may modify the value pointed to if, for example, allocating a
// child page table.
// @action: The walk action to take upon return. See <linux/pagewalk.h>.
// @walk: The walk parameters.
//
// struct xe_pt_walk_ops - Walk callbacks.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_pt_walk_ops {
//
// @pt_entry: Callback to be called for each page table entry prior
// to descending to the next level. The returned value of the action
// function parameter is honored.
//
    pub pt_entry: xe_pt_entry_fn,
//
// @pt_post_descend: Callback to be called for each page table entry
// after return from descending to the next level. The returned value
// of the action function parameter is ignored.
//
    pub pt_post_descend: xe_pt_entry_fn,
}

//
// xe_pt_covers - Whether the address range covers an entire entry in @level
// @addr: Start of the range.
// @end: End of range + 1.
// @level: Page table level.
// @walk: Page table walk info.
//
// This function is a helper to aid in determining whether a leaf page table
// entry can be inserted at this @level.
//
// Return: Whether the range provided covers exactly an entry at this level.
//
// xe_pt_num_entries - Number of page-table entries of a given range at this
// level
// @addr: Start address.
// @end: End address.
// @level: Page table level.
// @walk: Walk info.
//
// Return: The number of page table entries at this level between @addr and
// @end.
//
// xe_pt_offset - Offset of the page-table entry for a given address.
// @addr: The address.
// @level: Page table level.
// @walk: Walk info.
//
// Return: The page table entry offset for the given address in a
// page table with size indicated by @level.
//
