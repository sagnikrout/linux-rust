//! Automatically rewritten from C to Rust
//! Source: drivers/input/misc/rk805-pwrkey.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Rockchip RK805 PMIC Power Key driver
//
// Copyright (c) 2017, Fuzhou Rockchip Electronics Co., Ltd
//
// Author: Joseph Chen <chenjh@rock-chips.com>
//

#[no_mangle]
unsafe extern "C" fn pwrkey_fall_irq(irq: c_int, _pwr: *mut c_void) -> irqreturn_t {
    static irqreturn_t pwrkey_fall_irq(int irq, void *_pwr)
    {
    struct input_dev *pwr = _pwr;
    input_report_key(pwr, KEY_POWER, 1);
    input_sync(pwr);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn pwrkey_rise_irq(irq: c_int, _pwr: *mut c_void) -> irqreturn_t {
    static irqreturn_t pwrkey_rise_irq(int irq, void *_pwr)
    {
    struct input_dev *pwr = _pwr;
    input_report_key(pwr, KEY_POWER, 0);
    input_sync(pwr);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn rk805_pwrkey_probe(pdev: *mut platform_device) -> c_int {
    static int rk805_pwrkey_probe(struct platform_device *pdev)
    {
    struct input_dev *pwr;
    int fall_irq, rise_irq;
    int err;
    pwr = devm_input_allocate_device(&pdev.dev);
    if (!pwr) {
    dev_err(&pdev.dev, "Can't allocate power button\n");
    return -ENOMEM;
    }
    pwr.name = "rk805 pwrkey";
    pwr.phys = "rk805_pwrkey/input0";
    pwr.id.bustype = BUS_HOST;
    input_set_capability(pwr, EV_KEY, KEY_POWER);
    fall_irq = platform_get_irq(pdev, 0);
    if (fall_irq < 0)
    return fall_irq;
    rise_irq = platform_get_irq(pdev, 1);
    if (rise_irq < 0)
    return rise_irq;
    err = devm_request_any_context_irq(&pwr.dev, fall_irq,
    pwrkey_fall_irq,
    IRQF_TRIGGER_FALLING | IRQF_ONESHOT,
    "rk805_pwrkey_fall", pwr);
    if (err < 0) {
    dev_err(&pdev.dev, "Can't register fall irq: %d\n", err);
    return err;
    }
    err = devm_request_any_context_irq(&pwr.dev, rise_irq,
    pwrkey_rise_irq,
    IRQF_TRIGGER_RISING | IRQF_ONESHOT,
    "rk805_pwrkey_rise", pwr);
    if (err < 0) {
    dev_err(&pdev.dev, "Can't register rise irq: %d\n", err);
    return err;
    }
    err = input_register_device(pwr);
    if (err) {
    dev_err(&pdev.dev, "Can't register power button: %d\n", err);
    return err;
    }
    platform_set_drvdata(pdev, pwr);
    device_init_wakeup(&pdev.dev, true);
    return 0;
    }
    static struct platform_driver rk805_pwrkey_driver = {
    .probe	= rk805_pwrkey_probe,
    .driver	= {
    .name = "rk805-pwrkey",
    },
    };
    module_platform_driver(rk805_pwrkey_driver);
    MODULE_ALIAS("platform:rk805-pwrkey");
    MODULE_AUTHOR("Joseph Chen <chenjh@rock-chips.com>");
    MODULE_DESCRIPTION("RK805 PMIC Power Key driver");
    MODULE_LICENSE("GPL");
