//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-rs5c348.c
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
// A SPI driver for the Ricoh RS5C348 RTC
//
// Copyright (C) 2006 Atsushi Nemoto <anemo@mba.ocn.ne.jp>
//
// The board specific init code should provide characteristics of this
// device:
// Mode 1 (High-Active, Shift-Then-Sample), High Avtive CS
//

pub const RS5C348_REG_SECS: c_int = 0;
pub const RS5C348_REG_MINS: c_int = 1;
pub const RS5C348_REG_HOURS: c_int = 2;
pub const RS5C348_REG_WDAY: c_int = 3;
pub const RS5C348_REG_DAY: c_int = 4;
pub const RS5C348_REG_MONTH: c_int = 5;
pub const RS5C348_REG_YEAR: c_int = 6;
pub const RS5C348_REG_CTL1: c_int = 14;
pub const RS5C348_REG_CTL2: c_int = 15;
pub const RS5C348_SECS_MASK: c_uint = 0x7f;
pub const RS5C348_MINS_MASK: c_uint = 0x7f;
pub const RS5C348_HOURS_MASK: c_uint = 0x3f;
pub const RS5C348_WDAY_MASK: c_uint = 0x03;
pub const RS5C348_DAY_MASK: c_uint = 0x3f;
pub const RS5C348_MONTH_MASK: c_uint = 0x1f;
pub const RS5C348_BIT_PM: c_uint = 0x20	/* REG_HOURS */;
pub const RS5C348_BIT_Y2K: c_uint = 0x80	/* REG_MONTH */;
pub const RS5C348_BIT_24H: c_uint = 0x20	/* REG_CTL1 */;
pub const RS5C348_BIT_XSTP: c_uint = 0x10	/* REG_CTL2 */;
pub const RS5C348_BIT_VDET: c_uint = 0x40	/* REG_CTL2 */;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rs5c348_plat_data {
    pub rtc: *mut rtc_device,
    pub rtc_24h: c_int,
}

    static int
    rs5c348_rtc_set_time(struct device *dev, struct rtc_time *tm)
    {
    struct spi_device *spi = to_spi_device(dev);
    struct rs5c348_plat_data *pdata = dev_get_platdata(&spi.dev);
    u8 txbuf[5+7], *txp;
    int ret;
    ret = spi_w8r8(spi, RS5C348_CMD_R(RS5C348_REG_CTL2));
    if (ret < 0)
    return ret;
    if (ret & RS5C348_BIT_XSTP) {
    txbuf[0] = RS5C348_CMD_W(RS5C348_REG_CTL2);
    txbuf[1] = 0;
    ret = spi_write_then_read(spi, txbuf, 2, core::ptr::null_mut(), 0);
    if (ret < 0)
    return ret;
    }
// Transfer 5 bytes before writing SEC.  This gives 31us for carry.
    txp = txbuf;
    txbuf[0] = RS5C348_CMD_R(RS5C348_REG_CTL2); /* cmd, ctl2 */
    txbuf[1] = 0;	/* dummy */
    txbuf[2] = RS5C348_CMD_R(RS5C348_REG_CTL2); /* cmd, ctl2 */
    txbuf[3] = 0;	/* dummy */
    txbuf[4] = RS5C348_CMD_MW(RS5C348_REG_SECS); /* cmd, sec, ... */
    txp = &txbuf[5];
    txp[RS5C348_REG_SECS] = bin2bcd(tm.tm_sec);
    txp[RS5C348_REG_MINS] = bin2bcd(tm.tm_min);
    if (pdata.rtc_24h) {
    txp[RS5C348_REG_HOURS] = bin2bcd(tm.tm_hour);
    } else {
// hour 0 is AM12, noon is PM12
    txp[RS5C348_REG_HOURS] = bin2bcd((tm.tm_hour + 11) % 12 + 1) |
    (tm.tm_hour >= 12 ? RS5C348_BIT_PM : 0);
    }
    txp[RS5C348_REG_WDAY] = bin2bcd(tm.tm_wday);
    txp[RS5C348_REG_DAY] = bin2bcd(tm.tm_mday);
    txp[RS5C348_REG_MONTH] = bin2bcd(tm.tm_mon + 1) |
    (tm.tm_year >= 100 ? RS5C348_BIT_Y2K : 0);
    txp[RS5C348_REG_YEAR] = bin2bcd(tm.tm_year % 100);
// write in one transfer to avoid data inconsistency
    ret = spi_write_then_read(spi, txbuf, sizeof(txbuf), core::ptr::null_mut(), 0);
    udelay(62);	/* Tcsr 62us */
    return ret;
    }
    static int
    rs5c348_rtc_read_time(struct device *dev, struct rtc_time *tm)
    {
    struct spi_device *spi = to_spi_device(dev);
    struct rs5c348_plat_data *pdata = dev_get_platdata(&spi.dev);
    u8 txbuf[5], rxbuf[7];
    int ret;
    ret = spi_w8r8(spi, RS5C348_CMD_R(RS5C348_REG_CTL2));
    if (ret < 0)
    return ret;
    if (ret & RS5C348_BIT_VDET)
    dev_warn(&spi.dev, "voltage-low detected.\n");
    if (ret & RS5C348_BIT_XSTP) {
    dev_warn(&spi.dev, "oscillator-stop detected.\n");
    return -EINVAL;
    }
// Transfer 5 byte befores reading SEC.  This gives 31us for carry.
    txbuf[0] = RS5C348_CMD_R(RS5C348_REG_CTL2); /* cmd, ctl2 */
    txbuf[1] = 0;	/* dummy */
    txbuf[2] = RS5C348_CMD_R(RS5C348_REG_CTL2); /* cmd, ctl2 */
    txbuf[3] = 0;	/* dummy */
    txbuf[4] = RS5C348_CMD_MR(RS5C348_REG_SECS); /* cmd, sec, ... */
// read in one transfer to avoid data inconsistency
    ret = spi_write_then_read(spi, txbuf, sizeof(txbuf),
    rxbuf, sizeof(rxbuf));
    udelay(62);	/* Tcsr 62us */
    if (ret < 0)
    return ret;
    tm.tm_sec = bcd2bin(rxbuf[RS5C348_REG_SECS] & RS5C348_SECS_MASK);
    tm.tm_min = bcd2bin(rxbuf[RS5C348_REG_MINS] & RS5C348_MINS_MASK);
    tm.tm_hour = bcd2bin(rxbuf[RS5C348_REG_HOURS] & RS5C348_HOURS_MASK);
    if (!pdata.rtc_24h) {
    if (rxbuf[RS5C348_REG_HOURS] & RS5C348_BIT_PM) {
    tm.tm_hour -= 20;
    tm.tm_hour %= 12;
    tm.tm_hour += 12;
    } else
    tm.tm_hour %= 12;
    }
    tm.tm_wday = bcd2bin(rxbuf[RS5C348_REG_WDAY] & RS5C348_WDAY_MASK);
    tm.tm_mday = bcd2bin(rxbuf[RS5C348_REG_DAY] & RS5C348_DAY_MASK);
    tm.tm_mon =
    bcd2bin(rxbuf[RS5C348_REG_MONTH] & RS5C348_MONTH_MASK) - 1;
// year is 1900 + tm->tm_year
    tm.tm_year = bcd2bin(rxbuf[RS5C348_REG_YEAR]) +
    ((rxbuf[RS5C348_REG_MONTH] & RS5C348_BIT_Y2K) ? 100 : 0);
    return 0;
    }
    static const struct rtc_class_ops rs5c348_rtc_ops = {
    .read_time	= rs5c348_rtc_read_time,
    .set_time	= rs5c348_rtc_set_time,
    };
#[no_mangle]
unsafe extern "C" fn rs5c348_probe(spi: *mut spi_device) -> c_int {
    static int rs5c348_probe(struct spi_device *spi)
    {
    int ret;
    struct rtc_device *rtc;
    struct rs5c348_plat_data *pdata;
    pdata = devm_kzalloc(&spi.dev, sizeof(struct rs5c348_plat_data),
    GFP_KERNEL);
    if (!pdata)
    return -ENOMEM;
    spi.dev.platform_data = pdata;
// Check D7 of SECOND register
    ret = spi_w8r8(spi, RS5C348_CMD_R(RS5C348_REG_SECS));
    if (ret < 0 || (ret & 0x80)) {
    dev_err(&spi.dev, "not found.\n");
    return ret;
    }
    dev_info(&spi.dev, "spiclk %u KHz.\n",
    (spi.max_speed_hz + 500) / 1000);
    ret = spi_w8r8(spi, RS5C348_CMD_R(RS5C348_REG_CTL1));
    if (ret < 0)
    return ret;
    if (ret & RS5C348_BIT_24H)
    pdata.rtc_24h = 1;
    rtc = devm_rtc_allocate_device(&spi.dev);
    if (IS_ERR(rtc))
    return PTR_ERR(rtc);
    pdata.rtc = rtc;
    rtc.ops = &rs5c348_rtc_ops;
    return devm_rtc_register_device(rtc);
    }
    static struct spi_driver rs5c348_driver = {
    .driver = {
    .name	= "rtc-rs5c348",
    },
    .probe	= rs5c348_probe,
    };
    module_spi_driver(rs5c348_driver);
    MODULE_AUTHOR("Atsushi Nemoto <anemo@mba.ocn.ne.jp>");
    MODULE_DESCRIPTION("Ricoh RS5C348 RTC driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("spi:rtc-rs5c348");
