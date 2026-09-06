//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/intel/chtdc_ti_pwrbtn.c
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
// Power-button driver for Dollar Cove TI PMIC
// Copyright (C) 2014 Intel Corp
// Copyright (c) 2017 Takashi Iwai <tiwai@suse.de>
//

pub const CHTDC_TI_SIRQ_REG: c_uint = 0x3;

#[no_mangle]
unsafe extern "C" fn chtdc_ti_pwrbtn_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t chtdc_ti_pwrbtn_interrupt(int irq, void *dev_id)
    {
    struct input_dev *input = dev_id;
    struct device *dev = input.dev.parent;
    struct regmap *regmap = dev_get_drvdata(dev);
    int state;
    if (!regmap_read(regmap, CHTDC_TI_SIRQ_REG, &state)) {
    dev_dbg(dev, "SIRQ_REG=0x%x\n", state);
    input_report_key(input, KEY_POWER, !(state & SIRQ_PWRBTN_REL));
    input_sync(input);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn chtdc_ti_pwrbtn_probe(pdev: *mut platform_device) -> c_int {
    static int chtdc_ti_pwrbtn_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct intel_soc_pmic *pmic = dev_get_drvdata(dev.parent);
    struct input_dev *input;
    int irq, err;
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    input = devm_input_allocate_device(dev);
    if (!input)
    return -ENOMEM;
    input.name = pdev.name;
    input.phys = "power-button/input0";
    input.id.bustype = BUS_HOST;
    input_set_capability(input, EV_KEY, KEY_POWER);
    err = input_register_device(input);
    if (err)
    return err;
    dev_set_drvdata(dev, pmic.regmap);
    err = devm_request_threaded_irq(dev, irq, core::ptr::null_mut(),
    chtdc_ti_pwrbtn_interrupt,
    IRQF_ONESHOT, KBUILD_MODNAME, input);
    if (err)
    return err;
    device_init_wakeup(dev, true);
    dev_pm_set_wake_irq(dev, irq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn chtdc_ti_pwrbtn_remove(pdev: *mut platform_device) {
    static void chtdc_ti_pwrbtn_remove(struct platform_device *pdev)
    {
    dev_pm_clear_wake_irq(&pdev.dev);
    device_init_wakeup(&pdev.dev, false);
    }
    static const struct platform_device_id chtdc_ti_pwrbtn_id_table[] = {
    { .name = "chtdc_ti_pwrbtn" },
    {},
    };
    MODULE_DEVICE_TABLE(platform, chtdc_ti_pwrbtn_id_table);
    static struct platform_driver chtdc_ti_pwrbtn_driver = {
    .driver = {
    .name	= KBUILD_MODNAME,
    },
    .probe		= chtdc_ti_pwrbtn_probe,
    .remove		= chtdc_ti_pwrbtn_remove,
    .id_table	= chtdc_ti_pwrbtn_id_table,
    };
    module_platform_driver(chtdc_ti_pwrbtn_driver);
    MODULE_DESCRIPTION("Power-button driver for Dollar Cove TI PMIC");
    MODULE_LICENSE("GPL v2");
