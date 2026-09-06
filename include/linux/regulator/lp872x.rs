//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/regulator/lp872x.h
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
// Copyright 2012 Texas Instruments
//
// Author: Milo(Woogyom) Kim <milo.kim@ti.com>
//

pub const LP872X_MAX_REGULATORS: c_int = 9;
pub const LP8720_ENABLE_DELAY: c_int = 200;
pub const LP8725_ENABLE_DELAY: c_int = 30000;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lp872x_regulator_id {
    LP8720_ID_BASE,
    LP8720_ID_LDO1 = LP8720_ID_BASE,
    LP8720_ID_LDO2,
    LP8720_ID_LDO3,
    LP8720_ID_LDO4,
    LP8720_ID_LDO5,
    LP8720_ID_BUCK,

    LP8725_ID_BASE,
    LP8725_ID_LDO1 = LP8725_ID_BASE,
    LP8725_ID_LDO2,
    LP8725_ID_LDO3,
    LP8725_ID_LDO4,
    LP8725_ID_LDO5,
    LP8725_ID_LILO1,
    LP8725_ID_LILO2,
    LP8725_ID_BUCK1,
    LP8725_ID_BUCK2,

    LP872X_ID_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lp872x_dvs_sel {
    SEL_V1,
    SEL_V2,
}

//
// lp872x_dvs
// @gpio       : gpio descriptor for dvs control
// @vsel       : dvs selector for buck v1 or buck v2 register
// @init_state : initial dvs pin state
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lp872x_dvs {
    pub gpio: *mut gpio_desc,
    pub vsel: lp872x_dvs_sel,
    pub init_state: gpiod_flags,
}

//
// lp872x_regdata
// @id        : regulator id
// @init_data : init data for each regulator
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lp872x_regulator_data {
    pub id: lp872x_regulator_id,
    pub init_data: *mut regulator_init_data,
}

//
// lp872x_platform_data
// @general_config    : the value of LP872X_GENERAL_CFG register
// @update_config     : if LP872X_GENERAL_CFG register is updated, set true
// @regulator_data    : platform regulator id and init data
// @dvs               : dvs data for buck voltage control
// @enable_gpio       : gpio descriptor for enable control
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lp872x_platform_data {
    pub general_config: u8,
    pub update_config: bool,
    pub regulator_data: [lp872x_regulator_data; LP872X_MAX_REGULATORS],
    pub dvs: *mut lp872x_dvs,
    pub enable_gpio: *mut gpio_desc,
}
