//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-ds1302.c
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
// Dallas DS1302 RTC Support
//
// Copyright (C) 2002 David McCullough
// Copyright (C) 2003 - 2007 Paul Mundt
//

pub const RTC_CMD_READ: c_uint = 0x81		/* Read command */;
pub const RTC_CMD_WRITE: c_uint = 0x80		/* Write command */;
pub const RTC_CMD_WRITE_ENABLE: c_uint = 0x00		/* Write enable */;
pub const RTC_CMD_WRITE_DISABLE: c_uint = 0x80		/* Write disable */;
pub const RTC_ADDR_RAM0: c_uint = 0x20		/* Address of RAM0 */;
pub const RTC_ADDR_TCR: c_uint = 0x08		/* Address of trickle charge register */;
pub const RTC_CLCK_BURST: c_uint = 0x1F		/* Address of clock burst */;
pub const RTC_CLCK_LEN: c_uint = 0x08		/* Size of clock burst */;
pub const RTC_ADDR_CTRL: c_uint = 0x07		/* Address of control register */;
pub const RTC_ADDR_YEAR: c_uint = 0x06		/* Address of year register */;
pub const RTC_ADDR_DAY: c_uint = 0x05		/* Address of day of week register */;
pub const RTC_ADDR_MON: c_uint = 0x04		/* Address of month register */;
pub const RTC_ADDR_DATE: c_uint = 0x03		/* Address of day of month register */;
pub const RTC_ADDR_HOUR: c_uint = 0x02		/* Address of hour register */;
pub const RTC_ADDR_MIN: c_uint = 0x01		/* Address of minute register */;
pub const RTC_ADDR_SEC: c_uint = 0x00		/* Address of second register */;
#[no_mangle]
unsafe extern "C" fn ds1302_rtc_set_time(dev: *mut device, time: *mut rtc_time) -> c_int {
    static int ds1302_rtc_set_time(struct device *dev, struct rtc_time *time)
    {
    struct spi_device	*spi = dev_get_drvdata(dev);
    u8		buf[1 + RTC_CLCK_LEN];
    u8		*bp;
    int		status;
// Enable writing
    bp = buf;
// bp++ = RTC_ADDR_CTRL << 1 | RTC_CMD_WRITE;
// bp++ = RTC_CMD_WRITE_ENABLE;
    status = spi_write_then_read(spi, buf, 2,
    core::ptr::null_mut(), 0);
    if (status)
    return status;
// Write registers starting at the first time/date address.
    bp = buf;
// bp++ = RTC_CLCK_BURST << 1 | RTC_CMD_WRITE;
// bp++ = bin2bcd(time->tm_sec);
// bp++ = bin2bcd(time->tm_min);
// bp++ = bin2bcd(time->tm_hour);
// bp++ = bin2bcd(time->tm_mday);
// bp++ = bin2bcd(time->tm_mon + 1);
// bp++ = time->tm_wday + 1;
// bp++ = bin2bcd(time->tm_year % 100);
// bp++ = RTC_CMD_WRITE_DISABLE;
// use write-then-read since dma from stack is nonportable
    return spi_write_then_read(spi, buf, sizeof(buf),
    core::ptr::null_mut(), 0);
    }
#[no_mangle]
unsafe extern "C" fn ds1302_rtc_get_time(dev: *mut device, time: *mut rtc_time) -> c_int {
    static int ds1302_rtc_get_time(struct device *dev, struct rtc_time *time)
    {
    struct spi_device	*spi = dev_get_drvdata(dev);
    let mut addr: u8 = RTC_CLCK_BURST << 1 | RTC_CMD_READ;
    u8		buf[RTC_CLCK_LEN - 1];
    int		status;
// Use write-then-read to get all the date/time registers
// since dma from stack is nonportable
//
    status = spi_write_then_read(spi, &addr, sizeof(addr),
    buf, sizeof(buf));
    if (status < 0)
    return status;
// Decode the registers
    time.tm_sec = bcd2bin(buf[RTC_ADDR_SEC]);
    time.tm_min = bcd2bin(buf[RTC_ADDR_MIN]);
    time.tm_hour = bcd2bin(buf[RTC_ADDR_HOUR]);
    time.tm_wday = buf[RTC_ADDR_DAY] - 1;
    time.tm_mday = bcd2bin(buf[RTC_ADDR_DATE]);
    time.tm_mon = bcd2bin(buf[RTC_ADDR_MON]) - 1;
    time.tm_year = bcd2bin(buf[RTC_ADDR_YEAR]) + 100;
    return 0;
    }
    static const struct rtc_class_ops ds1302_rtc_ops = {
    .read_time	= ds1302_rtc_get_time,
    .set_time	= ds1302_rtc_set_time,
    };
#[no_mangle]
unsafe extern "C" fn ds1302_probe(spi: *mut spi_device) -> c_int {
    static int ds1302_probe(struct spi_device *spi)
    {
    struct rtc_device	*rtc;
    u8		addr;
    u8		buf[4];
    u8		*bp;
    int		status;
// Sanity check board setup data.  This may be hooked up
// in 3wire mode, but we don't care.  Note that unless
// there's an inverter in place, this needs SPI_CS_HIGH!
//
    if (spi.bits_per_word && (spi.bits_per_word != 8)) {
    dev_err(&spi.dev, "bad word length\n");
    return -EINVAL;
    } else if (spi.max_speed_hz > 2000000) {
    dev_err(&spi.dev, "speed is too high\n");
    return -EINVAL;
    } else if (spi.mode & SPI_CPHA) {
    dev_err(&spi.dev, "bad mode\n");
    return -EINVAL;
    }
    addr = RTC_ADDR_CTRL << 1 | RTC_CMD_READ;
    status = spi_write_then_read(spi, &addr, sizeof(addr), buf, 1);
    if (status < 0) {
    dev_err(&spi.dev, "control register read error %d\n",
    status);
    return status;
    }
    if ((buf[0] & ~RTC_CMD_WRITE_DISABLE) != 0) {
    status = spi_write_then_read(spi, &addr, sizeof(addr), buf, 1);
    if (status < 0) {
    dev_err(&spi.dev, "control register read error %d\n",
    status);
    return status;
    }
    if ((buf[0] & ~RTC_CMD_WRITE_DISABLE) != 0) {
    dev_err(&spi.dev, "junk in control register\n");
    return -ENODEV;
    }
    }
    if (buf[0] == 0) {
    bp = buf;
// bp++ = RTC_ADDR_CTRL << 1 | RTC_CMD_WRITE;
// bp++ = RTC_CMD_WRITE_DISABLE;
    status = spi_write_then_read(spi, buf, 2, core::ptr::null_mut(), 0);
    if (status < 0) {
    dev_err(&spi.dev, "control register write error %d\n",
    status);
    return status;
    }
    addr = RTC_ADDR_CTRL << 1 | RTC_CMD_READ;
    status = spi_write_then_read(spi, &addr, sizeof(addr), buf, 1);
    if (status < 0) {
    dev_err(&spi.dev,
    "error %d reading control register\n",
    status);
    return status;
    }
    if (buf[0] != RTC_CMD_WRITE_DISABLE) {
    dev_err(&spi.dev, "failed to detect chip\n");
    return -ENODEV;
    }
    }
    spi_set_drvdata(spi, spi);
    rtc = devm_rtc_device_register(&spi.dev, "ds1302",
    &ds1302_rtc_ops, THIS_MODULE);
    if (IS_ERR(rtc)) {
    status = PTR_ERR(rtc);
    dev_err(&spi.dev, "error %d registering rtc\n", status);
    return status;
    }
    return 0;
    }

    static const struct of_device_id ds1302_dt_ids[] = {
    { .compatible = "maxim,ds1302", },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, ds1302_dt_ids);

    static const struct spi_device_id ds1302_spi_ids[] = {
    { .name = "ds1302", },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(spi, ds1302_spi_ids);
    static struct spi_driver ds1302_driver = {
    .driver.name	= "rtc-ds1302",
    .driver.of_match_table = of_match_ptr(ds1302_dt_ids),
    .probe		= ds1302_probe,
    .id_table	= ds1302_spi_ids,
    };
    module_spi_driver(ds1302_driver);
    MODULE_DESCRIPTION("Dallas DS1302 RTC driver");
    MODULE_AUTHOR("Paul Mundt, David McCullough");
    MODULE_LICENSE("GPL v2");
