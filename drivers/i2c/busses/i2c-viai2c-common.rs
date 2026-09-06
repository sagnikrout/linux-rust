//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/i2c/busses/i2c-viai2c-common.h
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

// REG_CR Bit fields
pub const VIAI2C_REG_CR: c_uint = 0x00;

// REG_TCR Bit fields
pub const VIAI2C_REG_TCR: c_uint = 0x02;

// REG_CSR Bit fields
pub const VIAI2C_REG_CSR: c_uint = 0x04;

// REG_ISR Bit fields
pub const VIAI2C_REG_ISR: c_uint = 0x06;

// REG_IMR Bit fields
pub const VIAI2C_REG_IMR: c_uint = 0x08;

pub const VIAI2C_REG_CDR: c_uint = 0x0A;
pub const VIAI2C_REG_TR: c_uint = 0x0C;
pub const VIAI2C_REG_MCR: c_uint = 0x0E;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct viai2c {
    pub adapter: i2c_adapter,
    pub complete: completion,
    pub dev: *mut device,
    pub base: *mut void __iomem,
    pub clk: *mut clk,
    pub tcr: u16,
    pub irq: c_int,
    pub xfered_len: u16,
    pub msg: *mut i2c_msg,
    pub ret: c_int,
    pub last: bool,
    pub mode: c_uint,
    pub platform: c_uint,
    pub pltfm_priv: *mut c_void,
}

extern "C" {
    pub fn viai2c_wait_bus_not_busy(i2c: *mut viai2c) -> c_int;
}
extern "C" {
    pub fn viai2c_xfer(adap: *mut i2c_adapter, msgs[]: i2c_msg, num: c_int) -> c_int;
}
extern "C" {
    pub fn viai2c_init(pdev: *mut platform_device, pi2c: *mut viai2c, plat: c_int) -> c_int;
}
extern "C" {
    pub fn viai2c_irq_xfer(i2c: *mut viai2c) -> c_int;
}
