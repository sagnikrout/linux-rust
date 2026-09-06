//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/wiznet/w5100.h
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
// Ethernet driver for the WIZnet W5100 chip.
//
// Copyright (C) 2006-2008 WIZnet Co.,Ltd.
// Copyright (C) 2012 Mike Sinkovsky <msink@permonline.ru>
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct w5100_ops {
    pub may_sleep: bool,
    pub chip_id: c_int,
    pub addr): *mut *mut *mut int (read)(struct net_device ndev, u32,
    pub data): *mut *mut *mut int (write)(struct net_device ndev, u32 addr, u8,
    pub addr): *mut *mut *mut int (read16)(struct net_device ndev, u32,
    pub data): *mut *mut *mut int (write16)(struct net_device ndev, u32 addr, u16,
    pub len): *mut *mut *mut *mut int (readbulk)(struct net_device ndev, u32 addr, u8 buf, int,
    pub len): c_int,
    pub ndev): *mut *mut int (reset)(struct net_device,
    pub ndev): *mut *mut int (init)(struct net_device,
}

extern "C" {
    pub fn w5100_remove(dev: *mut device);
}
