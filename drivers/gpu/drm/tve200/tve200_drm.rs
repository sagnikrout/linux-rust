//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/tve200/tve200_drm.h
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
// Copyright (C) 2017 Linus Walleij <linus.walleij@linaro.org>
// Parts of this file were based on sources as follows:
//
// Copyright (C) 2006-2008 Intel Corporation
// Copyright (C) 2007 Amos Lee <amos_lee@storlinksemi.com>
// Copyright (C) 2007 Dave Airlie <airlied@linux.ie>
// Copyright (C) 2011 Texas Instruments
// Copyright (C) 2017 Eric Anholt
//

// Bits 2-31 are valid physical base addresses
pub const TVE200_Y_FRAME_BASE_ADDR: c_uint = 0x00;
pub const TVE200_U_FRAME_BASE_ADDR: c_uint = 0x04;
pub const TVE200_V_FRAME_BASE_ADDR: c_uint = 0x08;
pub const TVE200_INT_EN: c_uint = 0x0C;
pub const TVE200_INT_CLR: c_uint = 0x10;
pub const TVE200_INT_STAT: c_uint = 0x14;

pub const TVE200_CTRL: c_uint = 0x18;

// Bits 24..26 define the burst size after arbitration on the bus

//
// Bits 16..23 is the retry count*16 before issueing a new AHB transfer
// on the AHB bus.
//

// Bits 12..14 define the YCbCr ordering

// Bits 10..11 define the input resolution (framebuffer size)

// Bits 4 & 5 define when to fire the vblank IRQ

pub const TVE200_CTRL_2: c_uint = 0x1c;
pub const TVE200_CTRL_3: c_uint = 0x20;
pub const TVE200_CTRL_4: c_uint = 0x24;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tve200_drm_dev_private {
    pub drm: *mut drm_device,
    pub connector: *mut drm_connector,
    pub panel: *mut drm_panel,
    pub bridge: *mut drm_bridge,
    pub pipe: drm_simple_display_pipe,
    pub regs: *mut c_void,
    pub pclk: *mut clk,
    pub clk: *mut clk,
}

extern "C" {
    pub fn tve200_display_init(dev: *mut drm_device) -> c_int;
}
extern "C" {
    pub fn tve200_irq(irq: c_int, data: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn tve200_connector_init(dev: *mut drm_device) -> c_int;
}
extern "C" {
    pub fn tve200_encoder_init(dev: *mut drm_device) -> c_int;
}
