//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/dm-vdo/memory-alloc.h
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

// Custom memory allocation function that tracks memory usage
extern "C" {
    pub fn vdo_allocate_memory(size: usize, align: usize, what: *const c_char, ptr: *mut c_void) -> int __must_check;
}
//
// Allocate one or more elements of the indicated type, logging an error if the allocation fails.
// The memory will be zeroed.
//
// @COUNT: The number of objects to allocate
// @WHAT: What is being allocated (for error logging)
// @PTR: A pointer to hold the allocated memory
//
// Return: VDO_SUCCESS or an error code
//

//
// Allocate a structure with a flexible array member, with a specified number of elements, logging
// an error if the allocation fails. The memory will be zeroed.
//
// @COUNT: The number of objects to allocate
// @FIELD: The flexible array field at the end of the structure
// @WHAT: What is being allocated (for error logging)
// @PTR: A pointer to hold the allocated memory
//
// Return: VDO_SUCCESS or an error code
//

//
// Allocate memory starting on a cache line boundary, logging an error if the allocation fails. The
// memory will be zeroed.
//
// @size: The number of bytes to allocate
// @what: What is being allocated (for error logging)
// @ptr: A pointer to hold the allocated memory
//
// Return: VDO_SUCCESS or an error code
//
extern "C" {
    pub fn vdo_allocate_memory(_arg: size, _arg: L1_CACHE_BYTES, _arg: what, _arg: ptr) -> return;
}
//
// Allocate one element of the indicated type immediately, failing if the required memory is not
// immediately available.
//
// @size: The number of bytes to allocate
// @what: What is being allocated (for error logging)
//
// Return: pointer to the memory, or NULL if the memory is not available.
//
extern "C" {
    pub fn vdo_allocate_memory_nowait(size: usize, what: *const c_char) -> *mut void __must_check;
}
// Free memory allocated with vdo_allocate().
extern "C" {
    pub fn vdo_free(ptr: *mut c_void);
}
// ptr_ptr = NULL;
//
// Null out a pointer and return a copy to it. This macro should be used when passing a pointer to
// a function for which it is not safe to access the pointer once the function returns.
//

extern "C" {
    pub fn vdo_memory_init();
}
extern "C" {
    pub fn vdo_memory_exit();
}
extern "C" {
    pub fn vdo_unregister_allocating_thread();
}
extern "C" {
    pub fn vdo_get_memory_stats(bytes_used: *mut u64, peak_bytes_used: *mut u64);
}
extern "C" {
    pub fn vdo_report_memory_usage();
}
