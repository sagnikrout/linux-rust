//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/acpi/pmic/intel_pmic.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmic_table {
    pub /: *mut *mut int address; / operation region address,
    pub /: *mut *mut int reg; / corresponding thermal register,
    pub /: *mut *mut int bit; / control bit for power,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_pmic_opregion_data {
    pub value): *mut *mut *mut int (get_power)(struct regmap r, int reg, int bit, u64,
    pub on): *mut *mut *mut int (update_power)(struct regmap r, int reg, int bit, bool,
    pub reg): *mut *mut *mut int (get_raw_temp)(struct regmap r, int,
    pub raw_temp): *mut *mut *mut int (update_aux)(struct regmap r, int reg, int,
    pub value): *mut *mut *mut int (get_policy)(struct regmap r, int reg, int bit, u64,
    pub enable): *mut *mut *mut int (update_policy)(struct regmap r, int reg, int bit, int,
    pub mask): u32 reg_address, u32 value, u32,
    pub raw): c_int,
    pub power_table: *const pmic_table,
    pub power_table_count: c_int,
    pub thermal_table: *const pmic_table,
    pub thermal_table_count: c_int,
// For generic exec_mipi_pmic_seq_element handling
    pub pmic_i2c_address: c_int,
}
