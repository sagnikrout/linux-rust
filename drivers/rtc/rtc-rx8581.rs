//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-rx8581.c
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
// An I2C driver for the Epson RX8581 RTC
//
// Author: Martyn Welch <martyn.welch@ge.com>
// Copyright 2008 GE Intelligent Platforms Embedded Systems, Inc.
//
// Based on: rtc-pcf8563.c (An I2C driver for the Philips PCF8563 RTC)
// Copyright 2005-06 Tower Technologies
//

pub const RX8581_REG_SC: c_uint = 0x00 /* Second in BCD */;
pub const RX8581_REG_MN: c_uint = 0x01 /* Minute in BCD */;
pub const RX8581_REG_HR: c_uint = 0x02 /* Hour in BCD */;
pub const RX8581_REG_DW: c_uint = 0x03 /* Day of Week */;
pub const RX8581_REG_DM: c_uint = 0x04 /* Day of Month in BCD */;
pub const RX8581_REG_MO: c_uint = 0x05 /* Month in BCD */;
pub const RX8581_REG_YR: c_uint = 0x06 /* Year in BCD */;
pub const RX8581_REG_RAM: c_uint = 0x07 /* RAM */;
pub const RX8581_REG_AMN: c_uint = 0x08 /* Alarm Min in BCD*/;
pub const RX8581_REG_AHR: c_uint = 0x09 /* Alarm Hour in BCD */;
pub const RX8581_REG_ADM: c_uint = 0x0A;
pub const RX8581_REG_ADW: c_uint = 0x0A;
pub const RX8581_REG_TMR0: c_uint = 0x0B;
pub const RX8581_REG_TMR1: c_uint = 0x0C;
pub const RX8581_REG_EXT: c_uint = 0x0D /* Extension Register */;
pub const RX8581_REG_FLAG: c_uint = 0x0E /* Flag Register */;
pub const RX8581_REG_CTRL: c_uint = 0x0F /* Control Register */;
// Flag Register bit definitions
pub const RX8581_FLAG_UF: c_uint = 0x20 /* Update */;
pub const RX8581_FLAG_TF: c_uint = 0x10 /* Timer */;
pub const RX8581_FLAG_AF: c_uint = 0x08 /* Alarm */;
pub const RX8581_FLAG_VLF: c_uint = 0x02 /* Voltage Low */;
// Control Register bit definitions
pub const RX8581_CTRL_UIE: c_uint = 0x20 /* Update Interrupt Enable */;
pub const RX8581_CTRL_TIE: c_uint = 0x10 /* Timer Interrupt Enable */;
pub const RX8581_CTRL_AIE: c_uint = 0x08 /* Alarm Interrupt Enable */;
pub const RX8581_CTRL_STOP: c_uint = 0x02 /* STOP bit */;
pub const RX8581_CTRL_RESET: c_uint = 0x01 /* RESET bit */;
pub const RX8571_USER_RAM: c_uint = 0x10;
pub const RX8571_NVRAM_SIZE: c_uint = 0x10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx85x1_config {
    pub regmap: regmap_config,
    pub num_nvram: c_uint,
}

//
// In the routines that deal directly with the rx8581 hardware, we use
// rtc_time -- month 0-11, hour 0-23, yr = calendar year-epoch.
//
#[no_mangle]
unsafe extern "C" fn rx8581_rtc_read_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int rx8581_rtc_read_time(struct device *dev, struct rtc_time *tm)
    {
    struct i2c_client *client = to_i2c_client(dev);
    unsigned char date[7];
    unsigned int data;
    int err;
    struct regmap *regmap = i2c_get_clientdata(client);
// First we ensure that the "update flag" is not set, we read the
// time and date then re-read the "update flag". If the update flag
// has been set, we know that the time has changed during the read so
// we repeat the whole process again.
//
    err = regmap_read(regmap, RX8581_REG_FLAG, &data);
    if (err < 0)
    return err;
    if (data & RX8581_FLAG_VLF) {
    dev_warn(dev,
    "low voltage detected, date/time is not reliable.\n");
    return -EINVAL;
    }
    do {
// If update flag set, clear it
    if (data & RX8581_FLAG_UF) {
    err = regmap_write(regmap, RX8581_REG_FLAG,
    data & ~RX8581_FLAG_UF);
    if (err < 0)
    return err;
    }
// Now read time and date
    err = regmap_bulk_read(regmap, RX8581_REG_SC, date,
    sizeof(date));
    if (err < 0)
    return err;
// Check flag register
    err = regmap_read(regmap, RX8581_REG_FLAG, &data);
    if (err < 0)
    return err;
    } while (data & RX8581_FLAG_UF);
    dev_dbg(dev, "%s: raw data is sec=%02x, min=%02x, hr=%02x, "
    "wday=%02x, mday=%02x, mon=%02x, year=%02x\n",
    __func__,
    date[0], date[1], date[2], date[3], date[4], date[5], date[6]);
    tm.tm_sec = bcd2bin(date[RX8581_REG_SC] & 0x7F);
    tm.tm_min = bcd2bin(date[RX8581_REG_MN] & 0x7F);
    tm.tm_hour = bcd2bin(date[RX8581_REG_HR] & 0x3F); /* rtc hr 0-23 */
    tm.tm_wday = ilog2(date[RX8581_REG_DW] & 0x7F);
    tm.tm_mday = bcd2bin(date[RX8581_REG_DM] & 0x3F);
    tm.tm_mon = bcd2bin(date[RX8581_REG_MO] & 0x1F) - 1; /* rtc mn 1-12 */
    tm.tm_year = bcd2bin(date[RX8581_REG_YR]) + 100;
    dev_dbg(dev, "%s: tm is secs=%d, mins=%d, hours=%d, "
    "mday=%d, mon=%d, year=%d, wday=%d\n",
    __func__,
    tm.tm_sec, tm.tm_min, tm.tm_hour,
    tm.tm_mday, tm.tm_mon, tm.tm_year, tm.tm_wday);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rx8581_rtc_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int rx8581_rtc_set_time(struct device *dev, struct rtc_time *tm)
    {
    struct i2c_client *client = to_i2c_client(dev);
    int err;
    unsigned char buf[7];
    struct regmap *regmap = i2c_get_clientdata(client);
    dev_dbg(dev, "%s: secs=%d, mins=%d, hours=%d, "
    "mday=%d, mon=%d, year=%d, wday=%d\n",
    __func__,
    tm.tm_sec, tm.tm_min, tm.tm_hour,
    tm.tm_mday, tm.tm_mon, tm.tm_year, tm.tm_wday);
// hours, minutes and seconds
    buf[RX8581_REG_SC] = bin2bcd(tm.tm_sec);
    buf[RX8581_REG_MN] = bin2bcd(tm.tm_min);
    buf[RX8581_REG_HR] = bin2bcd(tm.tm_hour);
    buf[RX8581_REG_DM] = bin2bcd(tm.tm_mday);
// month, 1 - 12
    buf[RX8581_REG_MO] = bin2bcd(tm.tm_mon + 1);
// year and century
    buf[RX8581_REG_YR] = bin2bcd(tm.tm_year - 100);
    buf[RX8581_REG_DW] = (0x1 << tm.tm_wday);
// Stop the clock
    err = regmap_update_bits(regmap, RX8581_REG_CTRL,
    RX8581_CTRL_STOP, RX8581_CTRL_STOP);
    if (err < 0)
    return err;
// write register's data
    err = regmap_bulk_write(regmap, RX8581_REG_SC, buf, sizeof(buf));
    if (err < 0)
    return err;
// get VLF and clear it
    err = regmap_update_bits(regmap, RX8581_REG_FLAG, RX8581_FLAG_VLF, 0);
    if (err < 0)
    return err;
// Restart the clock
    return regmap_update_bits(regmap, RX8581_REG_CTRL,
    RX8581_CTRL_STOP, 0);
    }
    static const struct rtc_class_ops rx8581_rtc_ops = {
    .read_time	= rx8581_rtc_read_time,
    .set_time	= rx8581_rtc_set_time,
    };
    static int rx8571_nvram_read(void *priv, unsigned int offset, void *val,
    size_t bytes)
    {
    struct regmap *regmap = priv;
    return regmap_bulk_read(regmap, RX8571_USER_RAM + offset, val, bytes);
    }
    static int rx8571_nvram_write(void *priv, unsigned int offset, void *val,
    size_t bytes)
    {
    struct regmap *regmap = priv;
    return regmap_bulk_write(regmap, RX8571_USER_RAM + offset, val, bytes);
    }
    static int rx85x1_nvram_read(void *priv, unsigned int offset, void *val,
    size_t bytes)
    {
    struct regmap *regmap = priv;
    unsigned int tmp_val;
    int ret;
    ret = regmap_read(regmap, RX8581_REG_RAM, &tmp_val);
    (*(unsigned char *)val) = (unsigned char) tmp_val;
    return ret;
    }
    static int rx85x1_nvram_write(void *priv, unsigned int offset, void *val,
    size_t bytes)
    {
    struct regmap *regmap = priv;
    unsigned char tmp_val;
    tmp_val = *((unsigned char *)val);
    return regmap_write(regmap, RX8581_REG_RAM, (unsigned int)tmp_val);
    }
    static const struct rx85x1_config rx8581_config = {
    .regmap = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = 0xf,
    },
    .num_nvram = 1
    };
    static const struct rx85x1_config rx8571_config = {
    .regmap = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = 0x1f,
    },
    .num_nvram = 2
    };
#[no_mangle]
unsafe extern "C" fn rx8581_probe(client: *mut i2c_client) -> c_int {
    static int rx8581_probe(struct i2c_client *client)
    {
    struct regmap *regmap;
    const struct rx85x1_config *config = &rx8581_config;
    const void *data = of_device_get_match_data(&client.dev);
    struct rtc_device *rtc;
    static struct nvmem_config nvmem_cfg[] = {
    {
    .name = "rx85x1-",
    .word_size = 1,
    .stride = 1,
    .size = 1,
    .reg_read = rx85x1_nvram_read,
    .reg_write = rx85x1_nvram_write,
    }, {
    .name = "rx8571-",
    .word_size = 1,
    .stride = 1,
    .size = RX8571_NVRAM_SIZE,
    .reg_read = rx8571_nvram_read,
    .reg_write = rx8571_nvram_write,
    },
    };
    int ret, i;
    dev_dbg(&client.dev, "%s\n", __func__);
    if (data)
    config = data;
    regmap = devm_regmap_init_i2c(client, &config.regmap);
    if (IS_ERR(regmap))
    return PTR_ERR(regmap);
    i2c_set_clientdata(client, regmap);
    rtc = devm_rtc_allocate_device(&client.dev);
    if (IS_ERR(rtc))
    return PTR_ERR(rtc);
    rtc.ops = &rx8581_rtc_ops;
    rtc.range_min = RTC_TIMESTAMP_BEGIN_2000;
    rtc.range_max = RTC_TIMESTAMP_END_2099;
    rtc.start_secs = 0;
    rtc.set_start_time = true;
    ret = devm_rtc_register_device(rtc);
    for (i = 0; i < config.num_nvram; i++) {
    nvmem_cfg[i].priv = regmap;
    devm_rtc_nvmem_register(rtc, &nvmem_cfg[i]);
    }
    return ret;
    }
    static const struct i2c_device_id rx8581_id[] = {
    { .name = "rx8581" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, rx8581_id);
    static const __maybe_unused struct of_device_id rx8581_of_match[] = {
    { .compatible = "epson,rx8571", .data = &rx8571_config },
    { .compatible = "epson,rx8581", .data = &rx8581_config },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, rx8581_of_match);
    static struct i2c_driver rx8581_driver = {
    .driver		= {
    .name	= "rtc-rx8581",
    .of_match_table = of_match_ptr(rx8581_of_match),
    },
    .probe		= rx8581_probe,
    .id_table	= rx8581_id,
    };
    module_i2c_driver(rx8581_driver);
    MODULE_AUTHOR("Martyn Welch <martyn.welch@ge.com>");
    MODULE_DESCRIPTION("Epson RX-8571/RX-8581 RTC driver");
    MODULE_LICENSE("GPL");
