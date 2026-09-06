//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iio/imu/bmi323/bmi323.h
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


// SPDX-License-Identifier: GPL-2.0
//
// IIO driver for Bosch BMI323 6-Axis IMU
//
// Copyright (C) 2023, Jagath Jog J <jagathjog1996@gmail.com>
//

pub const BMI323_I2C_DUMMY: c_int = 2;
pub const BMI323_SPI_DUMMY: c_int = 1;
// Register map
pub const BMI323_CHIP_ID_REG: c_uint = 0x00;
pub const BMI323_CHIP_ID_VAL: c_uint = 0x0043;

pub const BMI323_ERR_REG: c_uint = 0x01;
pub const BMI323_STATUS_REG: c_uint = 0x02;

// Accelero/Gyro/Temp data registers
pub const BMI323_ACCEL_X_REG: c_uint = 0x03;
pub const BMI323_GYRO_X_REG: c_uint = 0x06;
pub const BMI323_TEMP_REG: c_uint = 0x09;

// Status registers
pub const BMI323_STATUS_INT1_REG: c_uint = 0x0D;
pub const BMI323_STATUS_INT2_REG: c_uint = 0x0E;

// Feature registers
pub const BMI323_FEAT_IO0_REG: c_uint = 0x10;

pub const BMI323_FEAT_IO1_REG: c_uint = 0x11;

pub const BMI323_FEAT_IO2_REG: c_uint = 0x12;
pub const BMI323_FEAT_IO_STATUS_REG: c_uint = 0x14;

pub const BMI323_FEAT_ENG_POLL: c_int = 2000;
pub const BMI323_FEAT_ENG_TIMEOUT: c_int = 10000;
// FIFO registers
pub const BMI323_FIFO_FILL_LEVEL_REG: c_uint = 0x15;
pub const BMI323_FIFO_DATA_REG: c_uint = 0x16;
// Accelero/Gyro config registers
pub const BMI323_ACC_CONF_REG: c_uint = 0x20;
pub const BMI323_GYRO_CONF_REG: c_uint = 0x21;

// FIFO registers
pub const BMI323_FIFO_WTRMRK_REG: c_uint = 0x35;
pub const BMI323_FIFO_CONF_REG: c_uint = 0x36;

pub const BMI323_FIFO_CTRL_REG: c_uint = 0x37;

// Interrupt pin config registers
pub const BMI323_IO_INT_CTR_REG: c_uint = 0x38;

pub const BMI323_IO_INT_CONF_REG: c_uint = 0x39;

pub const BMI323_INT_MAP1_REG: c_uint = 0x3A;
pub const BMI323_INT_MAP2_REG: c_uint = 0x3B;

// Feature registers
pub const BMI323_FEAT_CTRL_REG: c_uint = 0x40;

pub const BMI323_FEAT_DATA_ADDR: c_uint = 0x41;
pub const BMI323_FEAT_DATA_TX: c_uint = 0x42;
pub const BMI323_FEAT_DATA_STATUS: c_uint = 0x43;

pub const BMI323_FEAT_EVNT_EXT_REG: c_uint = 0x47;

pub const BMI323_CMD_REG: c_uint = 0x7E;
pub const BMI323_RST_VAL: c_uint = 0xDEAF;
pub const BMI323_CFG_RES_REG: c_uint = 0x7F;
// Extended registers
pub const BMI323_GEN_SET1_REG: c_uint = 0x02;

// Any Motion/No Motion config registers
pub const BMI323_ANYMO1_REG: c_uint = 0x05;
pub const BMI323_NOMO1_REG: c_uint = 0x08;
pub const BMI323_MO2_OFFSET: c_uint = 0x01;
pub const BMI323_MO3_OFFSET: c_uint = 0x02;

// Step counter config registers
pub const BMI323_STEP_SC1_REG: c_uint = 0x10;

pub const BMI323_STEP_LEN: c_int = 2;
// Tap gesture config registers
pub const BMI323_TAP1_REG: c_uint = 0x1E;

pub const BMI323_TAP2_REG: c_uint = 0x1F;

pub const BMI323_TAP3_REG: c_uint = 0x20;

pub const BMI323_MOTION_THRES_SCALE: c_int = 512;
pub const BMI323_MOTION_HYSTR_SCALE: c_int = 512;
pub const BMI323_MOTION_DURAT_SCALE: c_int = 50;
pub const BMI323_TAP_THRES_SCALE: c_int = 512;
pub const BMI323_DUR_BW_TAP_SCALE: c_int = 200;
pub const BMI323_QUITE_TIM_GES_SCALE: c_int = 25;
pub const BMI323_MAX_GES_DUR_SCALE: c_int = 25;
//
// The formula to calculate temperature in C.
// See datasheet section 6.1.1, Register Map Overview
//
// T_C = (temp_raw / 512) + 23
//
pub const BMI323_TEMP_OFFSET: c_int = 11776;
pub const BMI323_TEMP_SCALE: c_int = 1953125;
//
// The BMI323 features a FIFO with a capacity of 2048 bytes. Each frame
// consists of accelerometer (X, Y, Z) data and gyroscope (X, Y, Z) data,
// totaling 6 words or 12 bytes. The FIFO buffer can hold a total of
// 170 frames.
//
// If a watermark interrupt is configured for 170 frames, the interrupt will
// trigger when the FIFO reaches 169 frames, so limit the maximum watermark
// level to 169 frames. In terms of data, 169 frames would equal 1014 bytes,
// which is approximately 2 frames before the FIFO reaches its full capacity.
// See datasheet section 5.7.3 FIFO Buffer Interrupts
//
pub const BMI323_BYTES_PER_SAMPLE: c_int = 2;
pub const BMI323_FIFO_LENGTH_IN_BYTES: c_int = 2048;
pub const BMI323_FIFO_FRAME_LENGTH: c_int = 6;

extern "C" {
    pub fn bmi323_core_probe(dev: *mut device) -> c_int;
}
