//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/unaligned.h
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
// This is the most generic implementation of unaligned accesses
// and should work almost anywhere.
//

extern "C" {
    pub fn le16_to_cpu(_arg: __get_unaligned_t(__le16, _arg: p)) -> return;
}
extern "C" {
    pub fn le32_to_cpu(_arg: __get_unaligned_t(__le32, _arg: p)) -> return;
}
extern "C" {
    pub fn le64_to_cpu(_arg: __get_unaligned_t(__le64, _arg: p)) -> return;
}
extern "C" {
    pub fn be16_to_cpu(_arg: __get_unaligned_t(__be16, _arg: p)) -> return;
}
extern "C" {
    pub fn be32_to_cpu(_arg: __get_unaligned_t(__be32, _arg: p)) -> return;
}
extern "C" {
    pub fn be64_to_cpu(_arg: __get_unaligned_t(__be64, _arg: p)) -> return;
}
extern "C" {
    pub fn __get_unaligned_be24(_arg: p) -> return;
}
extern "C" {
    pub fn __get_unaligned_le24(_arg: p) -> return;
}
// p++ = (val >> 16) & 0xff;
// p++ = (val >> 8) & 0xff;
// p++ = val & 0xff;
// p++ = (val >> 8) & 0xff;
// p++ = (val >> 16) & 0xff;
// p++ = (val >> 40) & 0xff;
// p++ = (val >> 32) & 0xff;
// p++ = (val >> 24) & 0xff;
// p++ = (val >> 16) & 0xff;
// p++ = (val >> 8) & 0xff;
// p++ = val & 0xff;
extern "C" {
    pub fn __get_unaligned_be48(_arg: p) -> return;
}
