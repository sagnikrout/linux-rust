//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/netfilter/xt_NFQUEUE.h
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
// iptables module for using NFQUEUE mechanism
//
// (C) 2005 Harald Welte <laforge@netfilter.org>
//
// This software is distributed under GNU GPL v2, 1991
//

// target info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_NFQ_info {
    pub queuenum: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_NFQ_info_v1 {
    pub queuenum: __u16,
    pub queues_total: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_NFQ_info_v2 {
    pub queuenum: __u16,
    pub queues_total: __u16,
    pub bypass: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_NFQ_info_v3 {
    pub queuenum: __u16,
    pub queues_total: __u16,
    pub flags: __u16,
pub const NFQ_FLAG_BYPASS: c_uint = 0x01 /* for compatibility with v2 */;
pub const NFQ_FLAG_CPU_FANOUT: c_uint = 0x02 /* use current CPU (no hashing) */;
pub const NFQ_FLAG_MASK: c_uint = 0x03;
}
