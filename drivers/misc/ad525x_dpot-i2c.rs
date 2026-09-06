//! Automatically rewritten from C to Rust
//! Source: drivers/misc/ad525x_dpot-i2c.c
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
// Driver for the Analog Devices digital potentiometers (I2C bus)
//
// Copyright (C) 2010-2011 Michael Hennerich, Analog Devices Inc.
//

// I2C bus functions
#[no_mangle]
unsafe extern "C" fn write_d8(client: *mut c_void, val: u8) -> c_int {
    static int write_d8(void *client, u8 val)
    {
    return i2c_smbus_write_byte(client, val);
    }
#[no_mangle]
unsafe extern "C" fn write_r8d8(client: *mut c_void, reg: u8, val: u8) -> c_int {
    static int write_r8d8(void *client, u8 reg, u8 val)
    {
    return i2c_smbus_write_byte_data(client, reg, val);
    }
#[no_mangle]
unsafe extern "C" fn write_r8d16(client: *mut c_void, reg: u8, val: u16) -> c_int {
    static int write_r8d16(void *client, u8 reg, u16 val)
    {
    return i2c_smbus_write_word_data(client, reg, val);
    }
#[no_mangle]
unsafe extern "C" fn read_d8(client: *mut c_void) -> c_int {
    static int read_d8(void *client)
    {
    return i2c_smbus_read_byte(client);
    }
#[no_mangle]
unsafe extern "C" fn read_r8d8(client: *mut c_void, reg: u8) -> c_int {
    static int read_r8d8(void *client, u8 reg)
    {
    return i2c_smbus_read_byte_data(client, reg);
    }
#[no_mangle]
unsafe extern "C" fn read_r8d16(client: *mut c_void, reg: u8) -> c_int {
    static int read_r8d16(void *client, u8 reg)
    {
    return i2c_smbus_read_word_data(client, reg);
    }
    static const struct ad_dpot_bus_ops bops = {
    .read_d8	= read_d8,
    .read_r8d8	= read_r8d8,
    .read_r8d16	= read_r8d16,
    .write_d8	= write_d8,
    .write_r8d8	= write_r8d8,
    .write_r8d16	= write_r8d16,
    };
#[no_mangle]
unsafe extern "C" fn ad_dpot_i2c_probe(client: *mut i2c_client) -> c_int {
    static int ad_dpot_i2c_probe(struct i2c_client *client)
    {
    const struct i2c_device_id *id = i2c_client_get_device_id(client);
    struct ad_dpot_bus_data bdata = {
    .client = client,
    .bops = &bops,
    };
    if (!i2c_check_functionality(client.adapter,
    I2C_FUNC_SMBUS_WORD_DATA)) {
    dev_err(&client.dev, "SMBUS Word Data not Supported\n");
    return -EIO;
    }
    return ad_dpot_probe(&client.dev, &bdata, id.driver_data, id.name);
    }
#[no_mangle]
unsafe extern "C" fn ad_dpot_i2c_remove(client: *mut i2c_client) {
    static void ad_dpot_i2c_remove(struct i2c_client *client)
    {
    ad_dpot_remove(&client.dev);
    }
    static const struct i2c_device_id ad_dpot_id[] = {
    { .name = "ad5258", .driver_data = AD5258_ID },
    { .name = "ad5259", .driver_data = AD5259_ID },
    { .name = "ad5251", .driver_data = AD5251_ID },
    { .name = "ad5252", .driver_data = AD5252_ID },
    { .name = "ad5253", .driver_data = AD5253_ID },
    { .name = "ad5254", .driver_data = AD5254_ID },
    { .name = "ad5255", .driver_data = AD5255_ID },
    { .name = "ad5241", .driver_data = AD5241_ID },
    { .name = "ad5242", .driver_data = AD5242_ID },
    { .name = "ad5243", .driver_data = AD5243_ID },
    { .name = "ad5245", .driver_data = AD5245_ID },
    { .name = "ad5246", .driver_data = AD5246_ID },
    { .name = "ad5247", .driver_data = AD5247_ID },
    { .name = "ad5248", .driver_data = AD5248_ID },
    { .name = "ad5280", .driver_data = AD5280_ID },
    { .name = "ad5282", .driver_data = AD5282_ID },
    { .name = "adn2860", .driver_data = ADN2860_ID },
    { .name = "ad5273", .driver_data = AD5273_ID },
    { .name = "ad5161", .driver_data = AD5161_ID },
    { .name = "ad5171", .driver_data = AD5171_ID },
    { .name = "ad5170", .driver_data = AD5170_ID },
    { .name = "ad5172", .driver_data = AD5172_ID },
    { .name = "ad5173", .driver_data = AD5173_ID },
    { .name = "ad5272", .driver_data = AD5272_ID },
    { .name = "ad5274", .driver_data = AD5274_ID },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, ad_dpot_id);
    static struct i2c_driver ad_dpot_i2c_driver = {
    .driver = {
    .name	= "ad_dpot",
    .dev_groups = ad_dpot_groups,
    },
    .probe		= ad_dpot_i2c_probe,
    .remove		= ad_dpot_i2c_remove,
    .id_table	= ad_dpot_id,
    };
    module_i2c_driver(ad_dpot_i2c_driver);
    MODULE_AUTHOR("Michael Hennerich <michael.hennerich@analog.com>");
    MODULE_DESCRIPTION("digital potentiometer I2C bus driver");
    MODULE_LICENSE("GPL");
