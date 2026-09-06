//! Automatically rewritten from C to Rust
//! Source: drivers/input/keyboard/ipaq-micro-keys.c
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
// h3600 atmel micro companion support, key subdevice
// based on previous kernel 2.4 version
// Author : Alessandro Gardich <gremlin@gremlin.it>
// Author : Linus Walleij <linus.walleij@linaro.org>
//

    static const u16 micro_keycodes[] = {
    KEY_RECORD,		/* 1:  Record button			*/
    KEY_CALENDAR,		/* 2:  Calendar				*/
    KEY_ADDRESSBOOK,	/* 3:  Contacts (looks like Outlook)	*/
    KEY_MAIL,		/* 4:  Envelope (Q on older iPAQs)	*/
    KEY_HOMEPAGE,		/* 5:  Start (looks like swoopy arrow)	*/
    KEY_UP,			/* 6:  Up				*/
    KEY_RIGHT,		/* 7:  Right				*/
    KEY_LEFT,		/* 8:  Left				*/
    KEY_DOWN,		/* 9:  Down				*/
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipaq_micro_keys {
    pub micro: *mut ipaq_micro,
    pub input: *mut input_dev,
    pub codes: [u16; ARRAY_SIZE(micro_keycodes)],
}

#[no_mangle]
unsafe extern "C" fn micro_key_receive(data: *mut c_void, len: c_int, msg: *mut c_uchar) {
    static void micro_key_receive(void *data, int len, unsigned char *msg)
    {
    struct ipaq_micro_keys *keys = data;
    int key, down;
    if (len < 1)
    return;
    down = 0x80 & msg[0];
    key  = 0x7f & msg[0];
    if (key < ARRAY_SIZE(micro_keycodes)) {
    input_report_key(keys.input, keys.codes[key], down);
    input_sync(keys.input);
    }
    }
#[no_mangle]
unsafe extern "C" fn micro_key_start(keys: *mut ipaq_micro_keys) {
    static void micro_key_start(struct ipaq_micro_keys *keys)
    {
    guard(spinlock_irq)(&keys.micro.lock);
    keys.micro.key = micro_key_receive;
    keys.micro.key_data = keys;
    }
#[no_mangle]
unsafe extern "C" fn micro_key_stop(keys: *mut ipaq_micro_keys) {
    static void micro_key_stop(struct ipaq_micro_keys *keys)
    {
    guard(spinlock_irq)(&keys.micro.lock);
    keys.micro.key = core::ptr::null_mut();
    keys.micro.key_data = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn micro_key_open(input: *mut input_dev) -> c_int {
    static int micro_key_open(struct input_dev *input)
    {
    struct ipaq_micro_keys *keys = input_get_drvdata(input);
    micro_key_start(keys);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn micro_key_close(input: *mut input_dev) {
    static void micro_key_close(struct input_dev *input)
    {
    struct ipaq_micro_keys *keys = input_get_drvdata(input);
    micro_key_stop(keys);
    }
#[no_mangle]
unsafe extern "C" fn micro_key_probe(pdev: *mut platform_device) -> c_int {
    static int micro_key_probe(struct platform_device *pdev)
    {
    struct ipaq_micro_keys *keys;
    int error;
    int i;
    keys = devm_kzalloc(&pdev.dev, sizeof(*keys), GFP_KERNEL);
    if (!keys)
    return -ENOMEM;
    keys.micro = dev_get_drvdata(pdev.dev.parent);
    keys.input = devm_input_allocate_device(&pdev.dev);
    if (!keys.input)
    return -ENOMEM;
    keys.input.keycodesize = sizeof(micro_keycodes[0]);
    keys.input.keycodemax = ARRAY_SIZE(micro_keycodes);
    memcpy(keys.codes, micro_keycodes, sizeof(keys.codes));
    keys.input.keycode = keys.codes;
    __set_bit(EV_KEY, keys.input.evbit);
    for (i = 0; i < ARRAY_SIZE(micro_keycodes); i++)
    __set_bit(micro_keycodes[i], keys.input.keybit);
    keys.input.name = "h3600 micro keys";
    keys.input.open = micro_key_open;
    keys.input.close = micro_key_close;
    input_set_drvdata(keys.input, keys);
    error = input_register_device(keys.input);
    if (error)
    return error;
    platform_set_drvdata(pdev, keys);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn micro_key_suspend(dev: *mut device) -> c_int {
    static int micro_key_suspend(struct device *dev)
    {
    struct ipaq_micro_keys *keys = dev_get_drvdata(dev);
    micro_key_stop(keys);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn micro_key_resume(dev: *mut device) -> c_int {
    static int micro_key_resume(struct device *dev)
    {
    struct ipaq_micro_keys *keys = dev_get_drvdata(dev);
    struct input_dev *input = keys.input;
    guard(mutex)(&input.mutex);
    if (input_device_enabled(input))
    micro_key_start(keys);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(micro_key_dev_pm_ops,
    micro_key_suspend, micro_key_resume);
    static struct platform_driver micro_key_device_driver = {
    .driver = {
    .name    = "ipaq-micro-keys",
    .pm	= pm_sleep_ptr(&micro_key_dev_pm_ops),
    },
    .probe   = micro_key_probe,
    };
    module_platform_driver(micro_key_device_driver);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("driver for iPAQ Atmel micro keys");
    MODULE_ALIAS("platform:ipaq-micro-keys");
