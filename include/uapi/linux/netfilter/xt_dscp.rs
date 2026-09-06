//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/netfilter/xt_dscp.h
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
// x_tables module for matching the IPv4/IPv6 DSCP field
//
// (C) 2002 Harald Welte <laforge@gnumonks.org>
// This software is distributed under GNU GPL v2, 1991
//
// See RFC2474 for a description of the DSCP field within the IP Header.
//
// xt_dscp.h,v 1.3 2002/08/05 19:00:21 laforge Exp
//

pub const XT_DSCP_MASK: c_uint = 0xfc	/* 11111100 */;
pub const XT_DSCP_SHIFT: c_int = 2;
pub const XT_DSCP_MAX: c_uint = 0x3f	/* 00111111 */;
// match info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_dscp_info {
    pub dscp: __u8,
    pub invert: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_tos_match_info {
    pub tos_mask: __u8,
    pub tos_value: __u8,
    pub invert: __u8,
}
