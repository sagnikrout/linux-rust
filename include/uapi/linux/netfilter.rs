//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/netfilter.h
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

// Responses from hook functions.
pub const NF_DROP: c_int = 0;
pub const NF_ACCEPT: c_int = 1;
pub const NF_STOLEN: c_int = 2;
pub const NF_QUEUE: c_int = 3;
pub const NF_REPEAT: c_int = 4;

// we overload the higher bits for encoding auxiliary data such as the queue
// number or errno values. Not nice, but better than additional function
// arguments.
pub const NF_VERDICT_MASK: c_uint = 0x000000ff;
// extra verdict flags have mask 0x0000ff00
pub const NF_VERDICT_FLAG_QUEUE_BYPASS: c_uint = 0x00008000;
// queue number (NF_QUEUE) or errno (NF_DROP)
pub const NF_VERDICT_QMASK: c_uint = 0xffff0000;
pub const NF_VERDICT_QBITS: c_int = 16;

// only for userspace compatibility
// NF_VERDICT_BITS should be 8 now, but userspace might break if this changes
pub const NF_VERDICT_BITS: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nf_inet_hooks {
    NF_INET_PRE_ROUTING,
    NF_INET_LOCAL_IN,
    NF_INET_FORWARD,
    NF_INET_LOCAL_OUT,
    NF_INET_POST_ROUTING,
    NF_INET_NUMHOOKS,
    NF_INET_INGRESS = NF_INET_NUMHOOKS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nf_dev_hooks {
    NF_NETDEV_INGRESS,
    NF_NETDEV_EGRESS,
    NF_NETDEV_NUMHOOKS
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union nf_inet_addr {
    pub all: [__u32; 4],
    pub ip: __be32,
    pub ip6: [__be32; 4],
    pub in: in_addr,
    pub in6: in6_addr,
}
