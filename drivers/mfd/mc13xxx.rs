//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/mfd/mc13xxx.h
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
// Copyright 2012 Creative Product Design
// Marc Reilly <marc@cpdesign.com.au>
//

pub const MC13XXX_NUMREGS: c_uint = 0x3f;
pub const MC13XXX_IRQ_REG_CNT: c_int = 2;
pub const MC13XXX_IRQ_PER_REG: c_int = 24;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mc13xxx_variant {
    pub name: *const c_char,
    pub revision): *mut *mut *mut void (print_revision)(struct mc13xxx mc13xxx, u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mc13xxx {
    pub regmap: *mut regmap,
    pub dev: *mut device,
    pub variant: *const mc13xxx_variant,
    pub MC13XXX_IRQ_REG_CNT]: *mut *mut regmap_irq irqs[MC13XXX_IRQ_PER_REG,
    pub irq_chip: regmap_irq_chip,
    pub irq_data: *mut regmap_irq_chip_data,
    pub lock: mutex,
    pub irq: c_int,
    pub flags: c_int,
    pub adcflags: c_int,
}

extern "C" {
    pub fn mc13xxx_common_init(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn mc13xxx_common_exit(dev: *mut device);
}
