//! Automatically rewritten from C to Rust
//! Source: drivers/cpuidle/cpuidle-zynq.c
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
// Copyright (C) 2012-2013 Xilinx
//
// CPU idle support for Xilinx Zynq
//
// based on arch/arm/mach-at91/cpuidle.c
//
// The cpu idle uses wait-for-interrupt and RAM self refresh in order
// to implement two idle states -
// #1 wait-for-interrupt
// #2 wait-for-interrupt and RAM self refresh
//
// Maintainer: Michal Simek <michal.simek@amd.com>
//

pub const ZYNQ_MAX_STATES: c_int = 2;
// Actual code that puts the SoC in different idle states
    static int zynq_enter_idle(struct cpuidle_device *dev,
    struct cpuidle_driver *drv, int index)
    {
// Add code for DDR self refresh start
    cpu_do_idle();
    return index;
    }
    static struct cpuidle_driver zynq_idle_driver = {
    .name = "zynq_idle",
    .owner = THIS_MODULE,
    .states = {
    ARM_CPUIDLE_WFI_STATE,
    {
    .enter			= zynq_enter_idle,
    .exit_latency		= 10,
    .target_residency	= 10000,
    .name			= "RAM_SR",
    .desc			= "WFI and RAM Self Refresh",
    },
    },
    .safe_state_index = 0,
    .state_count = ZYNQ_MAX_STATES,
    };
// Initialize CPU idle by registering the idle states
#[no_mangle]
unsafe extern "C" fn zynq_cpuidle_probe(pdev: *mut platform_device) -> c_int {
    static int zynq_cpuidle_probe(struct platform_device *pdev)
    {
    pr_info("Xilinx Zynq CpuIdle Driver started\n");
    return cpuidle_register(&zynq_idle_driver, core::ptr::null_mut());
    }
    static struct platform_driver zynq_cpuidle_driver = {
    .driver = {
    .name = "cpuidle-zynq",
    },
    .probe = zynq_cpuidle_probe,
    };
    builtin_platform_driver(zynq_cpuidle_driver);
