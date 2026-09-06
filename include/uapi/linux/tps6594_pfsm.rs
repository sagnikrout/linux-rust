//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/tps6594_pfsm.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// Userspace ABI for TPS6594 PMIC Pre-configurable Finite State Machine
//
// Copyright (C) 2023 BayLibre Incorporated - https://www.baylibre.com
//

//
// struct pmic_state_opt - PMIC state options
// @gpio_retention: if enabled, power rails associated with GPIO retention remain active
// @ddr_retention: if enabled, power rails associated with DDR retention remain active
// @mcu_only_startup_dest: if enabled, startup destination state is MCU_ONLY
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmic_state_opt {
    pub gpio_retention: __u8,
    pub ddr_retention: __u8,
    pub mcu_only_startup_dest: __u8,
}

// Commands

