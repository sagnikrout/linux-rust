//! Automatically rewritten from C to Rust
//! Source: drivers/input/misc/gpio-vibra.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// GPIO vibrator driver
//
// Copyright (C) 2019 Luca Weiss <luca@z3ntu.xyz>
//
// Based on PWM vibrator driver:
// Copyright (C) 2017 Collabora Ltd.
//
// Based on previous work from:
// Copyright (C) 2012 Dmitry Torokhov <dmitry.torokhov@gmail.com>
//
// Based on PWM beeper driver:
// Copyright (C) 2010, Lars-Peter Clausen <lars@metafoo.de>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_vibrator {
    pub input: *mut input_dev,
    pub gpio: *mut gpio_desc,
    pub vcc: *mut regulator,
    pub play_work: work_struct,
    pub running: bool,
    pub vcc_on: bool,
}

#[no_mangle]
unsafe extern "C" fn gpio_vibrator_start(vibrator: *mut gpio_vibrator) -> c_int {
    static int gpio_vibrator_start(struct gpio_vibrator *vibrator)
    {
    struct device *pdev = vibrator.input.dev.parent;
    int err;
    if (!vibrator.vcc_on) {
    err = regulator_enable(vibrator.vcc);
    if (err) {
    dev_err(pdev, "failed to enable regulator: %d\n", err);
    return err;
    }
    vibrator.vcc_on = true;
    }
    gpiod_set_value_cansleep(vibrator.gpio, 1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gpio_vibrator_stop(vibrator: *mut gpio_vibrator) {
    static void gpio_vibrator_stop(struct gpio_vibrator *vibrator)
    {
    gpiod_set_value_cansleep(vibrator.gpio, 0);
    if (vibrator.vcc_on) {
    regulator_disable(vibrator.vcc);
    vibrator.vcc_on = false;
    }
    }
#[no_mangle]
unsafe extern "C" fn gpio_vibrator_play_work(work: *mut work_struct) {
    static void gpio_vibrator_play_work(struct work_struct *work)
    {
    struct gpio_vibrator *vibrator =
    container_of(work, struct gpio_vibrator, play_work);
    if (vibrator.running)
    gpio_vibrator_start(vibrator);
    else
    gpio_vibrator_stop(vibrator);
    }
    static int gpio_vibrator_play_effect(struct input_dev *dev, void *data,
    struct ff_effect *effect)
    {
    struct gpio_vibrator *vibrator = input_get_drvdata(dev);
    int level;
    level = effect.u.rumble.strong_magnitude;
    if (!level)
    level = effect.u.rumble.weak_magnitude;
    vibrator.running = level;
    schedule_work(&vibrator.play_work);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gpio_vibrator_close(input: *mut input_dev) {
    static void gpio_vibrator_close(struct input_dev *input)
    {
    struct gpio_vibrator *vibrator = input_get_drvdata(input);
    cancel_work_sync(&vibrator.play_work);
    gpio_vibrator_stop(vibrator);
    vibrator.running = false;
    }
#[no_mangle]
unsafe extern "C" fn gpio_vibrator_probe(pdev: *mut platform_device) -> c_int {
    static int gpio_vibrator_probe(struct platform_device *pdev)
    {
    struct gpio_vibrator *vibrator;
    int err;
    vibrator = devm_kzalloc(&pdev.dev, sizeof(*vibrator), GFP_KERNEL);
    if (!vibrator)
    return -ENOMEM;
    vibrator.input = devm_input_allocate_device(&pdev.dev);
    if (!vibrator.input)
    return -ENOMEM;
    vibrator.vcc = devm_regulator_get(&pdev.dev, "vcc");
    if (IS_ERR(vibrator.vcc))
    return dev_err_probe(&pdev.dev, PTR_ERR(vibrator.vcc),
    "Failed to request regulator\n");
    vibrator.gpio = devm_gpiod_get(&pdev.dev, "enable", GPIOD_OUT_LOW);
    if (IS_ERR(vibrator.gpio))
    return dev_err_probe(&pdev.dev, PTR_ERR(vibrator.gpio),
    "Failed to request main gpio\n");
    INIT_WORK(&vibrator.play_work, gpio_vibrator_play_work);
    vibrator.input.name = "gpio-vibrator";
    vibrator.input.id.bustype = BUS_HOST;
    vibrator.input.close = gpio_vibrator_close;
    input_set_drvdata(vibrator.input, vibrator);
    input_set_capability(vibrator.input, EV_FF, FF_RUMBLE);
    err = input_ff_create_memless(vibrator.input, core::ptr::null_mut(),
    gpio_vibrator_play_effect);
    if (err) {
    dev_err(&pdev.dev, "Couldn't create FF dev: %d\n", err);
    return err;
    }
    err = input_register_device(vibrator.input);
    if (err) {
    dev_err(&pdev.dev, "Couldn't register input dev: %d\n", err);
    return err;
    }
    platform_set_drvdata(pdev, vibrator);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gpio_vibrator_suspend(dev: *mut device) -> c_int {
    static int gpio_vibrator_suspend(struct device *dev)
    {
    struct platform_device *pdev = to_platform_device(dev);
    struct gpio_vibrator *vibrator = platform_get_drvdata(pdev);
    cancel_work_sync(&vibrator.play_work);
    if (vibrator.running)
    gpio_vibrator_stop(vibrator);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gpio_vibrator_resume(dev: *mut device) -> c_int {
    static int gpio_vibrator_resume(struct device *dev)
    {
    struct platform_device *pdev = to_platform_device(dev);
    struct gpio_vibrator *vibrator = platform_get_drvdata(pdev);
    if (vibrator.running)
    gpio_vibrator_start(vibrator);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(gpio_vibrator_pm_ops,
    gpio_vibrator_suspend, gpio_vibrator_resume);

    static const struct of_device_id gpio_vibra_dt_match_table[] = {
    { .compatible = "gpio-vibrator" },
    {}
    };
    MODULE_DEVICE_TABLE(of, gpio_vibra_dt_match_table);

    static struct platform_driver gpio_vibrator_driver = {
    .probe	= gpio_vibrator_probe,
    .driver	= {
    .name	= "gpio-vibrator",
    .pm	= pm_sleep_ptr(&gpio_vibrator_pm_ops),
    .of_match_table = of_match_ptr(gpio_vibra_dt_match_table),
    },
    };
    module_platform_driver(gpio_vibrator_driver);
    MODULE_AUTHOR("Luca Weiss <luca@z3ntu.xy>");
    MODULE_DESCRIPTION("GPIO vibrator driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:gpio-vibrator");
