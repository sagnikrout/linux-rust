//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/psample.h
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
pub struct psample_group {
    pub list: list_head,
    pub net: *mut net,
    pub group_num: u32,
    pub refcount: u32,
    pub seq: u32,
    pub rcu: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct psample_metadata {
    pub trunc_size: u32,
    pub in_ifindex: c_int,
    pub out_ifindex: c_int,
    pub out_tc: u16,
    pub /: *mut *mut u64 out_tc_occ; / bytes,
    pub /: *mut *mut u64 latency; / nanoseconds,
    pub user_cookie: *const u8,
    pub user_cookie_len: u32,
}

extern "C" {
    pub fn psample_group_take(group: *mut psample_group);
}
extern "C" {
    pub fn psample_group_put(group: *mut psample_group);
}

