//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/as3711.h
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
// AS3711 PMIC MFC driver header
//
// Copyright (C) 2012 Renesas Electronics Corporation
// Author: Guennadi Liakhovetski, <g.liakhovetski@gmx.de>
//
// Client data
//
// Register addresses

pub const AS3711_SD_2_VOLTAGE: c_int = 1;
pub const AS3711_SD_3_VOLTAGE: c_int = 2;
pub const AS3711_SD_4_VOLTAGE: c_int = 3;

pub const AS3711_LDO_2_VOLTAGE: c_int = 5;

pub const AS3711_LDO_4_VOLTAGE: c_int = 7;
pub const AS3711_LDO_5_VOLTAGE: c_int = 8;
pub const AS3711_LDO_6_VOLTAGE: c_int = 9;
pub const AS3711_LDO_7_VOLTAGE: c_uint = 0xa;
pub const AS3711_LDO_8_VOLTAGE: c_uint = 0xb;
pub const AS3711_SD_CONTROL: c_uint = 0x10;
pub const AS3711_GPIO_SIGNAL_OUT: c_uint = 0x20;
pub const AS3711_GPIO_SIGNAL_IN: c_uint = 0x21;
pub const AS3711_SD_CONTROL_1: c_uint = 0x30;
pub const AS3711_SD_CONTROL_2: c_uint = 0x31;
pub const AS3711_CURR_CONTROL: c_uint = 0x40;
pub const AS3711_CURR1_VALUE: c_uint = 0x43;
pub const AS3711_CURR2_VALUE: c_uint = 0x44;
pub const AS3711_CURR3_VALUE: c_uint = 0x45;
pub const AS3711_STEPUP_CONTROL_1: c_uint = 0x50;
pub const AS3711_STEPUP_CONTROL_2: c_uint = 0x51;
pub const AS3711_STEPUP_CONTROL_4: c_uint = 0x53;
pub const AS3711_STEPUP_CONTROL_5: c_uint = 0x54;
pub const AS3711_REG_STATUS: c_uint = 0x73;
pub const AS3711_INTERRUPT_STATUS_1: c_uint = 0x77;
pub const AS3711_INTERRUPT_STATUS_2: c_uint = 0x78;
pub const AS3711_INTERRUPT_STATUS_3: c_uint = 0x79;
pub const AS3711_CHARGER_STATUS_1: c_uint = 0x86;
pub const AS3711_CHARGER_STATUS_2: c_uint = 0x87;
pub const AS3711_ASIC_ID_1: c_uint = 0x90;
pub const AS3711_ASIC_ID_2: c_uint = 0x91;

// Regulators
#[repr(C)]
#[derive(Copy, Clone)]
pub struct as3711 {
    pub dev: *mut device,
    pub regmap: *mut regmap,
}

pub const AS3711_MAX_STEPDOWN: c_int = 4;
pub const AS3711_MAX_STEPUP: c_int = 2;
pub const AS3711_MAX_LDO: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum as3711_su2_feedback {
    AS3711_SU2_VOLTAGE,
    AS3711_SU2_CURR1,
    AS3711_SU2_CURR2,
    AS3711_SU2_CURR3,
    AS3711_SU2_CURR_AUTO,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum as3711_su2_fbprot {
    AS3711_SU2_LX_SD4,
    AS3711_SU2_GPIO2,
    AS3711_SU2_GPIO3,
    AS3711_SU2_GPIO4,
}

//
// Platform data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct as3711_regulator_pdata {
    pub init_data: [*mut regulator_init_data; AS3711_REGULATOR_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct as3711_bl_pdata {
    pub su1_fb: bool,
    pub su1_max_uA: c_int,
    pub su2_fb: bool,
    pub su2_max_uA: c_int,
    pub su2_feedback: as3711_su2_feedback,
    pub su2_fbprot: as3711_su2_fbprot,
    pub su2_auto_curr1: bool,
    pub su2_auto_curr2: bool,
    pub su2_auto_curr3: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct as3711_platform_data {
    pub regulator: as3711_regulator_pdata,
    pub backlight: as3711_bl_pdata,
}
