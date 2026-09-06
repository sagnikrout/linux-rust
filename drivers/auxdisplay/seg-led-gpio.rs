//! Automatically rewritten from C to Rust
//! Source: drivers/auxdisplay/seg-led-gpio.c
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
// Driver for a 7-segment LED display
//
// The decimal point LED present on some devices is currently not
// supported.
//
// Copyright (C) Allied Telesis Labs
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seg_led_priv {
    pub linedisp: linedisp,
    pub work: delayed_work,
    pub segment_gpios: *mut gpio_descs,
}

#[no_mangle]
unsafe extern "C" fn seg_led_update(work: *mut work_struct) {
    static void seg_led_update(struct work_struct *work)
    {
    struct seg_led_priv *priv = container_of(work, struct seg_led_priv, work.work);
    struct linedisp *linedisp = &priv.linedisp;
    struct linedisp_map *map = linedisp.map;
    DECLARE_BITMAP(values, 8) = { };
    bitmap_set_value8(values, map_to_seg7(&map.map.seg7, linedisp.buf[0]), 0);
    gpiod_multi_set_value_cansleep(priv.segment_gpios, values);
    }
#[no_mangle]
unsafe extern "C" fn seg_led_linedisp_get_map_type(linedisp: *mut linedisp) -> c_int {
    static int seg_led_linedisp_get_map_type(struct linedisp *linedisp)
    {
    struct seg_led_priv *priv = container_of(linedisp, struct seg_led_priv, linedisp);
    INIT_DELAYED_WORK(&priv.work, seg_led_update);
    return LINEDISP_MAP_SEG7;
    }
#[no_mangle]
unsafe extern "C" fn seg_led_linedisp_update(linedisp: *mut linedisp) {
    static void seg_led_linedisp_update(struct linedisp *linedisp)
    {
    struct seg_led_priv *priv = container_of(linedisp, struct seg_led_priv, linedisp);
    schedule_delayed_work(&priv.work, 0);
    }
    static const struct linedisp_ops seg_led_linedisp_ops = {
    .get_map_type = seg_led_linedisp_get_map_type,
    .update = seg_led_linedisp_update,
    };
#[no_mangle]
unsafe extern "C" fn seg_led_probe(pdev: *mut platform_device) -> c_int {
    static int seg_led_probe(struct platform_device *pdev)
    {
    struct seg_led_priv *priv;
    struct device *dev = &pdev.dev;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    platform_set_drvdata(pdev, priv);
    priv.segment_gpios = devm_gpiod_get_array(dev, "segment", GPIOD_OUT_LOW);
    if (IS_ERR(priv.segment_gpios))
    return PTR_ERR(priv.segment_gpios);
    if (priv.segment_gpios.ndescs < 7 || priv.segment_gpios.ndescs > 8)
    return -EINVAL;
    return linedisp_register(&priv.linedisp, dev, 1, &seg_led_linedisp_ops);
    }
#[no_mangle]
unsafe extern "C" fn seg_led_remove(pdev: *mut platform_device) {
    static void seg_led_remove(struct platform_device *pdev)
    {
    struct seg_led_priv *priv = platform_get_drvdata(pdev);
    cancel_delayed_work_sync(&priv.work);
    linedisp_unregister(&priv.linedisp);
    }
    static const struct of_device_id seg_led_of_match[] = {
    { .compatible = "gpio-7-segment"},
    {}
    };
    MODULE_DEVICE_TABLE(of, seg_led_of_match);
    static struct platform_driver seg_led_driver = {
    .probe = seg_led_probe,
    .remove = seg_led_remove,
    .driver = {
    .name = "seg-led-gpio",
    .of_match_table = seg_led_of_match,
    },
    };
    module_platform_driver(seg_led_driver);
    MODULE_AUTHOR("Chris Packham <chris.packham@alliedtelesis.co.nz>");
    MODULE_DESCRIPTION("7 segment LED driver");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("LINEDISP");
