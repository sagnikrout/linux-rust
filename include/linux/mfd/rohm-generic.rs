//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/rohm-generic.h
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
// Copyright (C) 2018 ROHM Semiconductors

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rohm_chip_type {
    ROHM_CHIP_TYPE_BD9571,
    ROHM_CHIP_TYPE_BD9573,
    ROHM_CHIP_TYPE_BD9574,
    ROHM_CHIP_TYPE_BD9576,
    ROHM_CHIP_TYPE_BD71815,
    ROHM_CHIP_TYPE_BD71828,
    ROHM_CHIP_TYPE_BD71837,
    ROHM_CHIP_TYPE_BD71847,
    ROHM_CHIP_TYPE_BD72720,
    ROHM_CHIP_TYPE_BD96801,
    ROHM_CHIP_TYPE_BD96802,
    ROHM_CHIP_TYPE_BD96805,
    ROHM_CHIP_TYPE_BD96806,
    ROHM_CHIP_TYPE_AMOUNT
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rohm_regmap_dev {
    pub dev: *mut device,
    pub regmap: *mut regmap,
}

pub const ROHM_DVS_LEVEL_VALID_AMOUNT: c_int = 5;
pub const ROHM_DVS_LEVEL_UNKNOWN: c_int = 0;
//
// struct rohm_dvs_config - dynamic voltage scaling register descriptions
//
// @level_map:		bitmap representing supported run-levels for this
// regulator
// @run_reg:		register address for regulator config at 'run' state
// @run_mask:		value mask for regulator voltages at 'run' state
// @run_on_mask:	enable mask for regulator at 'run' state
// @idle_reg:		register address for regulator config at 'idle' state
// @idle_mask:		value mask for regulator voltages at 'idle' state
// @idle_on_mask:	enable mask for regulator at 'idle' state
// @suspend_reg:	register address for regulator config at 'suspend' state
// @suspend_mask:	value mask for regulator voltages at 'suspend' state
// @suspend_on_mask:	enable mask for regulator at 'suspend' state
// @lpsr_reg:		register address for regulator config at 'lpsr' state
// @lpsr_mask:		value mask for regulator voltages at 'lpsr' state
// @lpsr_on_mask:	enable mask for regulator at 'lpsr' state
//
// Description of ROHM PMICs voltage configuration registers for different
// system states. This is used to correctly configure the PMIC at startup
// based on values read from DT.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rohm_dvs_config {
    pub level_map: u64,
    pub run_reg: c_uint,
    pub run_mask: c_uint,
    pub run_on_mask: c_uint,
    pub idle_reg: c_uint,
    pub idle_mask: c_uint,
    pub idle_on_mask: c_uint,
    pub suspend_reg: c_uint,
    pub suspend_mask: c_uint,
    pub suspend_on_mask: c_uint,
    pub lpsr_reg: c_uint,
    pub lpsr_mask: c_uint,
    pub lpsr_on_mask: c_uint,
    pub snvs_reg: c_uint,
    pub snvs_mask: c_uint,
    pub snvs_on_mask: c_uint,
}

