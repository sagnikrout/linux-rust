//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iio/gyro/fxas21002c.h
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
// Driver for NXP FXAS21002C Gyroscope - Header
//
// Copyright (C) 2019 Linaro Ltd.
//

pub const FXAS21002C_REG_STATUS: c_uint = 0x00;
pub const FXAS21002C_REG_OUT_X_MSB: c_uint = 0x01;
pub const FXAS21002C_REG_OUT_X_LSB: c_uint = 0x02;
pub const FXAS21002C_REG_OUT_Y_MSB: c_uint = 0x03;
pub const FXAS21002C_REG_OUT_Y_LSB: c_uint = 0x04;
pub const FXAS21002C_REG_OUT_Z_MSB: c_uint = 0x05;
pub const FXAS21002C_REG_OUT_Z_LSB: c_uint = 0x06;
pub const FXAS21002C_REG_DR_STATUS: c_uint = 0x07;
pub const FXAS21002C_REG_F_STATUS: c_uint = 0x08;
pub const FXAS21002C_REG_F_SETUP: c_uint = 0x09;
pub const FXAS21002C_REG_F_EVENT: c_uint = 0x0A;
pub const FXAS21002C_REG_INT_SRC_FLAG: c_uint = 0x0B;
pub const FXAS21002C_REG_WHO_AM_I: c_uint = 0x0C;
pub const FXAS21002C_REG_CTRL0: c_uint = 0x0D;
pub const FXAS21002C_REG_RT_CFG: c_uint = 0x0E;
pub const FXAS21002C_REG_RT_SRC: c_uint = 0x0F;
pub const FXAS21002C_REG_RT_THS: c_uint = 0x10;
pub const FXAS21002C_REG_RT_COUNT: c_uint = 0x11;
pub const FXAS21002C_REG_TEMP: c_uint = 0x12;
pub const FXAS21002C_REG_CTRL1: c_uint = 0x13;
pub const FXAS21002C_REG_CTRL2: c_uint = 0x14;
pub const FXAS21002C_REG_CTRL3: c_uint = 0x15;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fxas21002c_fields {
    F_DR_STATUS,
    F_OUT_X_MSB,
    F_OUT_X_LSB,
    F_OUT_Y_MSB,
    F_OUT_Y_LSB,
    F_OUT_Z_MSB,
    F_OUT_Z_LSB,
// DR_STATUS
    F_ZYX_OW, F_Z_OW, F_Y_OW, F_X_OW, F_ZYX_DR, F_Z_DR, F_Y_DR, F_X_DR,
// F_STATUS
    F_OVF, F_WMKF, F_CNT,
// F_SETUP
    F_MODE, F_WMRK,
// F_EVENT
    F_EVENT, FE_TIME,
// INT_SOURCE_FLAG
    F_BOOTEND, F_SRC_FIFO, F_SRC_RT, F_SRC_DRDY,
// WHO_AM_I
    F_WHO_AM_I,
// CTRL_REG0
    F_BW, F_SPIW, F_SEL, F_HPF_EN, F_FS,
// RT_CFG
    F_ELE, F_ZTEFE, F_YTEFE, F_XTEFE,
// RT_SRC
    F_EA, F_ZRT, F_ZRT_POL, F_YRT, F_YRT_POL, F_XRT, F_XRT_POL,
// RT_THS
    F_DBCNTM, F_THS,
// RT_COUNT
    F_RT_COUNT,
// TEMP
    F_TEMP,
// CTRL_REG1
    F_RST, F_ST, F_DR, F_ACTIVE, F_READY,
// CTRL_REG2
    F_INT_CFG_FIFO, F_INT_EN_FIFO, F_INT_CFG_RT, F_INT_EN_RT,
    F_INT_CFG_DRDY, F_INT_EN_DRDY, F_IPOL, F_PP_OD,
// CTRL_REG3
    F_WRAPTOONE, F_EXTCTRLEN, F_FS_DOUBLE,
// MAX FIELDS
    F_MAX_FIELDS,
}

extern "C" {
    pub fn fxas21002c_core_remove(dev: *mut device);
}
