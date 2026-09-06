//! Automatically rewritten from C to Rust
//! Source: drivers/usb/misc/usb4604.c
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
// Driver for SMSC USB4604 USB HSIC 4-port 2.0 hub controller driver
// Based on usb3503 driver
//
// Copyright (c) 2012-2013 Dongjin Kim (tobetter@gmail.com)
// Copyright (c) 2016 Linaro Ltd.
//

    enum usb4604_mode {
    USB4604_MODE_UNKNOWN,
    USB4604_MODE_HUB,
    USB4604_MODE_STANDBY,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb4604 {
    pub mode: enum usb4604_mode,
    pub dev: *mut device,
    pub gpio_reset: *mut gpio_desc,
}

#[no_mangle]
unsafe extern "C" fn usb4604_reset(hub: *mut usb4604, state: c_int) {
    static void usb4604_reset(struct usb4604 *hub, int state)
    {
    gpiod_set_value_cansleep(hub.gpio_reset, state);
// Wait for i2c logic to come up
    if (state)
    msleep(250);
    }
#[no_mangle]
unsafe extern "C" fn usb4604_connect(hub: *mut usb4604) -> c_int {
    static int usb4604_connect(struct usb4604 *hub)
    {
    struct device *dev = hub.dev;
    struct i2c_client *client = to_i2c_client(dev);
    int err;
    u8 connect_cmd[] = { 0xaa, 0x55, 0x00 };
    usb4604_reset(hub, 1);
    err = i2c_master_send(client, connect_cmd, ARRAY_SIZE(connect_cmd));
    if (err < 0) {
    usb4604_reset(hub, 0);
    return err;
    }
    hub.mode = USB4604_MODE_HUB;
    dev_dbg(dev, "switched to HUB mode\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn usb4604_switch_mode(hub: *mut usb4604, mode: enum usb4604_mode) -> c_int {
    static int usb4604_switch_mode(struct usb4604 *hub, enum usb4604_mode mode)
    {
    struct device *dev = hub.dev;
    let mut err: c_int = 0;
    switch (mode) {
    case USB4604_MODE_HUB:
    err = usb4604_connect(hub);
    break;
    case USB4604_MODE_STANDBY:
    usb4604_reset(hub, 0);
    dev_dbg(dev, "switched to STANDBY mode\n");
    break;
    default:
    dev_err(dev, "unknown mode is requested\n");
    err = -EINVAL;
    break;
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn usb4604_probe(hub: *mut usb4604) -> c_int {
    static int usb4604_probe(struct usb4604 *hub)
    {
    struct device *dev = hub.dev;
    struct device_node *np = dev.of_node;
    struct gpio_desc *gpio;
    let mut mode: u32 = USB4604_MODE_HUB;
    gpio = devm_gpiod_get_optional(dev, "reset", GPIOD_OUT_LOW);
    if (IS_ERR(gpio))
    return PTR_ERR(gpio);
    hub.gpio_reset = gpio;
    if (of_property_read_u32(np, "initial-mode", &hub.mode))
    hub.mode = mode;
    return usb4604_switch_mode(hub, hub.mode);
    }
#[no_mangle]
unsafe extern "C" fn usb4604_i2c_probe(i2c: *mut i2c_client) -> c_int {
    static int usb4604_i2c_probe(struct i2c_client *i2c)
    {
    struct usb4604 *hub;
    hub = devm_kzalloc(&i2c.dev, sizeof(*hub), GFP_KERNEL);
    if (!hub)
    return -ENOMEM;
    i2c_set_clientdata(i2c, hub);
    hub.dev = &i2c.dev;
    return usb4604_probe(hub);
    }
#[no_mangle]
unsafe extern "C" fn usb4604_i2c_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused usb4604_i2c_suspend(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct usb4604 *hub = i2c_get_clientdata(client);
    usb4604_switch_mode(hub, USB4604_MODE_STANDBY);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn usb4604_i2c_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused usb4604_i2c_resume(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct usb4604 *hub = i2c_get_clientdata(client);
    usb4604_switch_mode(hub, hub.mode);
    return 0;
    }
    static SIMPLE_DEV_PM_OPS(usb4604_i2c_pm_ops, usb4604_i2c_suspend,
    usb4604_i2c_resume);
    static const struct i2c_device_id usb4604_id[] = {
    { .name = "usb4604" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, usb4604_id);

    static const struct of_device_id usb4604_of_match[] = {
    { .compatible = "smsc,usb4604" },
    {}
    };
    MODULE_DEVICE_TABLE(of, usb4604_of_match);

    static struct i2c_driver usb4604_i2c_driver = {
    .driver = {
    .name = "usb4604",
    .pm = pm_ptr(&usb4604_i2c_pm_ops),
    .of_match_table = of_match_ptr(usb4604_of_match),
    },
    .probe		= usb4604_i2c_probe,
    .id_table	= usb4604_id,
    };
    module_i2c_driver(usb4604_i2c_driver);
    MODULE_DESCRIPTION("USB4604 USB HUB driver");
    MODULE_LICENSE("GPL v2");
