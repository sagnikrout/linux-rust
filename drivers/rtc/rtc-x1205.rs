//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-x1205.c
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
// An i2c driver for the Xicor/Intersil X1205 RTC
// Copyright 2004 Karen Spearel
// Copyright 2005 Alessandro Zummo
//
// please send all reports to:
// Karen Spearel <kas111 at gmail dot com>
// Alessandro Zummo <a.zummo@towertech.it>
//
// based on a lot of other RTC drivers.
//
// Information and datasheet:
// http://www.intersil.com/cda/deviceinfo/0,1477,X1205,00.html
//

// offsets into CCR area
pub const CCR_SEC: c_int = 0;
pub const CCR_MIN: c_int = 1;
pub const CCR_HOUR: c_int = 2;
pub const CCR_MDAY: c_int = 3;
pub const CCR_MONTH: c_int = 4;
pub const CCR_YEAR: c_int = 5;
pub const CCR_WDAY: c_int = 6;
pub const CCR_Y2K: c_int = 7;
pub const X1205_REG_SR: c_uint = 0x3F	/* status register */;
pub const X1205_REG_Y2K: c_uint = 0x37;
pub const X1205_REG_DW: c_uint = 0x36;
pub const X1205_REG_YR: c_uint = 0x35;
pub const X1205_REG_MO: c_uint = 0x34;
pub const X1205_REG_DT: c_uint = 0x33;
pub const X1205_REG_HR: c_uint = 0x32;
pub const X1205_REG_MN: c_uint = 0x31;
pub const X1205_REG_SC: c_uint = 0x30;
pub const X1205_REG_DTR: c_uint = 0x13;
pub const X1205_REG_ATR: c_uint = 0x12;
pub const X1205_REG_INT: c_uint = 0x11;
pub const X1205_REG_0: c_uint = 0x10;
pub const X1205_REG_Y2K1: c_uint = 0x0F;
pub const X1205_REG_DWA1: c_uint = 0x0E;
pub const X1205_REG_YRA1: c_uint = 0x0D;
pub const X1205_REG_MOA1: c_uint = 0x0C;
pub const X1205_REG_DTA1: c_uint = 0x0B;
pub const X1205_REG_HRA1: c_uint = 0x0A;
pub const X1205_REG_MNA1: c_uint = 0x09;
pub const X1205_REG_SCA1: c_uint = 0x08;
pub const X1205_REG_Y2K0: c_uint = 0x07;
pub const X1205_REG_DWA0: c_uint = 0x06;
pub const X1205_REG_YRA0: c_uint = 0x05;
pub const X1205_REG_MOA0: c_uint = 0x04;
pub const X1205_REG_DTA0: c_uint = 0x03;
pub const X1205_REG_HRA0: c_uint = 0x02;
pub const X1205_REG_MNA0: c_uint = 0x01;
pub const X1205_REG_SCA0: c_uint = 0x00;
pub const X1205_CCR_BASE: c_uint = 0x30	/* Base address of CCR */;
pub const X1205_ALM0_BASE: c_uint = 0x00	/* Base address of ALARM0 */;
pub const X1205_SR_RTCF: c_uint = 0x01	/* Clock failure */;
pub const X1205_SR_WEL: c_uint = 0x02	/* Write Enable Latch */;
pub const X1205_SR_RWEL: c_uint = 0x04	/* Register Write Enable */;
pub const X1205_SR_AL0: c_uint = 0x20	/* Alarm 0 match */;
pub const X1205_DTR_DTR0: c_uint = 0x01;
pub const X1205_DTR_DTR1: c_uint = 0x02;
pub const X1205_DTR_DTR2: c_uint = 0x04;
pub const X1205_HR_MIL: c_uint = 0x80	/* Set in ccr.hour for 24 hr mode */;
pub const X1205_INT_AL0E: c_uint = 0x20	/* Alarm 0 enable */;
    static struct i2c_driver x1205_driver;
//
// In the routines that deal directly with the x1205 hardware, we use
// rtc_time -- month 0-11, hour 0-23, yr = calendar year-epoch
// Epoch is initialized as 2000. Time is set to UTC.
//
    static int x1205_get_datetime(struct i2c_client *client, struct rtc_time *tm,
    unsigned char reg_base)
    {
    unsigned char dt_addr[2] = { 0, reg_base };
    unsigned char buf[8];
    int i;
    struct i2c_msg msgs[] = {
    {/* setup read ptr */
    .addr = client.addr,
    .len = 2,
    .buf = dt_addr
    },
    {/* read date */
    .addr = client.addr,
    .flags = I2C_M_RD,
    .len = 8,
    .buf = buf
    },
    };
// read date registers
    if (i2c_transfer(client.adapter, &msgs[0], 2) != 2) {
    dev_err(&client.dev, "%s: read error\n", __func__);
    return -EIO;
    }
    dev_dbg(&client.dev,
    "%s: raw read data - sec=%02x, min=%02x, hr=%02x, "
    "mday=%02x, mon=%02x, year=%02x, wday=%02x, y2k=%02x\n",
    __func__,
    buf[0], buf[1], buf[2], buf[3],
    buf[4], buf[5], buf[6], buf[7]);
// Mask out the enable bits if these are alarm registers
    if (reg_base < X1205_CCR_BASE)
    for (i = 0; i <= 4; i++)
    buf[i] &= 0x7F;
    tm.tm_sec = bcd2bin(buf[CCR_SEC]);
    tm.tm_min = bcd2bin(buf[CCR_MIN]);
    tm.tm_hour = bcd2bin(buf[CCR_HOUR] & 0x3F); /* hr is 0-23 */
    tm.tm_mday = bcd2bin(buf[CCR_MDAY]);
    tm.tm_mon = bcd2bin(buf[CCR_MONTH]) - 1; /* mon is 0-11 */
    tm.tm_year = bcd2bin(buf[CCR_YEAR])
    + (bcd2bin(buf[CCR_Y2K]) * 100) - 1900;
    tm.tm_wday = buf[CCR_WDAY];
    dev_dbg(&client.dev, "%s: tm is secs=%d, mins=%d, hours=%d, "
    "mday=%d, mon=%d, year=%d, wday=%d\n",
    __func__,
    tm.tm_sec, tm.tm_min, tm.tm_hour,
    tm.tm_mday, tm.tm_mon, tm.tm_year, tm.tm_wday);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn x1205_get_status(client: *mut i2c_client, sr: *mut c_uchar) -> c_int {
    static int x1205_get_status(struct i2c_client *client, unsigned char *sr)
    {
    static unsigned char sr_addr[2] = { 0, X1205_REG_SR };
    struct i2c_msg msgs[] = {
    {     /* setup read ptr */
    .addr = client.addr,
    .len = 2,
    .buf = sr_addr
    },
    {    /* read status */
    .addr = client.addr,
    .flags = I2C_M_RD,
    .len = 1,
    .buf = sr
    },
    };
// read status register
    if (i2c_transfer(client.adapter, &msgs[0], 2) != 2) {
    dev_err(&client.dev, "%s: read error\n", __func__);
    return -EIO;
    }
    return 0;
    }
    static int x1205_set_datetime(struct i2c_client *client, struct rtc_time *tm,
    u8 reg_base, unsigned char alm_enable)
    {
    int i, xfer;
    unsigned char rdata[10] = { 0, reg_base };
    unsigned char *buf = rdata + 2;
    static const unsigned char wel[3] = { 0, X1205_REG_SR,
    X1205_SR_WEL };
    static const unsigned char rwel[3] = { 0, X1205_REG_SR,
    X1205_SR_WEL | X1205_SR_RWEL };
    static const unsigned char diswe[3] = { 0, X1205_REG_SR, 0 };
    dev_dbg(&client.dev,
    "%s: sec=%d min=%d hour=%d mday=%d mon=%d year=%d wday=%d\n",
    __func__, tm.tm_sec, tm.tm_min, tm.tm_hour, tm.tm_mday,
    tm.tm_mon, tm.tm_year, tm.tm_wday);
    buf[CCR_SEC] = bin2bcd(tm.tm_sec);
    buf[CCR_MIN] = bin2bcd(tm.tm_min);
// set hour and 24hr bit
    buf[CCR_HOUR] = bin2bcd(tm.tm_hour) | X1205_HR_MIL;
    buf[CCR_MDAY] = bin2bcd(tm.tm_mday);
// month, 1 - 12
    buf[CCR_MONTH] = bin2bcd(tm.tm_mon + 1);
// year, since the rtc epoch
    buf[CCR_YEAR] = bin2bcd(tm.tm_year % 100);
    buf[CCR_WDAY] = tm.tm_wday & 0x07;
    buf[CCR_Y2K] = bin2bcd((tm.tm_year + 1900) / 100);
// If writing alarm registers, set compare bits on registers 0-4
    if (reg_base < X1205_CCR_BASE)
    for (i = 0; i <= 4; i++)
    buf[i] |= 0x80;
// this sequence is required to unlock the chip
    xfer = i2c_master_send(client, wel, 3);
    if (xfer != 3) {
    dev_err(&client.dev, "%s: wel - %d\n", __func__, xfer);
    return -EIO;
    }
    xfer = i2c_master_send(client, rwel, 3);
    if (xfer != 3) {
    dev_err(&client.dev, "%s: rwel - %d\n", __func__, xfer);
    return -EIO;
    }
    xfer = i2c_master_send(client, rdata, sizeof(rdata));
    if (xfer != sizeof(rdata)) {
    dev_err(&client.dev,
    "%s: result=%d addr=%02x, data=%02x\n",
    __func__,
    xfer, rdata[1], rdata[2]);
    return -EIO;
    }
// If we wrote to the nonvolatile region, wait 10msec for write cycle
    if (reg_base < X1205_CCR_BASE) {
    unsigned char al0e[3] = { 0, X1205_REG_INT, 0 };
    msleep(10);
// ...and set or clear the AL0E bit in the INT register
// Need to set RWEL again as the write has cleared it
    xfer = i2c_master_send(client, rwel, 3);
    if (xfer != 3) {
    dev_err(&client.dev,
    "%s: aloe rwel - %d\n",
    __func__,
    xfer);
    return -EIO;
    }
    if (alm_enable)
    al0e[2] = X1205_INT_AL0E;
    xfer = i2c_master_send(client, al0e, 3);
    if (xfer != 3) {
    dev_err(&client.dev,
    "%s: al0e - %d\n",
    __func__,
    xfer);
    return -EIO;
    }
// and wait 10msec again for this write to complete
    msleep(10);
    }
// disable further writes
    xfer = i2c_master_send(client, diswe, 3);
    if (xfer != 3) {
    dev_err(&client.dev, "%s: diswe - %d\n", __func__, xfer);
    return -EIO;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn x1205_fix_osc(client: *mut i2c_client) -> c_int {
    static int x1205_fix_osc(struct i2c_client *client)
    {
    int err;
    struct rtc_time tm;
    memset(&tm, 0, sizeof(tm));
    err = x1205_set_datetime(client, &tm, X1205_CCR_BASE, 0);
    if (err < 0)
    dev_err(&client.dev, "unable to restart the oscillator\n");
    return err;
    }
#[no_mangle]
unsafe extern "C" fn x1205_get_dtrim(client: *mut i2c_client, trim: *mut c_int) -> c_int {
    static int x1205_get_dtrim(struct i2c_client *client, int *trim)
    {
    unsigned char dtr;
    static unsigned char dtr_addr[2] = { 0, X1205_REG_DTR };
    struct i2c_msg msgs[] = {
    {	/* setup read ptr */
    .addr = client.addr,
    .len = 2,
    .buf = dtr_addr
    },
    {      /* read dtr */
    .addr = client.addr,
    .flags = I2C_M_RD,
    .len = 1,
    .buf = &dtr
    },
    };
// read dtr register
    if (i2c_transfer(client.adapter, &msgs[0], 2) != 2) {
    dev_err(&client.dev, "%s: read error\n", __func__);
    return -EIO;
    }
    dev_dbg(&client.dev, "%s: raw dtr=%x\n", __func__, dtr);
// trim = 0;
    if (dtr & X1205_DTR_DTR0)
// trim += 20;
    if (dtr & X1205_DTR_DTR1)
// trim += 10;
    if (dtr & X1205_DTR_DTR2)
// trim = -*trim;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn x1205_get_atrim(client: *mut i2c_client, trim: *mut c_int) -> c_int {
    static int x1205_get_atrim(struct i2c_client *client, int *trim)
    {
    s8 atr;
    static unsigned char atr_addr[2] = { 0, X1205_REG_ATR };
    struct i2c_msg msgs[] = {
    {/* setup read ptr */
    .addr = client.addr,
    .len = 2,
    .buf = atr_addr
    },
    {/* read atr */
    .addr = client.addr,
    .flags = I2C_M_RD,
    .len = 1,
    .buf = &atr
    },
    };
// read atr register
    if (i2c_transfer(client.adapter, &msgs[0], 2) != 2) {
    dev_err(&client.dev, "%s: read error\n", __func__);
    return -EIO;
    }
    dev_dbg(&client.dev, "%s: raw atr=%x\n", __func__, atr);
// atr is a two's complement value on 6 bits,
// perform sign extension. The formula is
// Catr = (atr * 0.25pF) + 11.00pF.
//
    atr = sign_extend32(atr, 5);
    dev_dbg(&client.dev, "%s: raw atr=%x (%d)\n", __func__, atr, atr);
// trim = (atr * 250) + 11000;
    dev_dbg(&client.dev, "%s: real=%d\n", __func__, *trim);
    return 0;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct x1205_limit {
    pub max: unsigned char reg, mask, min,,
}

#[no_mangle]
unsafe extern "C" fn x1205_validate_client(client: *mut i2c_client) -> c_int {
    static int x1205_validate_client(struct i2c_client *client)
    {
    int i, xfer;
// Probe array. We will read the register at the specified
// address and check if the given bits are zero.
//
    static const unsigned char probe_zero_pattern[] = {
// register, mask
    X1205_REG_SR,	0x18,
    X1205_REG_DTR,	0xF8,
    X1205_REG_ATR,	0xC0,
    X1205_REG_INT,	0x18,
    X1205_REG_0,	0xFF,
    };
    static const struct x1205_limit probe_limits_pattern[] = {
// register, mask, min, max
    { X1205_REG_Y2K,	0xFF,	19,	20	},
    { X1205_REG_DW,		0xFF,	0,	6	},
    { X1205_REG_YR,		0xFF,	0,	99	},
    { X1205_REG_MO,		0xFF,	0,	12	},
    { X1205_REG_DT,		0xFF,	0,	31	},
    { X1205_REG_HR,		0x7F,	0,	23	},
    { X1205_REG_MN,		0xFF,	0,	59	},
    { X1205_REG_SC,		0xFF,	0,	59	},
    { X1205_REG_Y2K1,	0xFF,	19,	20	},
    { X1205_REG_Y2K0,	0xFF,	19,	20	},
    };
// check that registers have bits a 0 where expected
    for (i = 0; i < ARRAY_SIZE(probe_zero_pattern); i += 2) {
    unsigned char buf;
    unsigned char addr[2] = { 0, probe_zero_pattern[i] };
    struct i2c_msg msgs[2] = {
    {
    .addr = client.addr,
    .len = 2,
    .buf = addr
    },
    {
    .addr = client.addr,
    .flags = I2C_M_RD,
    .len = 1,
    .buf = &buf
    },
    };
    xfer = i2c_transfer(client.adapter, msgs, 2);
    if (xfer != 2) {
    dev_err(&client.dev,
    "%s: could not read register %x\n",
    __func__, probe_zero_pattern[i]);
    return -EIO;
    }
    if ((buf & probe_zero_pattern[i+1]) != 0) {
    dev_err(&client.dev,
    "%s: register=%02x, zero pattern=%d, value=%x\n",
    __func__, probe_zero_pattern[i], i, buf);
    return -ENODEV;
    }
    }
// check limits (only registers with bcd values)
    for (i = 0; i < ARRAY_SIZE(probe_limits_pattern); i++) {
    unsigned char reg, value;
    unsigned char addr[2] = { 0, probe_limits_pattern[i].reg };
    struct i2c_msg msgs[2] = {
    {
    .addr = client.addr,
    .len = 2,
    .buf = addr
    },
    {
    .addr = client.addr,
    .flags = I2C_M_RD,
    .len = 1,
    .buf = &reg
    },
    };
    xfer = i2c_transfer(client.adapter, msgs, 2);
    if (xfer != 2) {
    dev_err(&client.dev,
    "%s: could not read register %x\n",
    __func__, probe_limits_pattern[i].reg);
    return -EIO;
    }
    value = bcd2bin(reg & probe_limits_pattern[i].mask);
    if (value > probe_limits_pattern[i].max ||
    value < probe_limits_pattern[i].min) {
    dev_dbg(&client.dev,
    "%s: register=%x, lim pattern=%d, value=%d\n",
    __func__, probe_limits_pattern[i].reg,
    i, value);
    return -ENODEV;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn x1205_rtc_read_alarm(dev: *mut device, alrm: *mut rtc_wkalrm) -> c_int {
    static int x1205_rtc_read_alarm(struct device *dev, struct rtc_wkalrm *alrm)
    {
    int err;
    unsigned char intreg, status;
    static unsigned char int_addr[2] = { 0, X1205_REG_INT };
    struct i2c_client *client = to_i2c_client(dev);
    struct i2c_msg msgs[] = {
    { /* setup read ptr */
    .addr = client.addr,
    .len = 2,
    .buf = int_addr
    },
    {/* read INT register */
    .addr = client.addr,
    .flags = I2C_M_RD,
    .len = 1,
    .buf = &intreg
    },
    };
// read interrupt register and status register
    if (i2c_transfer(client.adapter, &msgs[0], 2) != 2) {
    dev_err(&client.dev, "%s: read error\n", __func__);
    return -EIO;
    }
    err = x1205_get_status(client, &status);
    if (err == 0) {
    alrm.pending = (status & X1205_SR_AL0) ? 1 : 0;
    alrm.enabled = (intreg & X1205_INT_AL0E) ? 1 : 0;
    err = x1205_get_datetime(client, &alrm.time, X1205_ALM0_BASE);
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn x1205_rtc_set_alarm(dev: *mut device, alrm: *mut rtc_wkalrm) -> c_int {
    static int x1205_rtc_set_alarm(struct device *dev, struct rtc_wkalrm *alrm)
    {
    return x1205_set_datetime(to_i2c_client(dev),
    &alrm.time, X1205_ALM0_BASE, alrm.enabled);
    }
#[no_mangle]
unsafe extern "C" fn x1205_rtc_read_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int x1205_rtc_read_time(struct device *dev, struct rtc_time *tm)
    {
    return x1205_get_datetime(to_i2c_client(dev),
    tm, X1205_CCR_BASE);
    }
#[no_mangle]
unsafe extern "C" fn x1205_rtc_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int x1205_rtc_set_time(struct device *dev, struct rtc_time *tm)
    {
    return x1205_set_datetime(to_i2c_client(dev),
    tm, X1205_CCR_BASE, 0);
    }
#[no_mangle]
unsafe extern "C" fn x1205_rtc_proc(dev: *mut device, seq: *mut seq_file) -> c_int {
    static int x1205_rtc_proc(struct device *dev, struct seq_file *seq)
    {
    int err, dtrim, atrim;
    err = x1205_get_dtrim(to_i2c_client(dev), &dtrim);
    if (!err)
    seq_printf(seq, "digital_trim\t: %d ppm\n", dtrim);
    err = x1205_get_atrim(to_i2c_client(dev), &atrim);
    if (!err)
    seq_printf(seq, "analog_trim\t: %d.%02d pF\n",
    atrim / 1000, atrim % 1000);
    return 0;
    }
    static const struct rtc_class_ops x1205_rtc_ops = {
    .proc		= x1205_rtc_proc,
    .read_time	= x1205_rtc_read_time,
    .set_time	= x1205_rtc_set_time,
    .read_alarm	= x1205_rtc_read_alarm,
    .set_alarm	= x1205_rtc_set_alarm,
    };
    static ssize_t x1205_sysfs_show_atrim(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    int err, atrim;
    err = x1205_get_atrim(to_i2c_client(dev), &atrim);
    if (err)
    return err;
    return sprintf(buf, "%d.%02d pF\n", atrim / 1000, atrim % 1000);
    }
    static DEVICE_ATTR(atrim, S_IRUGO, x1205_sysfs_show_atrim, core::ptr::null_mut());
    static ssize_t x1205_sysfs_show_dtrim(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    int err, dtrim;
    err = x1205_get_dtrim(to_i2c_client(dev), &dtrim);
    if (err)
    return err;
    return sprintf(buf, "%d ppm\n", dtrim);
    }
    static DEVICE_ATTR(dtrim, S_IRUGO, x1205_sysfs_show_dtrim, core::ptr::null_mut());
#[no_mangle]
unsafe extern "C" fn x1205_sysfs_register(dev: *mut device) -> c_int {
    static int x1205_sysfs_register(struct device *dev)
    {
    int err;
    err = device_create_file(dev, &dev_attr_atrim);
    if (err)
    return err;
    err = device_create_file(dev, &dev_attr_dtrim);
    if (err)
    device_remove_file(dev, &dev_attr_atrim);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn x1205_sysfs_unregister(dev: *mut device) {
    static void x1205_sysfs_unregister(struct device *dev)
    {
    device_remove_file(dev, &dev_attr_atrim);
    device_remove_file(dev, &dev_attr_dtrim);
    }
#[no_mangle]
unsafe extern "C" fn x1205_probe(client: *mut i2c_client) -> c_int {
    static int x1205_probe(struct i2c_client *client)
    {
    let mut err: c_int = 0;
    unsigned char sr;
    struct rtc_device *rtc;
    dev_dbg(&client.dev, "%s\n", __func__);
    if (!i2c_check_functionality(client.adapter, I2C_FUNC_I2C))
    return -ENODEV;
    if (x1205_validate_client(client) < 0)
    return -ENODEV;
    rtc = devm_rtc_device_register(&client.dev, x1205_driver.driver.name,
    &x1205_rtc_ops, THIS_MODULE);
    if (IS_ERR(rtc))
    return PTR_ERR(rtc);
    i2c_set_clientdata(client, rtc);
// Check for power failures and eventually enable the osc
    err = x1205_get_status(client, &sr);
    if (!err) {
    if (sr & X1205_SR_RTCF) {
    dev_err(&client.dev,
    "power failure detected, "
    "please set the clock\n");
    udelay(50);
    x1205_fix_osc(client);
    }
    } else {
    dev_err(&client.dev, "couldn't read status\n");
    }
    err = x1205_sysfs_register(&client.dev);
    if (err)
    dev_err(&client.dev, "Unable to create sysfs entries\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn x1205_remove(client: *mut i2c_client) {
    static void x1205_remove(struct i2c_client *client)
    {
    x1205_sysfs_unregister(&client.dev);
    }
    static const struct i2c_device_id x1205_id[] = {
    { .name = "x1205" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, x1205_id);
    static const struct of_device_id x1205_dt_ids[] = {
    { .compatible = "xicor,x1205", },
    {},
    };
    MODULE_DEVICE_TABLE(of, x1205_dt_ids);
    static struct i2c_driver x1205_driver = {
    .driver		= {
    .name	= "rtc-x1205",
    .of_match_table = x1205_dt_ids,
    },
    .probe		= x1205_probe,
    .remove		= x1205_remove,
    .id_table	= x1205_id,
    };
    module_i2c_driver(x1205_driver);
    MODULE_AUTHOR(
    "Karen Spearel <kas111 at gmail dot com>, "
    "Alessandro Zummo <a.zummo@towertech.it>");
    MODULE_DESCRIPTION("Xicor/Intersil X1205 RTC driver");
    MODULE_LICENSE("GPL");
