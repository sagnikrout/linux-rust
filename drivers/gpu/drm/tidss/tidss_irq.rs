//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/tidss/tidss_irq.h
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
// Copyright (C) 2018 Texas Instruments Incorporated - https://www.ti.com
// Author: Tomi Valkeinen <tomi.valkeinen@ti.com>
//

//
// The IRQ status from various DISPC IRQ registers are packed into a single
// value, where the bits are defined as follows:
//
// bit group |dev|wb |mrg0|mrg1|mrg2|mrg3|plane0-3| <unused> |
// bit use   |D  |fou|FEOL|FEOL|FEOL|FEOL|  UUUU  |          |
// bit number|0  |1-3|4-7 |8-11|  12-19  | 20-23  |  24-31   |
//
// device bits:	D = Unused
// WB bits:	f = frame done wb, o = wb buffer overflow,
// u = wb buffer uncomplete
// vp bits:	F = frame done, E = vsync even, O = vsync odd, L = sync lost
// plane bits:	U = fifo underflow
//

extern "C" {
    pub fn GENMASK(_arg: DSS_IRQ_VP_BIT_N((ch), _arg: 3), _arg: DSS_IRQ_VP_BIT_N((ch), _arg: 0)) -> return;
}

extern "C" {
    pub fn tidss_irq_enable_vblank(crtc: *mut drm_crtc);
}
extern "C" {
    pub fn tidss_irq_disable_vblank(crtc: *mut drm_crtc);
}
extern "C" {
    pub fn tidss_irq_install(ddev: *mut drm_device, irq: c_uint) -> c_int;
}
extern "C" {
    pub fn tidss_irq_uninstall(ddev: *mut drm_device);
}
extern "C" {
    pub fn tidss_irq_resume(tidss: *mut tidss_device);
}
