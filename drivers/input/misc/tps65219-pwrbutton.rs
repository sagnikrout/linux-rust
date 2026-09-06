//! Automatically rewritten from C to Rust
//! Source: drivers/input/misc/tps65219-pwrbutton.c
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
// Driver for TPS65219 Push Button
//
// Copyright (C) 2022 BayLibre Incorporated - https://www.baylibre.com

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tps65219_pwrbutton {
    pub dev: *mut device,
    pub idev: *mut input_dev,
    pub phys: [c_char; 32],
}

#[no_mangle]
unsafe extern "C" fn tps65219_pb_push_irq(irq: c_int, _pwr: *mut c_void) -> irqreturn_t {
    static irqreturn_t tps65219_pb_push_irq(int irq, void *_pwr)
    {
    struct tps65219_pwrbutton *pwr = _pwr;
    input_report_key(pwr.idev, KEY_POWER, 1);
    pm_wakeup_event(pwr.dev, 0);
    input_sync(pwr.idev);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn tps65219_pb_release_irq(irq: c_int, _pwr: *mut c_void) -> irqreturn_t {
    static irqreturn_t tps65219_pb_release_irq(int irq, void *_pwr)
    {
    struct tps65219_pwrbutton *pwr = _pwr;
    input_report_key(pwr.idev, KEY_POWER, 0);
    input_sync(pwr.idev);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn tps65219_pb_probe(pdev: *mut platform_device) -> c_int {
    static int tps65219_pb_probe(struct platform_device *pdev)
    {
    struct tps65219 *tps = dev_get_drvdata(pdev.dev.parent);
    struct device *dev = &pdev.dev;
    struct tps65219_pwrbutton *pwr;
    struct input_dev *idev;
    int error;
    int push_irq;
    int release_irq;
    pwr = devm_kzalloc(dev, sizeof(*pwr), GFP_KERNEL);
    if (!pwr)
    return -ENOMEM;
    idev = devm_input_allocate_device(dev);
    if (!idev)
    return -ENOMEM;
    idev.name = pdev.name;
    snprintf(pwr.phys, sizeof(pwr.phys), "%s/input0",
    pdev.name);
    idev.phys = pwr.phys;
    idev.id.bustype = BUS_I2C;
    input_set_capability(idev, EV_KEY, KEY_POWER);
    pwr.dev = dev;
    pwr.idev = idev;
    device_init_wakeup(dev, true);
    push_irq = platform_get_irq(pdev, 0);
    if (push_irq < 0)
    return -EINVAL;
    release_irq = platform_get_irq(pdev, 1);
    if (release_irq < 0)
    return -EINVAL;
    error = devm_request_threaded_irq(dev, push_irq, core::ptr::null_mut(),
    tps65219_pb_push_irq,
    IRQF_ONESHOT,
    dev.init_name, pwr);
    if (error) {
    dev_err(dev, "failed to request push IRQ #%d: %d\n", push_irq,
    error);
    return error;
    }
    error = devm_request_threaded_irq(dev, release_irq, core::ptr::null_mut(),
    tps65219_pb_release_irq,
    IRQF_ONESHOT,
    dev.init_name, pwr);
    if (error) {
    dev_err(dev, "failed to request release IRQ #%d: %d\n",
    release_irq, error);
    return error;
    }
    error = input_register_device(idev);
    if (error) {
    dev_err(dev, "Can't register power button: %d\n", error);
    return error;
    }
// Enable interrupts for the pushbutton
    regmap_clear_bits(tps.regmap, TPS65219_REG_MASK_CONFIG,
    TPS65219_REG_MASK_INT_FOR_PB_MASK);
// Set PB/EN/VSENSE pin to be a pushbutton
    regmap_update_bits(tps.regmap, TPS65219_REG_MFP_2_CONFIG,
    TPS65219_MFP_2_EN_PB_VSENSE_MASK, TPS65219_MFP_2_PB);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tps65219_pb_remove(pdev: *mut platform_device) {
    static void tps65219_pb_remove(struct platform_device *pdev)
    {
    struct tps65219 *tps = dev_get_drvdata(pdev.dev.parent);
    int ret;
// Disable interrupt for the pushbutton
    ret = regmap_set_bits(tps.regmap, TPS65219_REG_MASK_CONFIG,
    TPS65219_REG_MASK_INT_FOR_PB_MASK);
    if (ret)
    dev_warn(&pdev.dev, "Failed to disable irq (%pe)\n", ERR_PTR(ret));
    }
    static const struct platform_device_id tps65219_pwrbtn_id_table[] = {
    { "tps65219-pwrbutton", },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(platform, tps65219_pwrbtn_id_table);
    static struct platform_driver tps65219_pb_driver = {
    .probe = tps65219_pb_probe,
    .remove = tps65219_pb_remove,
    .driver = {
    .name = "tps65219_pwrbutton",
    },
    .id_table = tps65219_pwrbtn_id_table,
    };
    module_platform_driver(tps65219_pb_driver);
    MODULE_DESCRIPTION("TPS65219 Power Button");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Markus Schneider-Pargmann <msp@baylibre.com");
