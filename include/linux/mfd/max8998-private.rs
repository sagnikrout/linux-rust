//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/max8998-private.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// max8998-private.h - Voltage regulator driver for the Maxim 8998
//
// Copyright (C) 2009-2010 Samsung Electronics
// Kyungmin Park <kyungmin.park@samsung.com>
// Marek Szyprowski <m.szyprowski@samsung.com>
//
pub const MAX8998_NUM_IRQ_REGS: c_int = 4;
// MAX 8998 registers
// IRQ definitions
// MAX8998 various variants

//
// struct max8998_dev - max8998 master device for sub-drivers
// @dev: master device of the chip (can be used to access platform data)
// @pdata: platform data for the driver and subdrivers
// @i2c: i2c client private data for regulator
// @rtc: i2c client private data for rtc
// @iolock: mutex for serializing io access
// @irqlock: mutex for buslock
// @irq_base: base IRQ number for max8998, required for IRQs
// @irq: generic IRQ number for max8998
// @ono: power onoff IRQ number for max8998
// @irq_masks_cur: currently active value
// @irq_masks_cache: cached hardware value
// @type: indicate which max8998 "variant" is used
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max8998_dev {
    pub dev: *mut device,
    pub pdata: *mut max8998_platform_data,
    pub i2c: *mut i2c_client,
    pub rtc: *mut i2c_client,
    pub iolock: mutex,
    pub irqlock: mutex,
    pub irq_base: c_uint,
    pub irq_domain: *mut irq_domain,
    pub irq: c_int,
    pub ono: c_int,
    pub irq_masks_cur: [u8; MAX8998_NUM_IRQ_REGS],
    pub irq_masks_cache: [u8; MAX8998_NUM_IRQ_REGS],
    pub type: c_ulong,
    pub wakeup: bool,
}

extern "C" {
    pub fn max8998_irq_init(max8998: *mut max8998_dev) -> c_int;
}
extern "C" {
    pub fn max8998_irq_exit(max8998: *mut max8998_dev);
}
extern "C" {
    pub fn max8998_irq_resume(max8998: *mut max8998_dev) -> c_int;
}
extern "C" {
    pub fn max8998_read_reg(i2c: *mut i2c_client, reg: u8, dest: *mut u8) -> c_int;
}
extern "C" {
    pub fn max8998_write_reg(i2c: *mut i2c_client, reg: u8, value: u8) -> c_int;
}
extern "C" {
    pub fn max8998_update_reg(i2c: *mut i2c_client, reg: u8, val: u8, mask: u8) -> c_int;
}
