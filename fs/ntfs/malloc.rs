//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ntfs/malloc.h
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
//
// malloc.h - NTFS kernel memory handling. Part of the Linux-NTFS project.
//
// Copyright (c) 2001-2005 Anton Altaparmakov
//

//
// __ntfs_malloc - allocate memory in multiples of pages
// @size:	number of bytes to allocate
// @gfp_mask:	extra flags for the allocator
//
// Internal function.  You probably want ntfs_malloc_nofs()...
//
// Allocates @size bytes of memory, rounded up to multiples of PAGE_SIZE and
// returns a pointer to the allocated memory.
//
// If there was insufficient memory to complete the request, return NULL.
// Depending on @gfp_mask the allocation may be guaranteed to succeed.
//
// kmalloc() has per-CPU caches so is faster for now.
extern "C" {
    pub fn kmalloc(_arg: PAGE_SIZE, ~__GFP_HIGHMEM: gfp_mask &) -> return;
}
// return (void *)__get_free_page(gfp_mask);
extern "C" {
    pub fn __vmalloc(_arg: size, _arg: gfp_mask) -> return;
}
//
// ntfs_malloc_nofs - allocate memory in multiples of pages
// @size:	number of bytes to allocate
//
// Allocates @size bytes of memory, rounded up to multiples of PAGE_SIZE and
// returns a pointer to the allocated memory.
//
// If there was insufficient memory to complete the request, return NULL.
//
extern "C" {
    pub fn __ntfs_malloc(_arg: size, __GFP_HIGHMEM: GFP_NOFS |) -> return;
}
//
// ntfs_malloc_nofs_nofail - allocate memory in multiples of pages
// @size:	number of bytes to allocate
//
// Allocates @size bytes of memory, rounded up to multiples of PAGE_SIZE and
// returns a pointer to the allocated memory.
//
// This function guarantees that the allocation will succeed.  It will sleep
// for as long as it takes to complete the allocation.
//
// If there was insufficient memory to complete the request, return NULL.
//
extern "C" {
    pub fn __ntfs_malloc(_arg: size, __GFP_NOFAIL: GFP_NOFS | __GFP_HIGHMEM |) -> return;
}
