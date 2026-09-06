//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/ipv6_frag.h
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
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ip6_defrag_users {
    IP6_DEFRAG_LOCAL_DELIVER,
    IP6_DEFRAG_CONNTRACK_IN,
    __IP6_DEFRAG_CONNTRACK_IN	= IP6_DEFRAG_CONNTRACK_IN + USHRT_MAX,
    IP6_DEFRAG_CONNTRACK_OUT,
    __IP6_DEFRAG_CONNTRACK_OUT	= IP6_DEFRAG_CONNTRACK_OUT + USHRT_MAX,
    IP6_DEFRAG_CONNTRACK_BRIDGE_IN,
    __IP6_DEFRAG_CONNTRACK_BRIDGE_IN = IP6_DEFRAG_CONNTRACK_BRIDGE_IN + USHRT_MAX,
}

//
// Equivalent of ipv4 struct ip
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct frag_queue {
    pub q: inet_frag_queue,
    pub iif: c_int,
    pub nhoffset: __u16,
    pub ecn: u8,
}

// Paired with the WRITE_ONCE() in fqdir_pre_exit().
// Don't send error if the first segment did not arrive.
// sk_buff::dev and sk_buff::rbnode are unionized. So we
// pull the head out of the tree in order to be able to
// deal with head->dev.
//
// Check if the upper layer header is truncated in the first fragment.

