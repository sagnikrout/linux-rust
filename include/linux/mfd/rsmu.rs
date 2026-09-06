//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/rsmu.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Core interface for Renesas Synchronization Management Unit (SMU) devices.
//
// Copyright (C) 2021 Integrated Device Technology, Inc., a Renesas Company.
//

// The supported devices are ClockMatrix, Sabre and SnowLotus
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rsmu_type {
    RSMU_CM		= 0x34000,
    RSMU_SABRE	= 0x33810,
    RSMU_SL		= 0x19850,
}

//
// struct rsmu_ddata - device data structure for sub devices.
//
// @dev:    i2c/spi device.
// @regmap: i2c/spi bus access.
// @lock:   mutex used by sub devices to make sure a series of
// bus access requests are not interrupted.
// @type:   RSMU device type.
// @page:   i2c/spi bus driver internal use only.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsmu_ddata {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub lock: mutex,
    pub type: rsmu_type,
    pub page: u32,
}
