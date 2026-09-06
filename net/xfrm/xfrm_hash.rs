//! Automatically rewritten from C Header to Rust Module
//! Source: net/xfrm/xfrm_hash.h
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

extern "C" {
    pub fn ntohl(_arg: addr->a4) -> return;
}
extern "C" {
    pub fn jhash2()addr->a6: *mut ( u32, _arg: 4, _arg: 0) -> return;
}
extern "C" {
    pub fn ntohl(__be32)sum: () -> return;
}
extern "C" {
    pub fn __xfrm6_addr_hash(__xfrm6_addr_hash(saddr: daddr) ^) -> return;
}
extern "C" {
    pub fn jhash2()addr->a6: *mut ( u32, _arg: pdw, _arg: initval) -> return;
}
extern "C" {
    pub fn xfrm_hash_free(n: *mut hlist_head, sz: c_uint);
}
