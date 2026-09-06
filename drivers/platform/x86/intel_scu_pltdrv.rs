//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/intel_scu_pltdrv.c
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
// Platform driver for the Intel SCU.
//
// Copyright (C) 2019, Intel Corporation
// Authors: Divya Sasidharan <divya.s.sasidharan@intel.com>
// Mika Westerberg <mika.westerberg@linux.intel.com>
// Rajmohan Mani <rajmohan.mani@intel.com>
//

#[no_mangle]
unsafe extern "C" fn intel_scu_platform_probe(pdev: *mut platform_device) -> c_int {
    static int intel_scu_platform_probe(struct platform_device *pdev)
    {
    let mut scu_data: intel_scu_ipc_data = {};
    struct intel_scu_ipc_dev *scu;
    const struct resource *res;
    scu_data.irq = platform_get_irq_optional(pdev, 0);
    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!res)
    return -ENOMEM;
    scu_data.mem = *res;
    scu = devm_intel_scu_ipc_register(&pdev.dev, &scu_data);
    if (IS_ERR(scu))
    return PTR_ERR(scu);
    platform_set_drvdata(pdev, scu);
    return 0;
    }
    static const struct acpi_device_id intel_scu_acpi_ids[] = {
    { "INTC1026" },
    {}
    };
    MODULE_DEVICE_TABLE(acpi, intel_scu_acpi_ids);
    static struct platform_driver intel_scu_platform_driver = {
    .probe = intel_scu_platform_probe,
    .driver = {
    .name = "intel_scu",
    .acpi_match_table = intel_scu_acpi_ids,
    },
    };
    module_platform_driver(intel_scu_platform_driver);
    MODULE_AUTHOR("Divya Sasidharan <divya.s.sasidharan@intel.com>");
    MODULE_AUTHOR("Mika Westerberg <mika.westerberg@linux.intel.com");
    MODULE_AUTHOR("Rajmohan Mani <rajmohan.mani@intel.com>");
    MODULE_DESCRIPTION("Intel SCU platform driver");
    MODULE_LICENSE("GPL v2");
