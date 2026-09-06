//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_pcode_api.h
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2022 Intel Corporation
//
// Internal to xe_pcode

pub const PCODE_ERROR_MASK: c_uint = 0xFF;
pub const PCODE_SUCCESS: c_uint = 0x0;
pub const PCODE_ILLEGAL_CMD: c_uint = 0x1;
pub const PCODE_TIMEOUT: c_uint = 0x2;
pub const PCODE_ILLEGAL_DATA: c_uint = 0x3;
pub const PCODE_ILLEGAL_SUBCOMMAND: c_uint = 0x4;
pub const PCODE_LOCKED: c_uint = 0x6;
pub const PCODE_GT_RATIO_OUT_OF_RANGE: c_uint = 0x10;
pub const PCODE_REJECTED: c_uint = 0x11;

// Min Freq QOS Table
pub const PCODE_WRITE_MIN_FREQ_TABLE: c_uint = 0x8;
pub const PCODE_READ_MIN_FREQ_TABLE: c_uint = 0x9;
pub const PCODE_FREQ_RING_RATIO_SHIFT: c_int = 16;
// PCODE Init
pub const DGFX_PCODE_STATUS: c_uint = 0x7E;
pub const DGFX_GET_INIT_STATUS: c_uint = 0x0;
pub const DGFX_INIT_STATUS_COMPLETE: c_uint = 0x1;

pub const PCODE_POWER_SETUP: c_uint = 0x7C;
pub const POWER_SETUP_SUBCOMMAND_READ_I1: c_uint = 0x4;
pub const POWER_SETUP_SUBCOMMAND_WRITE_I1: c_uint = 0x5;

pub const READ_PSYSGPU_POWER_LIMIT: c_uint = 0x6;
pub const WRITE_PSYSGPU_POWER_LIMIT: c_uint = 0x7;
pub const READ_PACKAGE_POWER_LIMIT: c_uint = 0x8;
pub const WRITE_PACKAGE_POWER_LIMIT: c_uint = 0x9;
pub const READ_PL_FROM_PCODE: c_uint = 0x0;
pub const READ_PL_FROM_FW: c_uint = 0x1;
pub const READ_PL_ACCEPTED: c_uint = 0x2;
pub const PCODE_THERMAL_INFO: c_uint = 0x25;
pub const READ_THERMAL_LIMITS: c_uint = 0x0;
pub const READ_THERMAL_CONFIG: c_uint = 0x1;
pub const READ_THERMAL_DATA: c_uint = 0x2;
pub const PCIE_SENSOR_GROUP_ID: c_uint = 0x2;

pub const PCODE_LATE_BINDING: c_uint = 0x5C;
pub const GET_CAPABILITY_STATUS: c_uint = 0x0;

pub const GET_VERSION_LOW: c_uint = 0x1;
pub const GET_VERSION_HIGH: c_uint = 0x2;

pub const FAN_TABLE: c_int = 1;
pub const VR_CONFIG: c_int = 2;
pub const PCODE_FREQUENCY_CONFIG: c_uint = 0x6e;
// Frequency Config Sub Commands (param1)
pub const PCODE_MBOX_FC_SC_READ_FUSED_P0: c_uint = 0x0;
pub const PCODE_MBOX_FC_SC_READ_FUSED_PN: c_uint = 0x1;
// Domain IDs (param2)
pub const PCODE_MBOX_DOMAIN_HBM: c_uint = 0x2;
pub const FAN_SPEED_CONTROL: c_uint = 0x7D;
pub const FSC_READ_NUM_FANS: c_uint = 0x4;

// PCODE_SCRATCH0

pub const CRITICAL_FAILURE: c_int = 4;
pub const NON_CRITICAL_FAILURE: c_int = 7;
// Auxiliary info bits

pub const DOWNGRADE_CAPABLE: c_int = 2;
