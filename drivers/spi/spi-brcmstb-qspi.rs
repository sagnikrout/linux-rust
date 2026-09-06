//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-brcmstb-qspi.c
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
// Copyright 2016 Broadcom
//

    static const struct of_device_id brcmstb_qspi_of_match[] = {
    { .compatible = "brcm,spi-brcmstb-qspi" },
    { .compatible = "brcm,spi-brcmstb-mspi" },
    {},
    };
    MODULE_DEVICE_TABLE(of, brcmstb_qspi_of_match);
#[no_mangle]
unsafe extern "C" fn brcmstb_qspi_probe(pdev: *mut platform_device) -> c_int {
    static int brcmstb_qspi_probe(struct platform_device *pdev)
    {
    return bcm_qspi_probe(pdev, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn brcmstb_qspi_remove(pdev: *mut platform_device) {
    static void brcmstb_qspi_remove(struct platform_device *pdev)
    {
    bcm_qspi_remove(pdev);
    }
    static struct platform_driver brcmstb_qspi_driver = {
    .probe			= brcmstb_qspi_probe,
    .remove			= brcmstb_qspi_remove,
    .driver = {
    .name		= "brcmstb_qspi",
    .pm		= pm_sleep_ptr(&bcm_qspi_pm_ops),
    .of_match_table = brcmstb_qspi_of_match,
    }
    };
    module_platform_driver(brcmstb_qspi_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Kamal Dasu");
    MODULE_DESCRIPTION("Broadcom SPI driver for settop SoC");
