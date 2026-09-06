//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/netfilter/nf_conntrack_common.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_conntrack_stat {
    pub found: c_uint,
    pub invalid: c_uint,
    pub insert: c_uint,
    pub insert_failed: c_uint,
    pub clash_resolve: c_uint,
    pub drop: c_uint,
    pub early_drop: c_uint,
    pub error: c_uint,
    pub expect_new: c_uint,
    pub expect_create: c_uint,
    pub expect_delete: c_uint,
    pub search_restart: c_uint,
    pub chaintoolong: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack {
    pub use: refcount_t,
}

extern "C" {
    pub fn nf_conntrack_destroy(nfct: *mut nf_conntrack);
}
// like nf_ct_put, but without module dependency on nf_conntrack
