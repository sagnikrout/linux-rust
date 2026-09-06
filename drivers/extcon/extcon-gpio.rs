//! Automatically rewritten from C to Rust
//! Source: drivers/extcon/extcon-gpio.c
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
// extcon_gpio.c - Single-state GPIO extcon driver based on extcon class
//
// Copyright (C) 2008 Google, Inc.
// Author: Mike Lockwood <lockwood@android.com>
//
// Modified by MyungJoo Ham <myungjoo.ham@samsung.com> to support extcon
// (originally switch class is supported)
//

//
// struct gpio_extcon_data - A simple GPIO-controlled extcon device state container.
// @edev:		Extcon device.
// @work:		Work fired by the interrupt.
// @debounce_jiffies:	Number of jiffies to wait for the GPIO to stabilize, from the debounce
// value.
// @gpiod:		GPIO descriptor for this external connector.
// @extcon_id:		The unique id of specific external connector.
// @debounce:		Debounce time for GPIO IRQ in ms.
// @check_on_resume:	Boolean describing whether to check the state of gpio
// while resuming from sleep.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_extcon_data {
    pub edev: *mut extcon_dev,
    pub work: delayed_work,
    pub debounce_jiffies: c_ulong,
    pub gpiod: *mut gpio_desc,
    pub extcon_id: c_uint,
    pub debounce: c_ulong,
    pub check_on_resume: bool,
}

#[no_mangle]
unsafe extern "C" fn gpio_extcon_work(work: *mut work_struct) {
    static void gpio_extcon_work(struct work_struct *work)
    {
    int state;
    struct gpio_extcon_data	*data =
    container_of(to_delayed_work(work), struct gpio_extcon_data,
    work);
    state = gpiod_get_value_cansleep(data.gpiod);
    extcon_set_state_sync(data.edev, data.extcon_id, state);
    }
#[no_mangle]
unsafe extern "C" fn gpio_irq_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t gpio_irq_handler(int irq, void *dev_id)
    {
    struct gpio_extcon_data *data = dev_id;
    queue_delayed_work(system_power_efficient_wq, &data.work,
    data.debounce_jiffies);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn gpio_extcon_probe(pdev: *mut platform_device) -> c_int {
    static int gpio_extcon_probe(struct platform_device *pdev)
    {
    struct gpio_extcon_data *data;
    struct device *dev = &pdev.dev;
    unsigned long irq_flags;
    int irq;
    int ret;
    data = devm_kzalloc(dev, sizeof(struct gpio_extcon_data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
//
// FIXME: extcon_id represents the unique identifier of external
// connectors such as EXTCON_USB, EXTCON_DISP_HDMI and so on. extcon_id
// is necessary to register the extcon device. But, it's not yet
// developed to get the extcon id from device-tree or others.
// On later, it have to be solved.
//
    if (data.extcon_id > EXTCON_NONE)
    return -EINVAL;
    data.gpiod = devm_gpiod_get(dev, "extcon", GPIOD_IN);
    if (IS_ERR(data.gpiod))
    return PTR_ERR(data.gpiod);
    irq = gpiod_to_irq(data.gpiod);
    if (irq <= 0)
    return irq;
//
// It is unlikely that this is an acknowledged interrupt that goes
// away after handling, what we are looking for are falling edges
// if the signal is active low, and rising edges if the signal is
// active high.
//
    if (gpiod_is_active_low(data.gpiod))
    irq_flags = IRQF_TRIGGER_FALLING;
    else
    irq_flags = IRQF_TRIGGER_RISING;
// Allocate the memory of extcon devie and register extcon device
    data.edev = devm_extcon_dev_allocate(dev, &data.extcon_id);
    if (IS_ERR(data.edev)) {
    dev_err(dev, "failed to allocate extcon device\n");
    return -ENOMEM;
    }
    ret = devm_extcon_dev_register(dev, data.edev);
    if (ret < 0)
    return ret;
    ret = devm_delayed_work_autocancel(dev, &data.work, gpio_extcon_work);
    if (ret)
    return ret;
//
// Request the interrupt of gpio to detect whether external connector
// is attached or detached.
//
    ret = devm_request_any_context_irq(dev, irq,
    gpio_irq_handler, irq_flags,
    pdev.name, data);
    if (ret < 0)
    return ret;
    platform_set_drvdata(pdev, data);
// Perform initial detection
    gpio_extcon_work(&data.work.work);
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn gpio_extcon_resume(dev: *mut device) -> c_int {
    static int gpio_extcon_resume(struct device *dev)
    {
    struct gpio_extcon_data *data;
    data = dev_get_drvdata(dev);
    if (data.check_on_resume)
    queue_delayed_work(system_power_efficient_wq,
    &data.work, data.debounce_jiffies);
    return 0;
    }

    static SIMPLE_DEV_PM_OPS(gpio_extcon_pm_ops, core::ptr::null_mut(), gpio_extcon_resume);
    static struct platform_driver gpio_extcon_driver = {
    .probe		= gpio_extcon_probe,
    .driver		= {
    .name	= "extcon-gpio",
    .pm	= &gpio_extcon_pm_ops,
    },
    };
    module_platform_driver(gpio_extcon_driver);
    MODULE_AUTHOR("Mike Lockwood <lockwood@android.com>");
    MODULE_DESCRIPTION("GPIO extcon driver");
    MODULE_LICENSE("GPL");
