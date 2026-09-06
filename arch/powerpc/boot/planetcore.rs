//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/boot/planetcore.h
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

// Prepare the table for processing, by turning all newlines
// into NULL bytes.
//
extern "C" {
    pub fn planetcore_prepare_table(table: *mut c_char);
}
// Return the value associated with a given key in text,
// decimal, or hex format.
//
// Returns zero/NULL on failure, non-zero on success.
//
extern "C" {
    pub fn planetcore_get_decimal(table: *const c_char, key: *const c_char, val: *mut u64) -> c_int;
}
extern "C" {
    pub fn planetcore_get_hex(table: *const c_char, key: *const c_char, val: *mut u64) -> c_int;
}
// Updates the device tree local-mac-address properties based
// on the EA tag.
//
extern "C" {
    pub fn planetcore_set_mac_addrs(table: *const c_char);
}
// Sets the linux,stdout-path in the /chosen node.  This requires the
// linux,planetcore-label property in each serial node.
//
extern "C" {
    pub fn planetcore_set_stdout_path(table: *const c_char);
}
