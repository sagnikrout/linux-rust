//! Automatically rewritten from C to Rust
//! Source: drivers/input/misc/da9055_onkey.c
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
// ON pin driver for Dialog DA9055 PMICs
//
// Copyright(c) 2012 Dialog Semiconductor Ltd.
//
// Author: David Dajun Chen <dchen@diasemi.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct da9055_onkey {
    pub da9055: *mut da9055,
    pub input: *mut input_dev,
    pub work: delayed_work,
}

#[no_mangle]
unsafe extern "C" fn da9055_onkey_query(onkey: *mut da9055_onkey) {
    static void da9055_onkey_query(struct da9055_onkey *onkey)
    {
    int key_stat;
    key_stat = da9055_reg_read(onkey.da9055, DA9055_REG_STATUS_A);
    if (key_stat < 0) {
    dev_err(onkey.da9055.dev,
    "Failed to read onkey event %d\n", key_stat);
    } else {
    key_stat &= DA9055_NOKEY_STS;
//
// Onkey status bit is cleared when onkey button is released.
//
    if (!key_stat) {
    input_report_key(onkey.input, KEY_POWER, 0);
    input_sync(onkey.input);
    }
    }
//
// Interrupt is generated only when the ONKEY pin is asserted.
// Hence the deassertion of the pin is simulated through work queue.
//
    if (key_stat)
    schedule_delayed_work(&onkey.work, msecs_to_jiffies(10));
    }
#[no_mangle]
unsafe extern "C" fn da9055_onkey_work(work: *mut work_struct) {
    static void da9055_onkey_work(struct work_struct *work)
    {
    struct da9055_onkey *onkey = container_of(work, struct da9055_onkey,
    work.work);
    da9055_onkey_query(onkey);
    }
#[no_mangle]
unsafe extern "C" fn da9055_onkey_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t da9055_onkey_irq(int irq, void *data)
    {
    struct da9055_onkey *onkey = data;
    input_report_key(onkey.input, KEY_POWER, 1);
    input_sync(onkey.input);
    da9055_onkey_query(onkey);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn da9055_onkey_probe(pdev: *mut platform_device) -> c_int {
    static int da9055_onkey_probe(struct platform_device *pdev)
    {
    struct da9055 *da9055 = dev_get_drvdata(pdev.dev.parent);
    struct da9055_onkey *onkey;
    struct input_dev *input_dev;
    int irq, err;
    irq = platform_get_irq_byname(pdev, "ONKEY");
    if (irq < 0)
    return -EINVAL;
    onkey = devm_kzalloc(&pdev.dev, sizeof(*onkey), GFP_KERNEL);
    if (!onkey) {
    dev_err(&pdev.dev, "Failed to allocate memory\n");
    return -ENOMEM;
    }
    input_dev = input_allocate_device();
    if (!input_dev) {
    dev_err(&pdev.dev, "Failed to allocate memory\n");
    return -ENOMEM;
    }
    onkey.input = input_dev;
    onkey.da9055 = da9055;
    input_dev.name = "da9055-onkey";
    input_dev.phys = "da9055-onkey/input0";
    input_dev.dev.parent = &pdev.dev;
    input_dev.evbit[0] = BIT_MASK(EV_KEY);
    __set_bit(KEY_POWER, input_dev.keybit);
    INIT_DELAYED_WORK(&onkey.work, da9055_onkey_work);
    err = request_threaded_irq(irq, core::ptr::null_mut(), da9055_onkey_irq,
    IRQF_TRIGGER_HIGH | IRQF_ONESHOT,
    "ONKEY", onkey);
    if (err < 0) {
    dev_err(&pdev.dev,
    "Failed to register ONKEY IRQ %d, error = %d\n",
    irq, err);
    goto err_free_input;
    }
    err = input_register_device(input_dev);
    if (err) {
    dev_err(&pdev.dev, "Unable to register input device, %d\n",
    err);
    goto err_free_irq;
    }
    platform_set_drvdata(pdev, onkey);
    return 0;
    err_free_irq:
    free_irq(irq, onkey);
    cancel_delayed_work_sync(&onkey.work);
    err_free_input:
    input_free_device(input_dev);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn da9055_onkey_remove(pdev: *mut platform_device) {
    static void da9055_onkey_remove(struct platform_device *pdev)
    {
    struct da9055_onkey *onkey = platform_get_drvdata(pdev);
    let mut irq: c_int = platform_get_irq_byname(pdev, "ONKEY");
    irq = regmap_irq_get_virq(onkey.da9055.irq_data, irq);
    free_irq(irq, onkey);
    cancel_delayed_work_sync(&onkey.work);
    input_unregister_device(onkey.input);
    }
    static struct platform_driver da9055_onkey_driver = {
    .probe	= da9055_onkey_probe,
    .remove	= da9055_onkey_remove,
    .driver	= {
    .name	= "da9055-onkey",
    },
    };
    module_platform_driver(da9055_onkey_driver);
    MODULE_AUTHOR("David Dajun Chen <dchen@diasemi.com>");
    MODULE_DESCRIPTION("Onkey driver for DA9055");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:da9055-onkey");
