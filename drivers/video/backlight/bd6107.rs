//! Automatically rewritten from C to Rust
//! Source: drivers/video/backlight/bd6107.c
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
// ROHM Semiconductor BD6107 LED Driver
//
// Copyright (C) 2013 Ideas on board SPRL
//
// Contact: Laurent Pinchart <laurent.pinchart@ideasonboard.com>
//

pub const BD6107_PSCNT1: c_uint = 0x00;

pub const BD6107_REGVSET: c_uint = 0x02;

pub const BD6107_LEDCNT1: c_uint = 0x03;

pub const BD6107_PORTSEL: c_uint = 0x04;

pub const BD6107_RGB1CNT1: c_uint = 0x05;
pub const BD6107_RGB1CNT2: c_uint = 0x06;
pub const BD6107_RGB1CNT3: c_uint = 0x07;
pub const BD6107_RGB1CNT4: c_uint = 0x08;
pub const BD6107_RGB1CNT5: c_uint = 0x09;
pub const BD6107_RGB1FLM: c_uint = 0x0a;
pub const BD6107_RGB2CNT1: c_uint = 0x0b;
pub const BD6107_RGB2CNT2: c_uint = 0x0c;
pub const BD6107_RGB2CNT3: c_uint = 0x0d;
pub const BD6107_RGB2CNT4: c_uint = 0x0e;
pub const BD6107_RGB2CNT5: c_uint = 0x0f;
pub const BD6107_RGB2FLM: c_uint = 0x10;
pub const BD6107_PSCONT3: c_uint = 0x11;
pub const BD6107_SMMONCNT: c_uint = 0x12;
pub const BD6107_DCDCCNT: c_uint = 0x13;
pub const BD6107_IOSEL: c_uint = 0x14;
pub const BD6107_OUT1: c_uint = 0x15;
pub const BD6107_OUT2: c_uint = 0x16;
pub const BD6107_MASK1: c_uint = 0x17;
pub const BD6107_MASK2: c_uint = 0x18;
pub const BD6107_FACTOR1: c_uint = 0x19;
pub const BD6107_FACTOR2: c_uint = 0x1a;
pub const BD6107_CLRFACT1: c_uint = 0x1b;
pub const BD6107_CLRFACT2: c_uint = 0x1c;
pub const BD6107_STATE1: c_uint = 0x1d;
pub const BD6107_LSIVER: c_uint = 0x1e;
pub const BD6107_GRPSEL: c_uint = 0x1f;
pub const BD6107_LEDCNT2: c_uint = 0x20;
pub const BD6107_LEDCNT3: c_uint = 0x21;
pub const BD6107_MCURRENT: c_uint = 0x22;
pub const BD6107_MAINCNT1: c_uint = 0x23;
pub const BD6107_MAINCNT2: c_uint = 0x24;
pub const BD6107_SLOPECNT: c_uint = 0x25;
pub const BD6107_MSLOPE: c_uint = 0x26;
pub const BD6107_RGBSLOPE: c_uint = 0x27;
pub const BD6107_TEST: c_uint = 0x29;
pub const BD6107_SFTRST: c_uint = 0x2a;
pub const BD6107_SFTRSTGD: c_uint = 0x2b;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bd6107 {
    pub client: *mut i2c_client,
    pub backlight: *mut backlight_device,
    pub pdata: *mut bd6107_platform_data,
    pub reset: *mut gpio_desc,
}

#[no_mangle]
unsafe extern "C" fn bd6107_write(bd: *mut bd6107, reg: u8, data: u8) -> c_int {
    static int bd6107_write(struct bd6107 *bd, u8 reg, u8 data)
    {
    return i2c_smbus_write_byte_data(bd.client, reg, data);
    }
#[no_mangle]
unsafe extern "C" fn bd6107_backlight_update_status(backlight: *mut backlight_device) -> c_int {
    static int bd6107_backlight_update_status(struct backlight_device *backlight)
    {
    struct bd6107 *bd = bl_get_data(backlight);
    let mut brightness: c_int = backlight_get_brightness(backlight);
    if (brightness) {
    bd6107_write(bd, BD6107_PORTSEL, BD6107_PORTSEL_LEDM(2) |
    BD6107_PORTSEL_LEDM(1) | BD6107_PORTSEL_LEDM(0));
    bd6107_write(bd, BD6107_MAINCNT1, brightness);
    bd6107_write(bd, BD6107_LEDCNT1, BD6107_LEDCNT1_LEDONOFF1);
    } else {
// Assert the reset line (gpiolib will handle active low)
    gpiod_set_value(bd.reset, 1);
    msleep(24);
    gpiod_set_value(bd.reset, 0);
    }
    return 0;
    }
    static bool bd6107_backlight_controls_device(struct backlight_device *backlight,
    struct device *display_dev)
    {
    struct bd6107 *bd = bl_get_data(backlight);
    return !bd.pdata.dev || bd.pdata.dev == display_dev;
    }
    static const struct backlight_ops bd6107_backlight_ops = {
    .options	 = BL_CORE_SUSPENDRESUME,
    .update_status	 = bd6107_backlight_update_status,
    .controls_device = bd6107_backlight_controls_device,
    };
#[no_mangle]
unsafe extern "C" fn bd6107_probe(client: *mut i2c_client) -> c_int {
    static int bd6107_probe(struct i2c_client *client)
    {
    struct bd6107_platform_data *pdata = dev_get_platdata(&client.dev);
    struct backlight_device *backlight;
    struct backlight_properties props;
    struct bd6107 *bd;
    if (pdata == core::ptr::null_mut()) {
    dev_err(&client.dev, "No platform data\n");
    return -EINVAL;
    }
    if (!i2c_check_functionality(client.adapter,
    I2C_FUNC_SMBUS_BYTE_DATA)) {
    dev_warn(&client.dev,
    "I2C adapter doesn't support I2C_FUNC_SMBUS_BYTE\n");
    return -EIO;
    }
    bd = devm_kzalloc(&client.dev, sizeof(*bd), GFP_KERNEL);
    if (!bd)
    return -ENOMEM;
    bd.client = client;
    bd.pdata = pdata;
//
// Request the reset GPIO line with GPIOD_OUT_HIGH meaning asserted,
// so in the machine descriptor table (or other hardware description),
// the line should be flagged as active low so this will assert
// the reset.
//
    bd.reset = devm_gpiod_get(&client.dev, "reset", GPIOD_OUT_HIGH);
    if (IS_ERR(bd.reset))
    return dev_err_probe(&client.dev, PTR_ERR(bd.reset),
    "unable to request reset GPIO\n");
    memset(&props, 0, sizeof(props));
    props.type = BACKLIGHT_RAW;
    props.max_brightness = 128;
    props.brightness = clamp_t(unsigned int, pdata.def_value, 0,
    props.max_brightness);
    backlight = devm_backlight_device_register(&client.dev,
    dev_name(&client.dev),
    &bd.client.dev, bd,
    &bd6107_backlight_ops, &props);
    if (IS_ERR(backlight)) {
    dev_err(&client.dev, "failed to register backlight\n");
    return PTR_ERR(backlight);
    }
    backlight_update_status(backlight);
    i2c_set_clientdata(client, backlight);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bd6107_remove(client: *mut i2c_client) {
    static void bd6107_remove(struct i2c_client *client)
    {
    struct backlight_device *backlight = i2c_get_clientdata(client);
    backlight.props.brightness = 0;
    backlight_update_status(backlight);
    }
    static const struct i2c_device_id bd6107_ids[] = {
    { .name = "bd6107" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, bd6107_ids);
    static struct i2c_driver bd6107_driver = {
    .driver = {
    .name = "bd6107",
    },
    .probe = bd6107_probe,
    .remove = bd6107_remove,
    .id_table = bd6107_ids,
    };
    module_i2c_driver(bd6107_driver);
    MODULE_DESCRIPTION("Rohm BD6107 Backlight Driver");
    MODULE_AUTHOR("Laurent Pinchart <laurent.pinchart@ideasonboard.com>");
    MODULE_LICENSE("GPL");
