//! Automatically rewritten from C to Rust
//! Source: drivers/pwm/pwm-lpss-platform.c
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
// Intel Low Power Subsystem PWM controller driver
//
// Copyright (C) 2014, Intel Corporation
//
// Derived from the original pwm-lpss.c
//

#[no_mangle]
unsafe extern "C" fn pwm_lpss_probe_platform(pdev: *mut platform_device) -> c_int {
    static int pwm_lpss_probe_platform(struct platform_device *pdev)
    {
    const struct pwm_lpss_boardinfo *info;
    struct pwm_chip *chip;
    void __iomem *base;
    info = device_get_match_data(&pdev.dev);
    if (!info)
    return -ENODEV;
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base))
    return PTR_ERR(base);
    chip = devm_pwm_lpss_probe(&pdev.dev, base, info);
    if (IS_ERR(chip))
    return PTR_ERR(chip);
//
// On Cherry Trail devices the GFX0._PS0 AML checks if the controller
// is on and if it is not on it turns it on and restores what it
// believes is the correct state to the PWM controller.
// Because of this we must disallow direct-complete, which keeps the
// controller (runtime)suspended on resume, to avoid 2 issues:
// 1. The controller getting turned on without the linux-pm code
// knowing about this. On devices where the controller is unused
// this causes it to stay on during the next suspend causing high
// battery drain (because S0i3 is not reached)
// 2. The state restoring code unexpectedly messing with the controller
//
// Leaving the controller runtime-suspended (skipping runtime-resume +
// normal-suspend) during suspend is fine.
//
    if (info.other_devices_aml_touches_pwm_regs)
    dev_pm_set_driver_flags(&pdev.dev, DPM_FLAG_NO_DIRECT_COMPLETE|
    DPM_FLAG_SMART_SUSPEND);
    pm_runtime_set_active(&pdev.dev);
    return devm_pm_runtime_enable(&pdev.dev);
    }
    static const struct acpi_device_id pwm_lpss_acpi_match[] = {
    { .id = "80860F09", .driver_data = (unsigned long)&pwm_lpss_byt_info },
    { .id = "80862288", .driver_data = (unsigned long)&pwm_lpss_bsw_info },
    { .id = "80862289", .driver_data = (unsigned long)&pwm_lpss_bsw_info },
    { .id = "80865AC8", .driver_data = (unsigned long)&pwm_lpss_bxt_info },
    { }
    };
    MODULE_DEVICE_TABLE(acpi, pwm_lpss_acpi_match);
    static struct platform_driver pwm_lpss_driver_platform = {
    .driver = {
    .name = "pwm-lpss",
    .acpi_match_table = pwm_lpss_acpi_match,
    },
    .probe = pwm_lpss_probe_platform,
    };
    module_platform_driver(pwm_lpss_driver_platform);
    MODULE_DESCRIPTION("PWM platform driver for Intel LPSS");
    MODULE_LICENSE("GPL v2");
    MODULE_IMPORT_NS("PWM_LPSS");
    MODULE_ALIAS("platform:pwm-lpss");
