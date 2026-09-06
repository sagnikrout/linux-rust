//! Automatically rewritten from C to Rust
//! Source: drivers/hwtracing/intel_th/acpi.c
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
// Intel(R) Trace Hub ACPI driver
//
// Copyright (C) 2017 Intel Corporation.
//

    static const struct intel_th_drvdata intel_th_acpi_pch = {
    .host_mode_only	= 1,
    };
    static const struct intel_th_drvdata intel_th_acpi_uncore = {
    .host_mode_only	= 1,
    };
    static const struct acpi_device_id intel_th_acpi_ids[] = {
    { "INTC1000",	(kernel_ulong_t)&intel_th_acpi_uncore },
    { "INTC1001",	(kernel_ulong_t)&intel_th_acpi_pch },
    { "",		0 },
    };
    MODULE_DEVICE_TABLE(acpi, intel_th_acpi_ids);
#[no_mangle]
unsafe extern "C" fn intel_th_acpi_probe(pdev: *mut platform_device) -> c_int {
    static int intel_th_acpi_probe(struct platform_device *pdev)
    {
    struct acpi_device *adev = ACPI_COMPANION(&pdev.dev);
    struct resource resource[TH_MMIO_END];
    const struct acpi_device_id *id;
    struct intel_th *th;
    int i, r;
    id = acpi_match_device(intel_th_acpi_ids, &pdev.dev);
    if (!id)
    return -ENODEV;
    for (i = 0, r = 0; i < pdev.num_resources && r < TH_MMIO_END; i++)
    if (pdev.resource[i].flags &
    (IORESOURCE_IRQ | IORESOURCE_MEM))
    resource[r++] = pdev.resource[i];
    th = intel_th_alloc(&pdev.dev, (void *)id.driver_data, resource, r);
    if (IS_ERR(th))
    return PTR_ERR(th);
    adev.driver_data = th;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn intel_th_acpi_remove(pdev: *mut platform_device) {
    static void intel_th_acpi_remove(struct platform_device *pdev)
    {
    struct intel_th *th = platform_get_drvdata(pdev);
    intel_th_free(th);
    }
    static struct platform_driver intel_th_acpi_driver = {
    .probe		= intel_th_acpi_probe,
    .remove		= intel_th_acpi_remove,
    .driver		= {
    .name			= DRIVER_NAME,
    .acpi_match_table	= intel_th_acpi_ids,
    },
    };
    module_platform_driver(intel_th_acpi_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("Intel(R) Trace Hub ACPI controller driver");
    MODULE_AUTHOR("Alexander Shishkin <alexander.shishkin@intel.com>");
