//! Automatically rewritten from C to Rust
//! Source: drivers/input/misc/stpmic1_onkey.c
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
// Copyright (C) STMicroelectronics 2018
// Author: Pascal Paillet <p.paillet@st.com> for STMicroelectronics.

//
// struct stpmic1_onkey - OnKey data
// @input_dev:		pointer to input device
// @irq_falling:	irq that we are hooked on to
// @irq_rising:		irq that we are hooked on to
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stpmic1_onkey {
    pub input_dev: *mut input_dev,
    pub irq_falling: c_int,
    pub irq_rising: c_int,
}

#[no_mangle]
unsafe extern "C" fn onkey_falling_irq(irq: c_int, ponkey: *mut c_void) -> irqreturn_t {
    static irqreturn_t onkey_falling_irq(int irq, void *ponkey)
    {
    struct stpmic1_onkey *onkey = ponkey;
    struct input_dev *input_dev = onkey.input_dev;
    input_report_key(input_dev, KEY_POWER, 1);
    pm_wakeup_event(input_dev.dev.parent, 0);
    input_sync(input_dev);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn onkey_rising_irq(irq: c_int, ponkey: *mut c_void) -> irqreturn_t {
    static irqreturn_t onkey_rising_irq(int irq, void *ponkey)
    {
    struct stpmic1_onkey *onkey = ponkey;
    struct input_dev *input_dev = onkey.input_dev;
    input_report_key(input_dev, KEY_POWER, 0);
    pm_wakeup_event(input_dev.dev.parent, 0);
    input_sync(input_dev);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn stpmic1_onkey_probe(pdev: *mut platform_device) -> c_int {
    static int stpmic1_onkey_probe(struct platform_device *pdev)
    {
    struct stpmic1 *pmic = dev_get_drvdata(pdev.dev.parent);
    struct device *dev = &pdev.dev;
    struct input_dev *input_dev;
    struct stpmic1_onkey *onkey;
    unsigned int val, reg = 0;
    int error;
    onkey = devm_kzalloc(dev, sizeof(*onkey), GFP_KERNEL);
    if (!onkey)
    return -ENOMEM;
    onkey.irq_falling = platform_get_irq_byname(pdev, "onkey-falling");
    if (onkey.irq_falling < 0)
    return onkey.irq_falling;
    onkey.irq_rising = platform_get_irq_byname(pdev, "onkey-rising");
    if (onkey.irq_rising < 0)
    return onkey.irq_rising;
    if (!device_property_read_u32(dev, "power-off-time-sec", &val)) {
    if (val > 0 && val <= 16) {
    dev_dbg(dev, "power-off-time=%d seconds\n", val);
    reg |= PONKEY_PWR_OFF;
    reg |= ((16 - val) & PONKEY_TURNOFF_TIMER_MASK);
    } else {
    dev_err(dev, "power-off-time-sec out of range\n");
    return -EINVAL;
    }
    }
    if (device_property_present(dev, "st,onkey-clear-cc-flag"))
    reg |= PONKEY_CC_FLAG_CLEAR;
    error = regmap_update_bits(pmic.regmap, PKEY_TURNOFF_CR,
    PONKEY_TURNOFF_MASK, reg);
    if (error) {
    dev_err(dev, "PKEY_TURNOFF_CR write failed: %d\n", error);
    return error;
    }
    if (device_property_present(dev, "st,onkey-pu-inactive")) {
    error = regmap_update_bits(pmic.regmap, PADS_PULL_CR,
    PONKEY_PU_INACTIVE,
    PONKEY_PU_INACTIVE);
    if (error) {
    dev_err(dev, "ONKEY Pads configuration failed: %d\n",
    error);
    return error;
    }
    }
    input_dev = devm_input_allocate_device(dev);
    if (!input_dev) {
    dev_err(dev, "Can't allocate Pwr Onkey Input Device\n");
    return -ENOMEM;
    }
    input_dev.name = "pmic_onkey";
    input_dev.phys = "pmic_onkey/input0";
    input_set_capability(input_dev, EV_KEY, KEY_POWER);
    onkey.input_dev = input_dev;
// interrupt is nested in a thread
    error = devm_request_threaded_irq(dev, onkey.irq_falling, core::ptr::null_mut(),
    onkey_falling_irq, IRQF_ONESHOT,
    dev_name(dev), onkey);
    if (error) {
    dev_err(dev, "Can't get IRQ Onkey Falling: %d\n", error);
    return error;
    }
    error = devm_request_threaded_irq(dev, onkey.irq_rising, core::ptr::null_mut(),
    onkey_rising_irq, IRQF_ONESHOT,
    dev_name(dev), onkey);
    if (error) {
    dev_err(dev, "Can't get IRQ Onkey Rising: %d\n", error);
    return error;
    }
    error = input_register_device(input_dev);
    if (error) {
    dev_err(dev, "Can't register power button: %d\n", error);
    return error;
    }
    platform_set_drvdata(pdev, onkey);
    device_init_wakeup(dev, true);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stpmic1_onkey_suspend(dev: *mut device) -> c_int {
    static int stpmic1_onkey_suspend(struct device *dev)
    {
    struct platform_device *pdev = to_platform_device(dev);
    struct stpmic1_onkey *onkey = platform_get_drvdata(pdev);
    if (device_may_wakeup(dev)) {
    enable_irq_wake(onkey.irq_falling);
    enable_irq_wake(onkey.irq_rising);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stpmic1_onkey_resume(dev: *mut device) -> c_int {
    static int stpmic1_onkey_resume(struct device *dev)
    {
    struct platform_device *pdev = to_platform_device(dev);
    struct stpmic1_onkey *onkey = platform_get_drvdata(pdev);
    if (device_may_wakeup(dev)) {
    disable_irq_wake(onkey.irq_falling);
    disable_irq_wake(onkey.irq_rising);
    }
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(stpmic1_onkey_pm,
    stpmic1_onkey_suspend,
    stpmic1_onkey_resume);
    static const struct of_device_id of_stpmic1_onkey_match[] = {
    { .compatible = "st,stpmic1-onkey" },
    { },
    };
    MODULE_DEVICE_TABLE(of, of_stpmic1_onkey_match);
    static struct platform_driver stpmic1_onkey_driver = {
    .probe	= stpmic1_onkey_probe,
    .driver	= {
    .name	= "stpmic1_onkey",
    .of_match_table = of_match_ptr(of_stpmic1_onkey_match),
    .pm	= pm_sleep_ptr(&stpmic1_onkey_pm),
    },
    };
    module_platform_driver(stpmic1_onkey_driver);
    MODULE_DESCRIPTION("Onkey driver for STPMIC1");
    MODULE_AUTHOR("Pascal Paillet <p.paillet@st.com>");
    MODULE_LICENSE("GPL v2");
