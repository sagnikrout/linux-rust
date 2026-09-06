//! Automatically rewritten from C to Rust
//! Source: drivers/net/mdio/mdio-mux-gpio.c
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
// Copyright (C) 2011, 2012 Cavium, Inc.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdio_mux_gpio_state {
    pub gpios: *mut gpio_descs,
    pub mux_handle: *mut c_void,
}

    static int mdio_mux_gpio_switch_fn(int current_child, int desired_child,
    void *data)
    {
    struct mdio_mux_gpio_state *s = data;
    DECLARE_BITMAP(values, BITS_PER_TYPE(desired_child));
    if (current_child == desired_child)
    return 0;
    values[0] = desired_child;
    gpiod_multi_set_value_cansleep(s.gpios, values);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mdio_mux_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int mdio_mux_gpio_probe(struct platform_device *pdev)
    {
    struct mdio_mux_gpio_state *s;
    struct gpio_descs *gpios;
    int r;
    gpios = devm_gpiod_get_array(&pdev.dev, core::ptr::null_mut(), GPIOD_OUT_LOW);
    if (IS_ERR(gpios))
    return PTR_ERR(gpios);
    s = devm_kzalloc(&pdev.dev, sizeof(*s), GFP_KERNEL);
    if (!s)
    return -ENOMEM;
    s.gpios = gpios;
    r = mdio_mux_init(&pdev.dev, pdev.dev.of_node,
    mdio_mux_gpio_switch_fn, &s.mux_handle, s, core::ptr::null_mut());
    if (r != 0)
    return r;
    pdev.dev.platform_data = s;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mdio_mux_gpio_remove(pdev: *mut platform_device) {
    static void mdio_mux_gpio_remove(struct platform_device *pdev)
    {
    struct mdio_mux_gpio_state *s = dev_get_platdata(&pdev.dev);
    mdio_mux_uninit(s.mux_handle);
    }
    static const struct of_device_id mdio_mux_gpio_match[] = {
    {
    .compatible = "mdio-mux-gpio",
    },
    {
// Legacy compatible property.
    .compatible = "cavium,mdio-mux-sn74cbtlv3253",
    },
    {},
    };
    MODULE_DEVICE_TABLE(of, mdio_mux_gpio_match);
    static struct platform_driver mdio_mux_gpio_driver = {
    .driver = {
    .name		= "mdio-mux-gpio",
    .of_match_table = mdio_mux_gpio_match,
    },
    .probe		= mdio_mux_gpio_probe,
    .remove		= mdio_mux_gpio_remove,
    };
    module_platform_driver(mdio_mux_gpio_driver);
    MODULE_DESCRIPTION(DRV_DESCRIPTION);
    MODULE_VERSION(DRV_VERSION);
    MODULE_AUTHOR("David Daney");
    MODULE_LICENSE("GPL v2");
