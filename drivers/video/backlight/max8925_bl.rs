//! Automatically rewritten from C to Rust
//! Source: drivers/video/backlight/max8925_bl.c
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
// Backlight driver for Maxim MAX8925
//
// Copyright (C) 2009 Marvell International Ltd.
// Haojian Zhuang <haojian.zhuang@marvell.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct max8925_backlight_data {
    pub chip: *mut max8925_chip,
    pub current_brightness: c_int,
    pub reg_mode_cntl: c_int,
    pub reg_cntl: c_int,
}

#[no_mangle]
unsafe extern "C" fn max8925_backlight_set(bl: *mut backlight_device, brightness: c_int) -> c_int {
    static int max8925_backlight_set(struct backlight_device *bl, int brightness)
    {
    struct max8925_backlight_data *data = bl_get_data(bl);
    struct max8925_chip *chip = data.chip;
    unsigned char value;
    int ret;
    if (brightness > MAX_BRIGHTNESS)
    value = MAX_BRIGHTNESS;
    else
    value = brightness;
    ret = max8925_reg_write(chip.i2c, data.reg_cntl, value);
    if (ret < 0)
    goto out;
    if (!data.current_brightness && brightness)
// enable WLED output
    ret = max8925_set_bits(chip.i2c, data.reg_mode_cntl, 1, 1);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !brightness) -> else {
    else if (!brightness)
// disable WLED output
    ret = max8925_set_bits(chip.i2c, data.reg_mode_cntl, 1, 0);
    if (ret < 0)
    goto out;
    dev_dbg(chip.dev, "set brightness %d\n", value);
    data.current_brightness = value;
    return 0;
    out:
    dev_dbg(chip.dev, "set brightness %d failure with return value:%d\n",
    value, ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn max8925_backlight_update_status(bl: *mut backlight_device) -> c_int {
    static int max8925_backlight_update_status(struct backlight_device *bl)
    {
    return max8925_backlight_set(bl, backlight_get_brightness(bl));
    }
#[no_mangle]
unsafe extern "C" fn max8925_backlight_get_brightness(bl: *mut backlight_device) -> c_int {
    static int max8925_backlight_get_brightness(struct backlight_device *bl)
    {
    struct max8925_backlight_data *data = bl_get_data(bl);
    struct max8925_chip *chip = data.chip;
    int ret;
    ret = max8925_reg_read(chip.i2c, data.reg_cntl);
    if (ret < 0)
    return -EINVAL;
    data.current_brightness = ret;
    dev_dbg(chip.dev, "get brightness %d\n", data.current_brightness);
    return ret;
    }
    static const struct backlight_ops max8925_backlight_ops = {
    .options	= BL_CORE_SUSPENDRESUME,
    .update_status	= max8925_backlight_update_status,
    .get_brightness	= max8925_backlight_get_brightness,
    };
#[no_mangle]
unsafe extern "C" fn max8925_backlight_dt_init(pdev: *mut platform_device) {
    static void max8925_backlight_dt_init(struct platform_device *pdev)
    {
    struct device_node *nproot = pdev.dev.parent.of_node, *np;
    struct max8925_backlight_pdata *pdata;
    u32 val;
    if (!nproot || !IS_ENABLED(CONFIG_OF))
    return;
    pdata = devm_kzalloc(&pdev.dev,
    sizeof(struct max8925_backlight_pdata),
    GFP_KERNEL);
    if (!pdata)
    return;
    np = of_get_child_by_name(nproot, "backlight");
    if (!np) {
    dev_err(&pdev.dev, "failed to find backlight node\n");
    return;
    }
    if (!of_property_read_u32(np, "maxim,max8925-dual-string", &val))
    pdata.dual_string = val;
    of_node_put(np);
    pdev.dev.platform_data = pdata;
    }
#[no_mangle]
unsafe extern "C" fn max8925_backlight_probe(pdev: *mut platform_device) -> c_int {
    static int max8925_backlight_probe(struct platform_device *pdev)
    {
    struct max8925_chip *chip = dev_get_drvdata(pdev.dev.parent);
    struct max8925_backlight_pdata *pdata;
    struct max8925_backlight_data *data;
    struct backlight_device *bl;
    struct backlight_properties props;
    struct resource *res;
    unsigned char value;
    let mut ret: c_int = 0;
    data = devm_kzalloc(&pdev.dev, sizeof(struct max8925_backlight_data),
    GFP_KERNEL);
    if (data == core::ptr::null_mut())
    return -ENOMEM;
    res = platform_get_resource(pdev, IORESOURCE_REG, 0);
    if (!res) {
    dev_err(&pdev.dev, "No REG resource for mode control!\n");
    return -ENXIO;
    }
    data.reg_mode_cntl = res.start;
    res = platform_get_resource(pdev, IORESOURCE_REG, 1);
    if (!res) {
    dev_err(&pdev.dev, "No REG resource for control!\n");
    return -ENXIO;
    }
    data.reg_cntl = res.start;
    data.chip = chip;
    data.current_brightness = 0;
    memset(&props, 0, sizeof(struct backlight_properties));
    props.type = BACKLIGHT_RAW;
    props.max_brightness = MAX_BRIGHTNESS;
    bl = devm_backlight_device_register(&pdev.dev, "max8925-backlight",
    &pdev.dev, data,
    &max8925_backlight_ops, &props);
    if (IS_ERR(bl)) {
    dev_err(&pdev.dev, "failed to register backlight\n");
    return PTR_ERR(bl);
    }
    bl.props.brightness = MAX_BRIGHTNESS;
    platform_set_drvdata(pdev, bl);
    value = 0;
    if (!pdev.dev.platform_data)
    max8925_backlight_dt_init(pdev);
    pdata = pdev.dev.platform_data;
    if (pdata) {
    if (pdata.lxw_scl)
    value |= (1 << 7);
    if (pdata.lxw_freq)
    value |= (LWX_FREQ(pdata.lxw_freq) << 4);
    if (pdata.dual_string)
    value |= (1 << 1);
    }
    ret = max8925_set_bits(chip.i2c, data.reg_mode_cntl, 0xfe, value);
    if (ret < 0)
    return ret;
    backlight_update_status(bl);
    return 0;
    }
    static struct platform_driver max8925_backlight_driver = {
    .driver		= {
    .name	= "max8925-backlight",
    },
    .probe		= max8925_backlight_probe,
    };
    module_platform_driver(max8925_backlight_driver);
    MODULE_DESCRIPTION("Backlight Driver for Maxim MAX8925");
    MODULE_AUTHOR("Haojian Zhuang <haojian.zhuang@marvell.com>");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:max8925-backlight");
