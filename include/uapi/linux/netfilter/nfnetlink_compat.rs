//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/netfilter/nfnetlink_compat.h
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

// Old nfnetlink macros for userspace
// nfnetlink groups: Up to 32 maximum
pub const NF_NETLINK_CONNTRACK_NEW: c_uint = 0x00000001;
pub const NF_NETLINK_CONNTRACK_UPDATE: c_uint = 0x00000002;
pub const NF_NETLINK_CONNTRACK_DESTROY: c_uint = 0x00000004;
pub const NF_NETLINK_CONNTRACK_EXP_NEW: c_uint = 0x00000008;
pub const NF_NETLINK_CONNTRACK_EXP_UPDATE: c_uint = 0x00000010;
pub const NF_NETLINK_CONNTRACK_EXP_DESTROY: c_uint = 0x00000020;
// Generic structure for encapsulation optional netfilter information.
// It is reminiscent of sockaddr, but with sa_family replaced
// with attribute type.
// ! This should someday be put somewhere generic as now rtnetlink and
// ! nfnetlink use the same attributes methods. - J. Schulist.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfattr {
    pub nfa_len: __u16,
    pub highest: *mut *mut __u16 nfa_type; / we use 15 bits for the type, and the,
// bit to indicate whether the payload is nested
}

// FIXME: Apart from NFNL_NFA_NESTED shamelessly copy and pasted from
// rtnetlink.h, it's time to put this in a generic file
pub const NFNL_NFA_NEST: c_uint = 0x8000;

pub const NFA_ALIGNTO: c_int = 4;

