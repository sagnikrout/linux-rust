//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/isp1760/isp1760-core.h
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
// Driver for the NXP ISP1760 chip
//
// Copyright 2021 Linaro, Rui Miguel Silva
// Copyright 2014 Laurent Pinchart
// Copyright 2007 Sebastian Siewior
//
// Contacts:
// Sebastian Siewior <bigeasy@linutronix.de>
// Laurent Pinchart <laurent.pinchart@ideasonboard.com>
// Rui Miguel Silva <rui.silva@linaro.org>
//

//
// Device flags that can vary from board to board.  All of these
// indicate the most "atypical" case, so that a devflags of 0 is
// a sane default configuration.
//
pub const ISP1760_FLAG_BUS_WIDTH_16: c_uint = 0x00000002 /* 16-bit data bus width */;
pub const ISP1760_FLAG_PERIPHERAL_EN: c_uint = 0x00000004 /* Port 1 supports Peripheral mode*/;
pub const ISP1760_FLAG_ANALOG_OC: c_uint = 0x00000008 /* Analog overcurrent */;
pub const ISP1760_FLAG_DACK_POL_HIGH: c_uint = 0x00000010 /* DACK active high */;
pub const ISP1760_FLAG_DREQ_POL_HIGH: c_uint = 0x00000020 /* DREQ active high */;
pub const ISP1760_FLAG_ISP1761: c_uint = 0x00000040 /* Chip is ISP1761 */;
pub const ISP1760_FLAG_INTR_POL_HIGH: c_uint = 0x00000080 /* Interrupt polarity active high */;
pub const ISP1760_FLAG_INTR_EDGE_TRIG: c_uint = 0x00000100 /* Interrupt edge triggered */;
pub const ISP1760_FLAG_ISP1763: c_uint = 0x00000200 /* Chip is ISP1763 */;
pub const ISP1760_FLAG_BUS_WIDTH_8: c_uint = 0x00000400 /* 8-bit data bus width */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp1760_device {
    pub dev: *mut device,
    pub devflags: c_uint,
    pub rst_gpio: *mut gpio_desc,
    pub hcd: isp1760_hcd,
    pub udc: isp1760_udc,
}

extern "C" {
    pub fn isp1760_unregister(dev: *mut device);
}
extern "C" {
    pub fn isp1760_set_pullup(isp: *mut isp1760_device, enable: bool);
}
