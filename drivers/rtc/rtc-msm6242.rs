//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-msm6242.c
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
// Oki MSM6242 RTC Driver
//
// Copyright 2009 Geert Uytterhoeven
//
// Based on the A2000 TOD code in arch/m68k/amiga/config.c
// Copyright (C) 1993 Hamish Macdonald
//

    enum {
    MSM6242_SECOND1		= 0x0,	/* 1-second digit register */
    MSM6242_SECOND10	= 0x1,	/* 10-second digit register */
    MSM6242_MINUTE1		= 0x2,	/* 1-minute digit register */
    MSM6242_MINUTE10	= 0x3,	/* 10-minute digit register */
    MSM6242_HOUR1		= 0x4,	/* 1-hour digit register */
    MSM6242_HOUR10		= 0x5,	/* PM/AM, 10-hour digit register */
    MSM6242_DAY1		= 0x6,	/* 1-day digit register */
    MSM6242_DAY10		= 0x7,	/* 10-day digit register */
    MSM6242_MONTH1		= 0x8,	/* 1-month digit register */
    MSM6242_MONTH10		= 0x9,	/* 10-month digit register */
    MSM6242_YEAR1		= 0xa,	/* 1-year digit register */
    MSM6242_YEAR10		= 0xb,	/* 10-year digit register */
    MSM6242_WEEK		= 0xc,	/* Week register */
    MSM6242_CD		= 0xd,	/* Control Register D */
    MSM6242_CE		= 0xe,	/* Control Register E */
    MSM6242_CF		= 0xf,	/* Control Register F */
    };

pub const MSM6242_WEEK_SUNDAY: c_int = 0;
pub const MSM6242_WEEK_MONDAY: c_int = 1;
pub const MSM6242_WEEK_TUESDAY: c_int = 2;
pub const MSM6242_WEEK_WEDNESDAY: c_int = 3;
pub const MSM6242_WEEK_THURSDAY: c_int = 4;
pub const MSM6242_WEEK_FRIDAY: c_int = 5;
pub const MSM6242_WEEK_SATURDAY: c_int = 6;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm6242_priv {
    pub regs: *mut u32 __iomem,
    pub rtc: *mut rtc_device,
}

    static inline unsigned int msm6242_read(struct msm6242_priv *priv,
    unsigned int reg)
    {
    return __raw_readl(&priv.regs[reg]) & 0xf;
    }
    static inline void msm6242_write(struct msm6242_priv *priv, unsigned int val,
    unsigned int reg)
    {
    __raw_writel(val, &priv.regs[reg]);
    }
#[no_mangle]
unsafe extern "C" fn msm6242_lock(priv: *mut msm6242_priv) {
    static void msm6242_lock(struct msm6242_priv *priv)
    {
    let mut cnt: c_int = 5;
    msm6242_write(priv, MSM6242_CD_HOLD|MSM6242_CD_IRQ_FLAG, MSM6242_CD);
    while ((msm6242_read(priv, MSM6242_CD) & MSM6242_CD_BUSY) && cnt) {
    msm6242_write(priv, MSM6242_CD_IRQ_FLAG, MSM6242_CD);
    udelay(70);
    msm6242_write(priv, MSM6242_CD_HOLD|MSM6242_CD_IRQ_FLAG, MSM6242_CD);
    cnt--;
    }
    if (!cnt)
    pr_warn("timed out waiting for RTC (0x%x)\n",
    msm6242_read(priv, MSM6242_CD));
    }
#[no_mangle]
unsafe extern "C" fn msm6242_unlock(priv: *mut msm6242_priv) {
    static void msm6242_unlock(struct msm6242_priv *priv)
    {
    msm6242_write(priv, MSM6242_CD_IRQ_FLAG, MSM6242_CD);
    }
#[no_mangle]
unsafe extern "C" fn msm6242_read_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int msm6242_read_time(struct device *dev, struct rtc_time *tm)
    {
    struct msm6242_priv *priv = dev_get_drvdata(dev);
    msm6242_lock(priv);
    tm.tm_sec  = msm6242_read(priv, MSM6242_SECOND10) * 10 +
    msm6242_read(priv, MSM6242_SECOND1);
    tm.tm_min  = msm6242_read(priv, MSM6242_MINUTE10) * 10 +
    msm6242_read(priv, MSM6242_MINUTE1);
    tm.tm_hour = (msm6242_read(priv, MSM6242_HOUR10) &
    MSM6242_HOUR10_HR_MASK) * 10 +
    msm6242_read(priv, MSM6242_HOUR1);
    tm.tm_mday = msm6242_read(priv, MSM6242_DAY10) * 10 +
    msm6242_read(priv, MSM6242_DAY1);
    tm.tm_wday = msm6242_read(priv, MSM6242_WEEK);
    tm.tm_mon  = msm6242_read(priv, MSM6242_MONTH10) * 10 +
    msm6242_read(priv, MSM6242_MONTH1) - 1;
    tm.tm_year = msm6242_read(priv, MSM6242_YEAR10) * 10 +
    msm6242_read(priv, MSM6242_YEAR1);
    if (tm.tm_year <= 69)
    tm.tm_year += 100;
    if (!(msm6242_read(priv, MSM6242_CF) & MSM6242_CF_24H)) {
    unsigned int pm = msm6242_read(priv, MSM6242_HOUR10) &
    MSM6242_HOUR10_PM;
    if (!pm && tm.tm_hour == 12)
    tm.tm_hour = 0;
#[no_mangle]
pub unsafe extern "C" fn if(12: pm && tm->tm_hour !=) -> else {
    else if (pm && tm.tm_hour != 12)
    tm.tm_hour += 12;
    }
    msm6242_unlock(priv);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn msm6242_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int msm6242_set_time(struct device *dev, struct rtc_time *tm)
    {
    struct msm6242_priv *priv = dev_get_drvdata(dev);
    msm6242_lock(priv);
    msm6242_write(priv, tm.tm_sec / 10, MSM6242_SECOND10);
    msm6242_write(priv, tm.tm_sec % 10, MSM6242_SECOND1);
    msm6242_write(priv, tm.tm_min / 10, MSM6242_MINUTE10);
    msm6242_write(priv, tm.tm_min % 10, MSM6242_MINUTE1);
    if (msm6242_read(priv, MSM6242_CF) & MSM6242_CF_24H)
    msm6242_write(priv, tm.tm_hour / 10, MSM6242_HOUR10);
#[no_mangle]
pub unsafe extern "C" fn if(12: tm->tm_hour >=) -> else {
    else if (tm.tm_hour >= 12)
    msm6242_write(priv, MSM6242_HOUR10_PM + (tm.tm_hour - 12) / 10,
    MSM6242_HOUR10);
    else
    msm6242_write(priv, tm.tm_hour / 10, MSM6242_HOUR10);
    msm6242_write(priv, tm.tm_hour % 10, MSM6242_HOUR1);
    msm6242_write(priv, tm.tm_mday / 10, MSM6242_DAY10);
    msm6242_write(priv, tm.tm_mday % 10, MSM6242_DAY1);
    if (tm.tm_wday != -1)
    msm6242_write(priv, tm.tm_wday, MSM6242_WEEK);
    msm6242_write(priv, (tm.tm_mon + 1) / 10, MSM6242_MONTH10);
    msm6242_write(priv, (tm.tm_mon + 1) % 10, MSM6242_MONTH1);
    if (tm.tm_year >= 100)
    tm.tm_year -= 100;
    msm6242_write(priv, tm.tm_year / 10, MSM6242_YEAR10);
    msm6242_write(priv, tm.tm_year % 10, MSM6242_YEAR1);
    msm6242_unlock(priv);
    return 0;
    }
    static const struct rtc_class_ops msm6242_rtc_ops = {
    .read_time	= msm6242_read_time,
    .set_time	= msm6242_set_time,
    };
#[no_mangle]
unsafe extern "C" fn msm6242_rtc_probe(pdev: *mut platform_device) -> int __init {
    static int __init msm6242_rtc_probe(struct platform_device *pdev)
    {
    struct resource *res;
    struct msm6242_priv *priv;
    struct rtc_device *rtc;
    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!res)
    return -ENODEV;
    priv = devm_kzalloc(&pdev.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.regs = devm_ioremap(&pdev.dev, res.start, resource_size(res));
    if (!priv.regs)
    return -ENOMEM;
    platform_set_drvdata(pdev, priv);
    rtc = devm_rtc_device_register(&pdev.dev, "rtc-msm6242",
    &msm6242_rtc_ops, THIS_MODULE);
    if (IS_ERR(rtc))
    return PTR_ERR(rtc);
    priv.rtc = rtc;
    return 0;
    }
    static struct platform_driver msm6242_rtc_driver = {
    .driver	= {
    .name	= "rtc-msm6242",
    },
    };
    module_platform_driver_probe(msm6242_rtc_driver, msm6242_rtc_probe);
    MODULE_AUTHOR("Geert Uytterhoeven <geert@linux-m68k.org>");
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Oki MSM6242 RTC driver");
    MODULE_ALIAS("platform:rtc-msm6242");
