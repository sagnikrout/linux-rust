//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/netfilter_ipv6.h
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
// IPv6-specific defines for netfilter.
// (C)1998 Rusty Russell -- This code is GPL.
// (C)1999 David Jeffery
// this header was blatantly ripped from netfilter_ipv4.h
// it's amazing what adding a bunch of 6s can do =8^)
//

// only for userspace compatibility
// IP6 Hooks
// After promisc drops, checksum checks.
pub const NF_IP6_PRE_ROUTING: c_int = 0;
// If the packet is destined for this box.
pub const NF_IP6_LOCAL_IN: c_int = 1;
// If the packet is destined for another interface.
pub const NF_IP6_FORWARD: c_int = 2;
// Packets coming from a local process.
pub const NF_IP6_LOCAL_OUT: c_int = 3;
// Packets about to hit the wire.
pub const NF_IP6_POST_ROUTING: c_int = 4;
pub const NF_IP6_NUMHOOKS: c_int = 5;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nf_ip6_hook_priorities {
    NF_IP6_PRI_FIRST = __KERNEL_INT_MIN,
    NF_IP6_PRI_RAW_BEFORE_DEFRAG = -450,
    NF_IP6_PRI_CONNTRACK_DEFRAG = -400,
    NF_IP6_PRI_RAW = -300,
    NF_IP6_PRI_SELINUX_FIRST = -225,
    NF_IP6_PRI_CONNTRACK = -200,
    NF_IP6_PRI_MANGLE = -150,
    NF_IP6_PRI_NAT_DST = -100,
    NF_IP6_PRI_FILTER = 0,
    NF_IP6_PRI_SECURITY = 50,
    NF_IP6_PRI_NAT_SRC = 100,
    NF_IP6_PRI_SELINUX_LAST = 225,
    NF_IP6_PRI_CONNTRACK_HELPER = 300,
    NF_IP6_PRI_LAST = __KERNEL_INT_MAX,
}
