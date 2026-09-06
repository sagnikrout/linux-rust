//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-fm3130.c
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
// rtc-fm3130.c - RTC driver for Ramtron FM3130 I2C chip.
//
// Copyright (C) 2008 Sergey Lapin
// Based on ds1307 driver by James Chapman and David Brownell
//

pub const FM3130_CLOCK_REGS: c_int = 7;
pub const FM3130_ALARM_REGS: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fm3130 {
    pub reg_addr_time: u8,
    pub reg_addr_alarm: u8,
    pub regs: [u8; 15],
    pub msg: [i2c_msg; 4],
    pub client: *mut i2c_client,
    pub rtc: *mut rtc_device,
    pub alarm_valid: c_int,
    pub data_valid: c_int,
}

    static const struct i2c_device_id fm3130_id[] = {
    { .name = "fm3130" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, fm3130_id);
pub const FM3130_MODE_NORMAL: c_int = 0;
pub const FM3130_MODE_WRITE: c_int = 1;
pub const FM3130_MODE_READ: c_int = 2;
#[no_mangle]
unsafe extern "C" fn fm3130_rtc_mode(dev: *mut device, mode: c_int) {
    static void fm3130_rtc_mode(struct device *dev, int mode)
    {
    struct fm3130 *fm3130 = dev_get_drvdata(dev);
    fm3130.regs[FM3130_RTC_CONTROL] =
    i2c_smbus_read_byte_data(fm3130.client, FM3130_RTC_CONTROL);
    switch (mode) {
    case FM3130_MODE_NORMAL:
    fm3130.regs[FM3130_RTC_CONTROL] &=
    ~(FM3130_RTC_CONTROL_BIT_WRITE |
    FM3130_RTC_CONTROL_BIT_READ);
    break;
    case FM3130_MODE_WRITE:
    fm3130.regs[FM3130_RTC_CONTROL] |= FM3130_RTC_CONTROL_BIT_WRITE;
    break;
    case FM3130_MODE_READ:
    fm3130.regs[FM3130_RTC_CONTROL] |= FM3130_RTC_CONTROL_BIT_READ;
    break;
    default:
    dev_dbg(dev, "invalid mode %d\n", mode);
    break;
    }
    i2c_smbus_write_byte_data(fm3130.client,
    FM3130_RTC_CONTROL, fm3130.regs[FM3130_RTC_CONTROL]);
    }
#[no_mangle]
unsafe extern "C" fn fm3130_get_time(dev: *mut device, t: *mut rtc_time) -> c_int {
    static int fm3130_get_time(struct device *dev, struct rtc_time *t)
    {
    struct fm3130 *fm3130 = dev_get_drvdata(dev);
    int		tmp;
    if (!fm3130.data_valid) {
// We have invalid data in RTC, probably due
    to battery faults or other problems. Return EIO
    for now, it will allow us to set data later instead
    of error during probing which disables device */
    return -EIO;
    }
    fm3130_rtc_mode(dev, FM3130_MODE_READ);
// read the RTC date and time registers all at once
    tmp = i2c_transfer(fm3130.client.adapter, fm3130.msg, 2);
    if (tmp != 2) {
    dev_err(dev, "%s error %d\n", "read", tmp);
    return -EIO;
    }
    fm3130_rtc_mode(dev, FM3130_MODE_NORMAL);
    dev_dbg(dev, "%s: %15ph\n", "read", fm3130.regs);
    t.tm_sec = bcd2bin(fm3130.regs[FM3130_RTC_SECONDS] & 0x7f);
    t.tm_min = bcd2bin(fm3130.regs[FM3130_RTC_MINUTES] & 0x7f);
    tmp = fm3130.regs[FM3130_RTC_HOURS] & 0x3f;
    t.tm_hour = bcd2bin(tmp);
    t.tm_wday = bcd2bin(fm3130.regs[FM3130_RTC_DAY] & 0x07) - 1;
    t.tm_mday = bcd2bin(fm3130.regs[FM3130_RTC_DATE] & 0x3f);
    tmp = fm3130.regs[FM3130_RTC_MONTHS] & 0x1f;
    t.tm_mon = bcd2bin(tmp) - 1;
// assume 20YY not 19YY, and ignore CF bit
    t.tm_year = bcd2bin(fm3130.regs[FM3130_RTC_YEARS]) + 100;
    dev_dbg(dev, "%s secs=%d, mins=%d, "
    "hours=%d, mday=%d, mon=%d, year=%d, wday=%d\n",
    "read", t.tm_sec, t.tm_min,
    t.tm_hour, t.tm_mday,
    t.tm_mon, t.tm_year, t.tm_wday);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fm3130_set_time(dev: *mut device, t: *mut rtc_time) -> c_int {
    static int fm3130_set_time(struct device *dev, struct rtc_time *t)
    {
    struct fm3130 *fm3130 = dev_get_drvdata(dev);
    int		tmp, i;
    u8		*buf = fm3130.regs;
    dev_dbg(dev, "%s secs=%d, mins=%d, "
    "hours=%d, mday=%d, mon=%d, year=%d, wday=%d\n",
    "write", t.tm_sec, t.tm_min,
    t.tm_hour, t.tm_mday,
    t.tm_mon, t.tm_year, t.tm_wday);
// first register addr
    buf[FM3130_RTC_SECONDS] = bin2bcd(t.tm_sec);
    buf[FM3130_RTC_MINUTES] = bin2bcd(t.tm_min);
    buf[FM3130_RTC_HOURS] = bin2bcd(t.tm_hour);
    buf[FM3130_RTC_DAY] = bin2bcd(t.tm_wday + 1);
    buf[FM3130_RTC_DATE] = bin2bcd(t.tm_mday);
    buf[FM3130_RTC_MONTHS] = bin2bcd(t.tm_mon + 1);
// assume 20YY not 19YY
    tmp = t.tm_year - 100;
    buf[FM3130_RTC_YEARS] = bin2bcd(tmp);
    dev_dbg(dev, "%s: %15ph\n", "write", buf);
    fm3130_rtc_mode(dev, FM3130_MODE_WRITE);
// Writing time registers, we don't support multibyte transfers
    for (i = 0; i < FM3130_CLOCK_REGS; i++) {
    i2c_smbus_write_byte_data(fm3130.client,
    FM3130_RTC_SECONDS + i,
    fm3130.regs[FM3130_RTC_SECONDS + i]);
    }
    fm3130_rtc_mode(dev, FM3130_MODE_NORMAL);
// We assume here that data are valid once written
    if (!fm3130.data_valid)
    fm3130.data_valid = 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fm3130_read_alarm(dev: *mut device, alrm: *mut rtc_wkalrm) -> c_int {
    static int fm3130_read_alarm(struct device *dev, struct rtc_wkalrm *alrm)
    {
    struct fm3130 *fm3130 = dev_get_drvdata(dev);
    int tmp;
    struct rtc_time *tm = &alrm.time;
    if (!fm3130.alarm_valid) {
//
// We have invalid alarm in RTC, probably due to battery faults
// or other problems. Return EIO for now, it will allow us to
// set alarm value later instead of error during probing which
// disables device
//
    return -EIO;
    }
// read the RTC alarm registers all at once
    tmp = i2c_transfer(fm3130.client.adapter, &fm3130.msg[2], 2);
    if (tmp != 2) {
    dev_err(dev, "%s error %d\n", "read", tmp);
    return -EIO;
    }
    dev_dbg(dev, "alarm read %02x %02x %02x %02x %02x\n",
    fm3130.regs[FM3130_ALARM_SECONDS],
    fm3130.regs[FM3130_ALARM_MINUTES],
    fm3130.regs[FM3130_ALARM_HOURS],
    fm3130.regs[FM3130_ALARM_DATE],
    fm3130.regs[FM3130_ALARM_MONTHS]);
    tm.tm_sec	= bcd2bin(fm3130.regs[FM3130_ALARM_SECONDS] & 0x7F);
    tm.tm_min	= bcd2bin(fm3130.regs[FM3130_ALARM_MINUTES] & 0x7F);
    tm.tm_hour	= bcd2bin(fm3130.regs[FM3130_ALARM_HOURS] & 0x3F);
    tm.tm_mday	= bcd2bin(fm3130.regs[FM3130_ALARM_DATE] & 0x3F);
    tm.tm_mon	= bcd2bin(fm3130.regs[FM3130_ALARM_MONTHS] & 0x1F);
    if (tm.tm_mon > 0)
    tm.tm_mon -= 1; /* RTC is 1-12, tm_mon is 0-11 */
    dev_dbg(dev, "%s secs=%d, mins=%d, "
    "hours=%d, mday=%d, mon=%d, year=%d, wday=%d\n",
    "read alarm", tm.tm_sec, tm.tm_min,
    tm.tm_hour, tm.tm_mday,
    tm.tm_mon, tm.tm_year, tm.tm_wday);
// check if alarm enabled
    fm3130.regs[FM3130_RTC_CONTROL] =
    i2c_smbus_read_byte_data(fm3130.client, FM3130_RTC_CONTROL);
    if ((fm3130.regs[FM3130_RTC_CONTROL] & FM3130_RTC_CONTROL_BIT_AEN) &&
    (~fm3130.regs[FM3130_RTC_CONTROL] &
    FM3130_RTC_CONTROL_BIT_CAL)) {
    alrm.enabled = 1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fm3130_set_alarm(dev: *mut device, alrm: *mut rtc_wkalrm) -> c_int {
    static int fm3130_set_alarm(struct device *dev, struct rtc_wkalrm *alrm)
    {
    struct fm3130 *fm3130 = dev_get_drvdata(dev);
    struct rtc_time *tm = &alrm.time;
    int i;
    dev_dbg(dev, "%s secs=%d, mins=%d, "
    "hours=%d, mday=%d, mon=%d, year=%d, wday=%d\n",
    "write alarm", tm.tm_sec, tm.tm_min,
    tm.tm_hour, tm.tm_mday,
    tm.tm_mon, tm.tm_year, tm.tm_wday);
    fm3130.regs[FM3130_ALARM_SECONDS] =
    (tm.tm_sec != -1) ? bin2bcd(tm.tm_sec) : 0x80;
    fm3130.regs[FM3130_ALARM_MINUTES] =
    (tm.tm_min != -1) ? bin2bcd(tm.tm_min) : 0x80;
    fm3130.regs[FM3130_ALARM_HOURS] =
    (tm.tm_hour != -1) ? bin2bcd(tm.tm_hour) : 0x80;
    fm3130.regs[FM3130_ALARM_DATE] =
    (tm.tm_mday != -1) ? bin2bcd(tm.tm_mday) : 0x80;
    fm3130.regs[FM3130_ALARM_MONTHS] =
    (tm.tm_mon != -1) ? bin2bcd(tm.tm_mon + 1) : 0x80;
    dev_dbg(dev, "alarm write %02x %02x %02x %02x %02x\n",
    fm3130.regs[FM3130_ALARM_SECONDS],
    fm3130.regs[FM3130_ALARM_MINUTES],
    fm3130.regs[FM3130_ALARM_HOURS],
    fm3130.regs[FM3130_ALARM_DATE],
    fm3130.regs[FM3130_ALARM_MONTHS]);
// Writing time registers, we don't support multibyte transfers
    for (i = 0; i < FM3130_ALARM_REGS; i++) {
    i2c_smbus_write_byte_data(fm3130.client,
    FM3130_ALARM_SECONDS + i,
    fm3130.regs[FM3130_ALARM_SECONDS + i]);
    }
    fm3130.regs[FM3130_RTC_CONTROL] =
    i2c_smbus_read_byte_data(fm3130.client, FM3130_RTC_CONTROL);
// enable or disable alarm
    if (alrm.enabled) {
    i2c_smbus_write_byte_data(fm3130.client, FM3130_RTC_CONTROL,
    (fm3130.regs[FM3130_RTC_CONTROL] &
    ~(FM3130_RTC_CONTROL_BIT_CAL)) |
    FM3130_RTC_CONTROL_BIT_AEN);
    } else {
    i2c_smbus_write_byte_data(fm3130.client, FM3130_RTC_CONTROL,
    fm3130.regs[FM3130_RTC_CONTROL] &
    ~(FM3130_RTC_CONTROL_BIT_CAL) &
    ~(FM3130_RTC_CONTROL_BIT_AEN));
    }
// We assume here that data is valid once written
    if (!fm3130.alarm_valid)
    fm3130.alarm_valid = 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fm3130_alarm_irq_enable(dev: *mut device, enabled: c_uint) -> c_int {
    static int fm3130_alarm_irq_enable(struct device *dev, unsigned int enabled)
    {
    struct fm3130 *fm3130 = dev_get_drvdata(dev);
    let mut ret: c_int = 0;
    fm3130.regs[FM3130_RTC_CONTROL] =
    i2c_smbus_read_byte_data(fm3130.client, FM3130_RTC_CONTROL);
    dev_dbg(dev, "alarm_irq_enable: enable=%d, FM3130_RTC_CONTROL=%02x\n",
    enabled, fm3130.regs[FM3130_RTC_CONTROL]);
    switch (enabled) {
    case 0:		/* alarm off */
    ret = i2c_smbus_write_byte_data(fm3130.client,
    FM3130_RTC_CONTROL, fm3130.regs[FM3130_RTC_CONTROL] &
    ~(FM3130_RTC_CONTROL_BIT_CAL) &
    ~(FM3130_RTC_CONTROL_BIT_AEN));
    break;
    case 1:		/* alarm on */
    ret = i2c_smbus_write_byte_data(fm3130.client,
    FM3130_RTC_CONTROL, (fm3130.regs[FM3130_RTC_CONTROL] &
    ~(FM3130_RTC_CONTROL_BIT_CAL)) |
    FM3130_RTC_CONTROL_BIT_AEN);
    break;
    default:
    ret = -EINVAL;
    break;
    }
    return ret;
    }
    static const struct rtc_class_ops fm3130_rtc_ops = {
    .read_time	= fm3130_get_time,
    .set_time	= fm3130_set_time,
    .read_alarm	= fm3130_read_alarm,
    .set_alarm	= fm3130_set_alarm,
    .alarm_irq_enable = fm3130_alarm_irq_enable,
    };
    static struct i2c_driver fm3130_driver;
#[no_mangle]
unsafe extern "C" fn fm3130_probe(client: *mut i2c_client) -> c_int {
    static int fm3130_probe(struct i2c_client *client)
    {
    struct fm3130		*fm3130;
    let mut err: c_int = -ENODEV;
    int			tmp;
    struct i2c_adapter	*adapter = client.adapter;
    if (!i2c_check_functionality(adapter,
    I2C_FUNC_I2C | I2C_FUNC_SMBUS_WRITE_BYTE_DATA))
    return -EIO;
    fm3130 = devm_kzalloc(&client.dev, sizeof(struct fm3130), GFP_KERNEL);
    if (!fm3130)
    return -ENOMEM;
    fm3130.client = client;
    i2c_set_clientdata(client, fm3130);
    fm3130.reg_addr_time = FM3130_RTC_SECONDS;
    fm3130.reg_addr_alarm = FM3130_ALARM_SECONDS;
// Messages to read time
    fm3130.msg[0].addr = client.addr;
    fm3130.msg[0].flags = 0;
    fm3130.msg[0].len = 1;
    fm3130.msg[0].buf = &fm3130.reg_addr_time;
    fm3130.msg[1].addr = client.addr;
    fm3130.msg[1].flags = I2C_M_RD;
    fm3130.msg[1].len = FM3130_CLOCK_REGS;
    fm3130.msg[1].buf = &fm3130.regs[FM3130_RTC_SECONDS];
// Messages to read alarm
    fm3130.msg[2].addr = client.addr;
    fm3130.msg[2].flags = 0;
    fm3130.msg[2].len = 1;
    fm3130.msg[2].buf = &fm3130.reg_addr_alarm;
    fm3130.msg[3].addr = client.addr;
    fm3130.msg[3].flags = I2C_M_RD;
    fm3130.msg[3].len = FM3130_ALARM_REGS;
    fm3130.msg[3].buf = &fm3130.regs[FM3130_ALARM_SECONDS];
    fm3130.alarm_valid = 0;
    fm3130.data_valid = 0;
    tmp = i2c_transfer(adapter, fm3130.msg, 4);
    if (tmp != 4) {
    dev_dbg(&client.dev, "read error %d\n", tmp);
    err = -EIO;
    goto exit_free;
    }
    fm3130.regs[FM3130_RTC_CONTROL] =
    i2c_smbus_read_byte_data(client, FM3130_RTC_CONTROL);
    fm3130.regs[FM3130_CAL_CONTROL] =
    i2c_smbus_read_byte_data(client, FM3130_CAL_CONTROL);
// Disabling calibration mode
    if (fm3130.regs[FM3130_RTC_CONTROL] & FM3130_RTC_CONTROL_BIT_CAL) {
    i2c_smbus_write_byte_data(client, FM3130_RTC_CONTROL,
    fm3130.regs[FM3130_RTC_CONTROL] &
    ~(FM3130_RTC_CONTROL_BIT_CAL));
    dev_warn(&client.dev, "Disabling calibration mode!\n");
    }
// Disabling read and write modes
    if (fm3130.regs[FM3130_RTC_CONTROL] & FM3130_RTC_CONTROL_BIT_WRITE ||
    fm3130.regs[FM3130_RTC_CONTROL] & FM3130_RTC_CONTROL_BIT_READ) {
    i2c_smbus_write_byte_data(client, FM3130_RTC_CONTROL,
    fm3130.regs[FM3130_RTC_CONTROL] &
    ~(FM3130_RTC_CONTROL_BIT_READ |
    FM3130_RTC_CONTROL_BIT_WRITE));
    dev_warn(&client.dev, "Disabling READ or WRITE mode!\n");
    }
// oscillator off?  turn it on, so clock can tick.
    if (fm3130.regs[FM3130_CAL_CONTROL] & FM3130_CAL_CONTROL_BIT_nOSCEN)
    i2c_smbus_write_byte_data(client, FM3130_CAL_CONTROL,
    fm3130.regs[FM3130_CAL_CONTROL] &
    ~(FM3130_CAL_CONTROL_BIT_nOSCEN));
// low battery?  clear flag, and warn
    if (fm3130.regs[FM3130_RTC_CONTROL] & FM3130_RTC_CONTROL_BIT_LB) {
    i2c_smbus_write_byte_data(client, FM3130_RTC_CONTROL,
    fm3130.regs[FM3130_RTC_CONTROL] &
    ~(FM3130_RTC_CONTROL_BIT_LB));
    dev_warn(&client.dev, "Low battery!\n");
    }
// check if Power On Reset bit is set
    if (fm3130.regs[FM3130_RTC_CONTROL] & FM3130_RTC_CONTROL_BIT_POR) {
    i2c_smbus_write_byte_data(client, FM3130_RTC_CONTROL,
    fm3130.regs[FM3130_RTC_CONTROL] &
    ~FM3130_RTC_CONTROL_BIT_POR);
    dev_dbg(&client.dev, "POR bit is set\n");
    }
// ACS is controlled by alarm
    i2c_smbus_write_byte_data(client, FM3130_ALARM_WP_CONTROL, 0x80);
// alarm registers sanity check
    tmp = bcd2bin(fm3130.regs[FM3130_RTC_SECONDS] & 0x7f);
    if (tmp > 59)
    goto bad_alarm;
    tmp = bcd2bin(fm3130.regs[FM3130_RTC_MINUTES] & 0x7f);
    if (tmp > 59)
    goto bad_alarm;
    tmp = bcd2bin(fm3130.regs[FM3130_RTC_HOURS] & 0x3f);
    if (tmp > 23)
    goto bad_alarm;
    tmp = bcd2bin(fm3130.regs[FM3130_RTC_DATE] & 0x3f);
    if (tmp == 0 || tmp > 31)
    goto bad_alarm;
    tmp = bcd2bin(fm3130.regs[FM3130_RTC_MONTHS] & 0x1f);
    if (tmp == 0 || tmp > 12)
    goto bad_alarm;
    fm3130.alarm_valid = 1;
    bad_alarm:
// clock registers sanity chek
    tmp = bcd2bin(fm3130.regs[FM3130_RTC_SECONDS] & 0x7f);
    if (tmp > 59)
    goto bad_clock;
    tmp = bcd2bin(fm3130.regs[FM3130_RTC_MINUTES] & 0x7f);
    if (tmp > 59)
    goto bad_clock;
    tmp = bcd2bin(fm3130.regs[FM3130_RTC_HOURS] & 0x3f);
    if (tmp > 23)
    goto bad_clock;
    tmp = bcd2bin(fm3130.regs[FM3130_RTC_DAY] & 0x7);
    if (tmp == 0 || tmp > 7)
    goto bad_clock;
    tmp = bcd2bin(fm3130.regs[FM3130_RTC_DATE] & 0x3f);
    if (tmp == 0 || tmp > 31)
    goto bad_clock;
    tmp = bcd2bin(fm3130.regs[FM3130_RTC_MONTHS] & 0x1f);
    if (tmp == 0 || tmp > 12)
    goto bad_clock;
    fm3130.data_valid = 1;
    bad_clock:
    if (!fm3130.data_valid || !fm3130.alarm_valid)
    dev_dbg(&client.dev, "%s: %15ph\n", "bogus registers",
    fm3130.regs);
// We won't bail out here because we just got invalid data.
    Time setting from u-boot doesn't work anyway */
    fm3130.rtc = devm_rtc_device_register(&client.dev, client.name,
    &fm3130_rtc_ops, THIS_MODULE);
    if (IS_ERR(fm3130.rtc)) {
    err = PTR_ERR(fm3130.rtc);
    dev_err(&client.dev,
    "unable to register the class device\n");
    goto exit_free;
    }
    return 0;
    exit_free:
    return err;
    }
    static struct i2c_driver fm3130_driver = {
    .driver = {
    .name	= "rtc-fm3130",
    },
    .probe		= fm3130_probe,
    .id_table	= fm3130_id,
    };
    module_i2c_driver(fm3130_driver);
    MODULE_DESCRIPTION("RTC driver for FM3130");
    MODULE_AUTHOR("Sergey Lapin <slapin@ossfans.org>");
    MODULE_LICENSE("GPL");
