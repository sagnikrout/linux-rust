//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/fsl_gtm.h
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
// Freescale General-purpose Timers Module
//
// Copyright 2006 Freescale Semiconductor, Inc.
// Shlomi Gridish <gridish@freescale.com>
// Jerry Huang <Chang-Ming.Huang@freescale.com>
// Copyright (c) MontaVista Software, Inc. 2008.
// Anton Vorontsov <avorontsov@ru.mvista.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gtm_timer {
    pub irq: c_uint,
    pub gtm: *mut gtm,
    pub requested: bool,
    pub gtcfr: *mut u8 __iomem,
    pub gtmdr: *mut __be16 __iomem,
    pub gtpsr: *mut __be16 __iomem,
    pub gtcnr: *mut __be16 __iomem,
    pub gtrfr: *mut __be16 __iomem,
    pub gtevr: *mut __be16 __iomem,
}

extern "C" {
    pub fn gtm_put_timer16(tmr: *mut gtm_timer);
}
extern "C" {
    pub fn gtm_stop_timer16(tmr: *mut gtm_timer);
}
extern "C" {
    pub fn gtm_ack_timer16(tmr: *mut gtm_timer, events: u16);
}
