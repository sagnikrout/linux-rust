//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-rx6110.c
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
// Driver for the Epson RTC module RX-6110 SA
//
// Copyright(C) 2015 Pengutronix, Steffen Trumtrar <kernel@pengutronix.de>
// Copyright(C) SEIKO EPSON CORPORATION 2013. All rights reserved.
//

// RX-6110 Register definitions
pub const RX6110_REG_SEC: c_uint = 0x10;
pub const RX6110_REG_MIN: c_uint = 0x11;
pub const RX6110_REG_HOUR: c_uint = 0x12;
pub const RX6110_REG_WDAY: c_uint = 0x13;
pub const RX6110_REG_MDAY: c_uint = 0x14;
pub const RX6110_REG_MONTH: c_uint = 0x15;
pub const RX6110_REG_YEAR: c_uint = 0x16;
pub const RX6110_REG_RES1: c_uint = 0x17;
pub const RX6110_REG_ALMIN: c_uint = 0x18;
pub const RX6110_REG_ALHOUR: c_uint = 0x19;
pub const RX6110_REG_ALWDAY: c_uint = 0x1A;
pub const RX6110_REG_TCOUNT0: c_uint = 0x1B;
pub const RX6110_REG_TCOUNT1: c_uint = 0x1C;
pub const RX6110_REG_EXT: c_uint = 0x1D;
pub const RX6110_REG_FLAG: c_uint = 0x1E;
pub const RX6110_REG_CTRL: c_uint = 0x1F;
pub const RX6110_REG_USER0: c_uint = 0x20;
pub const RX6110_REG_USER1: c_uint = 0x21;
pub const RX6110_REG_USER2: c_uint = 0x22;
pub const RX6110_REG_USER3: c_uint = 0x23;
pub const RX6110_REG_USER4: c_uint = 0x24;
pub const RX6110_REG_USER5: c_uint = 0x25;
pub const RX6110_REG_USER6: c_uint = 0x26;
pub const RX6110_REG_USER7: c_uint = 0x27;
pub const RX6110_REG_USER8: c_uint = 0x28;
pub const RX6110_REG_USER9: c_uint = 0x29;
pub const RX6110_REG_USERA: c_uint = 0x2A;
pub const RX6110_REG_USERB: c_uint = 0x2B;
pub const RX6110_REG_USERC: c_uint = 0x2C;
pub const RX6110_REG_USERD: c_uint = 0x2D;
pub const RX6110_REG_USERE: c_uint = 0x2E;
pub const RX6110_REG_USERF: c_uint = 0x2F;
pub const RX6110_REG_RES2: c_uint = 0x30;
pub const RX6110_REG_RES3: c_uint = 0x31;
pub const RX6110_REG_IRQ: c_uint = 0x32;

// Extension Register (1Dh) bit positions

// Flag Register (1Eh) bit positions

// Control Register (1Fh) bit positions

    enum {
    RTC_SEC = 0,
    RTC_MIN,
    RTC_HOUR,
    RTC_WDAY,
    RTC_MDAY,
    RTC_MONTH,
    RTC_YEAR,
    RTC_NR_TIME
    };

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx6110_data {
    pub rtc: *mut rtc_device,
    pub regmap: *mut regmap,
}

//
// rx6110_rtc_tm_to_data - convert rtc_time to native time encoding
//
// @tm: holds date and time
// @data: holds the encoding in rx6110 native form
//
#[no_mangle]
unsafe extern "C" fn rx6110_rtc_tm_to_data(tm: *mut rtc_time, data: *mut u8) -> c_int {
    static int rx6110_rtc_tm_to_data(struct rtc_time *tm, u8 *data)
    {
    pr_debug("%s: date %ptRr\n", __func__, tm);
//
// The year in the RTC is a value between 0 and 99.
// Assume that this represents the current century
// and disregard all other values.
//
    if (tm.tm_year < 100 || tm.tm_year >= 200)
    return -EINVAL;
    data[RTC_SEC] = bin2bcd(tm.tm_sec);
    data[RTC_MIN] = bin2bcd(tm.tm_min);
    data[RTC_HOUR] = bin2bcd(tm.tm_hour);
    data[RTC_WDAY] = BIT(bin2bcd(tm.tm_wday));
    data[RTC_MDAY] = bin2bcd(tm.tm_mday);
    data[RTC_MONTH] = bin2bcd(tm.tm_mon + 1);
    data[RTC_YEAR] = bin2bcd(tm.tm_year % 100);
    return 0;
    }
//
// rx6110_data_to_rtc_tm - convert native time encoding to rtc_time
//
// @data: holds the encoding in rx6110 native form
// @tm: holds date and time
//
#[no_mangle]
unsafe extern "C" fn rx6110_data_to_rtc_tm(data: *mut u8, tm: *mut rtc_time) -> c_int {
    static int rx6110_data_to_rtc_tm(u8 *data, struct rtc_time *tm)
    {
    tm.tm_sec = bcd2bin(data[RTC_SEC] & 0x7f);
    tm.tm_min = bcd2bin(data[RTC_MIN] & 0x7f);
// only 24-hour clock
    tm.tm_hour = bcd2bin(data[RTC_HOUR] & 0x3f);
    tm.tm_wday = ffs(data[RTC_WDAY] & 0x7f);
    tm.tm_mday = bcd2bin(data[RTC_MDAY] & 0x3f);
    tm.tm_mon = bcd2bin(data[RTC_MONTH] & 0x1f) - 1;
    tm.tm_year = bcd2bin(data[RTC_YEAR]) + 100;
    pr_debug("%s: date %ptRr\n", __func__, tm);
//
// The year in the RTC is a value between 0 and 99.
// Assume that this represents the current century
// and disregard all other values.
//
    if (tm.tm_year < 100 || tm.tm_year >= 200)
    return -EINVAL;
    return 0;
    }
//
// rx6110_set_time - set the current time in the rx6110 registers
//
// @dev: the rtc device in use
// @tm: holds date and time
//
// BUG: The HW assumes every year that is a multiple of 4 to be a leap
// year. Next time this is wrong is 2100, which will not be a leap year
//
// Note: If STOP is not set/cleared, the clock will start when the seconds
// register is written
//
#[no_mangle]
unsafe extern "C" fn rx6110_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int rx6110_set_time(struct device *dev, struct rtc_time *tm)
    {
    struct rx6110_data *rx6110 = dev_get_drvdata(dev);
    u8 data[RTC_NR_TIME];
    int ret;
    ret = rx6110_rtc_tm_to_data(tm, data);
    if (ret < 0)
    return ret;
// set STOP bit before changing clock/calendar
    ret = regmap_update_bits(rx6110.regmap, RX6110_REG_CTRL,
    RX6110_BIT_CTRL_STOP, RX6110_BIT_CTRL_STOP);
    if (ret)
    return ret;
    ret = regmap_bulk_write(rx6110.regmap, RX6110_REG_SEC, data,
    RTC_NR_TIME);
    if (ret)
    return ret;
// The time in the RTC is valid. Be sure to have VLF cleared.
    ret = regmap_update_bits(rx6110.regmap, RX6110_REG_FLAG,
    RX6110_BIT_FLAG_VLF, 0);
    if (ret)
    return ret;
// clear STOP bit after changing clock/calendar
    ret = regmap_update_bits(rx6110.regmap, RX6110_REG_CTRL,
    RX6110_BIT_CTRL_STOP, 0);
    return ret;
    }
//
// rx6110_get_time - get the current time from the rx6110 registers
// @dev: the rtc device in use
// @tm: holds date and time
//
#[no_mangle]
unsafe extern "C" fn rx6110_get_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int rx6110_get_time(struct device *dev, struct rtc_time *tm)
    {
    struct rx6110_data *rx6110 = dev_get_drvdata(dev);
    u8 data[RTC_NR_TIME];
    int flags;
    int ret;
    ret = regmap_read(rx6110.regmap, RX6110_REG_FLAG, &flags);
    if (ret)
    return -EINVAL;
// check for VLF Flag (set at power-on)
    if ((flags & RX6110_BIT_FLAG_VLF)) {
    dev_warn(dev, "Voltage low, data is invalid.\n");
    return -EINVAL;
    }
// read registers to date
    ret = regmap_bulk_read(rx6110.regmap, RX6110_REG_SEC, data,
    RTC_NR_TIME);
    if (ret)
    return ret;
    ret = rx6110_data_to_rtc_tm(data, tm);
    if (ret)
    return ret;
    dev_dbg(dev, "%s: date %ptRr\n", __func__, tm);
    return 0;
    }
    static const struct reg_sequence rx6110_default_regs[] = {
    { RX6110_REG_RES1,   0xB8 },
    { RX6110_REG_RES2,   0x00 },
    { RX6110_REG_RES3,   0x10 },
    { RX6110_REG_IRQ,    0x00 },
    { RX6110_REG_ALMIN,  0x00 },
    { RX6110_REG_ALHOUR, 0x00 },
    { RX6110_REG_ALWDAY, 0x00 },
    };
//
// rx6110_init - initialize the rx6110 registers
//
// @rx6110: pointer to the rx6110 struct in use
//
#[no_mangle]
unsafe extern "C" fn rx6110_init(rx6110: *mut rx6110_data) -> c_int {
    static int rx6110_init(struct rx6110_data *rx6110)
    {
    struct rtc_device *rtc = rx6110.rtc;
    int flags;
    int ret;
    ret = regmap_update_bits(rx6110.regmap, RX6110_REG_EXT,
    RX6110_BIT_EXT_TE, 0);
    if (ret)
    return ret;
    ret = regmap_register_patch(rx6110.regmap, rx6110_default_regs,
    ARRAY_SIZE(rx6110_default_regs));
    if (ret)
    return ret;
    ret = regmap_read(rx6110.regmap, RX6110_REG_FLAG, &flags);
    if (ret)
    return ret;
// check for VLF Flag (set at power-on)
    if ((flags & RX6110_BIT_FLAG_VLF))
    dev_warn(&rtc.dev, "Voltage low, data loss detected.\n");
// check for Alarm Flag
    if (flags & RX6110_BIT_FLAG_AF)
    dev_warn(&rtc.dev, "An alarm may have been missed.\n");
// check for Periodic Timer Flag
    if (flags & RX6110_BIT_FLAG_TF)
    dev_warn(&rtc.dev, "Periodic timer was detected\n");
// check for Update Timer Flag
    if (flags & RX6110_BIT_FLAG_UF)
    dev_warn(&rtc.dev, "Update timer was detected\n");
// clear all flags BUT VLF
    ret = regmap_update_bits(rx6110.regmap, RX6110_REG_FLAG,
    RX6110_BIT_FLAG_AF |
    RX6110_BIT_FLAG_UF |
    RX6110_BIT_FLAG_TF,
    0);
    return ret;
    }
    static const struct rtc_class_ops rx6110_rtc_ops = {
    .read_time = rx6110_get_time,
    .set_time = rx6110_set_time,
    };
#[no_mangle]
unsafe extern "C" fn rx6110_probe(rx6110: *mut rx6110_data, dev: *mut device) -> c_int {
    static int rx6110_probe(struct rx6110_data *rx6110, struct device *dev)
    {
    int err;
    rx6110.rtc = devm_rtc_device_register(dev,
    RX6110_DRIVER_NAME,
    &rx6110_rtc_ops, THIS_MODULE);
    if (IS_ERR(rx6110.rtc))
    return PTR_ERR(rx6110.rtc);
    err = rx6110_init(rx6110);
    if (err)
    return err;
    return 0;
    }

    static const struct regmap_config regmap_spi_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = RX6110_REG_IRQ,
    .read_flag_mask = 0x80,
    };
//
// rx6110_spi_probe - initialize rtc driver
// @spi: pointer to spi device
//
#[no_mangle]
unsafe extern "C" fn rx6110_spi_probe(spi: *mut spi_device) -> c_int {
    static int rx6110_spi_probe(struct spi_device *spi)
    {
    struct rx6110_data *rx6110;
    if ((spi.bits_per_word && spi.bits_per_word != 8) ||
    (spi.max_speed_hz > 2000000) ||
    (spi.mode != (SPI_CS_HIGH | SPI_CPOL | SPI_CPHA))) {
    dev_warn(&spi.dev, "SPI settings: bits_per_word: %d, max_speed_hz: %d, mode: %xh\n",
    spi.bits_per_word, spi.max_speed_hz, spi.mode);
    dev_warn(&spi.dev, "driving device in an unsupported mode");
    }
    rx6110 = devm_kzalloc(&spi.dev, sizeof(*rx6110), GFP_KERNEL);
    if (!rx6110)
    return -ENOMEM;
    rx6110.regmap = devm_regmap_init_spi(spi, &regmap_spi_config);
    if (IS_ERR(rx6110.regmap)) {
    dev_err(&spi.dev, "regmap init failed for rtc rx6110\n");
    return PTR_ERR(rx6110.regmap);
    }
    spi_set_drvdata(spi, rx6110);
    return rx6110_probe(rx6110, &spi.dev);
    }
    static const struct spi_device_id rx6110_spi_id[] = {
    { "rx6110", 0 },
    { }
    };
    MODULE_DEVICE_TABLE(spi, rx6110_spi_id);
    static const __maybe_unused struct of_device_id rx6110_spi_of_match[] = {
    { .compatible = "epson,rx6110" },
    { },
    };
    MODULE_DEVICE_TABLE(of, rx6110_spi_of_match);
    static struct spi_driver rx6110_spi_driver = {
    .driver = {
    .name = RX6110_DRIVER_NAME,
    .of_match_table = of_match_ptr(rx6110_spi_of_match),
    },
    .probe		= rx6110_spi_probe,
    .id_table	= rx6110_spi_id,
    };
#[no_mangle]
unsafe extern "C" fn rx6110_spi_register() -> c_int {
    static int rx6110_spi_register(void)
    {
    return spi_register_driver(&rx6110_spi_driver);
    }
#[no_mangle]
unsafe extern "C" fn rx6110_spi_unregister() {
    static void rx6110_spi_unregister(void)
    {
    spi_unregister_driver(&rx6110_spi_driver);
    }

#[no_mangle]
unsafe extern "C" fn rx6110_spi_register() -> c_int {
    static int rx6110_spi_register(void)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rx6110_spi_unregister() {
    static void rx6110_spi_unregister(void)
    {
    }

    static const struct regmap_config regmap_i2c_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = RX6110_REG_IRQ,
    .read_flag_mask = 0x80,
    };
#[no_mangle]
unsafe extern "C" fn rx6110_i2c_probe(client: *mut i2c_client) -> c_int {
    static int rx6110_i2c_probe(struct i2c_client *client)
    {
    struct i2c_adapter *adapter = client.adapter;
    struct rx6110_data *rx6110;
    if (!i2c_check_functionality(adapter, I2C_FUNC_SMBUS_BYTE_DATA
    | I2C_FUNC_SMBUS_I2C_BLOCK)) {
    dev_err(&adapter.dev,
    "doesn't support required functionality\n");
    return -EIO;
    }
    rx6110 = devm_kzalloc(&client.dev, sizeof(*rx6110), GFP_KERNEL);
    if (!rx6110)
    return -ENOMEM;
    rx6110.regmap = devm_regmap_init_i2c(client, &regmap_i2c_config);
    if (IS_ERR(rx6110.regmap)) {
    dev_err(&client.dev, "regmap init failed for rtc rx6110\n");
    return PTR_ERR(rx6110.regmap);
    }
    i2c_set_clientdata(client, rx6110);
    return rx6110_probe(rx6110, &client.dev);
    }
    static const struct acpi_device_id rx6110_i2c_acpi_match[] = {
    { "SECC6110" },
    { }
    };
    MODULE_DEVICE_TABLE(acpi, rx6110_i2c_acpi_match);
    static const struct i2c_device_id rx6110_i2c_id[] = {
    { .name = "rx6110" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, rx6110_i2c_id);
    static struct i2c_driver rx6110_i2c_driver = {
    .driver = {
    .name = RX6110_DRIVER_NAME,
    .acpi_match_table = rx6110_i2c_acpi_match,
    },
    .probe		= rx6110_i2c_probe,
    .id_table	= rx6110_i2c_id,
    };
#[no_mangle]
unsafe extern "C" fn rx6110_i2c_register() -> c_int {
    static int rx6110_i2c_register(void)
    {
    return i2c_add_driver(&rx6110_i2c_driver);
    }
#[no_mangle]
unsafe extern "C" fn rx6110_i2c_unregister() {
    static void rx6110_i2c_unregister(void)
    {
    i2c_del_driver(&rx6110_i2c_driver);
    }

#[no_mangle]
unsafe extern "C" fn rx6110_i2c_register() -> c_int {
    static int rx6110_i2c_register(void)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rx6110_i2c_unregister() {
    static void rx6110_i2c_unregister(void)
    {
    }

#[no_mangle]
unsafe extern "C" fn rx6110_module_init() -> int __init {
    static int __init rx6110_module_init(void)
    {
    int ret;
    ret = rx6110_spi_register();
    if (ret)
    return ret;
    ret = rx6110_i2c_register();
    if (ret)
    rx6110_spi_unregister();
    return ret;
    }
    module_init(rx6110_module_init);
#[no_mangle]
unsafe extern "C" fn rx6110_module_exit() -> void __exit {
    static void __exit rx6110_module_exit(void)
    {
    rx6110_spi_unregister();
    rx6110_i2c_unregister();
    }
    module_exit(rx6110_module_exit);
    MODULE_AUTHOR("Val Krutov <val.krutov@erd.epson.com>");
    MODULE_DESCRIPTION("RX-6110 SA RTC driver");
    MODULE_LICENSE("GPL");
