//! Automatically rewritten from C to Rust
//! Source: drivers/input/misc/tps6594-pwrbutton.c
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
// power button driver for TI TPS6594 PMICs
//
// Copyright (C) 2025 Critical Link LLC - https://www.criticallink.com
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tps6594_pwrbutton {
    pub dev: *mut device,
    pub idev: *mut input_dev,
    pub phys: [c_char; 32],
}

#[no_mangle]
unsafe extern "C" fn tps6594_pb_push_irq(irq: c_int, _pwr: *mut c_void) -> irqreturn_t {
    static irqreturn_t tps6594_pb_push_irq(int irq, void *_pwr)
    {
    struct tps6594_pwrbutton *pwr = _pwr;
    input_report_key(pwr.idev, KEY_POWER, 1);
    pm_wakeup_event(pwr.dev, 0);
    input_sync(pwr.idev);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn tps6594_pb_release_irq(irq: c_int, _pwr: *mut c_void) -> irqreturn_t {
    static irqreturn_t tps6594_pb_release_irq(int irq, void *_pwr)
    {
    struct tps6594_pwrbutton *pwr = _pwr;
    input_report_key(pwr.idev, KEY_POWER, 0);
    input_sync(pwr.idev);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn tps6594_pb_probe(pdev: *mut platform_device) -> c_int {
    static int tps6594_pb_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct tps6594_pwrbutton *pwr;
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
    tps6594_pb_push_irq,
    IRQF_ONESHOT,
    pdev.resource[0].name, pwr);
    if (error) {
    dev_err(dev, "failed to request push IRQ #%d: %d\n", push_irq,
    error);
    return error;
    }
    error = devm_request_threaded_irq(dev, release_irq, core::ptr::null_mut(),
    tps6594_pb_release_irq,
    IRQF_ONESHOT,
    pdev.resource[1].name, pwr);
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
    return 0;
    }
    static const struct platform_device_id tps6594_pwrbtn_id_table[] = {
    { "tps6594-pwrbutton", },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(platform, tps6594_pwrbtn_id_table);
    static struct platform_driver tps6594_pb_driver = {
    .probe = tps6594_pb_probe,
    .driver = {
    .name = "tps6594_pwrbutton",
    },
    .id_table = tps6594_pwrbtn_id_table,
    };
    module_platform_driver(tps6594_pb_driver);
    MODULE_DESCRIPTION("TPS6594 Power Button");
    MODULE_LICENSE("GPL");
