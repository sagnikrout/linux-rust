//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/ax88796.h
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
// include/net/ax88796.h
//
// Copyright 2005 Simtec Electronics
// Ben Dooks <ben@simtec.co.uk>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ax_plat_data {
    pub flags: c_uint,
    pub /: *mut *mut unsigned char wordlength; / 1 or 2,
    pub /: *mut *mut unsigned char dcr_val; / default value for DCR,
    pub /: *mut *mut unsigned char rcr_val; / default value for RCR,
    pub /: *mut *mut unsigned char gpoc_val; / default value for GPOC,
    pub /: *mut *mut *mut u32 reg_offsets; / register offsets,
    pub when: *mut *mut *mut u8 mac_addr; / MAC addr (only used,
// uses default ax88796 buffer if set to NULL
    pub star_page): *const *const unsigned char buf, int,
    pub ring_offset): *mut *mut sk_buff skb, int,
// returns nonzero if a pending interrupt request might be caused by
// the ax88796. Handles all interrupts if set to NULL
//
    pub pdev): *mut *mut int (check_irq)(struct platform_device,
}

// exported from ax88796.c for xsurf100.c
extern "C" {
    pub fn ax_NS8390_reinit(dev: *mut net_device);
}
