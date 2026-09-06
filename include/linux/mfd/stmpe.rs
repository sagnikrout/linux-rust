//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/stmpe.h
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
// Copyright (C) ST-Ericsson SA 2010
//
// Author: Rabin Vincent <rabin.vincent@stericsson.com> for ST-Ericsson
//

pub const STMPE811_REG_ADC_CTRL1: c_uint = 0x20;
pub const STMPE811_REG_ADC_CTRL2: c_uint = 0x21;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stmpe_block {
    STMPE_BLOCK_GPIO	= 1 << 0,
    STMPE_BLOCK_KEYPAD	= 1 << 1,
    STMPE_BLOCK_TOUCHSCREEN	= 1 << 2,
    STMPE_BLOCK_ADC		= 1 << 3,
    STMPE_BLOCK_PWM		= 1 << 4,
    STMPE_BLOCK_ROTATOR	= 1 << 5,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stmpe_partnum {
    STMPE610,
    STMPE801,
    STMPE811,
    STMPE1600,
    STMPE1601,
    STMPE1801,
    STMPE2401,
    STMPE2403,
    STMPE_NBR_PARTS
}

//
// For registers whose locations differ on variants,  the correct address is
// obtained by indexing stmpe->regs with one of the following.
//
// struct stmpe - STMPE MFD structure
// @vcc: optional VCC regulator
// @vio: optional VIO regulator
// @lock: lock protecting I/O operations
// @irq_lock: IRQ bus lock
// @dev: device, mostly for dev_dbg()
// @irq_domain: IRQ domain
// @client: client - i2c or spi
// @ci: client specific information
// @partnum: part number
// @variant: the detected STMPE model number
// @regs: list of addresses of registers which are at different addresses on
// different variants.  Indexed by one of STMPE_IDX_*.
// @irq: irq number for stmpe
// @num_gpios: number of gpios, differs for variants
// @ier: cache of IER registers for bus_lock
// @oldier: cache of IER registers for bus_lock
// @pdata: platform data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stmpe {
    pub vcc: *mut regulator,
    pub vio: *mut regulator,
    pub lock: mutex,
    pub irq_lock: mutex,
    pub dev: *mut device,
    pub domain: *mut irq_domain,
    pub client: *mut c_void,
    pub ci: *mut stmpe_client_info,
    pub partnum: stmpe_partnum,
    pub variant: *mut stmpe_variant_info,
    pub regs: *const u8,
    pub irq: c_int,
    pub num_gpios: c_int,
    pub ier: [u8; 2],
    pub oldier: [u8; 2],
    pub pdata: *mut stmpe_platform_data,
// For devices that use an ADC
    pub sample_time: u8,
    pub mod_12b: u8,
    pub ref_sel: u8,
    pub adc_freq: u8,
}

extern "C" {
    pub fn stmpe_reg_write(stmpe: *mut stmpe, reg: u8, data: u8) -> c_int;
}
extern "C" {
    pub fn stmpe_reg_read(stmpe: *mut stmpe, reg: u8) -> c_int;
}
extern "C" {
    pub fn stmpe_set_bits(stmpe: *mut stmpe, reg: u8, mask: u8, val: u8) -> c_int;
}
extern "C" {
    pub fn stmpe_enable(stmpe: *mut stmpe, blocks: c_uint) -> c_int;
}
extern "C" {
    pub fn stmpe_disable(stmpe: *mut stmpe, blocks: c_uint) -> c_int;
}
extern "C" {
    pub fn stmpe811_adc_common_init(stmpe: *mut stmpe) -> c_int;
}

