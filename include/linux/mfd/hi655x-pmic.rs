//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/hi655x-pmic.h
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
// Device driver for regulators in hi655x IC
//
// Copyright (c) 2016 HiSilicon Ltd.
//
// Authors:
// Chen Feng <puck.chen@hisilicon.com>
// Fei  Wang <w.f@huawei.com>
//

// Hi655x registers are mapped to memory bus in 4 bytes stride
pub const HI655X_STRIDE: c_int = 4;

pub const HI655X_BITS: c_int = 8;
pub const HI655X_NR_IRQ: c_int = 32;

pub const HI655X_IRQ_ARRAY: c_int = 4;
pub const HI655X_IRQ_MASK: c_uint = 0xFF;
pub const HI655X_IRQ_CLR: c_uint = 0xFF;
pub const HI655X_VER_REG: c_uint = 0x00;
pub const PMU_VER_START: c_uint = 0x10;
pub const PMU_VER_END: c_uint = 0x38;
pub const RESERVE_INT: c_int = 7;
pub const PWRON_D20R_INT: c_int = 6;
pub const PWRON_D20F_INT: c_int = 5;
pub const PWRON_D4SR_INT: c_int = 4;
pub const VSYS_6P0_D200UR_INT: c_int = 3;
pub const VSYS_UV_D3R_INT: c_int = 2;
pub const VSYS_2P5_R_INT: c_int = 1;
pub const OTMP_D1R_INT: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hi655x_pmic {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub gpio: *mut gpio_desc,
    pub ver: c_uint,
    pub irq_data: *mut regmap_irq_chip_data,
}
