//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/rheap.h
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
// include/asm-ppc/rheap.h
//
// Header file for the implementation of a remote heap.
//
// Author: Pantelis Antoniou <panto@intracom.gr>
//
// 2004 (c) INTRACOM S.A. Greece. This file is licensed under
// the terms of the GNU General Public License version 2. This program
// is licensed "as is" without any warranty of any kind, whether express
// or implied.
//

pub const RHIF_STATIC_INFO: c_uint = 0x1;
pub const RHIF_STATIC_BLOCK: c_uint = 0x2;
pub const RHGS_FREE: c_int = 0;
pub const RHGS_TAKEN: c_int = 1;
// Create a remote heap dynamically
// Destroy a remote heap, created by rh_create()
extern "C" {
    pub fn rh_destroy(info: *mut *mut rh_info_t);
}
// Initialize in place a remote info block
// Attach a free region to manage
extern "C" {
    pub fn rh_attach_region(info: *mut *mut rh_info_t, start: c_ulong, size: c_int) -> c_int;
}
// Detach a free region
extern "C" {
    pub fn rh_detach_region(info: *mut *mut rh_info_t, start: c_ulong, size: c_int) -> c_ulong;
}
// Allocate the given size from the remote heap (with alignment)
// Allocate the given size from the remote heap
extern "C" {
    pub fn rh_alloc(info: *mut *mut rh_info_t, size: c_int, owner: *const c_char) -> c_ulong;
}
// Allocate the given size from the given address
// Free the allocated area
extern "C" {
    pub fn rh_free(info: *mut *mut rh_info_t, start: c_ulong) -> c_int;
}
// Get stats for debugging purposes
// Simple dump of remote heap info
extern "C" {
    pub fn rh_dump(info: *mut *mut rh_info_t);
}
// Simple dump of remote info block
extern "C" {
    pub fn rh_dump_blk(info: *mut rh_info_t, blk: *mut rh_block_t);
}
// Set owner of taken block
extern "C" {
    pub fn rh_set_owner(info: *mut *mut rh_info_t, start: c_ulong, owner: *const c_char) -> c_int;
}
