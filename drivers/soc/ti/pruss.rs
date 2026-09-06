//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/soc/ti/pruss.h
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
// PRU-ICSS Subsystem user interfaces
//
// Copyright (C) 2015-2023 Texas Instruments Incorporated - http://www.ti.com
// MD Danish Anwar <danishanwar@ti.com>
//

//
// PRU_ICSS_CFG registers
// SYSCFG, ISRP, ISP, IESP, IECP, SCRP applicable on AMxxxx devices only
//
pub const PRUSS_CFG_REVID: c_uint = 0x00;
pub const PRUSS_CFG_SYSCFG: c_uint = 0x04;

pub const PRUSS_CFG_CGR: c_uint = 0x10;
pub const PRUSS_CFG_ISRP: c_uint = 0x14;
pub const PRUSS_CFG_ISP: c_uint = 0x18;
pub const PRUSS_CFG_IESP: c_uint = 0x1C;
pub const PRUSS_CFG_IECP: c_uint = 0x20;
pub const PRUSS_CFG_SCRP: c_uint = 0x24;
pub const PRUSS_CFG_PMAO: c_uint = 0x28;
pub const PRUSS_CFG_MII_RT: c_uint = 0x2C;
pub const PRUSS_CFG_IEPCLK: c_uint = 0x30;
pub const PRUSS_CFG_SPP: c_uint = 0x34;
pub const PRUSS_CFG_PIN_MX: c_uint = 0x40;
// PRUSS_GPCFG register bits

pub const PRUSS_GPCFG_PRU_GPI_MODE_SHIFT: c_int = 0;
pub const PRUSS_GPCFG_PRU_MUX_SEL_SHIFT: c_int = 26;

// PRUSS_MII_RT register bits

// PRUSS_SPP register bits

//
// pruss_cfg_read() - read a PRUSS CFG sub-module register
// @pruss: the pruss instance handle
// @reg: register offset within the CFG sub-module
// @val: pointer to return the value in
//
// Reads a given register within the PRUSS CFG sub-module and
// returns it through the passed-in @val pointer
//
// Return: 0 on success, or an error code otherwise
//
extern "C" {
    pub fn regmap_read(_arg: pruss->cfg_regmap, _arg: reg, _arg: val) -> return;
}
//
// pruss_cfg_update() - configure a PRUSS CFG sub-module register
// @pruss: the pruss instance handle
// @reg: register offset within the CFG sub-module
// @mask: bit mask to use for programming the @val
// @val: value to write
//
// Programs a given register within the PRUSS CFG sub-module
//
// Return: 0 on success, or an error code otherwise
//
extern "C" {
    pub fn regmap_update_bits(_arg: pruss->cfg_regmap, _arg: reg, _arg: mask, _arg: val) -> return;
}
