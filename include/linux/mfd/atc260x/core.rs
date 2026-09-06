//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/atc260x/core.h
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
// Core MFD defines for ATC260x PMICs
//
// Copyright (C) 2019 Manivannan Sadhasivam <manivannan.sadhasivam@linaro.org>
// Copyright (C) 2020 Cristian Ciocaltea <cristian.ciocaltea@gmail.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atc260x_type {
    ATC2603A = 0,
    ATC2603C,
    ATC2609A,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atc260x_ver {
    ATC260X_A = 0,
    ATC260X_B,
    ATC260X_C,
    ATC260X_D,
    ATC260X_E,
    ATC260X_F,
    ATC260X_G,
    ATC260X_H,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atc260x {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub regmap_irq_chip: *const regmap_irq_chip,
    pub irq_data: *mut regmap_irq_chip_data,
    pub /: *mut *mut *mut mutex regmap_mutex; / mutex for custom regmap locking,
    pub cells: *const mfd_cell,
    pub nr_cells: c_int,
    pub irq: c_int,
    pub ic_type: atc260x_type,
    pub ic_ver: atc260x_ver,
    pub type_name: *const c_char,
    pub rev_reg: c_uint,
    pub /: *const *const *const atc260x_init_regs init_regs; / regs for device init,
}

extern "C" {
    pub fn atc260x_match_device(atc260x: *mut atc260x, regmap_cfg: *mut regmap_config) -> c_int;
}
extern "C" {
    pub fn atc260x_device_probe(atc260x: *mut atc260x) -> c_int;
}
