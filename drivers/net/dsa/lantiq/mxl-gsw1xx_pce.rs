//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/dsa/lantiq/mxl-gsw1xx_pce.h
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
//
// PCE microcode code update for driver for MaxLinear GSW1xx switch chips
//
// Copyright (C) 2023 - 2024 MaxLinear Inc.
// Copyright (C) 2022 Snap One, LLC.  All rights reserved.
// Copyright (C) 2017 - 2019 Hauke Mehrtens <hauke@hauke-m.de>
// Copyright (C) 2012 John Crispin <john@phrozen.org>
// Copyright (C) 2010 Lantiq Deutschland
//

pub const INSTR: c_int = 0;
pub const IPV6: c_int = 1;
pub const LENACCU: c_int = 2;
// GSWIP_2.X
// parser's microcode flag type

// V22_2X (IPv6 issue fixed)
// value   mask    ns  fields      L  type     flags       ipv4_len
