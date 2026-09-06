//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/psci.h
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
// ARM Power State and Coordination Interface (PSCI) header
//
// This header holds common PSCI defines and macros shared
// by: ARM kernel, ARM64 kernel, KVM ARM/ARM64 and user space.
//
// Copyright (C) 2014 Linaro Ltd.
// Author: Anup Patel <anup.patel@linaro.org>
//
// PSCI v0.1 interface
//
// The PSCI v0.1 function numbers are implementation defined.
//
// Only PSCI return values such as: SUCCESS, NOT_SUPPORTED,
// INVALID_PARAMS, and DENIED defined below are applicable
// to PSCI v0.1.
//
// PSCI v0.2 interface
pub const PSCI_0_2_FN_BASE: c_uint = 0x84000000;

pub const PSCI_0_2_64BIT: c_uint = 0x40000000;

// PSCI v0.2 power state encoding for CPU_SUSPEND function
pub const PSCI_0_2_POWER_STATE_ID_MASK: c_uint = 0xffff;
pub const PSCI_0_2_POWER_STATE_ID_SHIFT: c_int = 0;
pub const PSCI_0_2_POWER_STATE_TYPE_SHIFT: c_int = 16;

pub const PSCI_0_2_POWER_STATE_AFFL_SHIFT: c_int = 24;

// PSCI extended power state encoding for CPU_SUSPEND function
pub const PSCI_1_0_EXT_POWER_STATE_ID_MASK: c_uint = 0xfffffff;
pub const PSCI_1_0_EXT_POWER_STATE_ID_SHIFT: c_int = 0;
pub const PSCI_1_0_EXT_POWER_STATE_TYPE_SHIFT: c_int = 30;

// PSCI v0.2 affinity level state returned by AFFINITY_INFO
pub const PSCI_0_2_AFFINITY_LEVEL_ON: c_int = 0;
pub const PSCI_0_2_AFFINITY_LEVEL_OFF: c_int = 1;
pub const PSCI_0_2_AFFINITY_LEVEL_ON_PENDING: c_int = 2;
// PSCI v0.2 multicore support in Trusted OS returned by MIGRATE_INFO_TYPE
pub const PSCI_0_2_TOS_UP_MIGRATE: c_int = 0;
pub const PSCI_0_2_TOS_UP_NO_MIGRATE: c_int = 1;
pub const PSCI_0_2_TOS_MP: c_int = 2;
// PSCI v1.1 reset type encoding for SYSTEM_RESET2
pub const PSCI_1_1_RESET_TYPE_SYSTEM_WARM_RESET: c_int = 0;
pub const PSCI_1_1_RESET_TYPE_VENDOR_START: c_uint = 0x80000000U;
// PSCI v1.3 hibernate type for SYSTEM_OFF2

// PSCI version decoding (independent of PSCI version)
pub const PSCI_VERSION_MAJOR_SHIFT: c_int = 16;

// PSCI features decoding (>=1.0)
pub const PSCI_1_0_FEATURES_CPU_SUSPEND_PF_SHIFT: c_int = 1;

pub const PSCI_1_0_SUSPEND_MODE_PC: c_int = 0;
pub const PSCI_1_0_SUSPEND_MODE_OSI: c_int = 1;
// PSCI return values (inclusive of all PSCI versions)
pub const PSCI_RET_SUCCESS: c_int = 0;

