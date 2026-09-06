//! Automatically rewritten from C to Rust
//! Source: drivers/video/backlight/88pm860x_bl.c
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
// Backlight driver for Marvell Semiconductor 88PM8606
//
// Copyright (C) 2009 Marvell International Ltd.
// Haojian Zhuang <haojian.zhuang@marvell.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm860x_backlight_data {
    pub chip: *mut pm860x_chip,
    pub i2c: *mut i2c_client,
    pub current_brightness: c_int,
    pub port: c_int,
    pub pwm: c_int,
    pub iset: c_int,
    pub reg_duty_cycle: c_int,
    pub reg_always_on: c_int,
    pub reg_current: c_int,
}

    static int backlight_power_set(struct pm860x_chip *chip, int port,
    int on)
    {
    let mut ret: c_int = -EINVAL;
    switch (port) {
    case 0:
    ret = on ? pm8606_osc_enable(chip, WLED1_DUTY) :
    pm8606_osc_disable(chip, WLED1_DUTY);
    break;
    case 1:
    ret = on ? pm8606_osc_enable(chip, WLED2_DUTY) :
    pm8606_osc_disable(chip, WLED2_DUTY);
    break;
    case 2:
    ret = on ? pm8606_osc_enable(chip, WLED3_DUTY) :
    pm8606_osc_disable(chip, WLED3_DUTY);
    break;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pm860x_backlight_set(bl: *mut backlight_device, brightness: c_int) -> c_int {
    static int pm860x_backlight_set(struct backlight_device *bl, int brightness)
    {
    struct pm860x_backlight_data *data = bl_get_data(bl);
    struct pm860x_chip *chip = data.chip;
    unsigned char value;
    int ret;
    if (brightness > MAX_BRIGHTNESS)
    value = MAX_BRIGHTNESS;
    else
    value = brightness;
    if (brightness)
    backlight_power_set(chip, data.port, 1);
    ret = pm860x_reg_write(data.i2c, data.reg_duty_cycle, value);
    if (ret < 0)
    goto out;
    if ((data.current_brightness == 0) && brightness) {
    if (data.iset) {
    ret = pm860x_set_bits(data.i2c, data.reg_current,
    CURRENT_BITMASK, data.iset);
    if (ret < 0)
    goto out;
    }
    if (data.pwm) {
    ret = pm860x_set_bits(data.i2c, PM8606_PWM,
    PM8606_PWM_FREQ_MASK, data.pwm);
    if (ret < 0)
    goto out;
    }
    if (brightness == MAX_BRIGHTNESS) {
// set WLED_ON bit as 100%
    ret = pm860x_set_bits(data.i2c, data.reg_always_on,
    PM8606_WLED_ON, PM8606_WLED_ON);
    }
    } else {
    if (brightness == MAX_BRIGHTNESS) {
// set WLED_ON bit as 100%
    ret = pm860x_set_bits(data.i2c, data.reg_always_on,
    PM8606_WLED_ON, PM8606_WLED_ON);
    } else {
// clear WLED_ON bit since it's not 100%
    ret = pm860x_set_bits(data.i2c, data.reg_always_on,
    PM8606_WLED_ON, 0);
    }
    }
    if (ret < 0)
    goto out;
    if (brightness == 0)
    backlight_power_set(chip, data.port, 0);
    dev_dbg(chip.dev, "set brightness %d\n", value);
    data.current_brightness = value;
    return 0;
    out:
    dev_dbg(chip.dev, "set brightness %d failure with return value: %d\n",
    value, ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pm860x_backlight_update_status(bl: *mut backlight_device) -> c_int {
    static int pm860x_backlight_update_status(struct backlight_device *bl)
    {
    return pm860x_backlight_set(bl, backlight_get_brightness(bl));
    }
#[no_mangle]
unsafe extern "C" fn pm860x_backlight_get_brightness(bl: *mut backlight_device) -> c_int {
    static int pm860x_backlight_get_brightness(struct backlight_device *bl)
    {
    struct pm860x_backlight_data *data = bl_get_data(bl);
    struct pm860x_chip *chip = data.chip;
    int ret;
    ret = pm860x_reg_read(data.i2c, data.reg_duty_cycle);
    if (ret < 0)
    goto out;
    data.current_brightness = ret;
    dev_dbg(chip.dev, "get brightness %d\n", data.current_brightness);
    return data.current_brightness;
    out:
    return -EINVAL;
    }
    static const struct backlight_ops pm860x_backlight_ops = {
    .options	= BL_CORE_SUSPENDRESUME,
    .update_status	= pm860x_backlight_update_status,
    .get_brightness	= pm860x_backlight_get_brightness,
    };

    static int pm860x_backlight_dt_init(struct platform_device *pdev,
    struct pm860x_backlight_data *data,
    char *name)
    {
    struct device_node *nproot;
    let mut iset: c_int = 0;
    nproot = of_get_child_by_name(pdev.dev.parent.of_node, "backlights");
    if (!nproot) {
    dev_err(&pdev.dev, "failed to find backlights node\n");
    return -ENODEV;
    }
    for_each_child_of_node_scoped(nproot, np) {
    if (of_node_name_eq(np, name)) {
    of_property_read_u32(np, "marvell,88pm860x-iset",
    &iset);
    data.iset = PM8606_WLED_CURRENT(iset);
    of_property_read_u32(np, "marvell,88pm860x-pwm",
    &data.pwm);
    break;
    }
    }
    of_node_put(nproot);
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn pm860x_backlight_probe(pdev: *mut platform_device) -> c_int {
    static int pm860x_backlight_probe(struct platform_device *pdev)
    {
    struct pm860x_chip *chip = dev_get_drvdata(pdev.dev.parent);
    struct pm860x_backlight_pdata *pdata = dev_get_platdata(&pdev.dev);
    struct pm860x_backlight_data *data;
    struct backlight_device *bl;
    struct resource *res;
    struct backlight_properties props;
    char name[MFD_NAME_SIZE];
    let mut ret: c_int = 0;
    data = devm_kzalloc(&pdev.dev, sizeof(struct pm860x_backlight_data),
    GFP_KERNEL);
    if (data == core::ptr::null_mut())
    return -ENOMEM;
    res = platform_get_resource_byname(pdev, IORESOURCE_REG, "duty cycle");
    if (!res) {
    dev_err(&pdev.dev, "No REG resource for duty cycle\n");
    return -ENXIO;
    }
    data.reg_duty_cycle = res.start;
    res = platform_get_resource_byname(pdev, IORESOURCE_REG, "always on");
    if (!res) {
    dev_err(&pdev.dev, "No REG resource for always on\n");
    return -ENXIO;
    }
    data.reg_always_on = res.start;
    res = platform_get_resource_byname(pdev, IORESOURCE_REG, "current");
    if (!res) {
    dev_err(&pdev.dev, "No REG resource for current\n");
    return -ENXIO;
    }
    data.reg_current = res.start;
    memset(name, 0, MFD_NAME_SIZE);
    sprintf(name, "backlight-%d", pdev.id);
    data.port = pdev.id;
    data.chip = chip;
    data.i2c = (chip.id == CHIP_PM8606) ? chip.client : chip.companion;
    data.current_brightness = MAX_BRIGHTNESS;
    if (pm860x_backlight_dt_init(pdev, data, name)) {
    if (pdata) {
    data.pwm = pdata.pwm;
    data.iset = pdata.iset;
    }
    }
    memset(&props, 0, sizeof(struct backlight_properties));
    props.type = BACKLIGHT_RAW;
    props.max_brightness = MAX_BRIGHTNESS;
    bl = devm_backlight_device_register(&pdev.dev, name, &pdev.dev, data,
    &pm860x_backlight_ops, &props);
    if (IS_ERR(bl)) {
    dev_err(&pdev.dev, "failed to register backlight\n");
    return PTR_ERR(bl);
    }
    bl.props.brightness = MAX_BRIGHTNESS;
    platform_set_drvdata(pdev, bl);
// read current backlight
    ret = pm860x_backlight_get_brightness(bl);
    if (ret < 0)
    return ret;
    backlight_update_status(bl);
    return 0;
    }
    static struct platform_driver pm860x_backlight_driver = {
    .driver		= {
    .name	= "88pm860x-backlight",
    },
    .probe		= pm860x_backlight_probe,
    };
    module_platform_driver(pm860x_backlight_driver);
    MODULE_DESCRIPTION("Backlight Driver for Marvell Semiconductor 88PM8606");
    MODULE_AUTHOR("Haojian Zhuang <haojian.zhuang@marvell.com>");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:88pm860x-backlight");
