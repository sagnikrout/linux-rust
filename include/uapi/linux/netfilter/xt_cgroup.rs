//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/netfilter/xt_cgroup.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_cgroup_info_v0 {
    pub id: __u32,
    pub invert: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_cgroup_info_v1 {
    pub has_path: __u8,
    pub has_classid: __u8,
    pub invert_path: __u8,
    pub invert_classid: __u8,
    pub path: [c_char; PATH_MAX],
    pub classid: __u32,
// kernel internal data
    pub __attribute__((aligned(8))): *mut *mut void priv,
}

pub const XT_CGROUP_PATH_MAX: c_int = 512;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_cgroup_info_v2 {
    pub has_path: __u8,
    pub has_classid: __u8,
    pub invert_path: __u8,
    pub invert_classid: __u8,
    pub path: [c_char; XT_CGROUP_PATH_MAX],
    pub classid: __u32,
}

// kernel internal data
extern "C" {
    pub fn __attribute__(_arg: (aligned(8))) -> *mut void		priv;
}
