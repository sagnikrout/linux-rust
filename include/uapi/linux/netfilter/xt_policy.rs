//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/netfilter/xt_policy.h
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

pub const XT_POLICY_MAX_ELEM: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xt_policy_flags {
    XT_POLICY_MATCH_IN	= 0x1,
    XT_POLICY_MATCH_OUT	= 0x2,
    XT_POLICY_MATCH_NONE	= 0x4,
    XT_POLICY_MATCH_STRICT	= 0x8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xt_policy_modes {
    XT_POLICY_MODE_TRANSPORT,
    XT_POLICY_MODE_TUNNEL
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_policy_spec {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union xt_policy_addr {
    pub a4: in_addr,
    pub a6: in6_addr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_policy_elem {

    pub saddr: nf_inet_addr,
    pub smask: nf_inet_addr,
    pub daddr: nf_inet_addr,
    pub dmask: nf_inet_addr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_policy_info {
    pub pol: [xt_policy_elem; XT_POLICY_MAX_ELEM],
    pub flags: __u16,
    pub len: __u16,
}
