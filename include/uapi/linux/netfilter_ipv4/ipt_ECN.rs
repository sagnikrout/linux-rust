//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/netfilter_ipv4/ipt_ECN.h
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
// Header file for iptables ipt_ECN target
//
// (C) 2002 by Harald Welte <laforge@gnumonks.org>
//
// This software is distributed under GNU GPL v2, 1991
//
// ipt_ECN.h,v 1.3 2002/05/29 12:17:40 laforge Exp
//

pub const IPT_ECN_OP_SET_IP: c_uint = 0x01	/* set ECN bits of IPv4 header */;
pub const IPT_ECN_OP_SET_ECE: c_uint = 0x10	/* set ECE bit of TCP header */;
pub const IPT_ECN_OP_SET_CWR: c_uint = 0x20	/* set CWR bit of TCP header */;
pub const IPT_ECN_OP_MASK: c_uint = 0xce;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipt_ECN_info {
    pub /: *mut *mut __u8 operation; / bitset of operations,
    pub /: *mut *mut __u8 ip_ect; / ECT codepoint of IPv4 header, pre-shifted,
    pub /: *mut *mut __u8 ece:1, cwr:1; / TCP ECT bits,
    pub tcp: },
    pub proto: },
}
