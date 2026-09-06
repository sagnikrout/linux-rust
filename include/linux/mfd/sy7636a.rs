//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/sy7636a.h
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
// Functions to access SY3686A power management chip.
//
// Copyright (C) 2021 reMarkable AS - http://www.remarkable.com
//
pub const SY7636A_REG_OPERATION_MODE_CRL: c_uint = 0x00;
// It is set if a gpio is used to control the regulator

pub const SY7636A_REG_VCOM_ADJUST_CTRL_L: c_uint = 0x01;
pub const SY7636A_REG_VCOM_ADJUST_CTRL_H: c_uint = 0x02;
pub const SY7636A_REG_VCOM_ADJUST_CTRL_MASK: c_uint = 0x01ff;
pub const SY7636A_REG_VLDO_VOLTAGE_ADJULST_CTRL: c_uint = 0x03;
pub const SY7636A_REG_POWER_ON_DELAY_TIME: c_uint = 0x06;
pub const SY7636A_REG_FAULT_FLAG: c_uint = 0x07;

pub const SY7636A_REG_TERMISTOR_READOUT: c_uint = 0x08;
pub const SY7636A_REG_MAX: c_uint = 0x08;
pub const VCOM_ADJUST_CTRL_MASK: c_uint = 0x1ff;
// Used to shift the high byte
pub const VCOM_ADJUST_CTRL_SHIFT: c_int = 8;
// Used to scale from VCOM_ADJUST_CTRL to mv
pub const VCOM_ADJUST_CTRL_SCAL: c_int = 10000;
pub const FAULT_FLAG_SHIFT: c_int = 1;
