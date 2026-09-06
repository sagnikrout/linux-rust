//! Automatically rewritten from C to Rust
//! Source: drivers/leds/leds-lm3530.c
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
// Copyright (C) 2011 ST-Ericsson SA.
// Copyright (C) 2009 Motorola, Inc.
//
// Simple driver for National Semiconductor LM3530 Backlight driver chip
//
// Author: Shreshtha Kumar SAHU <shreshthakumar.sahu@stericsson.com>
// based on leds-lm3530.c by Dan Murphy <D.Murphy@motorola.com>
//

pub const LM3530_GEN_CONFIG: c_uint = 0x10;
pub const LM3530_ALS_CONFIG: c_uint = 0x20;
pub const LM3530_BRT_RAMP_RATE: c_uint = 0x30;
pub const LM3530_ALS_IMP_SELECT: c_uint = 0x41;
pub const LM3530_BRT_CTRL_REG: c_uint = 0xA0;
pub const LM3530_ALS_ZB0_REG: c_uint = 0x60;
pub const LM3530_ALS_ZB1_REG: c_uint = 0x61;
pub const LM3530_ALS_ZB2_REG: c_uint = 0x62;
pub const LM3530_ALS_ZB3_REG: c_uint = 0x63;
pub const LM3530_ALS_Z0T_REG: c_uint = 0x70;
pub const LM3530_ALS_Z1T_REG: c_uint = 0x71;
pub const LM3530_ALS_Z2T_REG: c_uint = 0x72;
pub const LM3530_ALS_Z3T_REG: c_uint = 0x73;
pub const LM3530_ALS_Z4T_REG: c_uint = 0x74;
pub const LM3530_REG_MAX: c_int = 14;
// General Control Register

// ALS Config Register Options

// Brightness Ramp Rate Register

// ALS Resistor Select

// Zone Boundary Register defaults

// Zone Target Register defaults

// 7 bits are used for the brightness : LM3530_BRT_CTRL_REG

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lm3530_mode_map {
    pub mode: *const c_char,
    pub mode_val: enum lm3530_mode,
}

    static struct lm3530_mode_map mode_map[] = {
    { "man", LM3530_BL_MODE_MANUAL },
    { "als", LM3530_BL_MODE_ALS },
    { "pwm", LM3530_BL_MODE_PWM },
    };
//
// struct lm3530_data
// @led_dev: led class device
// @client: i2c client
// @pdata: LM3530 platform data
// @mode: mode of operation - manual, ALS, PWM
// @regulator: regulator
// @brightness: previous brightness value
// @enable: regulator is enabled
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lm3530_data {
    pub led_dev: led_classdev,
    pub client: *mut i2c_client,
    pub pdata: *mut lm3530_platform_data,
    pub mode: enum lm3530_mode,
    pub regulator: *mut regulator,
    pub brightness: enum led_brightness,
    pub enable: bool,
}

//
// struct lm3530_als_data
// @config  : value of ALS configuration register
// @imp_sel : value of ALS resistor select register
// @zone    : values of ALS ZB(Zone Boundary) registers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lm3530_als_data {
    pub config: u8,
    pub imp_sel: u8,
    pub zones: [u8; LM3530_ALS_ZB_MAX],
}

    static const u8 lm3530_reg[LM3530_REG_MAX] = {
    LM3530_GEN_CONFIG,
    LM3530_ALS_CONFIG,
    LM3530_BRT_RAMP_RATE,
    LM3530_ALS_IMP_SELECT,
    LM3530_BRT_CTRL_REG,
    LM3530_ALS_ZB0_REG,
    LM3530_ALS_ZB1_REG,
    LM3530_ALS_ZB2_REG,
    LM3530_ALS_ZB3_REG,
    LM3530_ALS_Z0T_REG,
    LM3530_ALS_Z1T_REG,
    LM3530_ALS_Z2T_REG,
    LM3530_ALS_Z3T_REG,
    LM3530_ALS_Z4T_REG,
    };
#[no_mangle]
unsafe extern "C" fn lm3530_get_mode_from_str(str: *const c_char) -> c_int {
    static int lm3530_get_mode_from_str(const char *str)
    {
    int i;
    for (i = 0; i < ARRAY_SIZE(mode_map); i++)
    if (sysfs_streq(str, mode_map[i].mode))
    return mode_map[i].mode_val;
    return -EINVAL;
    }
    static void lm3530_als_configure(struct lm3530_platform_data *pdata,
    struct lm3530_als_data *als)
    {
    int i;
    u32 als_vmin, als_vmax, als_vstep;
    if (pdata.als_vmax == 0) {
    pdata.als_vmin = 0;
    pdata.als_vmax = LM3530_ALS_WINDOW_mV;
    }
    als_vmin = pdata.als_vmin;
    als_vmax = pdata.als_vmax;
    if ((als_vmax - als_vmin) > LM3530_ALS_WINDOW_mV)
    pdata.als_vmax = als_vmax = als_vmin + LM3530_ALS_WINDOW_mV;
// n zone boundary makes n+1 zones
    als_vstep = (als_vmax - als_vmin) / (LM3530_ALS_ZB_MAX + 1);
    for (i = 0; i < LM3530_ALS_ZB_MAX; i++)
    als.zones[i] = (((als_vmin + LM3530_ALS_OFFSET_mV) +
    als_vstep + (i * als_vstep)) * LED_FULL) / 1000;
    als.config =
    (pdata.als_avrg_time << LM3530_ALS_AVG_TIME_SHIFT) |
    (LM3530_ENABLE_ALS) |
    (pdata.als_input_mode << LM3530_ALS_SEL_SHIFT);
    als.imp_sel =
    (pdata.als1_resistor_sel << LM3530_ALS1_IMP_SHIFT) |
    (pdata.als2_resistor_sel << LM3530_ALS2_IMP_SHIFT);
    }
#[no_mangle]
unsafe extern "C" fn lm3530_led_enable(drvdata: *mut lm3530_data) -> c_int {
    static int lm3530_led_enable(struct lm3530_data *drvdata)
    {
    int ret;
    if (drvdata.enable)
    return 0;
    ret = regulator_enable(drvdata.regulator);
    if (ret) {
    dev_err(drvdata.led_dev.dev, "Failed to enable vin:%d\n", ret);
    return ret;
    }
    drvdata.enable = true;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lm3530_led_disable(drvdata: *mut lm3530_data) {
    static void lm3530_led_disable(struct lm3530_data *drvdata)
    {
    int ret;
    if (!drvdata.enable)
    return;
    ret = regulator_disable(drvdata.regulator);
    if (ret) {
    dev_err(drvdata.led_dev.dev, "Failed to disable vin:%d\n",
    ret);
    return;
    }
    drvdata.enable = false;
    }
#[no_mangle]
unsafe extern "C" fn lm3530_init_registers(drvdata: *mut lm3530_data) -> c_int {
    static int lm3530_init_registers(struct lm3530_data *drvdata)
    {
    let mut ret: c_int = 0;
    int i;
    u8 gen_config;
    u8 brt_ramp;
    u8 brightness;
    u8 reg_val[LM3530_REG_MAX];
    struct lm3530_platform_data *pdata = drvdata.pdata;
    struct i2c_client *client = drvdata.client;
    struct lm3530_pwm_data *pwm = &pdata.pwm_data;
    struct lm3530_als_data als;
    memset(&als, 0, sizeof(struct lm3530_als_data));
    gen_config = (pdata.brt_ramp_law << LM3530_RAMP_LAW_SHIFT) |
    ((pdata.max_current & 7) << LM3530_MAX_CURR_SHIFT);
    switch (drvdata.mode) {
    case LM3530_BL_MODE_MANUAL:
    gen_config |= LM3530_ENABLE_I2C;
    break;
    case LM3530_BL_MODE_ALS:
    gen_config |= LM3530_ENABLE_I2C;
    lm3530_als_configure(pdata, &als);
    break;
    case LM3530_BL_MODE_PWM:
    gen_config |= LM3530_ENABLE_PWM | LM3530_ENABLE_PWM_SIMPLE |
    (pdata.pwm_pol_hi << LM3530_PWM_POL_SHIFT);
    break;
    }
    brt_ramp = (pdata.brt_ramp_fall << LM3530_BRT_RAMP_FALL_SHIFT) |
    (pdata.brt_ramp_rise << LM3530_BRT_RAMP_RISE_SHIFT);
    if (drvdata.brightness)
    brightness = drvdata.brightness;
    else
    brightness = drvdata.brightness = pdata.brt_val;
    if (brightness > drvdata.led_dev.max_brightness)
    brightness = drvdata.led_dev.max_brightness;
    reg_val[0] = gen_config;	/* LM3530_GEN_CONFIG */
    reg_val[1] = als.config;	/* LM3530_ALS_CONFIG */
    reg_val[2] = brt_ramp;		/* LM3530_BRT_RAMP_RATE */
    reg_val[3] = als.imp_sel;	/* LM3530_ALS_IMP_SELECT */
    reg_val[4] = brightness;	/* LM3530_BRT_CTRL_REG */
    reg_val[5] = als.zones[0];	/* LM3530_ALS_ZB0_REG */
    reg_val[6] = als.zones[1];	/* LM3530_ALS_ZB1_REG */
    reg_val[7] = als.zones[2];	/* LM3530_ALS_ZB2_REG */
    reg_val[8] = als.zones[3];	/* LM3530_ALS_ZB3_REG */
    reg_val[9] = LM3530_DEF_ZT_0;	/* LM3530_ALS_Z0T_REG */
    reg_val[10] = LM3530_DEF_ZT_1;	/* LM3530_ALS_Z1T_REG */
    reg_val[11] = LM3530_DEF_ZT_2;	/* LM3530_ALS_Z2T_REG */
    reg_val[12] = LM3530_DEF_ZT_3;	/* LM3530_ALS_Z3T_REG */
    reg_val[13] = LM3530_DEF_ZT_4;	/* LM3530_ALS_Z4T_REG */
    ret = lm3530_led_enable(drvdata);
    if (ret)
    return ret;
    for (i = 0; i < LM3530_REG_MAX; i++) {
// do not update brightness register when pwm mode
    if (lm3530_reg[i] == LM3530_BRT_CTRL_REG &&
    drvdata.mode == LM3530_BL_MODE_PWM) {
    if (pwm.pwm_set_intensity)
    pwm.pwm_set_intensity(reg_val[i],
    drvdata.led_dev.max_brightness);
    continue;
    }
    ret = i2c_smbus_write_byte_data(client,
    lm3530_reg[i], reg_val[i]);
    if (ret)
    break;
    }
    return ret;
    }
    static void lm3530_brightness_set(struct led_classdev *led_cdev,
    enum led_brightness brt_val)
    {
    int err;
    struct lm3530_data *drvdata =
    container_of(led_cdev, struct lm3530_data, led_dev);
    struct lm3530_platform_data *pdata = drvdata.pdata;
    struct lm3530_pwm_data *pwm = &pdata.pwm_data;
    let mut max_brightness: u8 = led_cdev.max_brightness;
    switch (drvdata.mode) {
    case LM3530_BL_MODE_MANUAL:
    if (!drvdata.enable) {
    err = lm3530_init_registers(drvdata);
    if (err) {
    dev_err(&drvdata.client.dev,
    "Register Init failed: %d\n", err);
    break;
    }
    }
// set the brightness in brightness control register
    err = i2c_smbus_write_byte_data(drvdata.client,
    LM3530_BRT_CTRL_REG, brt_val);
    if (err)
    dev_err(&drvdata.client.dev,
    "Unable to set brightness: %d\n", err);
    else
    drvdata.brightness = brt_val;
    if (brt_val == 0)
    lm3530_led_disable(drvdata);
    break;
    case LM3530_BL_MODE_ALS:
    break;
    case LM3530_BL_MODE_PWM:
    if (pwm.pwm_set_intensity)
    pwm.pwm_set_intensity(brt_val, max_brightness);
    break;
    default:
    break;
    }
    }
    static ssize_t mode_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct led_classdev *led_cdev = dev_get_drvdata(dev);
    struct lm3530_data *drvdata;
    int i, len = 0;
    drvdata = container_of(led_cdev, struct lm3530_data, led_dev);
    for (i = 0; i < ARRAY_SIZE(mode_map); i++)
    if (drvdata.mode == mode_map[i].mode_val)
    len += sprintf(buf + len, "[%s] ", mode_map[i].mode);
    else
    len += sprintf(buf + len, "%s ", mode_map[i].mode);
    len += sprintf(buf + len, "\n");
    return len;
    }
    static ssize_t mode_store(struct device *dev, struct device_attribute
// attr, const char *buf, size_t size)
    {
    struct led_classdev *led_cdev = dev_get_drvdata(dev);
    struct lm3530_data *drvdata;
    struct lm3530_pwm_data *pwm;
    u8 max_brightness;
    int mode, err;
    drvdata = container_of(led_cdev, struct lm3530_data, led_dev);
    pwm = &drvdata.pdata.pwm_data;
    max_brightness = led_cdev.max_brightness;
    mode = lm3530_get_mode_from_str(buf);
    if (mode < 0) {
    dev_err(dev, "Invalid mode\n");
    return mode;
    }
    drvdata.mode = mode;
// set pwm to low if unnecessary
    if (mode != LM3530_BL_MODE_PWM && pwm.pwm_set_intensity)
    pwm.pwm_set_intensity(0, max_brightness);
    err = lm3530_init_registers(drvdata);
    if (err) {
    dev_err(dev, "Setting %s Mode failed :%d\n", buf, err);
    return err;
    }
    return sizeof(drvdata.mode);
    }
    static DEVICE_ATTR_RW(mode);
    static struct attribute *lm3530_attrs[] = {
    &dev_attr_mode.attr,
    core::ptr::null_mut()
    };
    ATTRIBUTE_GROUPS(lm3530);
#[no_mangle]
unsafe extern "C" fn lm3530_probe(client: *mut i2c_client) -> c_int {
    static int lm3530_probe(struct i2c_client *client)
    {
    struct lm3530_platform_data *pdata = dev_get_platdata(&client.dev);
    struct lm3530_data *drvdata;
    let mut err: c_int = 0;
    if (pdata == core::ptr::null_mut()) {
    dev_err(&client.dev, "platform data required\n");
    return -ENODEV;
    }
// BL mode
    if (pdata.mode > LM3530_BL_MODE_PWM) {
    dev_err(&client.dev, "Illegal Mode request\n");
    return -EINVAL;
    }
    if (!i2c_check_functionality(client.adapter, I2C_FUNC_I2C)) {
    dev_err(&client.dev, "I2C_FUNC_I2C not supported\n");
    return -EIO;
    }
    drvdata = devm_kzalloc(&client.dev, sizeof(struct lm3530_data),
    GFP_KERNEL);
    if (drvdata == core::ptr::null_mut())
    return -ENOMEM;
    drvdata.mode = pdata.mode;
    drvdata.client = client;
    drvdata.pdata = pdata;
    drvdata.brightness = LED_OFF;
    drvdata.enable = false;
    drvdata.led_dev.name = LM3530_LED_DEV;
    drvdata.led_dev.brightness_set = lm3530_brightness_set;
    drvdata.led_dev.max_brightness = MAX_BRIGHTNESS;
    drvdata.led_dev.groups = lm3530_groups;
    i2c_set_clientdata(client, drvdata);
    drvdata.regulator = devm_regulator_get(&client.dev, "vin");
    if (IS_ERR(drvdata.regulator)) {
    dev_err(&client.dev, "regulator get failed\n");
    err = PTR_ERR(drvdata.regulator);
    drvdata.regulator = core::ptr::null_mut();
    return err;
    }
    if (drvdata.pdata.brt_val) {
    err = lm3530_init_registers(drvdata);
    if (err < 0) {
    dev_err(&client.dev,
    "Register Init failed: %d\n", err);
    return err;
    }
    }
    err = led_classdev_register(&client.dev, &drvdata.led_dev);
    if (err < 0) {
    dev_err(&client.dev, "Register led class failed: %d\n", err);
    return err;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lm3530_remove(client: *mut i2c_client) {
    static void lm3530_remove(struct i2c_client *client)
    {
    struct lm3530_data *drvdata = i2c_get_clientdata(client);
    lm3530_led_disable(drvdata);
    led_classdev_unregister(&drvdata.led_dev);
    }
    static const struct i2c_device_id lm3530_id[] = {
    { .name = LM3530_NAME },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, lm3530_id);
    static struct i2c_driver lm3530_i2c_driver = {
    .probe = lm3530_probe,
    .remove = lm3530_remove,
    .id_table = lm3530_id,
    .driver = {
    .name = LM3530_NAME,
    },
    };
    module_i2c_driver(lm3530_i2c_driver);
    MODULE_DESCRIPTION("Back Light driver for LM3530");
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Shreshtha Kumar SAHU <shreshthakumar.sahu@stericsson.com>");
