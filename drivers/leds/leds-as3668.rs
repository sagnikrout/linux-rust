//! Automatically rewritten from C to Rust
//! Source: drivers/leds/leds-as3668.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Osram AMS AS3668 LED Driver IC
//
// Copyright (C) 2025 Lukas Timmermann <linux@timmermann.space>
//

pub const AS3668_MAX_LEDS: c_int = 4;
// Chip Ident
pub const AS3668_CHIP_ID1_REG: c_uint = 0x3e;
pub const AS3668_CHIP_ID: c_uint = 0xa5;
// Current Control
pub const AS3668_CURR_MODE_REG: c_uint = 0x01;
pub const AS3668_CURR_MODE_OFF: c_uint = 0x0;
pub const AS3668_CURR_MODE_ON: c_uint = 0x1;

pub const AS3668_CURR1_REG: c_uint = 0x02;
pub const AS3668_CURR2_REG: c_uint = 0x03;
pub const AS3668_CURR3_REG: c_uint = 0x04;
pub const AS3668_CURR4_REG: c_uint = 0x05;

    ((mode) << 2) | \
    ((mode) << 4) | \
    ((mode) << 6))
#[repr(C)]
#[derive(Copy, Clone)]
pub struct as3668_led {
    pub cdev: led_classdev,
    pub chip: *mut as3668,
    pub fwnode: *mut fwnode_handle,
    pub mode_mask: u8,
    pub current_reg: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct as3668 {
    pub client: *mut i2c_client,
    pub leds: [as3668_led; AS3668_MAX_LEDS],
}

#[no_mangle]
unsafe extern "C" fn as3668_channel_mode_set(led: *mut as3668_led, mode: u8) -> c_int {
    static int as3668_channel_mode_set(struct as3668_led *led, u8 mode)
    {
    int ret;
    u8 channel_modes;
    ret = i2c_smbus_read_byte_data(led.chip.client, AS3668_CURR_MODE_REG);
    if (ret < 0) {
    dev_err(led.cdev.dev, "failed to read channel modes\n");
    return ret;
    }
    channel_modes = (u8)ret;
    channel_modes &= ~led.mode_mask;
    channel_modes |= led.mode_mask & (AS3668_CURR_MODE_PACK(mode));
    return i2c_smbus_write_byte_data(led.chip.client, AS3668_CURR_MODE_REG, channel_modes);
    }
#[no_mangle]
unsafe extern "C" fn as3668_brightness_get(cdev: *mut led_classdev) -> enum led_brightness {
    static enum led_brightness as3668_brightness_get(struct led_classdev *cdev)
    {
    struct as3668_led *led = container_of(cdev, struct as3668_led, cdev);
    return i2c_smbus_read_byte_data(led.chip.client, led.current_reg);
    }
#[no_mangle]
unsafe extern "C" fn as3668_brightness_set(cdev: *mut led_classdev, brightness: enum led_brightness) {
    static void as3668_brightness_set(struct led_classdev *cdev, enum led_brightness brightness)
    {
    struct as3668_led *led = container_of(cdev, struct as3668_led, cdev);
    int err;
    err = as3668_channel_mode_set(led, !!brightness);
    if (err)
    dev_err(cdev.dev, "failed to set channel mode: %d\n", err);
    err = i2c_smbus_write_byte_data(led.chip.client, led.current_reg, brightness);
    if (err)
    dev_err(cdev.dev, "failed to set brightness: %d\n", err);
    }
#[no_mangle]
unsafe extern "C" fn as3668_dt_init(as3668: *mut as3668) -> c_int {
    static int as3668_dt_init(struct as3668 *as3668)
    {
    struct device *dev = &as3668.client.dev;
    struct as3668_led *led;
    let mut init_data: led_init_data = {};
    int err;
    u32 reg;
    for_each_available_child_of_node_scoped(dev_of_node(dev), child) {
    err = of_property_read_u32(child, "reg", &reg);
    if (err)
    return dev_err_probe(dev, err, "failed to read 'reg' property");
    if (reg < 0 || reg >= AS3668_MAX_LEDS)
    return dev_err_probe(dev, -EINVAL,
    "unsupported LED: %d\n", reg);
    led = &as3668.leds[reg];
    led.fwnode = of_fwnode_handle(child);
    led.current_reg = reg + AS3668_CURR1_REG;
    led.mode_mask = AS3668_CURR1_MODE_MASK << (reg * 2);
    led.chip = as3668;
    led.cdev.max_brightness = U8_MAX;
    led.cdev.brightness_get = as3668_brightness_get;
    led.cdev.brightness_set = as3668_brightness_set;
    init_data.fwnode = led.fwnode;
    init_data.default_label = ":";
    err = devm_led_classdev_register_ext(dev, &led.cdev, &init_data);
    if (err)
    return dev_err_probe(dev, err, "failed to register LED %d\n", reg);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn as3668_probe(client: *mut i2c_client) -> c_int {
    static int as3668_probe(struct i2c_client *client)
    {
    struct as3668 *as3668;
    int err;
    u8 chip_id;
    chip_id = i2c_smbus_read_byte_data(client, AS3668_CHIP_ID1_REG);
    if (chip_id != AS3668_CHIP_ID)
    return dev_err_probe(&client.dev, -ENODEV,
    "expected chip ID 0x%02x, got 0x%02x\n",
    AS3668_CHIP_ID, chip_id);
    as3668 = devm_kzalloc(&client.dev, sizeof(*as3668), GFP_KERNEL);
    if (!as3668)
    return -ENOMEM;
    as3668.client = client;
    err = as3668_dt_init(as3668);
    if (err)
    return err;
// Set all four channel modes to 'off'
    err = i2c_smbus_write_byte_data(client, AS3668_CURR_MODE_REG,
    FIELD_PREP(AS3668_CURR1_MODE_MASK, AS3668_CURR_MODE_OFF) |
    FIELD_PREP(AS3668_CURR2_MODE_MASK, AS3668_CURR_MODE_OFF) |
    FIELD_PREP(AS3668_CURR3_MODE_MASK, AS3668_CURR_MODE_OFF) |
    FIELD_PREP(AS3668_CURR4_MODE_MASK, AS3668_CURR_MODE_OFF));
// Set initial currents to 0mA
    err |= i2c_smbus_write_byte_data(client, AS3668_CURR1_REG, 0);
    err |= i2c_smbus_write_byte_data(client, AS3668_CURR2_REG, 0);
    err |= i2c_smbus_write_byte_data(client, AS3668_CURR3_REG, 0);
    err |= i2c_smbus_write_byte_data(client, AS3668_CURR4_REG, 0);
    if (err)
    return dev_err_probe(&client.dev, -EIO, "failed to set zero initial current levels\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn as3668_remove(client: *mut i2c_client) {
    static void as3668_remove(struct i2c_client *client)
    {
    i2c_smbus_write_byte_data(client, AS3668_CURR_MODE_REG, 0);
    }
    static const struct i2c_device_id as3668_idtable[] = {
    { .name = "as3668" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, as3668_idtable);
    static const struct of_device_id as3668_match_table[] = {
    { .compatible = "ams,as3668" },
    { }
    };
    MODULE_DEVICE_TABLE(of, as3668_match_table);
    static struct i2c_driver as3668_driver = {
    .driver = {
    .name = "leds_as3668",
    .of_match_table = as3668_match_table,
    },
    .probe = as3668_probe,
    .remove = as3668_remove,
    .id_table = as3668_idtable,
    };
    module_i2c_driver(as3668_driver);
    MODULE_AUTHOR("Lukas Timmermann <linux@timmermann.space>");
    MODULE_DESCRIPTION("AS3668 LED driver");
    MODULE_LICENSE("GPL");
