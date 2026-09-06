//! Automatically rewritten from C to Rust
//! Source: drivers/misc/tps6594-esm.c
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
// ESM (Error Signal Monitor) driver for TI TPS6594/TPS6593/LP8764 PMICs
//
// Copyright (C) 2023 BayLibre Incorporated - https://www.baylibre.com
//

pub const TPS6594_DEV_REV_1: c_uint = 0x08;
#[no_mangle]
unsafe extern "C" fn tps6594_esm_isr(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t tps6594_esm_isr(int irq, void *dev_id)
    {
    struct platform_device *pdev = dev_id;
    int i;
    for (i = 0 ; i < pdev.num_resources ; i++) {
    if (irq == platform_get_irq_byname(pdev, pdev.resource[i].name)) {
    dev_err(pdev.dev.parent, "%s error detected\n", pdev.resource[i].name);
    return IRQ_HANDLED;
    }
    }
    return IRQ_NONE;
    }
#[no_mangle]
unsafe extern "C" fn tps6594_esm_probe(pdev: *mut platform_device) -> c_int {
    static int tps6594_esm_probe(struct platform_device *pdev)
    {
    struct tps6594 *tps = dev_get_drvdata(pdev.dev.parent);
    struct device *dev = &pdev.dev;
    unsigned int rev;
    int irq;
    int ret;
    int i;
//
// Due to a bug in revision 1 of the PMIC, the GPIO3 used for the
// SoC ESM function is used to power the load switch instead.
// As a consequence, ESM can not be used on those PMIC.
// Check the version and return an error in case of revision 1.
//
    ret = regmap_read(tps.regmap, TPS6594_REG_DEV_REV, &rev);
    if (ret)
    return dev_err_probe(dev, ret,
    "Failed to read PMIC revision\n");
    if (rev == TPS6594_DEV_REV_1)
    return dev_err_probe(dev, -ENODEV,
    "ESM not supported for revision 1 PMIC\n");
    for (i = 0; i < pdev.num_resources; i++) {
    irq = platform_get_irq_byname(pdev, pdev.resource[i].name);
    if (irq < 0)
    return irq;
    ret = devm_request_threaded_irq(dev, irq, core::ptr::null_mut(),
    tps6594_esm_isr, IRQF_ONESHOT,
    pdev.resource[i].name, pdev);
    if (ret)
    return ret;
    }
    ret = regmap_set_bits(tps.regmap, TPS6594_REG_ESM_SOC_MODE_CFG,
    TPS6594_BIT_ESM_SOC_EN | TPS6594_BIT_ESM_SOC_ENDRV);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to configure ESM\n");
    ret = regmap_set_bits(tps.regmap, TPS6594_REG_ESM_SOC_START_REG,
    TPS6594_BIT_ESM_SOC_START);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to start ESM\n");
    pm_runtime_enable(dev);
    pm_runtime_get_sync(dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tps6594_esm_remove(pdev: *mut platform_device) {
    static void tps6594_esm_remove(struct platform_device *pdev)
    {
    struct tps6594 *tps = dev_get_drvdata(pdev.dev.parent);
    struct device *dev = &pdev.dev;
    int ret;
    ret = regmap_clear_bits(tps.regmap, TPS6594_REG_ESM_SOC_START_REG,
    TPS6594_BIT_ESM_SOC_START);
    if (ret) {
    dev_err(dev, "Failed to stop ESM\n");
    goto out;
    }
    ret = regmap_clear_bits(tps.regmap, TPS6594_REG_ESM_SOC_MODE_CFG,
    TPS6594_BIT_ESM_SOC_EN | TPS6594_BIT_ESM_SOC_ENDRV);
    if (ret)
    dev_err(dev, "Failed to unconfigure ESM\n");
    out:
    pm_runtime_put_sync(dev);
    pm_runtime_disable(dev);
    }
#[no_mangle]
unsafe extern "C" fn tps6594_esm_suspend(dev: *mut device) -> c_int {
    static int tps6594_esm_suspend(struct device *dev)
    {
    struct tps6594 *tps = dev_get_drvdata(dev.parent);
    int ret;
    ret = regmap_clear_bits(tps.regmap, TPS6594_REG_ESM_SOC_START_REG,
    TPS6594_BIT_ESM_SOC_START);
    pm_runtime_put_sync(dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn tps6594_esm_resume(dev: *mut device) -> c_int {
    static int tps6594_esm_resume(struct device *dev)
    {
    struct tps6594 *tps = dev_get_drvdata(dev.parent);
    pm_runtime_get_sync(dev);
    return regmap_set_bits(tps.regmap, TPS6594_REG_ESM_SOC_START_REG,
    TPS6594_BIT_ESM_SOC_START);
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(tps6594_esm_pm_ops, tps6594_esm_suspend, tps6594_esm_resume);
    static struct platform_driver tps6594_esm_driver = {
    .driver	= {
    .name = "tps6594-esm",
    .pm = pm_sleep_ptr(&tps6594_esm_pm_ops),
    },
    .probe = tps6594_esm_probe,
    .remove = tps6594_esm_remove,
    };
    module_platform_driver(tps6594_esm_driver);
    MODULE_ALIAS("platform:tps6594-esm");
    MODULE_AUTHOR("Julien Panis <jpanis@baylibre.com>");
    MODULE_DESCRIPTION("TPS6594 Error Signal Monitor Driver");
    MODULE_LICENSE("GPL");
