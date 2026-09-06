//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/netfilter_ipv4/ip_tables.h
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
// 25-Jul-1998 Major changes to allow for ip chain table
//
// 3-Jan-2000 Named tables to allow packet selection for different uses.
//
// Format of an IP firewall descriptor
//
// src, dst, src_mask, dst_mask are always stored in network byte order.
// flags are stored in host byte order (of course).
// Port numbers are stored in HOST byte order.
//

extern "C" {
    pub fn ipt_unregister_table_exit(net: *mut net, name: *const c_char);
}
// Standard entry.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipt_standard {
    pub entry: ipt_entry,
    pub target: xt_standard_target,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipt_error {
    pub entry: ipt_entry,
    pub target: xt_error_target,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ipt_entry {
    pub ip: ipt_ip,
    pub nfcache: compat_uint_t,
    pub target_offset: __u16,
    pub next_offset: __u16,
    pub comefrom: compat_uint_t,
    pub counters: compat_xt_counters,
    pub elems: [c_uchar; ],
}

// Helper functions

