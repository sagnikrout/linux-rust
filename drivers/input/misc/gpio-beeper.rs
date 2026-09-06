//! Automatically rewritten from C to Rust
//! Source: drivers/input/misc/gpio-beeper.c
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
// Generic GPIO beeper driver
//
// Copyright (C) 2013-2014 Alexander Shiyan <shc_work@mail.ru>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_beeper {
    pub work: work_struct,
    pub desc: *mut gpio_desc,
    pub beeping: bool,
}

#[no_mangle]
unsafe extern "C" fn gpio_beeper_toggle(beep: *mut gpio_beeper, on: bool) {
    static void gpio_beeper_toggle(struct gpio_beeper *beep, bool on)
    {
    gpiod_set_value_cansleep(beep.desc, on);
    }
#[no_mangle]
unsafe extern "C" fn gpio_beeper_work(work: *mut work_struct) {
    static void gpio_beeper_work(struct work_struct *work)
    {
    struct gpio_beeper *beep = container_of(work, struct gpio_beeper, work);
    gpio_beeper_toggle(beep, beep.beeping);
    }
    static int gpio_beeper_event(struct input_dev *dev, unsigned int type,
    unsigned int code, int value)
    {
    struct gpio_beeper *beep = input_get_drvdata(dev);
    if (type != EV_SND || code != SND_BELL)
    return -ENOTSUPP;
    if (value < 0)
    return -EINVAL;
    beep.beeping = value;
// Schedule work to actually turn the beeper on or off
    schedule_work(&beep.work);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gpio_beeper_close(input: *mut input_dev) {
    static void gpio_beeper_close(struct input_dev *input)
    {
    struct gpio_beeper *beep = input_get_drvdata(input);
    cancel_work_sync(&beep.work);
    gpio_beeper_toggle(beep, false);
    }
#[no_mangle]
unsafe extern "C" fn gpio_beeper_probe(pdev: *mut platform_device) -> c_int {
    static int gpio_beeper_probe(struct platform_device *pdev)
    {
    struct gpio_beeper *beep;
    struct input_dev *input;
    beep = devm_kzalloc(&pdev.dev, sizeof(*beep), GFP_KERNEL);
    if (!beep)
    return -ENOMEM;
    beep.desc = devm_gpiod_get(&pdev.dev, core::ptr::null_mut(), GPIOD_OUT_LOW);
    if (IS_ERR(beep.desc))
    return PTR_ERR(beep.desc);
    input = devm_input_allocate_device(&pdev.dev);
    if (!input)
    return -ENOMEM;
    INIT_WORK(&beep.work, gpio_beeper_work);
    input.name		= pdev.name;
    input.id.bustype	= BUS_HOST;
    input.id.vendor	= 0x0001;
    input.id.product	= 0x0001;
    input.id.version	= 0x0100;
    input.close		= gpio_beeper_close;
    input.event		= gpio_beeper_event;
    input_set_capability(input, EV_SND, SND_BELL);
    input_set_drvdata(input, beep);
    return input_register_device(input);
    }

    static const struct of_device_id gpio_beeper_of_match[] = {
    { .compatible = "gpio-beeper", },
    { }
    };
    MODULE_DEVICE_TABLE(of, gpio_beeper_of_match);

    static struct platform_driver gpio_beeper_platform_driver = {
    .driver	= {
    .name		= BEEPER_MODNAME,
    .of_match_table	= of_match_ptr(gpio_beeper_of_match),
    },
    .probe	= gpio_beeper_probe,
    };
    module_platform_driver(gpio_beeper_platform_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Alexander Shiyan <shc_work@mail.ru>");
    MODULE_DESCRIPTION("Generic GPIO beeper driver");
