//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iio/health/afe440x.h
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
// AFE440X Heart Rate Monitors and Low-Cost Pulse Oximeters
//
// Copyright (C) 2015 Texas Instruments Incorporated - https://www.ti.com
// Andrew F. Davis <afd@ti.com>
//
// AFE440X registers
pub const AFE440X_CONTROL0: c_uint = 0x00;
pub const AFE440X_LED2STC: c_uint = 0x01;
pub const AFE440X_LED2ENDC: c_uint = 0x02;
pub const AFE440X_LED1LEDSTC: c_uint = 0x03;
pub const AFE440X_LED1LEDENDC: c_uint = 0x04;
pub const AFE440X_ALED2STC: c_uint = 0x05;
pub const AFE440X_ALED2ENDC: c_uint = 0x06;
pub const AFE440X_LED1STC: c_uint = 0x07;
pub const AFE440X_LED1ENDC: c_uint = 0x08;
pub const AFE440X_LED2LEDSTC: c_uint = 0x09;
pub const AFE440X_LED2LEDENDC: c_uint = 0x0a;
pub const AFE440X_ALED1STC: c_uint = 0x0b;
pub const AFE440X_ALED1ENDC: c_uint = 0x0c;
pub const AFE440X_LED2CONVST: c_uint = 0x0d;
pub const AFE440X_LED2CONVEND: c_uint = 0x0e;
pub const AFE440X_ALED2CONVST: c_uint = 0x0f;
pub const AFE440X_ALED2CONVEND: c_uint = 0x10;
pub const AFE440X_LED1CONVST: c_uint = 0x11;
pub const AFE440X_LED1CONVEND: c_uint = 0x12;
pub const AFE440X_ALED1CONVST: c_uint = 0x13;
pub const AFE440X_ALED1CONVEND: c_uint = 0x14;
pub const AFE440X_ADCRSTSTCT0: c_uint = 0x15;
pub const AFE440X_ADCRSTENDCT0: c_uint = 0x16;
pub const AFE440X_ADCRSTSTCT1: c_uint = 0x17;
pub const AFE440X_ADCRSTENDCT1: c_uint = 0x18;
pub const AFE440X_ADCRSTSTCT2: c_uint = 0x19;
pub const AFE440X_ADCRSTENDCT2: c_uint = 0x1a;
pub const AFE440X_ADCRSTSTCT3: c_uint = 0x1b;
pub const AFE440X_ADCRSTENDCT3: c_uint = 0x1c;
pub const AFE440X_PRPCOUNT: c_uint = 0x1d;
pub const AFE440X_CONTROL1: c_uint = 0x1e;
pub const AFE440X_LEDCNTRL: c_uint = 0x22;
pub const AFE440X_CONTROL2: c_uint = 0x23;
pub const AFE440X_ALARM: c_uint = 0x29;
pub const AFE440X_LED2VAL: c_uint = 0x2a;
pub const AFE440X_ALED2VAL: c_uint = 0x2b;
pub const AFE440X_LED1VAL: c_uint = 0x2c;
pub const AFE440X_ALED1VAL: c_uint = 0x2d;
pub const AFE440X_LED2_ALED2VAL: c_uint = 0x2e;
pub const AFE440X_LED1_ALED1VAL: c_uint = 0x2f;
pub const AFE440X_CONTROL3: c_uint = 0x31;
pub const AFE440X_PDNCYCLESTC: c_uint = 0x32;
pub const AFE440X_PDNCYCLEENDC: c_uint = 0x33;
// CONTROL0 register fields

// CONTROL1 register fields

// TIAGAIN register fields

// CONTROL2 register fields

// CONTROL3 register fields

// CONTROL0 values
pub const AFE440X_CONTROL0_WRITE: c_uint = 0x0;
pub const AFE440X_CONTROL0_READ: c_uint = 0x1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct afe440x_val_table {
    pub integer: c_int,
    pub fract: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct afe440x_attr {
    pub dev_attr: device_attribute,
    pub field: c_uint,
    pub val_table: *const afe440x_val_table,
    pub table_size: c_uint,
}

