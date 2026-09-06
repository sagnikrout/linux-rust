//! Automatically rewritten from C to Rust
//! Source: drivers/video/backlight/max25014.c
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
// Backlight driver for Maxim MAX25014
//
// Copyright (C) 2025 GOcontroll B.V.
// Author: Maud Spierings <maudspierings@gocontroll.com>
//

pub const MAX25014_ISET_DEFAULT_100: c_int = 11;
pub const MAX_BRIGHTNESS: c_int = 100;
pub const MIN_BRIGHTNESS: c_int = 0;

pub const TON_MIN: c_int = 0;
pub const MAX25014_DEV_ID: c_uint = 0x00;
pub const MAX25014_REV_ID: c_uint = 0x01;
pub const MAX25014_ISET: c_uint = 0x02;
pub const MAX25014_IMODE: c_uint = 0x03;
pub const MAX25014_TON1H: c_uint = 0x04;
pub const MAX25014_TON1L: c_uint = 0x05;
pub const MAX25014_TON2H: c_uint = 0x06;
pub const MAX25014_TON2L: c_uint = 0x07;
pub const MAX25014_TON3H: c_uint = 0x08;
pub const MAX25014_TON3L: c_uint = 0x09;
pub const MAX25014_TON4H: c_uint = 0x0A;
pub const MAX25014_TON4L: c_uint = 0x0B;
pub const MAX25014_TON_1_4_LSB: c_uint = 0x0C;
pub const MAX25014_SETTING: c_uint = 0x12;
pub const MAX25014_DISABLE: c_uint = 0x13;
pub const MAX25014_BSTMON: c_uint = 0x14;
pub const MAX25014_IOUT1: c_uint = 0x15;
pub const MAX25014_IOUT2: c_uint = 0x16;
pub const MAX25014_IOUT3: c_uint = 0x17;
pub const MAX25014_IOUT4: c_uint = 0x18;
pub const MAX25014_OPEN: c_uint = 0x1B;
pub const MAX25014_SHORTGND: c_uint = 0x1C;
pub const MAX25014_SHORTED_LED: c_uint = 0x1D;
pub const MAX25014_MASK: c_uint = 0x1E;
pub const MAX25014_DIAG: c_uint = 0x1F;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct max25014 {
    pub client: *mut i2c_client,
    pub bl: *mut backlight_device,
    pub regmap: *mut regmap,
    pub enable: *mut gpio_desc,
    pub iset: u32,
    pub strings_mask: u8,
}

    static const struct regmap_config max25014_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = MAX25014_DIAG,
    };
#[no_mangle]
unsafe extern "C" fn max25014_initial_power_state(maxim: *mut max25014) -> c_int {
    static int max25014_initial_power_state(struct max25014 *maxim)
    {
    uint32_t val;
    int ret;
    ret = regmap_read(maxim.regmap, MAX25014_ISET, &val);
    if (ret)
    return ret;
    return val & MAX25014_ISET_ENA ? BACKLIGHT_POWER_ON : BACKLIGHT_POWER_OFF;
    }
#[no_mangle]
unsafe extern "C" fn max25014_check_errors(maxim: *mut max25014) -> c_int {
    static int max25014_check_errors(struct max25014 *maxim)
    {
    uint32_t val;
    uint8_t i;
    int ret;
    ret = regmap_read(maxim.regmap, MAX25014_OPEN, &val);
    if (ret)
    return ret;
    if (val) {
    dev_err(&maxim.client.dev, "Open led strings detected on:\n");
    for (i = 0; i < 4; i++) {
    if (val & 1 << i)
    dev_err(&maxim.client.dev, "string %d\n", i + 1);
    }
    return -EIO;
    }
    ret = regmap_read(maxim.regmap, MAX25014_SHORTGND, &val);
    if (ret)
    return ret;
    if (val) {
    dev_err(&maxim.client.dev, "Short to ground detected on:\n");
    for (i = 0; i < 4; i++) {
    if (val & 1 << i)
    dev_err(&maxim.client.dev, "string %d\n", i + 1);
    }
    return -EIO;
    }
    ret = regmap_read(maxim.regmap, MAX25014_SHORTED_LED, &val);
    if (ret)
    return ret;
    if (val) {
    dev_err(&maxim.client.dev, "Shorted led detected on:\n");
    for (i = 0; i < 4; i++) {
    if (val & 1 << i)
    dev_err(&maxim.client.dev, "string %d\n", i + 1);
    }
    return -EIO;
    }
    ret = regmap_read(maxim.regmap, MAX25014_DIAG, &val);
    if (ret)
    return ret;
//
// The HW_RST bit always starts at 1 after power up.
// It is reset on first read, does not indicate an error.
//
    if (val && val != MAX25014_DIAG_HW_RST) {
    if (val & MAX25014_DIAG_OT)
    dev_err(&maxim.client.dev,
    "Overtemperature shutdown\n");
    if (val & MAX25014_DIAG_OTW)
    dev_err(&maxim.client.dev,
    "Chip is getting too hot (>125C)\n");
    if (val & MAX25014_DIAG_BSTOV)
    dev_err(&maxim.client.dev,
    "Boost converter overvoltage\n");
    if (val & MAX25014_DIAG_BSTUV)
    dev_err(&maxim.client.dev,
    "Boost converter undervoltage\n");
    if (val & MAX25014_DIAG_IREFOOR)
    dev_err(&maxim.client.dev, "IREF out of range\n");
    return -EIO;
    }
    return 0;
    }
//
// 1. disable unused strings
// 2. set dim mode
// 3. set setting register
// 4. enable the backlight
//
#[no_mangle]
unsafe extern "C" fn max25014_configure(maxim: *mut max25014, initial_state: c_int) -> c_int {
    static int max25014_configure(struct max25014 *maxim, int initial_state)
    {
    uint32_t val;
    int ret;
    ret = regmap_read(maxim.regmap, MAX25014_DISABLE, &val);
    if (ret)
    return ret;
//
// Strings can only be disabled when MAX25014_ISET_ENA == 0, check if
// it needs to be changed at all to prevent the backlight flashing when
// it is configured correctly by the bootloader
//
    if (!((val & MAX25014_DISABLE_DIS_MASK) == maxim.strings_mask)) {
    if (initial_state == BACKLIGHT_POWER_ON) {
    ret = regmap_write(maxim.regmap, MAX25014_ISET, 0);
    if (ret)
    return ret;
    }
    ret = regmap_write(maxim.regmap, MAX25014_DISABLE, maxim.strings_mask);
    if (ret)
    return ret;
    }
    ret = regmap_write(maxim.regmap, MAX25014_IMODE, MAX25014_IMODE_HDIM);
    if (ret)
    return ret;
    ret = regmap_read(maxim.regmap, MAX25014_SETTING, &val);
    if (ret)
    return ret;
    ret = regmap_write(maxim.regmap, MAX25014_SETTING,
    val & ~MAX25014_SETTING_FPWM);
    if (ret)
    return ret;
    return regmap_write(maxim.regmap, MAX25014_ISET,
    maxim.iset | MAX25014_ISET_ENA |
    MAX25014_ISET_PSEN);
    }
#[no_mangle]
unsafe extern "C" fn max25014_update_status(bl_dev: *mut backlight_device) -> c_int {
    static int max25014_update_status(struct backlight_device *bl_dev)
    {
    struct max25014 *maxim = bl_get_data(bl_dev);
    uint32_t reg;
    int ret;
    reg  = TON_STEP * backlight_get_brightness(bl_dev);
//
// 18 bit number lowest, 2 bits in first register,
// next lowest 8 in the L register, next 8 in the H register
// Seemingly setting the strength of only one string controls all of
// them, individual settings don't affect the outcome.
//
    ret = regmap_write(maxim.regmap, MAX25014_TON_1_4_LSB, reg & 0b00000011);
    if (ret != 0)
    return ret;
    ret = regmap_write(maxim.regmap, MAX25014_TON1L, (reg >> 2) & 0b11111111);
    if (ret != 0)
    return ret;
    return regmap_write(maxim.regmap, MAX25014_TON1H, (reg >> 10) & 0b11111111);
    }
    static const struct backlight_ops max25014_bl_ops = {
    .options = BL_CORE_SUSPENDRESUME,
    .update_status = max25014_update_status,
    };
    static int max25014_parse_dt(struct max25014 *maxim,
    uint32_t *initial_brightness)
    {
    struct device *dev = &maxim.client.dev;
    struct device_node *node = dev.of_node;
    uint32_t strings[4];
    int res, i;
    res = of_property_count_u32_elems(node, "maxim,strings");
    if (res == 4) {
    of_property_read_u32_array(node, "maxim,strings", strings, 4);
    for (i = 0; i < 4; i++) {
    if (strings[i] == 0)
    maxim.strings_mask |= 1 << i;
    }
    } else {
    maxim.strings_mask = 0;
    }
// initial_brightness = 50U;
    of_property_read_u32(node, "default-brightness", initial_brightness);
    maxim.iset = MAX25014_ISET_DEFAULT_100;
    of_property_read_u32(node, "maxim,iset", &maxim.iset);
    if (maxim.iset > 15)
    return dev_err_probe(dev, -EINVAL,
    "Invalid iset, should be a value from 0-15, entered was %d\n",
    maxim.iset);
    if (*initial_brightness > 100)
    return dev_err_probe(dev, -EINVAL,
    "Invalid initial brightness, should be a value from 0-100, entered was %d\n",
// initial_brightness);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn max25014_probe(cl: *mut i2c_client) -> c_int {
    static int max25014_probe(struct i2c_client *cl)
    {
    const struct i2c_device_id *id = i2c_client_get_device_id(cl);
    struct backlight_properties props;
    let mut initial_brightness: u32 = 50;
    struct backlight_device *bl;
    struct max25014 *maxim;
    int ret;
    maxim = devm_kzalloc(&cl.dev, sizeof(struct max25014), GFP_KERNEL);
    if (!maxim)
    return -ENOMEM;
    maxim.client = cl;
    ret = max25014_parse_dt(maxim, &initial_brightness);
    if (ret)
    return ret;
    ret = devm_regulator_get_enable(&maxim.client.dev, "power");
    if (ret)
    return dev_err_probe(&maxim.client.dev, ret,
    "failed to get power-supply");
    maxim.enable = devm_gpiod_get_optional(&maxim.client.dev, "enable",
    GPIOD_OUT_HIGH);
    if (IS_ERR(maxim.enable))
    return dev_err_probe(&maxim.client.dev, PTR_ERR(maxim.enable),
    "failed to get enable gpio\n");
// Datasheet Electrical Characteristics tSTARTUP 2ms
    fsleep(2000);
    maxim.regmap = devm_regmap_init_i2c(cl, &max25014_regmap_config);
    if (IS_ERR(maxim.regmap))
    return dev_err_probe(&maxim.client.dev, PTR_ERR(maxim.regmap),
    "failed to initialize the i2c regmap\n");
    i2c_set_clientdata(cl, maxim);
    ret = max25014_check_errors(maxim);
    if (ret) /* error is already reported in the above function */
    return ret;
    ret = max25014_initial_power_state(maxim);
    if (ret < 0)
    return dev_err_probe(&maxim.client.dev, ret, "Could not get enabled state\n");
    memset(&props, 0, sizeof(struct backlight_properties));
    props.type = BACKLIGHT_PLATFORM;
    props.max_brightness = MAX_BRIGHTNESS;
    props.brightness = initial_brightness;
    props.scale = BACKLIGHT_SCALE_LINEAR;
    props.power = ret;
    ret = max25014_configure(maxim, ret);
    if (ret)
    return dev_err_probe(&maxim.client.dev, ret, "device config error");
    bl = devm_backlight_device_register(&maxim.client.dev, id.name,
    &maxim.client.dev, maxim,
    &max25014_bl_ops, &props);
    if (IS_ERR(bl))
    return dev_err_probe(&maxim.client.dev, PTR_ERR(bl),
    "failed to register backlight\n");
    maxim.bl = bl;
    backlight_update_status(maxim.bl);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn max25014_remove(cl: *mut i2c_client) {
    static void max25014_remove(struct i2c_client *cl)
    {
    struct max25014 *maxim = i2c_get_clientdata(cl);
    backlight_device_set_brightness(maxim.bl, 0);
    gpiod_set_value_cansleep(maxim.enable, 0);
    }
    static const struct of_device_id max25014_dt_ids[] = {
    { .compatible = "maxim,max25014", },
    { }
    };
    MODULE_DEVICE_TABLE(of, max25014_dt_ids);
    static const struct i2c_device_id max25014_ids[] = {
    { "max25014" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, max25014_ids);
    static struct i2c_driver max25014_driver = {
    .driver = {
    .name = KBUILD_MODNAME,
    .of_match_table = of_match_ptr(max25014_dt_ids),
    },
    .probe = max25014_probe,
    .remove = max25014_remove,
    .id_table = max25014_ids,
    };
    module_i2c_driver(max25014_driver);
    MODULE_DESCRIPTION("Maxim MAX25014 backlight driver");
    MODULE_AUTHOR("Maud Spierings <maudspierings@gocontroll.com>");
    MODULE_LICENSE("GPL");
