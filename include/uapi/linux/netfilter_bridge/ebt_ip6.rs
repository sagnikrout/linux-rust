//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/netfilter_bridge/ebt_ip6.h
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
//
// ebt_ip6
//
// Authors:
// Kuo-Lang Tseng <kuo-lang.tseng@intel.com>
// Manohar Castelino <manohar.r.castelino@intel.com>
//
// Jan 11, 2008
//

pub const EBT_IP6_SOURCE: c_uint = 0x01;
pub const EBT_IP6_DEST: c_uint = 0x02;
pub const EBT_IP6_TCLASS: c_uint = 0x04;
pub const EBT_IP6_PROTO: c_uint = 0x08;
pub const EBT_IP6_SPORT: c_uint = 0x10;
pub const EBT_IP6_DPORT: c_uint = 0x20;
pub const EBT_IP6_ICMP6: c_uint = 0x40;

// the same values are used for the invflags
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ebt_ip6_info {
    pub saddr: in6_addr,
    pub daddr: in6_addr,
    pub smsk: in6_addr,
    pub dmsk: in6_addr,
    pub tclass: __u8,
    pub protocol: __u8,
    pub bitmask: __u8,
    pub invflags: __u8,
    pub sport: [__u16; 2],
    pub icmpv6_type: [__u8; 2],
}
