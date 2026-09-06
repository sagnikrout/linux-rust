//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/lp8755.h
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
// LP8755 High Performance Power Management Unit Driver:System Interface Driver
//
// Copyright (C) 2012 Texas Instruments
//
// Author: Daniel(Geon Si) Jeong <daniel.jeong@ti.com>
// G.Shark Jeong <gshark.jeong@gmail.com>
//

//
// PWR FAULT : power fault detected
// OCP : over current protect activated
// OVP : over voltage protect activated
// TEMP_WARN : thermal warning
// TEMP_SHDN : thermal shutdonw detected
// I_LOAD : current measured
//

pub const LP8755_EVENT_OVP: c_uint = 0x10000;
pub const LP8755_EVENT_TEMP_WARN: c_uint = 0x2000;

pub const LP8755_EVENT_I_LOAD: c_uint = 0x40000;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lp8755_bucks {
    LP8755_BUCK0 = 0,
    LP8755_BUCK1,
    LP8755_BUCK2,
    LP8755_BUCK3,
    LP8755_BUCK4,
    LP8755_BUCK5,
    LP8755_BUCK_MAX,
}

//
// multiphase configuration options
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lp8755_mphase_config {
    MPHASE_CONF0,
    MPHASE_CONF1,
    MPHASE_CONF2,
    MPHASE_CONF3,
    MPHASE_CONF4,
    MPHASE_CONF5,
    MPHASE_CONF6,
    MPHASE_CONF7,
    MPHASE_CONF8,
    MPHASE_CONF_MAX
}

//
// struct lp8755_platform_data
// @mphase_type : Multiphase Switcher Configurations.
// @buck_data   : buck0~6 init voltage in uV
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lp8755_platform_data {
    pub mphase: c_int,
    pub buck_data: [*mut regulator_init_data; LP8755_BUCK_MAX],
}
