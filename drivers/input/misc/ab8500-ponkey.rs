//! Automatically rewritten from C to Rust
//! Source: drivers/input/misc/ab8500-ponkey.c
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
// Copyright (C) ST-Ericsson SA 2010
//
// Author: Sundar Iyer <sundar.iyer@stericsson.com> for ST-Ericsson
//
// AB8500 Power-On Key handler
//

//
// struct ab8500_ponkey - ab8500 ponkey information
// @idev: pointer to input device
// @ab8500: ab8500 parent
// @irq_dbf: irq number for falling transition
// @irq_dbr: irq number for rising transition
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ab8500_ponkey {
    pub idev: *mut input_dev,
    pub ab8500: *mut ab8500,
    pub irq_dbf: c_int,
    pub irq_dbr: c_int,
}

// AB8500 gives us an interrupt when ONKEY is held
#[no_mangle]
unsafe extern "C" fn ab8500_ponkey_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t ab8500_ponkey_handler(int irq, void *data)
    {
    struct ab8500_ponkey *ponkey = data;
    if (irq == ponkey.irq_dbf)
    input_report_key(ponkey.idev, KEY_POWER, true);
#[no_mangle]
pub unsafe extern "C" fn if(ponkey->irq_dbr: irq ==) -> else {
    else if (irq == ponkey.irq_dbr)
    input_report_key(ponkey.idev, KEY_POWER, false);
    input_sync(ponkey.idev);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn ab8500_ponkey_probe(pdev: *mut platform_device) -> c_int {
    static int ab8500_ponkey_probe(struct platform_device *pdev)
    {
    struct ab8500 *ab8500 = dev_get_drvdata(pdev.dev.parent);
    struct ab8500_ponkey *ponkey;
    struct input_dev *input;
    int irq_dbf, irq_dbr;
    int error;
    irq_dbf = platform_get_irq_byname(pdev, "ONKEY_DBF");
    if (irq_dbf < 0)
    return irq_dbf;
    irq_dbr = platform_get_irq_byname(pdev, "ONKEY_DBR");
    if (irq_dbr < 0)
    return irq_dbr;
    ponkey = devm_kzalloc(&pdev.dev, sizeof(struct ab8500_ponkey),
    GFP_KERNEL);
    if (!ponkey)
    return -ENOMEM;
    input = devm_input_allocate_device(&pdev.dev);
    if (!input)
    return -ENOMEM;
    ponkey.idev = input;
    ponkey.ab8500 = ab8500;
    ponkey.irq_dbf = irq_dbf;
    ponkey.irq_dbr = irq_dbr;
    input.name = "AB8500 POn(PowerOn) Key";
    input.dev.parent = &pdev.dev;
    input_set_capability(input, EV_KEY, KEY_POWER);
    error = devm_request_any_context_irq(&pdev.dev, ponkey.irq_dbf,
    ab8500_ponkey_handler, 0,
    "ab8500-ponkey-dbf", ponkey);
    if (error < 0) {
    dev_err(ab8500.dev, "Failed to request dbf IRQ#%d: %d\n",
    ponkey.irq_dbf, error);
    return error;
    }
    error = devm_request_any_context_irq(&pdev.dev, ponkey.irq_dbr,
    ab8500_ponkey_handler, 0,
    "ab8500-ponkey-dbr", ponkey);
    if (error < 0) {
    dev_err(ab8500.dev, "Failed to request dbr IRQ#%d: %d\n",
    ponkey.irq_dbr, error);
    return error;
    }
    error = input_register_device(ponkey.idev);
    if (error) {
    dev_err(ab8500.dev, "Can't register input device: %d\n", error);
    return error;
    }
    return 0;
    }

    static const struct of_device_id ab8500_ponkey_match[] = {
    { .compatible = "stericsson,ab8500-ponkey", },
    {}
    };
    MODULE_DEVICE_TABLE(of, ab8500_ponkey_match);

    static struct platform_driver ab8500_ponkey_driver = {
    .driver		= {
    .name	= "ab8500-poweron-key",
    .of_match_table = of_match_ptr(ab8500_ponkey_match),
    },
    .probe		= ab8500_ponkey_probe,
    };
    module_platform_driver(ab8500_ponkey_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Sundar Iyer <sundar.iyer@stericsson.com>");
    MODULE_DESCRIPTION("ST-Ericsson AB8500 Power-ON(Pon) Key driver");
