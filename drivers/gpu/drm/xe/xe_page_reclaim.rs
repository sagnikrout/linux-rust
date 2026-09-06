//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_page_reclaim.h
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2025 Intel Corporation
//

pub const XE_PAGE_RECLAIM_MAX_ENTRIES: c_int = 512;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_guc_page_reclaim_entry {
    pub qw: u64,
// valid reclaim entry bit

//
// offset order of page size to be reclaimed
// page_size = 1 << (XE_PTE_SHIFT + reclamation_size)
//

// lower 20 bits of the physical address

// upper 20 bits of the physical address

    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_page_reclaim_list {
// @entries: array of page reclaim entries, page allocated
    pub entries: *mut xe_guc_page_reclaim_entry,
// @num_entries: number of entries
    pub num_entries: c_int,

}

//
// xe_page_reclaim_list_is_new() - Check if PRL is new allocation
// @prl: Pointer to page reclaim list
//
// PRL indicates it hasn't been allocated through both values being NULL
//
// xe_page_reclaim_list_valid() - Check if the page reclaim list is valid
// @prl: Pointer to page reclaim list
//
// PRL uses the XE_PAGE_RECLAIM_INVALID_LIST to indicate that a PRL
// is unusable.
//
extern "C" {
    pub fn xe_page_reclaim_skip(tile: *mut xe_tile, vma: *mut xe_vma) -> bool;
}
extern "C" {
    pub fn xe_page_reclaim_list_invalidate(prl: *mut xe_page_reclaim_list);
}
//
// xe_page_reclaim_list_abort() - Invalidate a PRL and log an abort reason
// @gt: GT owning the page reclaim request
// @prl: Page reclaim list to invalidate
// @fmt: format string for the log message with args
//
// Abort page reclaim process by invalidating PRL and doing any relevant logging.
//

extern "C" {
    pub fn xe_page_reclaim_list_init(prl: *mut xe_page_reclaim_list);
}
extern "C" {
    pub fn xe_page_reclaim_list_alloc_entries(prl: *mut xe_page_reclaim_list) -> c_int;
}
//
// xe_page_reclaim_entries_get() - Increment the reference count of page reclaim entries.
// @entries: Pointer to the array of page reclaim entries.
//
// This function increments the reference count of the backing page.
//
// xe_page_reclaim_entries_put() - Decrement the reference count of page reclaim entries.
// @entries: Pointer to the array of page reclaim entries.
//
// This function decrements the reference count of the backing page
// and frees it if the count reaches zero.
//
extern "C" {
    pub fn xe_guc_page_reclaim_done_handler(guc: *mut xe_guc, msg: *mut u32, len: u32) -> c_int;
}
