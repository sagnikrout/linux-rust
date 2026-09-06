//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/atmel-flexcom.c
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
// Driver for Atmel Flexcom
//
// Copyright (C) 2015 Atmel Corporation
//
// Author: Cyrille Pitchen <cyrille.pitchen@atmel.com>
//

// I/O register offsets
pub const FLEX_MR: c_uint = 0x0	/* Mode Register */;
pub const FLEX_VERSION: c_uint = 0xfc	/* Version Register */;
// Mode Register bit fields

    FLEX_MR_OPMODE_MASK)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmel_flexcom {
    pub base: *mut void __iomem,
    pub opmode: u32,
    pub clk: *mut clk,
}

#[no_mangle]
unsafe extern "C" fn atmel_flexcom_probe(pdev: *mut platform_device) -> c_int {
    static int atmel_flexcom_probe(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    struct atmel_flexcom *ddata;
    int err;
    ddata = devm_kzalloc(&pdev.dev, sizeof(*ddata), GFP_KERNEL);
    if (!ddata)
    return -ENOMEM;
    platform_set_drvdata(pdev, ddata);
    err = of_property_read_u32(np, "atmel,flexcom-mode", &ddata.opmode);
    if (err)
    return err;
    if (ddata.opmode < ATMEL_FLEXCOM_MODE_USART ||
    ddata.opmode > ATMEL_FLEXCOM_MODE_TWI)
    return -EINVAL;
    ddata.base = devm_platform_get_and_ioremap_resource(pdev, 0, core::ptr::null_mut());
    if (IS_ERR(ddata.base))
    return PTR_ERR(ddata.base);
    ddata.clk = devm_clk_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(ddata.clk))
    return PTR_ERR(ddata.clk);
    err = clk_prepare_enable(ddata.clk);
    if (err)
    return err;
//
// Set the Operating Mode in the Mode Register: only the selected device
// is clocked. Hence, registers of the other serial devices remain
// inaccessible and are read as zero. Also the external I/O lines of the
// Flexcom are muxed to reach the selected device.
//
    writel(FLEX_MR_OPMODE(ddata.opmode), ddata.base + FLEX_MR);
    clk_disable_unprepare(ddata.clk);
    return devm_of_platform_populate(&pdev.dev);
    }
    static const struct of_device_id atmel_flexcom_of_match[] = {
    { .compatible = "atmel,sama5d2-flexcom" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, atmel_flexcom_of_match);
#[no_mangle]
unsafe extern "C" fn atmel_flexcom_resume_noirq(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused atmel_flexcom_resume_noirq(struct device *dev)
    {
    struct atmel_flexcom *ddata = dev_get_drvdata(dev);
    int err;
    u32 val;
    err = clk_prepare_enable(ddata.clk);
    if (err)
    return err;
    val = FLEX_MR_OPMODE(ddata.opmode);
    writel(val, ddata.base + FLEX_MR);
    clk_disable_unprepare(ddata.clk);
    return 0;
    }
    static const struct dev_pm_ops __maybe_unused atmel_flexcom_pm_ops = {
    .resume_noirq = atmel_flexcom_resume_noirq,
    };
    static struct platform_driver atmel_flexcom_driver = {
    .probe	= atmel_flexcom_probe,
    .driver	= {
    .name		= "atmel_flexcom",
    .pm		= pm_ptr(&atmel_flexcom_pm_ops),
    .of_match_table	= atmel_flexcom_of_match,
    },
    };
    module_platform_driver(atmel_flexcom_driver);
    MODULE_AUTHOR("Cyrille Pitchen <cyrille.pitchen@atmel.com>");
    MODULE_DESCRIPTION("Atmel Flexcom MFD driver");
    MODULE_LICENSE("GPL v2");
