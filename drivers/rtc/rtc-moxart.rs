//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-moxart.c
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


// SPDX-License-Identifier: GPL-2.0
//
// MOXA ART RTC driver.
//
// Copyright (C) 2013 Jonas Jensen
//
// Jonas Jensen <jonas.jensen@gmail.com>
//
// Based on code from
// Moxa Technology Co., Ltd. <www.moxa.com>
//

pub const GPIO_RTC_RESERVED: c_uint = 0x0C;
pub const GPIO_RTC_DATA_SET: c_uint = 0x10;
pub const GPIO_RTC_DATA_CLEAR: c_uint = 0x14;
pub const GPIO_RTC_PIN_PULL_ENABLE: c_uint = 0x18;
pub const GPIO_RTC_PIN_PULL_TYPE: c_uint = 0x1C;
pub const GPIO_RTC_INT_ENABLE: c_uint = 0x20;
pub const GPIO_RTC_INT_RAW_STATE: c_uint = 0x24;
pub const GPIO_RTC_INT_MASKED_STATE: c_uint = 0x28;
pub const GPIO_RTC_INT_MASK: c_uint = 0x2C;
pub const GPIO_RTC_INT_CLEAR: c_uint = 0x30;
pub const GPIO_RTC_INT_TRIGGER: c_uint = 0x34;
pub const GPIO_RTC_INT_BOTH: c_uint = 0x38;
pub const GPIO_RTC_INT_RISE_NEG: c_uint = 0x3C;
pub const GPIO_RTC_BOUNCE_ENABLE: c_uint = 0x40;
pub const GPIO_RTC_BOUNCE_PRE_SCALE: c_uint = 0x44;
pub const GPIO_RTC_PROTECT_W: c_uint = 0x8E;
pub const GPIO_RTC_PROTECT_R: c_uint = 0x8F;
pub const GPIO_RTC_YEAR_W: c_uint = 0x8C;
pub const GPIO_RTC_YEAR_R: c_uint = 0x8D;
pub const GPIO_RTC_DAY_W: c_uint = 0x8A;
pub const GPIO_RTC_DAY_R: c_uint = 0x8B;
pub const GPIO_RTC_MONTH_W: c_uint = 0x88;
pub const GPIO_RTC_MONTH_R: c_uint = 0x89;
pub const GPIO_RTC_DATE_W: c_uint = 0x86;
pub const GPIO_RTC_DATE_R: c_uint = 0x87;
pub const GPIO_RTC_HOURS_W: c_uint = 0x84;
pub const GPIO_RTC_HOURS_R: c_uint = 0x85;
pub const GPIO_RTC_MINUTES_W: c_uint = 0x82;
pub const GPIO_RTC_MINUTES_R: c_uint = 0x83;
pub const GPIO_RTC_SECONDS_W: c_uint = 0x80;
pub const GPIO_RTC_SECONDS_R: c_uint = 0x81;
pub const GPIO_RTC_DELAY_TIME: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct moxart_rtc {
    pub rtc: *mut rtc_device,
    pub rtc_lock: spinlock_t,
    pub gpio_data: *mut gpio_desc,
    pub gpio_sclk: *mut gpio_desc,
    pub gpio_reset: *mut gpio_desc,
}

    static int day_of_year[12] =	{ 0, 31, 59, 90, 120, 151, 181,
    212, 243, 273, 304, 334 };
#[no_mangle]
unsafe extern "C" fn moxart_rtc_write_byte(dev: *mut device, data: u8) {
    static void moxart_rtc_write_byte(struct device *dev, u8 data)
    {
    struct moxart_rtc *moxart_rtc = dev_get_drvdata(dev);
    int i;
    for (i = 0; i < 8; i++, data >>= 1) {
    gpiod_set_value(moxart_rtc.gpio_sclk, 0);
    gpiod_set_value(moxart_rtc.gpio_data, ((data & 1) == 1));
    udelay(GPIO_RTC_DELAY_TIME);
    gpiod_set_value(moxart_rtc.gpio_sclk, 1);
    udelay(GPIO_RTC_DELAY_TIME);
    }
    }
#[no_mangle]
unsafe extern "C" fn moxart_rtc_read_byte(dev: *mut device) -> u8 {
    static u8 moxart_rtc_read_byte(struct device *dev)
    {
    struct moxart_rtc *moxart_rtc = dev_get_drvdata(dev);
    int i;
    let mut data: u8 = 0;
    for (i = 0; i < 8; i++) {
    gpiod_set_value(moxart_rtc.gpio_sclk, 0);
    udelay(GPIO_RTC_DELAY_TIME);
    gpiod_set_value(moxart_rtc.gpio_sclk, 1);
    udelay(GPIO_RTC_DELAY_TIME);
    if (gpiod_get_value(moxart_rtc.gpio_data))
    data |= (1 << i);
    udelay(GPIO_RTC_DELAY_TIME);
    }
    return data;
    }
#[no_mangle]
unsafe extern "C" fn moxart_rtc_read_register(dev: *mut device, cmd: u8) -> u8 {
    static u8 moxart_rtc_read_register(struct device *dev, u8 cmd)
    {
    struct moxart_rtc *moxart_rtc = dev_get_drvdata(dev);
    u8 data;
    unsigned long flags;
    local_irq_save(flags);
    gpiod_direction_output(moxart_rtc.gpio_data, 0);
    gpiod_set_value(moxart_rtc.gpio_reset, 1);
    udelay(GPIO_RTC_DELAY_TIME);
    moxart_rtc_write_byte(dev, cmd);
    gpiod_direction_input(moxart_rtc.gpio_data);
    udelay(GPIO_RTC_DELAY_TIME);
    data = moxart_rtc_read_byte(dev);
    gpiod_set_value(moxart_rtc.gpio_sclk, 0);
    gpiod_set_value(moxart_rtc.gpio_reset, 0);
    udelay(GPIO_RTC_DELAY_TIME);
    local_irq_restore(flags);
    return data;
    }
#[no_mangle]
unsafe extern "C" fn moxart_rtc_write_register(dev: *mut device, cmd: u8, data: u8) {
    static void moxart_rtc_write_register(struct device *dev, u8 cmd, u8 data)
    {
    struct moxart_rtc *moxart_rtc = dev_get_drvdata(dev);
    unsigned long flags;
    local_irq_save(flags);
    gpiod_direction_output(moxart_rtc.gpio_data, 0);
    gpiod_set_value(moxart_rtc.gpio_reset, 1);
    udelay(GPIO_RTC_DELAY_TIME);
    moxart_rtc_write_byte(dev, cmd);
    moxart_rtc_write_byte(dev, data);
    gpiod_set_value(moxart_rtc.gpio_sclk, 0);
    gpiod_set_value(moxart_rtc.gpio_reset, 0);
    udelay(GPIO_RTC_DELAY_TIME);
    local_irq_restore(flags);
    }
#[no_mangle]
unsafe extern "C" fn moxart_rtc_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int moxart_rtc_set_time(struct device *dev, struct rtc_time *tm)
    {
    struct moxart_rtc *moxart_rtc = dev_get_drvdata(dev);
    spin_lock_irq(&moxart_rtc.rtc_lock);
    moxart_rtc_write_register(dev, GPIO_RTC_PROTECT_W, 0);
    moxart_rtc_write_register(dev, GPIO_RTC_YEAR_W,
    (((tm.tm_year - 100) / 10) << 4) |
    ((tm.tm_year - 100) % 10));
    moxart_rtc_write_register(dev, GPIO_RTC_MONTH_W,
    (((tm.tm_mon + 1) / 10) << 4) |
    ((tm.tm_mon + 1) % 10));
    moxart_rtc_write_register(dev, GPIO_RTC_DATE_W,
    ((tm.tm_mday / 10) << 4) |
    (tm.tm_mday % 10));
    moxart_rtc_write_register(dev, GPIO_RTC_HOURS_W,
    ((tm.tm_hour / 10) << 4) |
    (tm.tm_hour % 10));
    moxart_rtc_write_register(dev, GPIO_RTC_MINUTES_W,
    ((tm.tm_min / 10) << 4) |
    (tm.tm_min % 10));
    moxart_rtc_write_register(dev, GPIO_RTC_SECONDS_W,
    ((tm.tm_sec / 10) << 4) |
    (tm.tm_sec % 10));
    moxart_rtc_write_register(dev, GPIO_RTC_PROTECT_W, 0x80);
    spin_unlock_irq(&moxart_rtc.rtc_lock);
    dev_dbg(dev, "%s: success tm_year=%d tm_mon=%d\n"
    "tm_mday=%d tm_hour=%d tm_min=%d tm_sec=%d\n",
    __func__, tm.tm_year, tm.tm_mon, tm.tm_mday,
    tm.tm_hour, tm.tm_min, tm.tm_sec);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn moxart_rtc_read_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int moxart_rtc_read_time(struct device *dev, struct rtc_time *tm)
    {
    struct moxart_rtc *moxart_rtc = dev_get_drvdata(dev);
    unsigned char v;
    spin_lock_irq(&moxart_rtc.rtc_lock);
    v = moxart_rtc_read_register(dev, GPIO_RTC_SECONDS_R);
    tm.tm_sec = (((v & 0x70) >> 4) * 10) + (v & 0x0F);
    v = moxart_rtc_read_register(dev, GPIO_RTC_MINUTES_R);
    tm.tm_min = (((v & 0x70) >> 4) * 10) + (v & 0x0F);
    v = moxart_rtc_read_register(dev, GPIO_RTC_HOURS_R);
    if (v & 0x80) { /* 12-hour mode */
    tm.tm_hour = (((v & 0x10) >> 4) * 10) + (v & 0x0F);
    if (v & 0x20) { /* PM mode */
    tm.tm_hour += 12;
    if (tm.tm_hour >= 24)
    tm.tm_hour = 0;
    }
    } else { /* 24-hour mode */
    tm.tm_hour = (((v & 0x30) >> 4) * 10) + (v & 0x0F);
    }
    v = moxart_rtc_read_register(dev, GPIO_RTC_DATE_R);
    tm.tm_mday = (((v & 0x30) >> 4) * 10) + (v & 0x0F);
    v = moxart_rtc_read_register(dev, GPIO_RTC_MONTH_R);
    tm.tm_mon = (((v & 0x10) >> 4) * 10) + (v & 0x0F);
    tm.tm_mon--;
    v = moxart_rtc_read_register(dev, GPIO_RTC_YEAR_R);
    tm.tm_year = (((v & 0xF0) >> 4) * 10) + (v & 0x0F);
    tm.tm_year += 100;
    if (tm.tm_year <= 69)
    tm.tm_year += 100;
    v = moxart_rtc_read_register(dev, GPIO_RTC_DAY_R);
    tm.tm_wday = (v & 0x0f) - 1;
    tm.tm_yday = day_of_year[tm.tm_mon];
    tm.tm_yday += (tm.tm_mday - 1);
    if (tm.tm_mon >= 2) {
    if (!(tm.tm_year % 4) && (tm.tm_year % 100))
    tm.tm_yday++;
    }
    tm.tm_isdst = 0;
    spin_unlock_irq(&moxart_rtc.rtc_lock);
    return 0;
    }
    static const struct rtc_class_ops moxart_rtc_ops = {
    .read_time	= moxart_rtc_read_time,
    .set_time	= moxart_rtc_set_time,
    };
#[no_mangle]
unsafe extern "C" fn moxart_rtc_probe(pdev: *mut platform_device) -> c_int {
    static int moxart_rtc_probe(struct platform_device *pdev)
    {
    struct moxart_rtc *moxart_rtc;
    let mut ret: c_int = 0;
    moxart_rtc = devm_kzalloc(&pdev.dev, sizeof(*moxart_rtc), GFP_KERNEL);
    if (!moxart_rtc)
    return -ENOMEM;
    moxart_rtc.gpio_data = devm_gpiod_get(&pdev.dev, "rtc-data",
    GPIOD_IN);
    ret = PTR_ERR_OR_ZERO(moxart_rtc.gpio_data);
    if (ret) {
    dev_err(&pdev.dev, "can't get rtc data gpio: %d\n", ret);
    return ret;
    }
    moxart_rtc.gpio_sclk = devm_gpiod_get(&pdev.dev, "rtc-sclk",
    GPIOD_ASIS);
    ret = PTR_ERR_OR_ZERO(moxart_rtc.gpio_sclk);
    if (ret) {
    dev_err(&pdev.dev, "can't get rtc sclk gpio: %d\n", ret);
    return ret;
    }
    moxart_rtc.gpio_reset = devm_gpiod_get(&pdev.dev, "rtc-reset",
    GPIOD_ASIS);
    ret = PTR_ERR_OR_ZERO(moxart_rtc.gpio_reset);
    if (ret) {
    dev_err(&pdev.dev, "can't get rtc reset gpio: %d\n", ret);
    return ret;
    }
    spin_lock_init(&moxart_rtc.rtc_lock);
    platform_set_drvdata(pdev, moxart_rtc);
    moxart_rtc.rtc = devm_rtc_device_register(&pdev.dev, pdev.name,
    &moxart_rtc_ops,
    THIS_MODULE);
    if (IS_ERR(moxart_rtc.rtc)) {
    dev_err(&pdev.dev, "devm_rtc_device_register failed\n");
    return PTR_ERR(moxart_rtc.rtc);
    }
    return 0;
    }
    static const struct of_device_id moxart_rtc_match[] = {
    { .compatible = "moxa,moxart-rtc" },
    { },
    };
    MODULE_DEVICE_TABLE(of, moxart_rtc_match);
    static struct platform_driver moxart_rtc_driver = {
    .probe	= moxart_rtc_probe,
    .driver	= {
    .name		= "moxart-rtc",
    .of_match_table	= moxart_rtc_match,
    },
    };
    module_platform_driver(moxart_rtc_driver);
    MODULE_DESCRIPTION("MOXART RTC driver");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Jonas Jensen <jonas.jensen@gmail.com>");
