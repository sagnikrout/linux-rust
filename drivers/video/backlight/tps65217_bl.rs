//! Automatically rewritten from C to Rust
//! Source: drivers/video/backlight/tps65217_bl.c
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
// tps65217_bl.c
//
// TPS65217 backlight driver
//
// Copyright (C) 2012 Matthias Kaehlcke
// Author: Matthias Kaehlcke <matthias@kaehlcke.net>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tps65217_bl {
    pub tps: *mut tps65217,
    pub dev: *mut device,
    pub bl: *mut backlight_device,
    pub is_enabled: bool,
}

#[no_mangle]
unsafe extern "C" fn tps65217_bl_enable(tps65217_bl: *mut tps65217_bl) -> c_int {
    static int tps65217_bl_enable(struct tps65217_bl *tps65217_bl)
    {
    int rc;
    rc = tps65217_set_bits(tps65217_bl.tps, TPS65217_REG_WLEDCTRL1,
    TPS65217_WLEDCTRL1_ISINK_ENABLE,
    TPS65217_WLEDCTRL1_ISINK_ENABLE, TPS65217_PROTECT_NONE);
    if (rc) {
    dev_err(tps65217_bl.dev,
    "failed to enable backlight: %d\n", rc);
    return rc;
    }
    tps65217_bl.is_enabled = true;
    dev_dbg(tps65217_bl.dev, "backlight enabled\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tps65217_bl_disable(tps65217_bl: *mut tps65217_bl) -> c_int {
    static int tps65217_bl_disable(struct tps65217_bl *tps65217_bl)
    {
    int rc;
    rc = tps65217_clear_bits(tps65217_bl.tps,
    TPS65217_REG_WLEDCTRL1,
    TPS65217_WLEDCTRL1_ISINK_ENABLE,
    TPS65217_PROTECT_NONE);
    if (rc) {
    dev_err(tps65217_bl.dev,
    "failed to disable backlight: %d\n", rc);
    return rc;
    }
    tps65217_bl.is_enabled = false;
    dev_dbg(tps65217_bl.dev, "backlight disabled\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tps65217_bl_update_status(bl: *mut backlight_device) -> c_int {
    static int tps65217_bl_update_status(struct backlight_device *bl)
    {
    struct tps65217_bl *tps65217_bl = bl_get_data(bl);
    int rc;
    let mut brightness: c_int = backlight_get_brightness(bl);
    if (brightness > 0) {
    rc = tps65217_reg_write(tps65217_bl.tps,
    TPS65217_REG_WLEDCTRL2,
    brightness - 1,
    TPS65217_PROTECT_NONE);
    if (rc) {
    dev_err(tps65217_bl.dev,
    "failed to set brightness level: %d\n", rc);
    return rc;
    }
    dev_dbg(tps65217_bl.dev, "brightness set to %d\n", brightness);
    if (!tps65217_bl.is_enabled)
    rc = tps65217_bl_enable(tps65217_bl);
    } else {
    rc = tps65217_bl_disable(tps65217_bl);
    }
    return rc;
    }
    static const struct backlight_ops tps65217_bl_ops = {
    .options	= BL_CORE_SUSPENDRESUME,
    .update_status	= tps65217_bl_update_status,
    };
    static int tps65217_bl_hw_init(struct tps65217_bl *tps65217_bl,
    struct tps65217_bl_pdata *pdata)
    {
    int rc;
    rc = tps65217_bl_disable(tps65217_bl);
    if (rc)
    return rc;
    switch (pdata.isel) {
    case TPS65217_BL_ISET1:
// select ISET_1 current level
    rc = tps65217_clear_bits(tps65217_bl.tps,
    TPS65217_REG_WLEDCTRL1,
    TPS65217_WLEDCTRL1_ISEL,
    TPS65217_PROTECT_NONE);
    if (rc) {
    dev_err(tps65217_bl.dev,
    "failed to select ISET1 current level: %d)\n",
    rc);
    return rc;
    }
    dev_dbg(tps65217_bl.dev, "selected ISET1 current level\n");
    break;
    case TPS65217_BL_ISET2:
// select ISET2 current level
    rc = tps65217_set_bits(tps65217_bl.tps, TPS65217_REG_WLEDCTRL1,
    TPS65217_WLEDCTRL1_ISEL,
    TPS65217_WLEDCTRL1_ISEL, TPS65217_PROTECT_NONE);
    if (rc) {
    dev_err(tps65217_bl.dev,
    "failed to select ISET2 current level: %d\n",
    rc);
    return rc;
    }
    dev_dbg(tps65217_bl.dev, "selected ISET2 current level\n");
    break;
    default:
    dev_err(tps65217_bl.dev,
    "invalid value for current level: %d\n", pdata.isel);
    return -EINVAL;
    }
// set PWM frequency
    rc = tps65217_set_bits(tps65217_bl.tps,
    TPS65217_REG_WLEDCTRL1,
    TPS65217_WLEDCTRL1_FDIM_MASK,
    pdata.fdim,
    TPS65217_PROTECT_NONE);
    if (rc) {
    dev_err(tps65217_bl.dev,
    "failed to select PWM dimming frequency: %d\n",
    rc);
    return rc;
    }
    return 0;
    }

    static struct tps65217_bl_pdata *
    tps65217_bl_parse_dt(struct platform_device *pdev)
    {
    struct tps65217 *tps = dev_get_drvdata(pdev.dev.parent);
    struct device_node *node;
    struct tps65217_bl_pdata *pdata, *err;
    u32 val;
    node = of_get_child_by_name(tps.dev.of_node, "backlight");
    if (!node)
    return ERR_PTR(-ENODEV);
    pdata = devm_kzalloc(&pdev.dev, sizeof(*pdata), GFP_KERNEL);
    if (!pdata) {
    err = ERR_PTR(-ENOMEM);
    goto err;
    }
    pdata.isel = TPS65217_BL_ISET1;
    if (!of_property_read_u32(node, "isel", &val)) {
    if (val < TPS65217_BL_ISET1 ||
    val > TPS65217_BL_ISET2) {
    dev_err(&pdev.dev,
    "invalid 'isel' value in the device tree\n");
    err = ERR_PTR(-EINVAL);
    goto err;
    }
    pdata.isel = val;
    }
    pdata.fdim = TPS65217_BL_FDIM_200HZ;
    if (!of_property_read_u32(node, "fdim", &val)) {
    switch (val) {
    case 100:
    pdata.fdim = TPS65217_BL_FDIM_100HZ;
    break;
    case 200:
    pdata.fdim = TPS65217_BL_FDIM_200HZ;
    break;
    case 500:
    pdata.fdim = TPS65217_BL_FDIM_500HZ;
    break;
    case 1000:
    pdata.fdim = TPS65217_BL_FDIM_1000HZ;
    break;
    default:
    dev_err(&pdev.dev,
    "invalid 'fdim' value in the device tree\n");
    err = ERR_PTR(-EINVAL);
    goto err;
    }
    }
    if (!of_property_read_u32(node, "default-brightness", &val)) {
    if (val > 100) {
    dev_err(&pdev.dev,
    "invalid 'default-brightness' value in the device tree\n");
    err = ERR_PTR(-EINVAL);
    goto err;
    }
    pdata.dft_brightness = val;
    }
    of_node_put(node);
    return pdata;
    err:
    of_node_put(node);
    return err;
    }

    static struct tps65217_bl_pdata *
    tps65217_bl_parse_dt(struct platform_device *pdev)
    {
    return core::ptr::null_mut();
    }

#[no_mangle]
unsafe extern "C" fn tps65217_bl_probe(pdev: *mut platform_device) -> c_int {
    static int tps65217_bl_probe(struct platform_device *pdev)
    {
    int rc;
    struct tps65217 *tps = dev_get_drvdata(pdev.dev.parent);
    struct tps65217_bl *tps65217_bl;
    struct tps65217_bl_pdata *pdata;
    struct backlight_properties bl_props;
    pdata = tps65217_bl_parse_dt(pdev);
    if (IS_ERR(pdata))
    return PTR_ERR(pdata);
    tps65217_bl = devm_kzalloc(&pdev.dev, sizeof(*tps65217_bl),
    GFP_KERNEL);
    if (tps65217_bl == core::ptr::null_mut())
    return -ENOMEM;
    tps65217_bl.tps = tps;
    tps65217_bl.dev = &pdev.dev;
    tps65217_bl.is_enabled = false;
    rc = tps65217_bl_hw_init(tps65217_bl, pdata);
    if (rc)
    return rc;
    memset(&bl_props, 0, sizeof(struct backlight_properties));
    bl_props.type = BACKLIGHT_RAW;
    bl_props.max_brightness = 100;
    tps65217_bl.bl = devm_backlight_device_register(&pdev.dev, pdev.name,
    tps65217_bl.dev, tps65217_bl,
    &tps65217_bl_ops, &bl_props);
    if (IS_ERR(tps65217_bl.bl)) {
    dev_err(tps65217_bl.dev,
    "registration of backlight device failed: %d\n", rc);
    return PTR_ERR(tps65217_bl.bl);
    }
    tps65217_bl.bl.props.brightness = pdata.dft_brightness;
    backlight_update_status(tps65217_bl.bl);
    platform_set_drvdata(pdev, tps65217_bl);
    return 0;
    }

    static const struct of_device_id tps65217_bl_of_match[] = {
    { .compatible = "ti,tps65217-bl", },
    { /* sentinel */ },
    };
    MODULE_DEVICE_TABLE(of, tps65217_bl_of_match);

    static struct platform_driver tps65217_bl_driver = {
    .probe		= tps65217_bl_probe,
    .driver		= {
    .name	= "tps65217-bl",
    .of_match_table = of_match_ptr(tps65217_bl_of_match),
    },
    };
    module_platform_driver(tps65217_bl_driver);
    MODULE_DESCRIPTION("TPS65217 Backlight driver");
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Matthias Kaehlcke <matthias@kaehlcke.net>");
