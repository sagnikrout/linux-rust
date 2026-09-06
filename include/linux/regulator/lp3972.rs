//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/regulator/lp3972.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// National Semiconductors LP3972 PMIC chip client interface
//
// Based on lp3971.h
//

pub const LP3972_LDO1: c_int = 0;
pub const LP3972_LDO2: c_int = 1;
pub const LP3972_LDO3: c_int = 2;
pub const LP3972_LDO4: c_int = 3;
pub const LP3972_LDO5: c_int = 4;
pub const LP3972_DCDC1: c_int = 5;
pub const LP3972_DCDC2: c_int = 6;
pub const LP3972_DCDC3: c_int = 7;
pub const LP3972_NUM_REGULATORS: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lp3972_regulator_subdev {
    pub id: c_int,
    pub initdata: *mut regulator_init_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lp3972_platform_data {
    pub num_regulators: c_int,
    pub regulators: *mut lp3972_regulator_subdev,
}
