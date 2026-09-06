//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-rx8111.c
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
// Driver for Epson RX8111 RTC.
//
// Copyright (C) 2023 Axis Communications AB
//

pub const RX8111_REG_SEC: c_uint = 0x10	/* Second counter. */;
pub const RX8111_REG_MIN: c_uint = 0x11	/* Minute counter */;
pub const RX8111_REG_HOUR: c_uint = 0x12	/* Hour counter. */;
pub const RX8111_REG_WEEK: c_uint = 0x13	/* Week day counter. */;
pub const RX8111_REG_DAY: c_uint = 0x14	/* Month day counter. */;
pub const RX8111_REG_MONTH: c_uint = 0x15	/* Month counter. */;
pub const RX8111_REG_YEAR: c_uint = 0x16	/* Year counter. */;
pub const RX8111_REG_ALARM_MIN: c_uint = 0x17	/* Alarm minute. */;
pub const RX8111_REG_ALARM_HOUR: c_uint = 0x18	/* Alarm hour. */;
pub const RX8111_REG_ALARM_WEEK_DAY: c_uint = 0x19	/* Alarm week or month day. */;
pub const RX8111_REG_TIMER_COUNTER0: c_uint = 0x1a	/* Timer counter LSB. */;
pub const RX8111_REG_TIMER_COUNTER1: c_uint = 0x1b	/* Timer counter. */;
pub const RX8111_REG_TIMER_COUNTER2: c_uint = 0x1c	/* Timer counter MSB. */;
pub const RX8111_REG_EXT: c_uint = 0x1d	/* Extension register. */;
pub const RX8111_REG_FLAG: c_uint = 0x1e	/* Flag register. */;
pub const RX8111_REG_CTRL: c_uint = 0x1f	/* Control register. */;
pub const RX8111_REG_TS_1_1000_SEC: c_uint = 0x20	/* Timestamp 256 or 512 Hz . */;
pub const RX8111_REG_TS_1_100_SEC: c_uint = 0x21	/* Timestamp 1 - 128 Hz. */;
pub const RX8111_REG_TS_SEC: c_uint = 0x22	/* Timestamp second. */;
pub const RX8111_REG_TS_MIN: c_uint = 0x23	/* Timestamp minute. */;
pub const RX8111_REG_TS_HOUR: c_uint = 0x24	/* Timestamp hour. */;
pub const RX8111_REG_TS_WEEK: c_uint = 0x25	/* Timestamp week day. */;
pub const RX8111_REG_TS_DAY: c_uint = 0x26	/* Timestamp month day. */;
pub const RX8111_REG_TS_MONTH: c_uint = 0x27	/* Timestamp month. */;
pub const RX8111_REG_TS_YEAR: c_uint = 0x28	/* Timestamp year. */;
pub const RX8111_REG_TS_STATUS: c_uint = 0x29	/* Timestamp status. */;
pub const RX8111_REG_EVIN_SETTING: c_uint = 0x2b	/* Timestamp trigger setting. */;
pub const RX8111_REG_ALARM_SEC: c_uint = 0x2c	/* Alarm second. */;
pub const RX8111_REG_TIMER_CTRL: c_uint = 0x2d	/* Timer control. */;
pub const RX8111_REG_TS_CTRL0: c_uint = 0x2e	/* Timestamp control 0. */;
pub const RX8111_REG_CMD_TRIGGER: c_uint = 0x2f	/* Timestamp trigger. */;
pub const RX8111_REG_PWR_SWITCH_CTRL: c_uint = 0x32	/* Power switch control. */;
pub const RX8111_REG_STATUS_MON: c_uint = 0x33	/* Status monitor. */;
pub const RX8111_REG_TS_CTRL1: c_uint = 0x34	/* Timestamp control 1. */;
pub const RX8111_REG_TS_CTRL2: c_uint = 0x35	/* Timestamp control 2. */;
pub const RX8111_REG_TS_CTRL3: c_uint = 0x36	/* Timestamp control 3. */;

    enum rx8111_regfield {
// RX8111_REG_EXT.
    RX8111_REGF_TSEL0,
    RX8111_REGF_TSEL1,
    RX8111_REGF_ETS,
    RX8111_REGF_WADA,
    RX8111_REGF_TE,
    RX8111_REGF_USEL,
    RX8111_REGF_FSEL0,
    RX8111_REGF_FSEL1,
// RX8111_REG_FLAG.
    RX8111_REGF_XST,
    RX8111_REGF_VLF,
    RX8111_REGF_EVF,
    RX8111_REGF_AF,
    RX8111_REGF_TF,
    RX8111_REGF_UF,
    RX8111_REGF_POR,
// RX8111_REG_CTRL.
    RX8111_REGF_STOP,
    RX8111_REGF_EIE,
    RX8111_REGF_AIE,
    RX8111_REGF_TIE,
    RX8111_REGF_UIE,
// RX8111_REG_PWR_SWITCH_CTRL.
    RX8111_REGF_SMPT0,
    RX8111_REGF_SMPT1,
    RX8111_REGF_SWSEL0,
    RX8111_REGF_SWSEL1,
    RX8111_REGF_INIEN,
    RX8111_REGF_CHGEN,
// RX8111_REG_STATUS_MON.
    RX8111_REGF_VLOW,
// Sentinel value.
    RX8111_REGF_MAX
    };
    static const struct reg_field rx8111_regfields[] = {
    [RX8111_REGF_TSEL0] = REG_FIELD(RX8111_REG_EXT, 0, 0),
    [RX8111_REGF_TSEL1] = REG_FIELD(RX8111_REG_EXT, 1, 1),
    [RX8111_REGF_ETS]   = REG_FIELD(RX8111_REG_EXT, 2, 2),
    [RX8111_REGF_WADA]  = REG_FIELD(RX8111_REG_EXT, 3, 3),
    [RX8111_REGF_TE]    = REG_FIELD(RX8111_REG_EXT, 4, 4),
    [RX8111_REGF_USEL]  = REG_FIELD(RX8111_REG_EXT, 5, 5),
    [RX8111_REGF_FSEL0] = REG_FIELD(RX8111_REG_EXT, 6, 6),
    [RX8111_REGF_FSEL1] = REG_FIELD(RX8111_REG_EXT, 7, 7),
    [RX8111_REGF_XST] = REG_FIELD(RX8111_REG_FLAG, 0, 0),
    [RX8111_REGF_VLF] = REG_FIELD(RX8111_REG_FLAG, 1, 1),
    [RX8111_REGF_EVF] = REG_FIELD(RX8111_REG_FLAG, 2, 2),
    [RX8111_REGF_AF]  = REG_FIELD(RX8111_REG_FLAG, 3, 3),
    [RX8111_REGF_TF]  = REG_FIELD(RX8111_REG_FLAG, 4, 4),
    [RX8111_REGF_UF]  = REG_FIELD(RX8111_REG_FLAG, 5, 5),
    [RX8111_REGF_POR] = REG_FIELD(RX8111_REG_FLAG, 7, 7),
    [RX8111_REGF_STOP] = REG_FIELD(RX8111_REG_CTRL, 0, 0),
    [RX8111_REGF_EIE]  = REG_FIELD(RX8111_REG_CTRL, 2, 2),
    [RX8111_REGF_AIE]  = REG_FIELD(RX8111_REG_CTRL, 3, 3),
    [RX8111_REGF_TIE]  = REG_FIELD(RX8111_REG_CTRL, 4, 4),
    [RX8111_REGF_UIE]  = REG_FIELD(RX8111_REG_CTRL, 5, 5),
    [RX8111_REGF_SMPT0]  = REG_FIELD(RX8111_REG_PWR_SWITCH_CTRL, 0, 0),
    [RX8111_REGF_SMPT1]  = REG_FIELD(RX8111_REG_PWR_SWITCH_CTRL, 1, 1),
    [RX8111_REGF_SWSEL0] = REG_FIELD(RX8111_REG_PWR_SWITCH_CTRL, 2, 2),
    [RX8111_REGF_SWSEL1] = REG_FIELD(RX8111_REG_PWR_SWITCH_CTRL, 3, 3),
    [RX8111_REGF_INIEN]  = REG_FIELD(RX8111_REG_PWR_SWITCH_CTRL, 6, 6),
    [RX8111_REGF_CHGEN]  = REG_FIELD(RX8111_REG_PWR_SWITCH_CTRL, 7, 7),
    [RX8111_REGF_VLOW]  = REG_FIELD(RX8111_REG_STATUS_MON, 1, 1),
    };
    static const struct regmap_config rx8111_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = RX8111_REG_TS_CTRL3,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx8111_data {
    pub regmap: *mut regmap,
    pub regfields: [*mut regmap_field; RX8111_REGF_MAX],
    pub dev: *mut device,
    pub rtc: *mut rtc_device,
}

#[no_mangle]
unsafe extern "C" fn rx8111_read_vl_flag(data: *mut rx8111_data, vlval: *mut c_uint) -> c_int {
    static int rx8111_read_vl_flag(struct rx8111_data *data, unsigned int *vlval)
    {
    int ret;
    ret = regmap_field_read(data.regfields[RX8111_REGF_VLF], vlval);
    if (ret)
    dev_dbg(data.dev, "Could not read VL flag (%d)", ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rx8111_read_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int rx8111_read_time(struct device *dev, struct rtc_time *tm)
    {
    struct rx8111_data *data = dev_get_drvdata(dev);
    u8 buf[RX8111_TIME_BUF_SZ];
    unsigned int regval;
    int ret;
// Check status.
    ret = regmap_read(data.regmap, RX8111_REG_FLAG, &regval);
    if (ret) {
    dev_dbg(data.dev, "Could not read flag register (%d)\n", ret);
    return ret;
    }
    if (FIELD_GET(RX8111_FLAG_XST_BIT, regval)) {
    dev_dbg(data.dev,
    "Crystal oscillation stopped, time is not reliable\n");
    return -EINVAL;
    }
    if (FIELD_GET(RX8111_FLAG_VLF_BIT, regval)) {
    dev_dbg(data.dev,
    "Low voltage detected, time is not reliable\n");
    return -EINVAL;
    }
    ret = regmap_field_read(data.regfields[RX8111_REGF_STOP], &regval);
    if (ret) {
    dev_dbg(data.dev, "Could not read clock status (%d)\n", ret);
    return ret;
    }
    if (regval) {
    dev_dbg(data.dev, "Clock stopped, time is not reliable\n");
    return -EINVAL;
    }
// Read time.
    ret = regmap_bulk_read(data.regmap, RX8111_REG_SEC, buf,
    ARRAY_SIZE(buf));
    if (ret) {
    dev_dbg(data.dev, "Could not bulk read time (%d)\n", ret);
    return ret;
    }
    tm.tm_sec = bcd2bin(buf[0]);
    tm.tm_min = bcd2bin(buf[1]);
    tm.tm_hour = bcd2bin(buf[2]);
    tm.tm_wday = ffs(buf[3]) - 1;
    tm.tm_mday = bcd2bin(buf[4]);
    tm.tm_mon = bcd2bin(buf[5]) - 1;
    tm.tm_year = bcd2bin(buf[6]) + 100;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rx8111_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int rx8111_set_time(struct device *dev, struct rtc_time *tm)
    {
    struct rx8111_data *data = dev_get_drvdata(dev);
    u8 buf[RX8111_TIME_BUF_SZ];
    int ret;
    buf[0] = bin2bcd(tm.tm_sec);
    buf[1] = bin2bcd(tm.tm_min);
    buf[2] = bin2bcd(tm.tm_hour);
    buf[3] = BIT(tm.tm_wday);
    buf[4] = bin2bcd(tm.tm_mday);
    buf[5] = bin2bcd(tm.tm_mon + 1);
    buf[6] = bin2bcd(tm.tm_year - 100);
    ret = regmap_clear_bits(data.regmap, RX8111_REG_FLAG,
    RX8111_FLAG_XST_BIT | RX8111_FLAG_VLF_BIT);
    if (ret)
    return ret;
// Stop the clock.
    ret = regmap_field_write(data.regfields[RX8111_REGF_STOP], 1);
    if (ret) {
    dev_dbg(data.dev, "Could not stop the clock (%d)\n", ret);
    return ret;
    }
// Set the time.
    ret = regmap_bulk_write(data.regmap, RX8111_REG_SEC, buf,
    ARRAY_SIZE(buf));
    if (ret) {
    dev_dbg(data.dev, "Could not bulk write time (%d)\n", ret);
//
// We don't bother with trying to start the clock again. We
// check for this in rx8111_read_time() (and thus force user to
// call rx8111_set_time() to try again).
//
    return ret;
    }
// Start the clock.
    ret = regmap_field_write(data.regfields[RX8111_REGF_STOP], 0);
    if (ret) {
    dev_dbg(data.dev, "Could not start the clock (%d)\n", ret);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rx8111_ioctl(dev: *mut device, cmd: c_uint, arg: c_ulong) -> c_int {
    static int rx8111_ioctl(struct device *dev, unsigned int cmd, unsigned long arg)
    {
    struct rx8111_data *data = dev_get_drvdata(dev);
    unsigned int regval;
    unsigned int vlval;
    int ret;
    switch (cmd) {
    case RTC_VL_READ:
    ret = rx8111_read_vl_flag(data, &regval);
    if (ret)
    return ret;
    vlval = regval ? RTC_VL_DATA_INVALID : 0;
    ret = regmap_field_read(data.regfields[RX8111_REGF_VLOW],
    &regval);
    if (ret)
    return ret;
    vlval |= regval ? RTC_VL_BACKUP_LOW : 0;
    return put_user(vlval, (typeof(vlval) __user *)arg);
    default:
    return -ENOIOCTLCMD;
    }
    }
    static const struct rtc_class_ops rx8111_rtc_ops = {
    .read_time = rx8111_read_time,
    .set_time = rx8111_set_time,
    .ioctl = rx8111_ioctl,
    };
#[no_mangle]
unsafe extern "C" fn rx8111_probe(client: *mut i2c_client) -> c_int {
    static int rx8111_probe(struct i2c_client *client)
    {
    struct rx8111_data *data;
    struct rtc_device *rtc;
    size_t i;
    data = devm_kmalloc(&client.dev, sizeof(*data), GFP_KERNEL);
    if (!data) {
    dev_dbg(&client.dev, "Could not allocate device data\n");
    return -ENOMEM;
    }
    data.dev = &client.dev;
    dev_set_drvdata(data.dev, data);
    data.regmap = devm_regmap_init_i2c(client, &rx8111_regmap_config);
    if (IS_ERR(data.regmap)) {
    dev_dbg(data.dev, "Could not initialize regmap\n");
    return PTR_ERR(data.regmap);
    }
    for (i = 0; i < RX8111_REGF_MAX; ++i) {
    data.regfields[i] = devm_regmap_field_alloc(
    data.dev, data.regmap, rx8111_regfields[i]);
    if (IS_ERR(data.regfields[i])) {
    dev_dbg(data.dev,
    "Could not allocate register field %zu\n", i);
    return PTR_ERR(data.regfields[i]);
    }
    }
    rtc = devm_rtc_allocate_device(data.dev);
    if (IS_ERR(rtc)) {
    dev_dbg(data.dev, "Could not allocate rtc device\n");
    return PTR_ERR(rtc);
    }
    rtc.ops = &rx8111_rtc_ops;
    rtc.range_min = RTC_TIMESTAMP_BEGIN_2000;
    rtc.range_max = RTC_TIMESTAMP_END_2099;
    clear_bit(RTC_FEATURE_ALARM, rtc.features);
    return devm_rtc_register_device(rtc);
    }
    static const struct of_device_id rx8111_of_match[] = {
    {
    .compatible = "epson,rx8111",
    },
    {}
    };
    MODULE_DEVICE_TABLE(of, rx8111_of_match);
    static struct i2c_driver rx8111_driver = {
    .driver = {
    .name = "rtc-rx8111",
    .of_match_table = rx8111_of_match,
    },
    .probe = rx8111_probe,
    };
    module_i2c_driver(rx8111_driver);
    MODULE_AUTHOR("Waqar Hameed <waqar.hameed@axis.com>");
    MODULE_DESCRIPTION("Epson RX8111 RTC driver");
    MODULE_LICENSE("GPL");
