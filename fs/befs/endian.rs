//! Automatically rewritten from C Header to Rust Module
//! Source: fs/befs/endian.h
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
//
// linux/fs/befs/endian.h
//
// Copyright (C) 2001 Will Dyson <will_dyson@pobox.com>
//
// Partially based on similar funtions in the sysv driver.
//

// Macro flag: #define LINUX_BEFS_ENDIAN

extern "C" {
    pub fn le64_to_cpu(__le64)n: () -> return;
}
extern "C" {
    pub fn be64_to_cpu(__be64)n: () -> return;
}
extern "C" {
    pub fn le32_to_cpu(__le32)n: () -> return;
}
extern "C" {
    pub fn be32_to_cpu(__be32)n: () -> return;
}
extern "C" {
    pub fn le16_to_cpu(__le16)n: () -> return;
}
extern "C" {
    pub fn be16_to_cpu(__be16)n: () -> return;
}
// Composite types below here
