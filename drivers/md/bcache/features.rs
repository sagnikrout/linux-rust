//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/bcache/features.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note

pub const BCH_FEATURE_COMPAT: c_int = 0;
pub const BCH_FEATURE_RO_COMPAT: c_int = 1;
pub const BCH_FEATURE_INCOMPAT: c_int = 2;
pub const BCH_FEATURE_TYPE_MASK: c_uint = 0x03;
// Feature set definition
// Incompat feature set
// 32bit bucket size, obsoleted
pub const BCH_FEATURE_INCOMPAT_OBSO_LARGE_BUCKET: c_uint = 0x0001;
// real bucket size is (1 << bucket_size)
pub const BCH_FEATURE_INCOMPAT_LOG_LARGE_BUCKET_SIZE: c_uint = 0x0002;
pub const BCH_FEATURE_COMPAT_SUPP: c_int = 0;
pub const BCH_FEATURE_RO_COMPAT_SUPP: c_int = 0;

extern "C" {
    pub fn bch_print_cache_set_feature_compat(c: *mut cache_set, buf: *mut c_char, size: c_int) -> c_int;
}
extern "C" {
    pub fn bch_print_cache_set_feature_ro_compat(c: *mut cache_set, buf: *mut c_char, size: c_int) -> c_int;
}
extern "C" {
    pub fn bch_print_cache_set_feature_incompat(c: *mut cache_set, buf: *mut c_char, size: c_int) -> c_int;
}
