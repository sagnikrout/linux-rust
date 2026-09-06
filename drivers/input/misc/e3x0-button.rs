//! Automatically rewritten from C to Rust
//! Source: drivers/input/misc/e3x0-button.c
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
// Copyright (c) 2014, National Instruments Corp. All rights reserved.
//
// Driver for NI Ettus Research USRP E3x0 Button Driver
//

#[no_mangle]
unsafe extern "C" fn e3x0_button_release_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t e3x0_button_release_handler(int irq, void *data)
    {
    struct input_dev *idev = data;
    input_report_key(idev, KEY_POWER, 0);
    input_sync(idev);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn e3x0_button_press_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t e3x0_button_press_handler(int irq, void *data)
    {
    struct input_dev *idev = data;
    input_report_key(idev, KEY_POWER, 1);
    pm_wakeup_event(idev.dev.parent, 0);
    input_sync(idev);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn e3x0_button_suspend(dev: *mut device) -> c_int {
    static int e3x0_button_suspend(struct device *dev)
    {
    struct platform_device *pdev = to_platform_device(dev);
    if (device_may_wakeup(dev))
    enable_irq_wake(platform_get_irq_byname(pdev, "press"));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn e3x0_button_resume(dev: *mut device) -> c_int {
    static int e3x0_button_resume(struct device *dev)
    {
    struct platform_device *pdev = to_platform_device(dev);
    if (device_may_wakeup(dev))
    disable_irq_wake(platform_get_irq_byname(pdev, "press"));
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(e3x0_button_pm_ops,
    e3x0_button_suspend, e3x0_button_resume);
#[no_mangle]
unsafe extern "C" fn e3x0_button_probe(pdev: *mut platform_device) -> c_int {
    static int e3x0_button_probe(struct platform_device *pdev)
    {
    struct input_dev *input;
    int irq_press, irq_release;
    int error;
    irq_press = platform_get_irq_byname(pdev, "press");
    if (irq_press < 0)
    return irq_press;
    irq_release = platform_get_irq_byname(pdev, "release");
    if (irq_release < 0)
    return irq_release;
    input = devm_input_allocate_device(&pdev.dev);
    if (!input)
    return -ENOMEM;
    input.name = "NI Ettus Research USRP E3x0 Button Driver";
    input.phys = "e3x0_button/input0";
    input.dev.parent = &pdev.dev;
    input_set_capability(input, EV_KEY, KEY_POWER);
    error = devm_request_irq(&pdev.dev, irq_press,
    e3x0_button_press_handler, 0,
    "e3x0-button", input);
    if (error) {
    dev_err(&pdev.dev, "Failed to request 'press' IRQ#%d: %d\n",
    irq_press, error);
    return error;
    }
    error = devm_request_irq(&pdev.dev, irq_release,
    e3x0_button_release_handler, 0,
    "e3x0-button", input);
    if (error) {
    dev_err(&pdev.dev, "Failed to request 'release' IRQ#%d: %d\n",
    irq_release, error);
    return error;
    }
    error = input_register_device(input);
    if (error) {
    dev_err(&pdev.dev, "Can't register input device: %d\n", error);
    return error;
    }
    device_init_wakeup(&pdev.dev, 1);
    return 0;
    }

    static const struct of_device_id e3x0_button_match[] = {
    { .compatible = "ettus,e3x0-button", },
    { }
    };
    MODULE_DEVICE_TABLE(of, e3x0_button_match);

    static struct platform_driver e3x0_button_driver = {
    .driver		= {
    .name	= "e3x0-button",
    .of_match_table = of_match_ptr(e3x0_button_match),
    .pm	= pm_sleep_ptr(&e3x0_button_pm_ops),
    },
    .probe		= e3x0_button_probe,
    };
    module_platform_driver(e3x0_button_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Moritz Fischer <moritz.fischer@ettus.com>");
    MODULE_DESCRIPTION("NI Ettus Research USRP E3x0 Button driver");
    MODULE_ALIAS("platform:e3x0-button");
