//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpib/common/ibsys.h
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

pub const MAX_GPIB_PRIMARY_ADDRESS: c_int = 30;
pub const MAX_GPIB_SECONDARY_ADDRESS: c_int = 31;
extern "C" {
    pub fn gpib_allocate_board(board: *mut gpib_board) -> c_int;
}
extern "C" {
    pub fn gpib_deallocate_board(board: *mut gpib_board);
}
extern "C" {
    pub fn num_status_bytes(dev: *const gpib_status_queue) -> c_uint;
}
extern "C" {
    pub fn autopoll_all_devices(board: *mut gpib_board) -> c_int;
}
