//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/firmware/imx/svc/pm.h
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
// Copyright (C) 2016 Freescale Semiconductor, Inc.
// Copyright 2017-2018 NXP
//
// Header file containing the public API for the System Controller (SC)
// Power Management (PM) function. This includes functions for power state
// control, clock control, reset control, and wake-up event control.
//
// PM_SVC (SVC) Power Management Service
//
// Module for the Power Management (PM) service.
//

//
// This type is used to indicate RPC PM function calls.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum imx_sc_pm_func {
    IMX_SC_PM_FUNC_UNKNOWN = 0,
    IMX_SC_PM_FUNC_SET_SYS_POWER_MODE = 19,
    IMX_SC_PM_FUNC_SET_PARTITION_POWER_MODE = 1,
    IMX_SC_PM_FUNC_GET_SYS_POWER_MODE = 2,
    IMX_SC_PM_FUNC_SET_RESOURCE_POWER_MODE = 3,
    IMX_SC_PM_FUNC_GET_RESOURCE_POWER_MODE = 4,
    IMX_SC_PM_FUNC_REQ_LOW_POWER_MODE = 16,
    IMX_SC_PM_FUNC_SET_CPU_RESUME_ADDR = 17,
    IMX_SC_PM_FUNC_REQ_SYS_IF_POWER_MODE = 18,
    IMX_SC_PM_FUNC_SET_CLOCK_RATE = 5,
    IMX_SC_PM_FUNC_GET_CLOCK_RATE = 6,
    IMX_SC_PM_FUNC_CLOCK_ENABLE = 7,
    IMX_SC_PM_FUNC_SET_CLOCK_PARENT = 14,
    IMX_SC_PM_FUNC_GET_CLOCK_PARENT = 15,
    IMX_SC_PM_FUNC_RESET = 13,
    IMX_SC_PM_FUNC_RESET_REASON = 10,
    IMX_SC_PM_FUNC_BOOT = 8,
    IMX_SC_PM_FUNC_REBOOT = 9,
    IMX_SC_PM_FUNC_REBOOT_PARTITION = 12,
    IMX_SC_PM_FUNC_CPU_START = 11,
}

//
// Defines for ALL parameters
//

//
// Defines for SC PM Power Mode
//

//
// Defines for SC PM CLK
//

//
// Defines for SC PM CLK Parent
//

