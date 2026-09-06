//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/netfilter/xt_TPROXY.h
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

// TPROXY target is capable of marking the packet to perform
// redirection. We can get rid of that whenever we get support for
// mutliple targets in the same rule.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_tproxy_target_info {
    pub mark_mask: __u32,
    pub mark_value: __u32,
    pub laddr: __be32,
    pub lport: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_tproxy_target_info_v1 {
    pub mark_mask: __u32,
    pub mark_value: __u32,
    pub laddr: nf_inet_addr,
    pub lport: __be16,
}
