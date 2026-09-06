//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iio/adc/versal-sysmon.h
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
// AMD Versal SysMon driver
//
// Copyright (C) 2019 - 2022, Xilinx, Inc.
// Copyright (C) 2022 - 2026, Advanced Micro Devices, Inc.
//

// Register offsets (sorted by address)
pub const SYSMON_NPI_LOCK: c_uint = 0x000C;
pub const SYSMON_ISR: c_uint = 0x0044;
pub const SYSMON_IMR: c_uint = 0x0048;
pub const SYSMON_IER: c_uint = 0x004C;
pub const SYSMON_IDR: c_uint = 0x0050;
pub const SYSMON_CONFIG: c_uint = 0x0100;
pub const SYSMON_TEMP_MAX: c_uint = 0x1030;
pub const SYSMON_TEMP_MIN: c_uint = 0x1034;
pub const SYSMON_SUPPLY_BASE: c_uint = 0x1040;
pub const SYSMON_ALARM_FLAG: c_uint = 0x1018;
pub const SYSMON_ALARM_REG: c_uint = 0x1940;
pub const SYSMON_SUPPLY_EN_AVG_BASE: c_uint = 0x1958;
pub const SYSMON_TEMP_TH_LOW: c_uint = 0x1970;
pub const SYSMON_TEMP_TH_UP: c_uint = 0x1974;
pub const SYSMON_SUPPLY_TH_LOW: c_uint = 0x1980;
pub const SYSMON_SUPPLY_TH_UP: c_uint = 0x1C80;
pub const SYSMON_TEMP_EV_CFG: c_uint = 0x1F84;
pub const SYSMON_TEMP_MIN_MIN: c_uint = 0x1F8C;
pub const SYSMON_TEMP_MAX_MAX: c_uint = 0x1F90;
pub const SYSMON_STATUS_RESET: c_uint = 0x1F94;
pub const SYSMON_TEMP_SAT_BASE: c_uint = 0x1FAC;
pub const SYSMON_TEMP_EN_AVG_BASE: c_uint = 0x24B4;
pub const SYSMON_MAX_REG: c_uint = 0x24C0;
// NPI unlock value written to SYSMON_NPI_LOCK
pub const SYSMON_NPI_UNLOCK_CODE: c_uint = 0xF9E8D7C6;
// Register stride: 4 bytes per 32-bit register
pub const SYSMON_REG_STRIDE: c_int = 4;
pub const SYSMON_SUPPLY_IDX_MAX: c_int = 159;
pub const SYSMON_TEMP_SAT_MAX: c_int = 64;
pub const SYSMON_NO_OF_EVENTS: c_int = 32;

// ISR/IMR temperature alarm mask (bit 9)

// SYSMON_CONFIG: supply oversampling ratio

// SYSMON_CONFIG: temperature satellite oversampling ratio

// Per-channel averaging enable register counts
pub const SYSMON_SUPPLY_EN_AVG_COUNT: c_int = 5;
pub const SYSMON_TEMP_EN_AVG_COUNT: c_int = 2;
// Supply voltage conversion register fields

// Q8.7 fractional shift

pub const SYSMON_SUPPLY_MANTISSA_BITS: c_int = 16;
// Bits per alarm register
pub const SYSMON_ALARM_BITS_PER_REG: c_int = 32;
pub const SYSMON_UNMASK_WORK_DELAY_MS: c_int = 500;
//
// struct sysmon - Driver data for Versal SysMon
// @regmap: register map for hardware access
// @lock: protects read-modify-write sequences on threshold registers
// and cached state that spans multiple regmap calls
// @irq_lock: protects interrupt mask register updates (MMIO path only)
// @masked_temp: currently masked temperature alarm bits
// @temp_mask: temperature interrupt configuration mask
// @temp_hysteresis: cached DEVICE_TEMP hysteresis in millicelsius
// @sysmon_unmask_work: re-enables events after alarm condition clears
// @temp_oversampling: current temp oversampling ratio
// @supply_oversampling: current supply oversampling ratio
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sysmon {
    pub regmap: *mut regmap,
//
// Protects read-modify-write sequences on threshold registers
// and cached state (oversampling ratios, hysteresis values)
// that spans multiple regmap calls.
//
    pub lock: mutex,
//
// Protects interrupt mask register updates.  Only used on the
// MMIO path (fast_io regmap); I2C has no IRQ and never reaches
// the event code that takes this lock.
//
    pub irq_lock: spinlock_t,
    pub masked_temp: c_uint,
    pub temp_mask: c_uint,
    pub temp_hysteresis: c_int,
    pub sysmon_unmask_work: delayed_work,
    pub temp_oversampling: c_uint,
    pub supply_oversampling: c_uint,
}

extern "C" {
    pub fn devm_versal_sysmon_core_probe(dev: *mut device, regmap: *mut regmap) -> c_int;
}
