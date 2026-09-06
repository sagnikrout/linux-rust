//! Automatically rewritten from C to Rust
//! Source: drivers/input/misc/da9052_onkey.c
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
// ON pin driver for Dialog DA9052 PMICs
//
// Copyright(c) 2012 Dialog Semiconductor Ltd.
//
// Author: David Dajun Chen <dchen@diasemi.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct da9052_onkey {
    pub da9052: *mut da9052,
    pub input: *mut input_dev,
    pub work: delayed_work,
}

#[no_mangle]
unsafe extern "C" fn da9052_onkey_query(onkey: *mut da9052_onkey) {
    static void da9052_onkey_query(struct da9052_onkey *onkey)
    {
    int ret;
    ret = da9052_reg_read(onkey.da9052, DA9052_STATUS_A_REG);
    if (ret < 0) {
    dev_err(onkey.da9052.dev,
    "Failed to read onkey event err=%d\n", ret);
    } else {
//
// Since interrupt for deassertion of ONKEY pin is not
// generated, onkey event state determines the onkey
// button state.
//
    let mut pressed: bool = !(ret & DA9052_STATUSA_NONKEY);
    input_report_key(onkey.input, KEY_POWER, pressed);
    input_sync(onkey.input);
//
// Interrupt is generated only when the ONKEY pin
// is asserted.  Hence the deassertion of the pin
// is simulated through work queue.
//
    if (pressed)
    schedule_delayed_work(&onkey.work,
    msecs_to_jiffies(50));
    }
    }
#[no_mangle]
unsafe extern "C" fn da9052_onkey_work(work: *mut work_struct) {
    static void da9052_onkey_work(struct work_struct *work)
    {
    struct da9052_onkey *onkey = container_of(work, struct da9052_onkey,
    work.work);
    da9052_onkey_query(onkey);
    }
#[no_mangle]
unsafe extern "C" fn da9052_onkey_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t da9052_onkey_irq(int irq, void *data)
    {
    struct da9052_onkey *onkey = data;
    da9052_onkey_query(onkey);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn da9052_onkey_probe(pdev: *mut platform_device) -> c_int {
    static int da9052_onkey_probe(struct platform_device *pdev)
    {
    struct da9052 *da9052 = dev_get_drvdata(pdev.dev.parent);
    struct da9052_onkey *onkey;
    struct input_dev *input_dev;
    int error;
    if (!da9052) {
    dev_err(&pdev.dev, "Failed to get the driver's data\n");
    return -EINVAL;
    }
    onkey = kzalloc_obj(*onkey);
    input_dev = input_allocate_device();
    if (!onkey || !input_dev) {
    dev_err(&pdev.dev, "Failed to allocate memory\n");
    error = -ENOMEM;
    goto err_free_mem;
    }
    onkey.input = input_dev;
    onkey.da9052 = da9052;
    INIT_DELAYED_WORK(&onkey.work, da9052_onkey_work);
    input_dev.name = "da9052-onkey";
    input_dev.phys = "da9052-onkey/input0";
    input_dev.dev.parent = &pdev.dev;
    input_dev.evbit[0] = BIT_MASK(EV_KEY);
    __set_bit(KEY_POWER, input_dev.keybit);
    error = da9052_request_irq(onkey.da9052, DA9052_IRQ_NONKEY, "ONKEY",
    da9052_onkey_irq, onkey);
    if (error < 0) {
    dev_err(onkey.da9052.dev,
    "Failed to register ONKEY IRQ: %d\n", error);
    goto err_free_mem;
    }
    error = input_register_device(onkey.input);
    if (error) {
    dev_err(&pdev.dev, "Unable to register input device, %d\n",
    error);
    goto err_free_irq;
    }
    platform_set_drvdata(pdev, onkey);
    return 0;
    err_free_irq:
    da9052_free_irq(onkey.da9052, DA9052_IRQ_NONKEY, onkey);
    cancel_delayed_work_sync(&onkey.work);
    err_free_mem:
    input_free_device(input_dev);
    kfree(onkey);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn da9052_onkey_remove(pdev: *mut platform_device) {
    static void da9052_onkey_remove(struct platform_device *pdev)
    {
    struct da9052_onkey *onkey = platform_get_drvdata(pdev);
    da9052_free_irq(onkey.da9052, DA9052_IRQ_NONKEY, onkey);
    cancel_delayed_work_sync(&onkey.work);
    input_unregister_device(onkey.input);
    kfree(onkey);
    }
    static struct platform_driver da9052_onkey_driver = {
    .probe	= da9052_onkey_probe,
    .remove	= da9052_onkey_remove,
    .driver	= {
    .name	= "da9052-onkey",
    },
    };
    module_platform_driver(da9052_onkey_driver);
    MODULE_AUTHOR("David Dajun Chen <dchen@diasemi.com>");
    MODULE_DESCRIPTION("Onkey driver for DA9052");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:da9052-onkey");
