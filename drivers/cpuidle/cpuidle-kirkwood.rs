//! Automatically rewritten from C to Rust
//! Source: drivers/cpuidle/cpuidle-kirkwood.c
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
// CPU idle Marvell Kirkwood SoCs
//
// The cpu idle uses wait-for-interrupt and DDR self refresh in order
// to implement two idle states -
// #1 wait-for-interrupt
// #2 wait-for-interrupt and DDR self refresh
//
// Maintainer: Jason Cooper <jason@lakedaemon.net>
// Maintainer: Andrew Lunn <andrew@lunn.ch>
//

pub const KIRKWOOD_MAX_STATES: c_int = 2;
    static void __iomem *ddr_operation_base;
// Actual code that puts the SoC in different idle states
    static int kirkwood_enter_idle(struct cpuidle_device *dev,
    struct cpuidle_driver *drv,
    int index)
    {
    writel(0x7, ddr_operation_base);
    cpu_do_idle();
    return index;
    }
    static struct cpuidle_driver kirkwood_idle_driver = {
    .name			= "kirkwood_idle",
    .owner			= THIS_MODULE,
    .states[0]		= ARM_CPUIDLE_WFI_STATE,
    .states[1]		= {
    .enter			= kirkwood_enter_idle,
    .exit_latency		= 10,
    .target_residency	= 100000,
    .name			= "DDR SR",
    .desc			= "WFI and DDR Self Refresh",
    },
    .state_count = KIRKWOOD_MAX_STATES,
    };
// Initialize CPU idle by registering the idle states
#[no_mangle]
unsafe extern "C" fn kirkwood_cpuidle_probe(pdev: *mut platform_device) -> c_int {
    static int kirkwood_cpuidle_probe(struct platform_device *pdev)
    {
    ddr_operation_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(ddr_operation_base))
    return PTR_ERR(ddr_operation_base);
    return cpuidle_register(&kirkwood_idle_driver, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn kirkwood_cpuidle_remove(pdev: *mut platform_device) {
    static void kirkwood_cpuidle_remove(struct platform_device *pdev)
    {
    cpuidle_unregister(&kirkwood_idle_driver);
    }
    static struct platform_driver kirkwood_cpuidle_driver = {
    .probe = kirkwood_cpuidle_probe,
    .remove = kirkwood_cpuidle_remove,
    .driver = {
    .name = "kirkwood_cpuidle",
    },
    };
    module_platform_driver(kirkwood_cpuidle_driver);
    MODULE_AUTHOR("Andrew Lunn <andrew@lunn.ch>");
    MODULE_DESCRIPTION("Kirkwood cpu idle driver");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:kirkwood-cpuidle");
