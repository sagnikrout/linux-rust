//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/intel/mrfld_pwrbtn.c
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
// Power-button driver for Basin Cove PMIC
//
// Copyright (c) 2019, Intel Corporation.
// Author: Andy Shevchenko <andriy.shevchenko@linux.intel.com>
//

pub const BCOVE_PBSTATUS: c_uint = 0x27;

#[no_mangle]
unsafe extern "C" fn mrfld_pwrbtn_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t mrfld_pwrbtn_interrupt(int irq, void *dev_id)
    {
    struct input_dev *input = dev_id;
    struct device *dev = input.dev.parent;
    struct regmap *regmap = dev_get_drvdata(dev);
    unsigned int state;
    int ret;
    ret = regmap_read(regmap, BCOVE_PBSTATUS, &state);
    if (ret)
    return IRQ_NONE;
    dev_dbg(dev, "PBSTATUS=0x%x\n", state);
    input_report_key(input, KEY_POWER, !(state & BCOVE_PBSTATUS_PBLVL));
    input_sync(input);
    regmap_update_bits(regmap, BCOVE_MIRQLVL1, BCOVE_LVL1_PWRBTN, 0);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn mrfld_pwrbtn_probe(pdev: *mut platform_device) -> c_int {
    static int mrfld_pwrbtn_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct intel_soc_pmic *pmic = dev_get_drvdata(dev.parent);
    struct input_dev *input;
    int irq, ret;
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    input = devm_input_allocate_device(dev);
    if (!input)
    return -ENOMEM;
    input.name = pdev.name;
    input.phys = "power-button/input0";
    input.id.bustype = BUS_HOST;
    input.dev.parent = dev;
    input_set_capability(input, EV_KEY, KEY_POWER);
    ret = input_register_device(input);
    if (ret)
    return ret;
    dev_set_drvdata(dev, pmic.regmap);
    ret = devm_request_threaded_irq(dev, irq, core::ptr::null_mut(), mrfld_pwrbtn_interrupt,
    IRQF_ONESHOT | IRQF_SHARED, pdev.name,
    input);
    if (ret)
    return ret;
    regmap_update_bits(pmic.regmap, BCOVE_MIRQLVL1, BCOVE_LVL1_PWRBTN, 0);
    regmap_update_bits(pmic.regmap, BCOVE_MPBIRQ, BCOVE_PBIRQ_PBTN, 0);
    device_init_wakeup(dev, true);
    dev_pm_set_wake_irq(dev, irq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mrfld_pwrbtn_remove(pdev: *mut platform_device) {
    static void mrfld_pwrbtn_remove(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    dev_pm_clear_wake_irq(dev);
    device_init_wakeup(dev, false);
    }
    static const struct platform_device_id mrfld_pwrbtn_id_table[] = {
    { .name = "mrfld_bcove_pwrbtn" },
    {}
    };
    MODULE_DEVICE_TABLE(platform, mrfld_pwrbtn_id_table);
    static struct platform_driver mrfld_pwrbtn_driver = {
    .driver = {
    .name	= "mrfld_bcove_pwrbtn",
    },
    .probe		= mrfld_pwrbtn_probe,
    .remove		= mrfld_pwrbtn_remove,
    .id_table	= mrfld_pwrbtn_id_table,
    };
    module_platform_driver(mrfld_pwrbtn_driver);
    MODULE_DESCRIPTION("Power-button driver for Basin Cove PMIC");
    MODULE_LICENSE("GPL v2");
