//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/tps6507x.h
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


// linux/mfd/tps6507x.h
//
// Functions to access TPS65070 power management chip.
//
// Copyright (c) 2009 RidgeRun (todd.fischer@ridgerun.com)
//
// For licencing details see kernel-base/COPYING
//
// ----------------------------------------------------------------------------
// Registers, all 8 bits
// ----------------------------------------------------------------------------
//
// Register definitions

pub const TPS6507X_ADCONFIG_INPUT_AD_IN1: c_int = 0;
pub const TPS6507X_ADCONFIG_INPUT_AD_IN2: c_int = 1;
pub const TPS6507X_ADCONFIG_INPUT_AD_IN3: c_int = 2;
pub const TPS6507X_ADCONFIG_INPUT_AD_IN4: c_int = 3;
pub const TPS6507X_ADCONFIG_INPUT_TS_PIN: c_int = 4;
pub const TPS6507X_ADCONFIG_INPUT_BAT_CURRENT: c_int = 5;
pub const TPS6507X_ADCONFIG_INPUT_AC_VOLTAGE: c_int = 6;
pub const TPS6507X_ADCONFIG_INPUT_SYS_VOLTAGE: c_int = 7;
pub const TPS6507X_ADCONFIG_INPUT_CHARGER_VOLTAGE: c_int = 8;
pub const TPS6507X_ADCONFIG_INPUT_BAT_VOLTAGE: c_int = 9;
pub const TPS6507X_ADCONFIG_INPUT_THRESHOLD_VOLTAGE: c_int = 10;
pub const TPS6507X_ADCONFIG_INPUT_ISET1_VOLTAGE: c_int = 11;
pub const TPS6507X_ADCONFIG_INPUT_ISET2_VOLTAGE: c_int = 12;
pub const TPS6507X_ADCONFIG_INPUT_REAL_TSC: c_int = 14;
pub const TPS6507X_ADCONFIG_INPUT_TSC: c_int = 15;

pub const TPS6507X_TSCMODE_X_POSITION: c_int = 0;
pub const TPS6507X_TSCMODE_Y_POSITION: c_int = 1;
pub const TPS6507X_TSCMODE_PRESSURE: c_int = 2;
pub const TPS6507X_TSCMODE_X_PLATE: c_int = 3;
pub const TPS6507X_TSCMODE_Y_PLATE: c_int = 4;
pub const TPS6507X_TSCMODE_STANDBY: c_int = 5;
pub const TPS6507X_TSCMODE_ADC_INPUT: c_int = 6;
pub const TPS6507X_TSCMODE_DISABLE: c_int = 7;

// VDCDC MASK

//
// struct tps6507x_board - packages regulator and touchscreen init data
// @tps6507x_regulator_data: regulator initialization values
//
// Board data may be used to initialize regulator and touchscreen.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tps6507x_board {
    pub tps6507x_pmic_init_data: *mut regulator_init_data,
    pub tps6507x_ts_init_data: *mut touchscreen_init_data,
}

//
// struct tps6507x_dev - tps6507x sub-driver chip access routines
// @read_dev() - I2C register read function
// @write_dev() - I2C register write function
//
// Device data may be used to access the TPS6507x chip
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tps6507x_dev {
    pub dev: *mut device,
    pub i2c_client: *mut i2c_client,
    pub dest): *mut c_void,
    pub src): *mut c_void,
// Client devices
    pub pmic: *mut tps6507x_pmic,
}
