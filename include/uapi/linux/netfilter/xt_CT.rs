//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/netfilter/xt_CT.h
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
pub struct xt_ct_target_info {
    pub flags: __u16,
    pub zone: __u16,
    pub ct_events: __u32,
    pub exp_events: __u32,
    pub helper: [c_char; 16],
// Used internally by the kernel
    pub __attribute__((aligned(8))): *mut *mut nf_conn ct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_ct_target_info_v1 {
    pub flags: __u16,
    pub zone: __u16,
    pub ct_events: __u32,
    pub exp_events: __u32,
    pub helper: [c_char; 16],
    pub timeout: [c_char; 32],
// Used internally by the kernel
    pub __attribute__((aligned(8))): *mut *mut nf_conn ct,
}
