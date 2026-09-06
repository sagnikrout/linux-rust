//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/ttm/ttm_backup.h
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
// Copyright © 2024 Intel Corporation
//

//
// ttm_backup_handle_to_page_ptr() - Convert handle to struct page pointer
// @handle: The handle to convert.
//
// Converts an opaque handle received from a ttm_backup_backup_*()
// function to an (invalid) struct page pointer suitable for a struct page array.
//
// Return: An (invalid) struct page pointer.
//
// ttm_backup_page_ptr_is_handle() - Whether a struct page pointer is a handle
// @page: The struct page pointer to check.
//
// Return: true if the struct page pointer is a handld returned from
// ttm_backup_handle_to_page_ptr(). False otherwise.
//
// ttm_backup_page_ptr_to_handle() - Convert a struct page pointer to a handle
// @page: The struct page pointer to convert
//
// Return: The handle that was previously used in
// ttm_backup_handle_to_page_ptr() to obtain a struct page pointer, suitable
// for use as argument in the struct ttm_backup_drop() or
// ttm_backup_copy_page() functions.
//
extern "C" {
    pub fn ttm_backup_drop(backup: *mut file, handle: pgoff_t);
}
extern "C" {
    pub fn ttm_backup_fini(backup: *mut file);
}
extern "C" {
    pub fn ttm_backup_bytes_avail() -> u64;
}
