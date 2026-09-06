//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/regulator/tps62360.h
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
// tps62360.h -- TI tps62360
//
// Interface for regulator driver for TI TPS62360 Processor core supply
//
// Copyright (C) 2012 NVIDIA Corporation
// Author: Laxman Dewangan <ldewangan@nvidia.com>
//
// struct tps62360_regulator_platform_data - tps62360 regulator platform data.
//
// @reg_init_data: The regulator init data.
// @en_discharge: Enable discharge the output capacitor via internal
// register.
// @en_internal_pulldn: internal pull down enable or not.
// @vsel0_def_state: Default state of vsel0. 1 if it is high else 0.
// @vsel1_def_state: Default state of vsel1. 1 if it is high else 0.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tps62360_regulator_platform_data {
    pub reg_init_data: *mut regulator_init_data,
    pub en_discharge: bool,
    pub en_internal_pulldn: bool,
    pub vsel0_def_state: c_int,
    pub vsel1_def_state: c_int,
}
