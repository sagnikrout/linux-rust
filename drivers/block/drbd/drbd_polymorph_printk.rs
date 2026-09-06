//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/block/drbd/drbd_polymorph_printk.h
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

// Macro flag: #define __drbd_printk_drbd_device_unprep()

// Macro flag: #define __drbd_printk_drbd_peer_device_unprep()

// Macro flag: #define __drbd_printk_drbd_resource_unprep(resource)

// Macro flag: #define __drbd_printk_drbd_connection_unprep()
extern "C" {
    pub fn drbd_printk_with_wrong_object_type();
}
extern "C" {
    pub fn drbd_dyn_dbg_with_wrong_object_type();
}

//
// expect  -  Make an assertion
//
// Unlike the assert macro, this macro returns a boolean result.
//

