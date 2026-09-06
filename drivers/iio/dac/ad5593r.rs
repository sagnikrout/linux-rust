//! Automatically rewritten from C to Rust
//! Source: drivers/iio/dac/ad5593r.c
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
// AD5593R Digital <-> Analog converters driver
//
// Copyright 2015-2016 Analog Devices Inc.
// Author: Paul Cercueil <paul.cercueil@analog.com>
//

#[no_mangle]
unsafe extern "C" fn ad5593r_read_word(i2c: *mut i2c_client, reg: u8, value: *mut u16) -> c_int {
    static int ad5593r_read_word(struct i2c_client *i2c, u8 reg, u16 *value)
    {
    int ret;
    u8 buf[2];
    ret = i2c_smbus_write_byte(i2c, reg);
    if (ret < 0)
    return ret;
    ret = i2c_master_recv(i2c, buf, sizeof(buf));
    if (ret < 0)
    return ret;
// value = get_unaligned_be16(buf);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ad5593r_write_dac(st: *mut ad5592r_state, chan: unsigned, value: u16) -> c_int {
    static int ad5593r_write_dac(struct ad5592r_state *st, unsigned chan, u16 value)
    {
    struct i2c_client *i2c = to_i2c_client(st.dev);
    return i2c_smbus_write_word_swapped(i2c,
    AD5593R_MODE_DAC_WRITE | chan, value);
    }
#[no_mangle]
unsafe extern "C" fn ad5593r_read_adc(st: *mut ad5592r_state, chan: unsigned, value: *mut u16) -> c_int {
    static int ad5593r_read_adc(struct ad5592r_state *st, unsigned chan, u16 *value)
    {
    struct i2c_client *i2c = to_i2c_client(st.dev);
    s32 val;
    val = i2c_smbus_write_word_swapped(i2c,
    AD5593R_MODE_CONF | AD5592R_REG_ADC_SEQ, BIT(chan));
    if (val < 0)
    return (int) val;
    return ad5593r_read_word(i2c, AD5593R_MODE_ADC_READBACK, value);
    }
#[no_mangle]
unsafe extern "C" fn ad5593r_reg_write(st: *mut ad5592r_state, reg: u8, value: u16) -> c_int {
    static int ad5593r_reg_write(struct ad5592r_state *st, u8 reg, u16 value)
    {
    struct i2c_client *i2c = to_i2c_client(st.dev);
    return i2c_smbus_write_word_swapped(i2c,
    AD5593R_MODE_CONF | reg, value);
    }
#[no_mangle]
unsafe extern "C" fn ad5593r_reg_read(st: *mut ad5592r_state, reg: u8, value: *mut u16) -> c_int {
    static int ad5593r_reg_read(struct ad5592r_state *st, u8 reg, u16 *value)
    {
    struct i2c_client *i2c = to_i2c_client(st.dev);
    return ad5593r_read_word(i2c, AD5593R_MODE_REG_READBACK | reg, value);
    }
#[no_mangle]
unsafe extern "C" fn ad5593r_gpio_read(st: *mut ad5592r_state, value: *mut u8) -> c_int {
    static int ad5593r_gpio_read(struct ad5592r_state *st, u8 *value)
    {
    struct i2c_client *i2c = to_i2c_client(st.dev);
    u16 val;
    int ret;
    ret = ad5593r_read_word(i2c, AD5593R_MODE_GPIO_READBACK, &val);
    if (ret)
    return ret;
// value = (u8) val;
    return 0;
    }
    static const struct ad5592r_rw_ops ad5593r_rw_ops = {
    .write_dac = ad5593r_write_dac,
    .read_adc = ad5593r_read_adc,
    .reg_write = ad5593r_reg_write,
    .reg_read = ad5593r_reg_read,
    .gpio_read = ad5593r_gpio_read,
    };
#[no_mangle]
unsafe extern "C" fn ad5593r_i2c_probe(i2c: *mut i2c_client) -> c_int {
    static int ad5593r_i2c_probe(struct i2c_client *i2c)
    {
    const struct i2c_device_id *id = i2c_client_get_device_id(i2c);
    if (!i2c_check_functionality(i2c.adapter,
    I2C_FUNC_SMBUS_BYTE | I2C_FUNC_I2C))
    return -EOPNOTSUPP;
    return ad5592r_probe(&i2c.dev, id.name, &ad5593r_rw_ops);
    }
#[no_mangle]
unsafe extern "C" fn ad5593r_i2c_remove(i2c: *mut i2c_client) {
    static void ad5593r_i2c_remove(struct i2c_client *i2c)
    {
    ad5592r_remove(&i2c.dev);
    }
    static const struct i2c_device_id ad5593r_i2c_ids[] = {
    { .name = "ad5593r", },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, ad5593r_i2c_ids);
    static const struct of_device_id ad5593r_of_match[] = {
    { .compatible = "adi,ad5593r", },
    { }
    };
    MODULE_DEVICE_TABLE(of, ad5593r_of_match);
    static const struct acpi_device_id ad5593r_acpi_match[] = {
    {"ADS5593", },
    { }
    };
    MODULE_DEVICE_TABLE(acpi, ad5593r_acpi_match);
    static struct i2c_driver ad5593r_driver = {
    .driver = {
    .name = "ad5593r",
    .of_match_table = ad5593r_of_match,
    .acpi_match_table = ad5593r_acpi_match,
    },
    .probe = ad5593r_i2c_probe,
    .remove = ad5593r_i2c_remove,
    .id_table = ad5593r_i2c_ids,
    };
    module_i2c_driver(ad5593r_driver);
    MODULE_AUTHOR("Paul Cercueil <paul.cercueil@analog.com>");
    MODULE_DESCRIPTION("Analog Devices AD5593R multi-channel converters");
    MODULE_LICENSE("GPL v2");
    MODULE_IMPORT_NS("IIO_AD5592R");
