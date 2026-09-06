//! Automatically rewritten from C to Rust
//! Source: drivers/input/misc/cpcap-pwrbutton.c
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
// CPCAP Power Button Input Driver
//
// Copyright (C) 2017 Sebastian Reichel <sre@kernel.org>
//

pub const CPCAP_IRQ_ON: c_int = 23;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpcap_power_button {
    pub regmap: *mut regmap,
    pub idev: *mut input_dev,
    pub dev: *mut device,
}

#[no_mangle]
unsafe extern "C" fn powerbutton_irq(irq: c_int, _button: *mut c_void) -> irqreturn_t {
    static irqreturn_t powerbutton_irq(int irq, void *_button)
    {
    struct cpcap_power_button *button = _button;
    int val;
    val = cpcap_sense_virq(button.regmap, irq);
    if (val < 0) {
    dev_err(button.dev, "irq read failed: %d", val);
    return IRQ_HANDLED;
    }
    pm_wakeup_event(button.dev, 0);
    input_report_key(button.idev, KEY_POWER, val);
    input_sync(button.idev);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn cpcap_power_button_probe(pdev: *mut platform_device) -> c_int {
    static int cpcap_power_button_probe(struct platform_device *pdev)
    {
    struct cpcap_power_button *button;
    int irq;
    int err;
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    button = devm_kmalloc(&pdev.dev, sizeof(*button), GFP_KERNEL);
    if (!button)
    return -ENOMEM;
    button.idev = devm_input_allocate_device(&pdev.dev);
    if (!button.idev)
    return -ENOMEM;
    button.regmap = dev_get_regmap(pdev.dev.parent, core::ptr::null_mut());
    if (!button.regmap)
    return -ENODEV;
    button.dev = &pdev.dev;
    button.idev.name = "cpcap-pwrbutton";
    button.idev.phys = "cpcap-pwrbutton/input0";
    input_set_capability(button.idev, EV_KEY, KEY_POWER);
    err = devm_request_threaded_irq(&pdev.dev, irq, core::ptr::null_mut(),
    powerbutton_irq, IRQF_ONESHOT, "cpcap_pwrbutton", button);
    if (err < 0) {
    dev_err(&pdev.dev, "IRQ request failed: %d\n", err);
    return err;
    }
    err = input_register_device(button.idev);
    if (err) {
    dev_err(&pdev.dev, "Input register failed: %d\n", err);
    return err;
    }
    device_init_wakeup(&pdev.dev, true);
    return 0;
    }

    static const struct of_device_id cpcap_pwrbutton_dt_match_table[] = {
    { .compatible = "motorola,cpcap-pwrbutton" },
    {},
    };
    MODULE_DEVICE_TABLE(of, cpcap_pwrbutton_dt_match_table);

    static struct platform_driver cpcap_power_button_driver = {
    .probe		= cpcap_power_button_probe,
    .driver		= {
    .name	= "cpcap-pwrbutton",
    .of_match_table = of_match_ptr(cpcap_pwrbutton_dt_match_table),
    },
    };
    module_platform_driver(cpcap_power_button_driver);
    MODULE_ALIAS("platform:cpcap-pwrbutton");
    MODULE_DESCRIPTION("CPCAP Power Button");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Sebastian Reichel <sre@kernel.org>");
