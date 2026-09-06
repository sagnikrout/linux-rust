//! Automatically rewritten from C to Rust
//! Source: drivers/extcon/extcon-max3355.c
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
// Maxim Integrated MAX3355 USB OTG chip extcon driver
//
// Copyright (C)  2014-2015 Cogent Embedded, Inc.
// Author: Sergei Shtylyov <sergei.shtylyov@cogentembedded.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct max3355_data {
    pub edev: *mut extcon_dev,
    pub id_gpiod: *mut gpio_desc,
    pub shdn_gpiod: *mut gpio_desc,
}

    static const unsigned int max3355_cable[] = {
    EXTCON_USB,
    EXTCON_USB_HOST,
    EXTCON_NONE,
    };
#[no_mangle]
unsafe extern "C" fn max3355_id_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t max3355_id_irq(int irq, void *dev_id)
    {
    struct max3355_data *data = dev_id;
    let mut id: c_int = gpiod_get_value_cansleep(data.id_gpiod);
    if (id) {
//
// ID = 1 means USB HOST cable detached.
// As we don't have event for USB peripheral cable attached,
// we simulate USB peripheral attach here.
//
    extcon_set_state_sync(data.edev, EXTCON_USB_HOST, false);
    extcon_set_state_sync(data.edev, EXTCON_USB, true);
    } else {
//
// ID = 0 means USB HOST cable attached.
// As we don't have event for USB peripheral cable detached,
// we simulate USB peripheral detach here.
//
    extcon_set_state_sync(data.edev, EXTCON_USB, false);
    extcon_set_state_sync(data.edev, EXTCON_USB_HOST, true);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn max3355_probe(pdev: *mut platform_device) -> c_int {
    static int max3355_probe(struct platform_device *pdev)
    {
    struct max3355_data *data;
    struct gpio_desc *gpiod;
    int irq, err;
    data = devm_kzalloc(&pdev.dev, sizeof(struct max3355_data),
    GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    gpiod = devm_gpiod_get(&pdev.dev, "id", GPIOD_IN);
    if (IS_ERR(gpiod)) {
    dev_err(&pdev.dev, "failed to get ID_OUT GPIO\n");
    return PTR_ERR(gpiod);
    }
    data.id_gpiod = gpiod;
    gpiod = devm_gpiod_get(&pdev.dev, "maxim,shdn", GPIOD_OUT_HIGH);
    if (IS_ERR(gpiod)) {
    dev_err(&pdev.dev, "failed to get SHDN# GPIO\n");
    return PTR_ERR(gpiod);
    }
    data.shdn_gpiod = gpiod;
    data.edev = devm_extcon_dev_allocate(&pdev.dev, max3355_cable);
    if (IS_ERR(data.edev)) {
    dev_err(&pdev.dev, "failed to allocate extcon device\n");
    return PTR_ERR(data.edev);
    }
    err = devm_extcon_dev_register(&pdev.dev, data.edev);
    if (err < 0) {
    dev_err(&pdev.dev, "failed to register extcon device\n");
    return err;
    }
    irq = gpiod_to_irq(data.id_gpiod);
    if (irq < 0) {
    dev_err(&pdev.dev, "failed to translate ID_OUT GPIO to IRQ\n");
    return irq;
    }
    err = devm_request_threaded_irq(&pdev.dev, irq, core::ptr::null_mut(), max3355_id_irq,
    IRQF_ONESHOT | IRQF_NO_SUSPEND |
    IRQF_TRIGGER_RISING |
    IRQF_TRIGGER_FALLING,
    pdev.name, data);
    if (err < 0) {
    dev_err(&pdev.dev, "failed to request ID_OUT IRQ\n");
    return err;
    }
    platform_set_drvdata(pdev, data);
// Perform initial detection
    max3355_id_irq(irq, data);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn max3355_remove(pdev: *mut platform_device) {
    static void max3355_remove(struct platform_device *pdev)
    {
    struct max3355_data *data = platform_get_drvdata(pdev);
    gpiod_set_value_cansleep(data.shdn_gpiod, 0);
    }
    static const struct of_device_id max3355_match_table[] = {
    { .compatible = "maxim,max3355", },
    { }
    };
    MODULE_DEVICE_TABLE(of, max3355_match_table);
    static struct platform_driver max3355_driver = {
    .probe		= max3355_probe,
    .remove		= max3355_remove,
    .driver		= {
    .name	= "extcon-max3355",
    .of_match_table = max3355_match_table,
    },
    };
    module_platform_driver(max3355_driver);
    MODULE_AUTHOR("Sergei Shtylyov <sergei.shtylyov@cogentembedded.com>");
    MODULE_DESCRIPTION("Maxim MAX3355 extcon driver");
    MODULE_LICENSE("GPL v2");
