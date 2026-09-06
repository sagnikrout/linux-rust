//! Automatically rewritten from C to Rust
//! Source: drivers/regulator/wm831x-isink.c
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
// wm831x-isink.c  --  Current sink driver for the WM831x series
//
// Copyright 2009 Wolfson Microelectronics PLC.
//
// Author: Mark Brown <broonie@opensource.wolfsonmicro.com>

pub const WM831X_ISINK_MAX_NAME: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm831x_isink {
    pub name: [c_char; WM831X_ISINK_MAX_NAME],
    pub desc: regulator_desc,
    pub reg: c_int,
    pub wm831x: *mut wm831x,
    pub regulator: *mut regulator_dev,
}

#[no_mangle]
unsafe extern "C" fn wm831x_isink_enable(rdev: *mut regulator_dev) -> c_int {
    static int wm831x_isink_enable(struct regulator_dev *rdev)
    {
    struct wm831x_isink *isink = rdev_get_drvdata(rdev);
    struct wm831x *wm831x = isink.wm831x;
    int ret;
// We have a two stage enable: first start the ISINK...
    ret = wm831x_set_bits(wm831x, isink.reg, WM831X_CS1_ENA,
    WM831X_CS1_ENA);
    if (ret != 0)
    return ret;
// ...then enable drive
    ret = wm831x_set_bits(wm831x, isink.reg, WM831X_CS1_DRIVE,
    WM831X_CS1_DRIVE);
    if (ret != 0)
    wm831x_set_bits(wm831x, isink.reg, WM831X_CS1_ENA, 0);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn wm831x_isink_disable(rdev: *mut regulator_dev) -> c_int {
    static int wm831x_isink_disable(struct regulator_dev *rdev)
    {
    struct wm831x_isink *isink = rdev_get_drvdata(rdev);
    struct wm831x *wm831x = isink.wm831x;
    int ret;
    ret = wm831x_set_bits(wm831x, isink.reg, WM831X_CS1_DRIVE, 0);
    if (ret < 0)
    return ret;
    return wm831x_set_bits(wm831x, isink.reg, WM831X_CS1_ENA, 0);
    }
#[no_mangle]
unsafe extern "C" fn wm831x_isink_is_enabled(rdev: *mut regulator_dev) -> c_int {
    static int wm831x_isink_is_enabled(struct regulator_dev *rdev)
    {
    struct wm831x_isink *isink = rdev_get_drvdata(rdev);
    struct wm831x *wm831x = isink.wm831x;
    int ret;
    ret = wm831x_reg_read(wm831x, isink.reg);
    if (ret < 0)
    return ret;
    if ((ret & (WM831X_CS1_ENA | WM831X_CS1_DRIVE)) ==
    (WM831X_CS1_ENA | WM831X_CS1_DRIVE))
    return 1;
    else
    return 0;
    }
    static const struct regulator_ops wm831x_isink_ops = {
    .is_enabled = wm831x_isink_is_enabled,
    .enable = wm831x_isink_enable,
    .disable = wm831x_isink_disable,
    .set_current_limit = regulator_set_current_limit_regmap,
    .get_current_limit = regulator_get_current_limit_regmap,
    };
#[no_mangle]
unsafe extern "C" fn wm831x_isink_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t wm831x_isink_irq(int irq, void *data)
    {
    struct wm831x_isink *isink = data;
    regulator_notifier_call_chain(isink.regulator,
    REGULATOR_EVENT_OVER_CURRENT,
    core::ptr::null_mut());
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn wm831x_isink_probe(pdev: *mut platform_device) -> c_int {
    static int wm831x_isink_probe(struct platform_device *pdev)
    {
    struct wm831x *wm831x = dev_get_drvdata(pdev.dev.parent);
    struct wm831x_pdata *pdata = dev_get_platdata(wm831x.dev);
    struct wm831x_isink *isink;
    let mut id: c_int = pdev.id % ARRAY_SIZE(pdata.isink);
    let mut config: regulator_config = { };
    struct resource *res;
    int ret, irq;
    dev_dbg(&pdev.dev, "Probing ISINK%d\n", id + 1);
    if (pdata == core::ptr::null_mut() || pdata.isink[id] == core::ptr::null_mut())
    return -ENODEV;
    isink = devm_kzalloc(&pdev.dev, sizeof(struct wm831x_isink),
    GFP_KERNEL);
    if (!isink)
    return -ENOMEM;
    isink.wm831x = wm831x;
    res = platform_get_resource(pdev, IORESOURCE_REG, 0);
    if (res == core::ptr::null_mut()) {
    dev_err(&pdev.dev, "No REG resource\n");
    ret = -EINVAL;
    goto err;
    }
    isink.reg = res.start;
// For current parts this is correct; probably need to revisit
// in future.
//
    snprintf(isink.name, sizeof(isink.name), "ISINK%d", id + 1);
    isink.desc.name = isink.name;
    isink.desc.id = id;
    isink.desc.ops = &wm831x_isink_ops;
    isink.desc.type = REGULATOR_CURRENT;
    isink.desc.owner = THIS_MODULE;
    isink.desc.curr_table = wm831x_isinkv_values;
    isink.desc.n_current_limits = ARRAY_SIZE(wm831x_isinkv_values);
    isink.desc.csel_reg = isink.reg;
    isink.desc.csel_mask = WM831X_CS1_ISEL_MASK;
    config.dev = pdev.dev.parent;
    config.init_data = pdata.isink[id];
    config.driver_data = isink;
    config.regmap = wm831x.regmap;
    isink.regulator = devm_regulator_register(&pdev.dev, &isink.desc,
    &config);
    if (IS_ERR(isink.regulator)) {
    ret = PTR_ERR(isink.regulator);
    dev_err(wm831x.dev, "Failed to register ISINK%d: %d\n",
    id + 1, ret);
    goto err;
    }
    irq = wm831x_irq(wm831x, platform_get_irq(pdev, 0));
    ret = devm_request_threaded_irq(&pdev.dev, irq, core::ptr::null_mut(),
    wm831x_isink_irq,
    IRQF_TRIGGER_RISING | IRQF_ONESHOT,
    isink.name,
    isink);
    if (ret != 0) {
    dev_err(&pdev.dev, "Failed to request ISINK IRQ %d: %d\n",
    irq, ret);
    goto err;
    }
    platform_set_drvdata(pdev, isink);
    return 0;
    err:
    return ret;
    }
    static struct platform_driver wm831x_isink_driver = {
    .probe = wm831x_isink_probe,
    .driver		= {
    .name	= "wm831x-isink",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    },
    };
#[no_mangle]
unsafe extern "C" fn wm831x_isink_init() -> int __init {
    static int __init wm831x_isink_init(void)
    {
    int ret;
    ret = platform_driver_register(&wm831x_isink_driver);
    if (ret != 0)
    pr_err("Failed to register WM831x ISINK driver: %d\n", ret);
    return ret;
    }
    subsys_initcall(wm831x_isink_init);
#[no_mangle]
unsafe extern "C" fn wm831x_isink_exit() -> void __exit {
    static void __exit wm831x_isink_exit(void)
    {
    platform_driver_unregister(&wm831x_isink_driver);
    }
    module_exit(wm831x_isink_exit);
// Module information
    MODULE_AUTHOR("Mark Brown");
    MODULE_DESCRIPTION("WM831x current sink driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:wm831x-isink");
