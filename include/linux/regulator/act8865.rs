//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/regulator/act8865.h
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
// act8865.h  --  Voltage regulation for active-semi act88xx PMUs
//
// Copyright (C) 2013 Atmel Corporation.
//

//
// act8865_regulator_data - regulator data
// @id: regulator id
// @name: regulator name
// @init_data: regulator init data
// @of_node: device tree node (optional)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct act8865_regulator_data {
    pub id: c_int,
    pub name: *const c_char,
    pub init_data: *mut regulator_init_data,
    pub of_node: *mut device_node,
}

//
// act8865_platform_data - platform data for act8865
// @num_regulators: number of regulators used
// @regulators: pointer to regulators used
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct act8865_platform_data {
    pub num_regulators: c_int,
    pub regulators: *mut act8865_regulator_data,
}
