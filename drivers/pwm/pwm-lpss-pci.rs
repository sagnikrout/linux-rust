//! Automatically rewritten from C to Rust
//! Source: drivers/pwm/pwm-lpss-pci.c
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
// Intel Low Power Subsystem PWM controller PCI driver
//
// Copyright (C) 2014, Intel Corporation
//
// Derived from the original pwm-lpss.c
//

    static int pwm_lpss_probe_pci(struct pci_dev *pdev,
    const struct pci_device_id *id)
    {
    const struct pwm_lpss_boardinfo *info;
    void __iomem *io_base;
    struct pwm_chip *chip;
    int err;
    err = pcim_enable_device(pdev);
    if (err < 0)
    return err;
    io_base = pcim_iomap_region(pdev, 0, "pwm-lpss");
    if (IS_ERR(io_base))
    return PTR_ERR(io_base);
    info = (struct pwm_lpss_boardinfo *)id.driver_data;
    chip = devm_pwm_lpss_probe(&pdev.dev, io_base, info);
    if (IS_ERR(chip))
    return PTR_ERR(chip);
    pm_runtime_put(&pdev.dev);
    pm_runtime_allow(&pdev.dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pwm_lpss_remove_pci(pdev: *mut pci_dev) {
    static void pwm_lpss_remove_pci(struct pci_dev *pdev)
    {
    pm_runtime_forbid(&pdev.dev);
    pm_runtime_get_sync(&pdev.dev);
    }
    static const struct pci_device_id pwm_lpss_pci_ids[] = {
    { PCI_VDEVICE(INTEL, 0x0ac8), .driver_data = (unsigned long)&pwm_lpss_bxt_info },
    { PCI_VDEVICE(INTEL, 0x0f08), .driver_data = (unsigned long)&pwm_lpss_byt_info },
    { PCI_VDEVICE(INTEL, 0x0f09), .driver_data = (unsigned long)&pwm_lpss_byt_info },
    { PCI_VDEVICE(INTEL, 0x11a5), .driver_data = (unsigned long)&pwm_lpss_tng_info },
    { PCI_VDEVICE(INTEL, 0x1ac8), .driver_data = (unsigned long)&pwm_lpss_bxt_info },
    { PCI_VDEVICE(INTEL, 0x2288), .driver_data = (unsigned long)&pwm_lpss_bsw_info },
    { PCI_VDEVICE(INTEL, 0x2289), .driver_data = (unsigned long)&pwm_lpss_bsw_info },
    { PCI_VDEVICE(INTEL, 0x31c8), .driver_data = (unsigned long)&pwm_lpss_bxt_info },
    { PCI_VDEVICE(INTEL, 0x5ac8), .driver_data = (unsigned long)&pwm_lpss_bxt_info },
    { }
    };
    MODULE_DEVICE_TABLE(pci, pwm_lpss_pci_ids);
    static struct pci_driver pwm_lpss_driver_pci = {
    .name = "pwm-lpss",
    .id_table = pwm_lpss_pci_ids,
    .probe = pwm_lpss_probe_pci,
    .remove = pwm_lpss_remove_pci,
    };
    module_pci_driver(pwm_lpss_driver_pci);
    MODULE_DESCRIPTION("PWM PCI driver for Intel LPSS");
    MODULE_LICENSE("GPL v2");
    MODULE_IMPORT_NS("PWM_LPSS");
