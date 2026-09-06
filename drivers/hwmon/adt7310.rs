//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/adt7310.c
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
// ADT7310/ADT7310 digital temperature sensor driver
//
// Copyright 2012-2013 Analog Devices Inc.
// Author: Lars-Peter Clausen <lars@metafoo.de>
//

pub const ADT7310_STATUS: c_int = 0;
pub const ADT7310_CONFIG: c_int = 1;
pub const ADT7310_TEMPERATURE: c_int = 2;
pub const ADT7310_ID: c_int = 3;
pub const ADT7310_T_CRIT: c_int = 4;
pub const ADT7310_T_HYST: c_int = 5;
pub const ADT7310_T_ALARM_HIGH: c_int = 6;
pub const ADT7310_T_ALARM_LOW: c_int = 7;
    static const u8 adt7310_reg_table[] = {
    [ADT7X10_TEMPERATURE]   = ADT7310_TEMPERATURE,
    [ADT7X10_STATUS]	= ADT7310_STATUS,
    [ADT7X10_CONFIG]	= ADT7310_CONFIG,
    [ADT7X10_T_ALARM_HIGH]	= ADT7310_T_ALARM_HIGH,
    [ADT7X10_T_ALARM_LOW]	= ADT7310_T_ALARM_LOW,
    [ADT7X10_T_CRIT]	= ADT7310_T_CRIT,
    [ADT7X10_T_HYST]	= ADT7310_T_HYST,
    [ADT7X10_ID]		= ADT7310_ID,
    };
pub const ADT7310_CMD_REG_OFFSET: c_int = 3;
pub const ADT7310_CMD_READ: c_uint = 0x40;

#[no_mangle]
unsafe extern "C" fn adt7310_spi_read_word(spi: *mut spi_device, reg: u8) -> c_int {
    static int adt7310_spi_read_word(struct spi_device *spi, u8 reg)
    {
    return spi_w8r16be(spi, AD7310_COMMAND(reg) | ADT7310_CMD_READ);
    }
#[no_mangle]
unsafe extern "C" fn adt7310_spi_write_word(spi: *mut spi_device, reg: u8, data: u16) -> c_int {
    static int adt7310_spi_write_word(struct spi_device *spi, u8 reg, u16 data)
    {
    u8 buf[3];
    buf[0] = AD7310_COMMAND(reg);
    put_unaligned_be16(data, &buf[1]);
    return spi_write(spi, buf, sizeof(buf));
    }
#[no_mangle]
unsafe extern "C" fn adt7310_spi_read_byte(spi: *mut spi_device, reg: u8) -> c_int {
    static int adt7310_spi_read_byte(struct spi_device *spi, u8 reg)
    {
    return spi_w8r8(spi, AD7310_COMMAND(reg) | ADT7310_CMD_READ);
    }
#[no_mangle]
unsafe extern "C" fn adt7310_spi_write_byte(spi: *mut spi_device, reg: u8, data: u8) -> c_int {
    static int adt7310_spi_write_byte(struct spi_device *spi, u8 reg, u8 data)
    {
    u8 buf[2];
    buf[0] = AD7310_COMMAND(reg);
    buf[1] = data;
    return spi_write(spi, buf, sizeof(buf));
    }
#[no_mangle]
unsafe extern "C" fn adt7310_regmap_is_volatile(dev: *mut device, reg: c_uint) -> bool {
    static bool adt7310_regmap_is_volatile(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case ADT7X10_TEMPERATURE:
    case ADT7X10_STATUS:
    return true;
    default:
    return false;
    }
    }
#[no_mangle]
unsafe extern "C" fn adt7310_reg_read(context: *mut c_void, reg: c_uint, val: *mut c_uint) -> c_int {
    static int adt7310_reg_read(void *context, unsigned int reg, unsigned int *val)
    {
    struct spi_device *spi = context;
    int regval;
    switch (reg) {
    case ADT7X10_TEMPERATURE:
    case ADT7X10_T_ALARM_HIGH:
    case ADT7X10_T_ALARM_LOW:
    case ADT7X10_T_CRIT:
    regval = adt7310_spi_read_word(spi, reg);
    break;
    default:
    regval = adt7310_spi_read_byte(spi, reg);
    break;
    }
    if (regval < 0)
    return regval;
// val = regval;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adt7310_reg_write(context: *mut c_void, reg: c_uint, val: c_uint) -> c_int {
    static int adt7310_reg_write(void *context, unsigned int reg, unsigned int val)
    {
    struct spi_device *spi = context;
    int ret;
    switch (reg) {
    case ADT7X10_TEMPERATURE:
    case ADT7X10_T_ALARM_HIGH:
    case ADT7X10_T_ALARM_LOW:
    case ADT7X10_T_CRIT:
    ret = adt7310_spi_write_word(spi, reg, val);
    break;
    default:
    ret = adt7310_spi_write_byte(spi, reg, val);
    break;
    }
    return ret;
    }
    static const struct regmap_config adt7310_regmap_config = {
    .reg_bits = 8,
    .val_bits = 16,
    .cache_type = REGCACHE_MAPLE,
    .volatile_reg = adt7310_regmap_is_volatile,
    .reg_read = adt7310_reg_read,
    .reg_write = adt7310_reg_write,
    };
#[no_mangle]
unsafe extern "C" fn adt7310_spi_probe(spi: *mut spi_device) -> c_int {
    static int adt7310_spi_probe(struct spi_device *spi)
    {
    struct regmap *regmap;
    regmap = devm_regmap_init(&spi.dev, core::ptr::null_mut(), spi, &adt7310_regmap_config);
    if (IS_ERR(regmap))
    return PTR_ERR(regmap);
    return adt7x10_probe(&spi.dev, spi_get_device_id(spi).name, spi.irq,
    regmap);
    }
    static const struct spi_device_id adt7310_id[] = {
    { "adt7310", 0 },
    { "adt7320", 0 },
    {}
    };
    MODULE_DEVICE_TABLE(spi, adt7310_id);
    static struct spi_driver adt7310_driver = {
    .driver = {
    .name	= "adt7310",
    .pm	= pm_sleep_ptr(&adt7x10_dev_pm_ops),
    },
    .probe		= adt7310_spi_probe,
    .id_table	= adt7310_id,
    };
    module_spi_driver(adt7310_driver);
    MODULE_AUTHOR("Lars-Peter Clausen <lars@metafoo.de>");
    MODULE_DESCRIPTION("ADT7310/ADT7320 driver");
    MODULE_LICENSE("GPL");
