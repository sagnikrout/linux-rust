//! Automatically rewritten from C to Rust
//! Source: drivers/cpuidle/cpuidle-calxeda.c
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
// Copyright 2012 Calxeda, Inc.
//
// Based on arch/arm/plat-mxc/cpuidle.c: #v3.7
// Copyright 2012 Freescale Semiconductor, Inc.
// Copyright 2012 Linaro Ltd.
//
// Maintainer: Rob Herring <rob.herring@calxeda.com>
//

    ((0 << PSCI_0_2_POWER_STATE_ID_SHIFT) | \
    (0 << PSCI_0_2_POWER_STATE_AFFL_SHIFT) | \
    (PSCI_POWER_STATE_TYPE_POWER_DOWN << PSCI_0_2_POWER_STATE_TYPE_SHIFT))
#[no_mangle]
unsafe extern "C" fn calxeda_idle_finish(val: c_ulong) -> c_int {
    static int calxeda_idle_finish(unsigned long val)
    {
    return psci_ops.cpu_suspend(CALXEDA_IDLE_PARAM, __pa(cpu_resume));
    }
    static int calxeda_pwrdown_idle(struct cpuidle_device *dev,
    struct cpuidle_driver *drv,
    int index)
    {
    cpu_pm_enter();
    cpu_suspend(0, calxeda_idle_finish);
    cpu_pm_exit();
    return index;
    }
    static struct cpuidle_driver calxeda_idle_driver = {
    .name = "calxeda_idle",
    .states = {
    ARM_CPUIDLE_WFI_STATE,
    {
    .name = "PG",
    .desc = "Power Gate",
    .exit_latency = 30,
    .power_usage = 50,
    .target_residency = 200,
    .enter = calxeda_pwrdown_idle,
    },
    },
    .state_count = 2,
    };
#[no_mangle]
unsafe extern "C" fn calxeda_cpuidle_probe(pdev: *mut platform_device) -> c_int {
    static int calxeda_cpuidle_probe(struct platform_device *pdev)
    {
    return cpuidle_register(&calxeda_idle_driver, core::ptr::null_mut());
    }
    static struct platform_driver calxeda_cpuidle_plat_driver = {
    .driver = {
    .name = "cpuidle-calxeda",
    },
    .probe = calxeda_cpuidle_probe,
    };
    builtin_platform_driver(calxeda_cpuidle_plat_driver);
