//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/ethoc.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// linux/include/net/ethoc.h
//
// Copyright (C) 2008-2009 Avionic Design GmbH
//
// Written by Thierry Reding <thierry.reding@avionic-design.de>
//
pub const LINUX_NET_ETHOC_H: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethoc_platform_data {
    pub hwaddr: [u8; IFHWADDRLEN],
    pub phy_id: i8,
    pub eth_clkfreq: u32,
    pub big_endian: bool,
}
