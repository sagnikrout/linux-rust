//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/platform/mellanox/mlxbf-bootctl.h
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
// Copyright (c) 2019, Mellanox Technologies. All rights reserved.
//
// Request that the on-chip watchdog be enabled, or disabled, after
// the next chip soft reset. This call does not affect the current
// status of the on-chip watchdog. If non-zero, the argument
// specifies the watchdog interval in seconds. If zero, the watchdog
// will not be enabled after the next soft reset. Non-zero errors are
// returned as documented below.
//
pub const MLXBF_BOOTCTL_SET_POST_RESET_WDOG: c_uint = 0x82000000;
//
// Query the status which has been requested for the on-chip watchdog
// after the next chip soft reset. Returns the interval as set by
// MLXBF_BOOTCTL_SET_POST_RESET_WDOG.
//
pub const MLXBF_BOOTCTL_GET_POST_RESET_WDOG: c_uint = 0x82000001;
//
// Request that a specific boot action be taken at the next soft
// reset. By default, the boot action is set by external chip pins,
// which are sampled on hard reset. Note that the boot action
// requested by this call will persist on subsequent resets unless
// this service, or the MLNX_SET_SECOND_RESET_ACTION service, is
// invoked. See below for the available MLNX_BOOT_xxx parameter
// values. Non-zero errors are returned as documented below.
//
pub const MLXBF_BOOTCTL_SET_RESET_ACTION: c_uint = 0x82000002;
//
// Return the specific boot action which will be taken at the next
// soft reset. Returns the reset action (see below for the parameter
// values for MLXBF_BOOTCTL_SET_RESET_ACTION).
//
pub const MLXBF_BOOTCTL_GET_RESET_ACTION: c_uint = 0x82000003;
//
// Request that a specific boot action be taken at the soft reset
// after the next soft reset. For a specified valid boot mode, the
// effect of this call is identical to that of invoking
// MLXBF_BOOTCTL_SET_RESET_ACTION after the next chip soft reset; in
// particular, after that reset, the action for the now next reset can
// be queried with MLXBF_BOOTCTL_GET_RESET_ACTION and modified with
// MLXBF_BOOTCTL_SET_RESET_ACTION. You may also specify the parameter as
// MLNX_BOOT_NONE, which is equivalent to specifying that no call to
// MLXBF_BOOTCTL_SET_RESET_ACTION be taken after the next chip soft reset.
// This call does not affect the action to be taken at the next soft
// reset. Non-zero errors are returned as documented below.
//
pub const MLXBF_BOOTCTL_SET_SECOND_RESET_ACTION: c_uint = 0x82000004;
//
// Return the specific boot action which will be taken at the soft
// reset after the next soft reset; this will be one of the valid
// actions for MLXBF_BOOTCTL_SET_SECOND_RESET_ACTION.
//
pub const MLXBF_BOOTCTL_GET_SECOND_RESET_ACTION: c_uint = 0x82000005;
//
// Return the fuse status of the current chip. The caller should specify
// with the second argument if the state of the lifecycle fuses or the
// version of secure boot fuse keys left should be returned.
//
pub const MLXBF_BOOTCTL_GET_TBB_FUSE_STATUS: c_uint = 0x82000006;
// Reset eMMC by programming the RST_N register.
pub const MLXBF_BOOTCTL_SET_EMMC_RST_N: c_uint = 0x82000007;
pub const MLXBF_BOOTCTL_GET_DIMM_INFO: c_uint = 0x82000008;
//
// Initiate Firmware Reset via TYU. This might be invoked during the reset
// flow in isolation mode.
//
pub const MLXBF_BOOTCTL_FW_RESET: c_uint = 0x8200000D;
//
// SMC function IDs to set, get and lock the manufacturing information
// stored within the eeprom.
//
pub const MLXBF_BOOTCTL_SET_MFG_INFO: c_uint = 0x8200000E;
pub const MLXBF_BOOTCTL_GET_MFG_INFO: c_uint = 0x8200000F;
pub const MLXBF_BOOTCTL_LOCK_MFG_INFO: c_uint = 0x82000011;
//
// SMC function IDs to set and get the large ICM carveout size
// stored in the eeprom.
//
pub const MLNX_HANDLE_SET_ICM_INFO: c_uint = 0x82000012;
pub const MLNX_HANDLE_GET_ICM_INFO: c_uint = 0x82000013;
pub const MAX_ICM_BUFFER_SIZE: c_int = 10;
//
// SMC function ID to set the ARM boot state to up
//
pub const MLNX_HANDLE_OS_UP: c_uint = 0x82000014;
//
// SMC function ID to get and clear the RTC low voltage bit
//
pub const MLNX_HANDLE_GET_RTC_LOW_BATT: c_uint = 0x82000023;
// SMC function IDs for SiP Service queries
pub const MLXBF_BOOTCTL_SIP_SVC_CALL_COUNT: c_uint = 0x8200ff00;
pub const MLXBF_BOOTCTL_SIP_SVC_UID: c_uint = 0x8200ff01;
pub const MLXBF_BOOTCTL_SIP_SVC_VERSION: c_uint = 0x8200ff03;
// ARM Standard Service Calls version numbers
pub const MLXBF_BOOTCTL_SVC_VERSION_MAJOR: c_uint = 0x0;
pub const MLXBF_BOOTCTL_SVC_VERSION_MINOR: c_uint = 0x2;
// Number of svc calls defined.
pub const MLXBF_BOOTCTL_NUM_SVC_CALLS: c_int = 12;
// Valid reset actions for MLXBF_BOOTCTL_SET_RESET_ACTION.

// Valid arguments for requesting the fuse status.

// Additional value to disable the MLXBF_BOOTCTL_SET_SECOND_RESET_ACTION.
pub const MLXBF_BOOTCTL_NONE: c_uint = 0x7fffffff /* Don't change next boot action */;
pub const MLXBF_LARGE_ICMC_MAX_STRING_SIZE: c_int = 16;
pub const MLXBF_LARGE_ICMC_SIZE_MIN: c_uint = 0x80;
pub const MLXBF_LARGE_ICMC_SIZE_MAX: c_uint = 0x100000;

