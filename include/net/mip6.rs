//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/mip6.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C)2003-2006 Helsinki University of Technology
// Copyright (C)2003-2006 USAGI/WIDE Project
//
// Authors:
// Noriaki TAKAMIYA @USAGI
// Masahide NAKAMURA @USAGI
// YOSHIFUJI Hideaki @USAGI
//

//
// Mobility Header
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip6_mh {
    pub ip6mh_proto: __u8,
    pub ip6mh_hdrlen: __u8,
    pub ip6mh_type: __u8,
    pub ip6mh_reserved: __u8,
    pub ip6mh_cksum: __u16,
// Followed by type specific messages
    pub data: [__u8; ],
    pub __packed: },

