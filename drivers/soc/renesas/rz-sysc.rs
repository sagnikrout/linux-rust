//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/soc/renesas/rz-sysc.h
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
// Renesas RZ System Controller
//
// Copyright (C) 2024 Renesas Electronics Corp.
//

//
// struct rz_syc_soc_id_init_data - RZ SYSC SoC identification initialization data
// @family: RZ SoC family
// @id: RZ SoC expected ID
// @devid_offset: SYSC SoC ID register offset
// @revision_mask: SYSC SoC ID revision mask
// @specific_id_mask: SYSC SoC ID specific ID mask
// @print_id: print SoC-specific extended device identification
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rz_sysc_soc_id_init_data {
    pub family: *const *const c_char,
    pub id: u32,
    pub devid_offset: u32,
    pub revision_mask: u32,
    pub specific_id_mask: u32,
    pub soc_dev_attr): *mut soc_device_attribute,
}

//
// struct rz_sysc_init_data - RZ SYSC initialization data
// @soc_id_init_data: RZ SYSC SoC ID initialization data
// @writeable_reg: Regmap writeable register check function
// @readable_reg: Regmap readable register check function
// @max_register: Maximum SYSC register offset to be used by the regmap config
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rz_sysc_init_data {
    pub soc_id_init_data: *const rz_sysc_soc_id_init_data,
    pub reg): *mut *mut *mut bool (writeable_reg)(struct device dev, unsigned int,
    pub reg): *mut *mut *mut bool (readable_reg)(struct device dev, unsigned int,
    pub max_register: u32,
}
