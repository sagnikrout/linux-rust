//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/regulator/db8500-prcmu.h
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
// Copyright (C) ST-Ericsson SA 2010
//
// Author: Bengt Jonsson <bengt.g.jonsson@stericsson.com> for ST-Ericsson
//
// Interface to power domain regulators on DB8500
//
// Number of DB8500 regulators and regulator enumeration
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum db8500_regulator_id {
    DB8500_REGULATOR_VAPE,
    DB8500_REGULATOR_VARM,
    DB8500_REGULATOR_VMODEM,
    DB8500_REGULATOR_VPLL,
    DB8500_REGULATOR_VSMPS1,
    DB8500_REGULATOR_VSMPS2,
    DB8500_REGULATOR_VSMPS3,
    DB8500_REGULATOR_VRF1,
    DB8500_REGULATOR_SWITCH_SVAMMDSP,
    DB8500_REGULATOR_SWITCH_SVAMMDSPRET,
    DB8500_REGULATOR_SWITCH_SVAPIPE,
    DB8500_REGULATOR_SWITCH_SIAMMDSP,
    DB8500_REGULATOR_SWITCH_SIAMMDSPRET,
    DB8500_REGULATOR_SWITCH_SIAPIPE,
    DB8500_REGULATOR_SWITCH_SGA,
    DB8500_REGULATOR_SWITCH_B2R2_MCDE,
    DB8500_REGULATOR_SWITCH_ESRAM12,
    DB8500_REGULATOR_SWITCH_ESRAM12RET,
    DB8500_REGULATOR_SWITCH_ESRAM34,
    DB8500_REGULATOR_SWITCH_ESRAM34RET,
    DB8500_NUM_REGULATORS
}
