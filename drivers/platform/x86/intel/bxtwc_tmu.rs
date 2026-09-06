//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/intel/bxtwc_tmu.c
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
// Intel BXT Whiskey Cove PMIC TMU driver
//
// Copyright (C) 2016 Intel Corporation. All rights reserved.
//
// This driver adds TMU (Time Management Unit) support for Intel BXT platform.
// It enables the alarm wake-up functionality in the TMU unit of Whiskey Cove
// PMIC.
//

pub const BXTWC_TMUIRQ: c_uint = 0x4fb6;
pub const BXTWC_MIRQLVL1: c_uint = 0x4e0e;
pub const BXTWC_MTMUIRQ_REG: c_uint = 0x4fb7;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcove_tmu {
    pub irq: c_int,
    pub dev: *mut device,
    pub regmap: *mut regmap,
}

#[no_mangle]
unsafe extern "C" fn bxt_wcove_tmu_irq_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t bxt_wcove_tmu_irq_handler(int irq, void *data)
    {
    struct wcove_tmu *wctmu = data;
    unsigned int tmu_irq;
// Read TMU interrupt reg
    regmap_read(wctmu.regmap, BXTWC_TMUIRQ, &tmu_irq);
    if (tmu_irq & BXTWC_TMU_ALRM_IRQ) {
// clear TMU irq
    regmap_write(wctmu.regmap, BXTWC_TMUIRQ, tmu_irq);
    return IRQ_HANDLED;
    }
    return IRQ_NONE;
    }
#[no_mangle]
unsafe extern "C" fn bxt_wcove_tmu_probe(pdev: *mut platform_device) -> c_int {
    static int bxt_wcove_tmu_probe(struct platform_device *pdev)
    {
    struct intel_soc_pmic *pmic = dev_get_drvdata(pdev.dev.parent);
    struct wcove_tmu *wctmu;
    int ret;
    wctmu = devm_kzalloc(&pdev.dev, sizeof(*wctmu), GFP_KERNEL);
    if (!wctmu)
    return -ENOMEM;
    wctmu.dev = &pdev.dev;
    wctmu.regmap = pmic.regmap;
    wctmu.irq = platform_get_irq(pdev, 0);
    if (wctmu.irq < 0)
    return wctmu.irq;
    ret = devm_request_threaded_irq(&pdev.dev, wctmu.irq,
    core::ptr::null_mut(), bxt_wcove_tmu_irq_handler,
    IRQF_ONESHOT, "bxt_wcove_tmu", wctmu);
    if (ret) {
    dev_err(&pdev.dev, "request irq failed: %d,virq: %d\n",
    ret, wctmu.irq);
    return ret;
    }
// Unmask TMU second level Wake & System alarm
    regmap_update_bits(wctmu.regmap, BXTWC_MTMUIRQ_REG,
    BXTWC_TMU_ALRM_MASK, 0);
    platform_set_drvdata(pdev, wctmu);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bxt_wcove_tmu_remove(pdev: *mut platform_device) {
    static void bxt_wcove_tmu_remove(struct platform_device *pdev)
    {
    struct wcove_tmu *wctmu = platform_get_drvdata(pdev);
    unsigned int val;
// Mask TMU interrupts
    regmap_read(wctmu.regmap, BXTWC_MIRQLVL1, &val);
    regmap_write(wctmu.regmap, BXTWC_MIRQLVL1,
    val | BXTWC_MIRQLVL1_MTMU);
    regmap_read(wctmu.regmap, BXTWC_MTMUIRQ_REG, &val);
    regmap_write(wctmu.regmap, BXTWC_MTMUIRQ_REG,
    val | BXTWC_TMU_ALRM_MASK);
    }

#[no_mangle]
unsafe extern "C" fn bxtwc_tmu_suspend(dev: *mut device) -> c_int {
    static int bxtwc_tmu_suspend(struct device *dev)
    {
    struct wcove_tmu *wctmu = dev_get_drvdata(dev);
    enable_irq_wake(wctmu.irq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bxtwc_tmu_resume(dev: *mut device) -> c_int {
    static int bxtwc_tmu_resume(struct device *dev)
    {
    struct wcove_tmu *wctmu = dev_get_drvdata(dev);
    disable_irq_wake(wctmu.irq);
    return 0;
    }

    static SIMPLE_DEV_PM_OPS(bxtwc_tmu_pm_ops, bxtwc_tmu_suspend, bxtwc_tmu_resume);
    static const struct platform_device_id bxt_wcove_tmu_id_table[] = {
    { .name = "bxt_wcove_tmu" },
    {},
    };
    MODULE_DEVICE_TABLE(platform, bxt_wcove_tmu_id_table);
    static struct platform_driver bxt_wcove_tmu_driver = {
    .probe = bxt_wcove_tmu_probe,
    .remove = bxt_wcove_tmu_remove,
    .driver = {
    .name = "bxt_wcove_tmu",
    .pm     = &bxtwc_tmu_pm_ops,
    },
    .id_table = bxt_wcove_tmu_id_table,
    };
    module_platform_driver(bxt_wcove_tmu_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Nilesh Bacchewar <nilesh.bacchewar@intel.com>");
    MODULE_DESCRIPTION("BXT Whiskey Cove TMU Driver");
