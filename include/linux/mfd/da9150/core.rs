//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/da9150/core.h
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
// DA9150 MFD Driver - Core Data
//
// Copyright (c) 2014 Dialog Semiconductor
//
// Author: Adam Thomson <Adam.Thomson.Opensource@diasemi.com>
//

// I2C address paging
pub const DA9150_REG_PAGE_SHIFT: c_int = 8;
pub const DA9150_REG_PAGE_MASK: c_uint = 0xFF;
// IRQs
pub const DA9150_NUM_IRQ_REGS: c_int = 4;
pub const DA9150_IRQ_VBUS: c_int = 0;
pub const DA9150_IRQ_CHG: c_int = 1;
pub const DA9150_IRQ_TCLASS: c_int = 2;
pub const DA9150_IRQ_TJUNC: c_int = 3;
pub const DA9150_IRQ_VFAULT: c_int = 4;
pub const DA9150_IRQ_CONF: c_int = 5;
pub const DA9150_IRQ_DAT: c_int = 6;
pub const DA9150_IRQ_DTYPE: c_int = 7;
pub const DA9150_IRQ_ID: c_int = 8;
pub const DA9150_IRQ_ADP: c_int = 9;
pub const DA9150_IRQ_SESS_END: c_int = 10;
pub const DA9150_IRQ_SESS_VLD: c_int = 11;
pub const DA9150_IRQ_FG: c_int = 12;
pub const DA9150_IRQ_GP: c_int = 13;
pub const DA9150_IRQ_TBAT: c_int = 14;
pub const DA9150_IRQ_GPIOA: c_int = 15;
pub const DA9150_IRQ_GPIOB: c_int = 16;
pub const DA9150_IRQ_GPIOC: c_int = 17;
pub const DA9150_IRQ_GPIOD: c_int = 18;
pub const DA9150_IRQ_GPADC: c_int = 19;
pub const DA9150_IRQ_WKUP: c_int = 20;
// I2C sub-device address
pub const DA9150_QIF_I2C_ADDR_LSB: c_uint = 0x5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct da9150_fg_pdata {
    pub /: *mut *mut u32 update_interval; / msecs,
    pub /: *mut *mut u8 warn_soc_lvl; / % value,
    pub /: *mut *mut u8 crit_soc_lvl; / % value,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct da9150_pdata {
    pub irq_base: c_int,
    pub fg_pdata: *mut da9150_fg_pdata,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct da9150 {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub core_qif: *mut i2c_client,
    pub regmap_irq_data: *mut regmap_irq_chip_data,
    pub irq: c_int,
    pub irq_base: c_int,
}

// Device I/O - Query Interface for FG and standard register access
extern "C" {
    pub fn da9150_read_qif(da9150: *mut da9150, addr: u8, count: c_int, buf: *mut u8);
}
extern "C" {
    pub fn da9150_write_qif(da9150: *mut da9150, addr: u8, count: c_int, buf: *const u8);
}
extern "C" {
    pub fn da9150_reg_read(da9150: *mut da9150, reg: u16) -> u8;
}
extern "C" {
    pub fn da9150_reg_write(da9150: *mut da9150, reg: u16, val: u8);
}
extern "C" {
    pub fn da9150_set_bits(da9150: *mut da9150, reg: u16, mask: u8, val: u8);
}
extern "C" {
    pub fn da9150_bulk_read(da9150: *mut da9150, reg: u16, count: c_int, buf: *mut u8);
}
extern "C" {
    pub fn da9150_bulk_write(da9150: *mut da9150, reg: u16, count: c_int, buf: *const u8);
}
