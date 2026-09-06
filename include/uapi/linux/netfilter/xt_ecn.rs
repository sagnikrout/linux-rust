//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/netfilter/xt_ecn.h
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
// iptables module for matching the ECN header in IPv4 and TCP header
//
// (C) 2002 Harald Welte <laforge@gnumonks.org>
//
// This software is distributed under GNU GPL v2, 1991
//
// ipt_ecn.h,v 1.4 2002/08/05 19:39:00 laforge Exp
//

pub const XT_ECN_OP_MATCH_IP: c_uint = 0x01;
pub const XT_ECN_OP_MATCH_ECE: c_uint = 0x10;
pub const XT_ECN_OP_MATCH_CWR: c_uint = 0x20;
pub const XT_ECN_OP_MATCH_MASK: c_uint = 0xce;
// match info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_ecn_info {
    pub operation: __u8,
    pub invert: __u8,
    pub ip_ect: __u8,
    pub ect: __u8,
    pub tcp: },
    pub proto: },
}
