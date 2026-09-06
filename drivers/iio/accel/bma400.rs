//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iio/accel/bma400.h
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
// Register constants and other forward declarations needed by the bma400
// sources.
//
// Copyright 2019 Dan Robertson <dan@dlrobertson.com>
//

//
// Read-Only Registers
//
// Chip ID of BMA 400 devices found in the chip ID register.
pub const BMA400_ID_REG_VAL: c_uint = 0x90;
// Status and ID registers
pub const BMA400_CHIP_ID_REG: c_uint = 0x00;
pub const BMA400_ERR_REG: c_uint = 0x02;
pub const BMA400_STATUS_REG: c_uint = 0x03;
// Acceleration registers
pub const BMA400_ACC_X_LSB_REG: c_uint = 0x04;
pub const BMA400_ACC_X_MSB_REG: c_uint = 0x05;
pub const BMA400_ACC_Y_LSB_REG: c_uint = 0x06;
pub const BMA400_ACC_Y_MSB_REG: c_uint = 0x07;
pub const BMA400_ACC_Z_LSB_REG: c_uint = 0x08;
pub const BMA400_ACC_Z_MSB_REG: c_uint = 0x09;
// Sensor time registers
pub const BMA400_SENSOR_TIME0_REG: c_uint = 0x0a;
pub const BMA400_SENSOR_TIME1_REG: c_uint = 0x0b;
pub const BMA400_SENSOR_TIME2_REG: c_uint = 0x0c;
// Event and interrupt registers
pub const BMA400_EVENT_REG: c_uint = 0x0d;
pub const BMA400_INT_STAT0_REG: c_uint = 0x0e;

pub const BMA400_INT_STAT1_REG: c_uint = 0x0f;

pub const BMA400_INT_STAT2_REG: c_uint = 0x10;
// Bit present in all INT_STAT registers

// Temperature register
pub const BMA400_TEMP_DATA_REG: c_uint = 0x11;
// FIFO length and data registers
pub const BMA400_FIFO_LENGTH0_REG: c_uint = 0x12;
pub const BMA400_FIFO_LENGTH1_REG: c_uint = 0x13;
pub const BMA400_FIFO_DATA_REG: c_uint = 0x14;
// Step count registers
pub const BMA400_STEP_CNT0_REG: c_uint = 0x15;
pub const BMA400_STEP_CNT1_REG: c_uint = 0x16;
pub const BMA400_STEP_CNT3_REG: c_uint = 0x17;
pub const BMA400_STEP_STAT_REG: c_uint = 0x18;
pub const BMA400_STEP_RAW_LEN: c_uint = 0x03;
//
// Read-write configuration registers
//
pub const BMA400_ACC_CONFIG0_REG: c_uint = 0x19;

pub const BMA400_ACC_CONFIG1_REG: c_uint = 0x1a;

pub const BMA400_ACC_CONFIG1_ODR_MIN_RAW: c_uint = 0x05;
pub const BMA400_ACC_CONFIG1_ODR_LP_RAW: c_uint = 0x06;
pub const BMA400_ACC_CONFIG1_ODR_MAX_RAW: c_uint = 0x0b;
pub const BMA400_ACC_CONFIG1_ODR_MAX_HZ: c_int = 800;
pub const BMA400_ACC_CONFIG1_ODR_MIN_WHOLE_HZ: c_int = 25;
pub const BMA400_ACC_CONFIG1_ODR_MIN_HZ: c_int = 12;

pub const BMA400_ACC_CONFIG2_REG: c_uint = 0x1b;
// Interrupt registers
pub const BMA400_INT_CONFIG0_REG: c_uint = 0x1f;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bma400_generic_intr {
    BMA400_GEN1_INTR = 0x1,
    BMA400_GEN2_INTR = 0x2,
}

pub const BMA400_INT_CONFIG1_REG: c_uint = 0x20;

pub const BMA400_INT1_MAP_REG: c_uint = 0x21;
pub const BMA400_INT12_MAP_REG: c_uint = 0x23;
pub const BMA400_INT_IO_CTRL_REG: c_uint = 0x24;

// Generic interrupts register
pub const BMA400_GENINT_CONFIG_REG_BASE: c_uint = 0x3f;
pub const BMA400_NUM_GENINT_CONFIG_REGS: c_int = 11;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bma400_accel_data_src {
    ACCEL_FILT1 = 0x0,
    ACCEL_FILT2 = 0x1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bma400_ref_updt_mode {
    BMA400_REF_MANUAL_UPDT_MODE = 0x0,
    BMA400_REF_ONETIME_UPDT_MODE = 0x1,
    BMA400_REF_EVERYTIME_UPDT_MODE = 0x2,
    BMA400_REF_EVERYTIME_LP_UPDT_MODE = 0x3,
}

pub const BMA400_GEN_CONFIG1_OFF: c_uint = 0x01;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bma400_genintr_acceleval_axescomb {
    BMA400_EVAL_X_OR_Y_OR_Z = 0x0,
    BMA400_EVAL_X_AND_Y_AND_Z = 0x1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bma400_detect_criterion {
    BMA400_DETECT_INACTIVITY = 0x0,
    BMA400_DETECT_ACTIVITY = 0x1,
}

// TAP config registers
pub const BMA400_TAP_CONFIG_REG: c_uint = 0x57;

pub const BMA400_TAP_CONFIG1_REG: c_uint = 0x58;

pub const BMA400_TAP_TIM_LIST_LEN: c_int = 4;
pub const BMA400_CMD_REG: c_uint = 0x7e;
//
// BMA400_SCALE_MIN macro value represents m/s^2 for 1 LSB before
// converting to micro values for +-2g range.
//
// For +-2g - 1 LSB = 0.976562 milli g = 0.009576 m/s^2
// For +-4g - 1 LSB = 1.953125 milli g = 0.019153 m/s^2
// For +-16g - 1 LSB = 7.8125 milli g = 0.076614 m/s^2
//
// The raw value which is used to select the different ranges is determined
// by the first bit set position from the scale value, so BMA400_SCALE_MIN
// should be odd.
//
// Scale values for +-2g, +-4g, +-8g and +-16g are populated into bma400_scales
// array by left shifting BMA400_SCALE_MIN.
// e.g.:
// To select +-2g = 9577 << 0 = raw value to write is 0.
// To select +-8g = 9577 << 2 = raw value to write is 2.
// To select +-16g = 9577 << 3 = raw value to write is 3.
//
pub const BMA400_ACC_SCALE_MIN: c_int = 9577;
pub const BMA400_ACC_SCALE_MAX: c_int = 76617;
