//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/fsl_pm.h
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
// Support Power Management
//
// Copyright 2014-2015 Freescale Semiconductor Inc.
//
pub const E500_PM_PH10: c_int = 1;
pub const E500_PM_PH15: c_int = 2;
pub const E500_PM_PH20: c_int = 3;
pub const E500_PM_PH30: c_int = 4;

pub const PLAT_PM_SLEEP: c_int = 20;
pub const PLAT_PM_LPM20: c_int = 30;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_pm_ops {
// mask pending interrupts to the RCPM from MPIC
    pub cpu): *mut *mut void (irq_mask)(int,
// unmask pending interrupts to the RCPM from MPIC
    pub cpu): *mut *mut void (irq_unmask)(int,
    pub state): *mut *mut void (cpu_enter_state)(int cpu, int,
    pub state): *mut *mut void (cpu_exit_state)(int cpu, int,
    pub cpu): *mut *mut void (cpu_up_prepare)(int,
    pub cpu): *mut *mut void (cpu_die)(int,
    pub (*plat_enter_sleep)(void): *mut c_int,
    pub freeze): *mut *mut void (freeze_time_base)(bool,
// keep the power of IP blocks during sleep/deep sleep
    pub mask): *mut *mut void (set_ip_power)(bool enable, u32,
// get platform supported power management modes
    pub (*get_pm_modes)(void): *mut c_uint,
}

extern "C" {
    pub fn fsl_rcpm_init() -> int __init;
}
