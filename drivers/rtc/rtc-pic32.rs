//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-pic32.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// PIC32 RTC driver
//
// Joshua Henderson <joshua.henderson@microchip.com>
// Copyright (C) 2016 Microchip Technology Inc.  All rights reserved.
//

pub const PIC32_RTCCON: c_uint = 0x00;

pub const PIC32_RTCALRM: c_uint = 0x10;

pub const PIC32_RTCALRM_AMASK: c_uint = 0x0F00;
pub const PIC32_RTCALRM_ARPT: c_uint = 0xFF;
pub const PIC32_RTCHOUR: c_uint = 0x23;
pub const PIC32_RTCMIN: c_uint = 0x22;
pub const PIC32_RTCSEC: c_uint = 0x21;
pub const PIC32_RTCYEAR: c_uint = 0x33;
pub const PIC32_RTCMON: c_uint = 0x32;
pub const PIC32_RTCDAY: c_uint = 0x31;
pub const PIC32_ALRMTIME: c_uint = 0x40;
pub const PIC32_ALRMDATE: c_uint = 0x50;
pub const PIC32_ALRMHOUR: c_uint = 0x43;
pub const PIC32_ALRMMIN: c_uint = 0x42;
pub const PIC32_ALRMSEC: c_uint = 0x41;
pub const PIC32_ALRMYEAR: c_uint = 0x53;
pub const PIC32_ALRMMON: c_uint = 0x52;
pub const PIC32_ALRMDAY: c_uint = 0x51;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pic32_rtc_dev {
    pub rtc: *mut rtc_device,
    pub reg_base: *mut void __iomem,
    pub clk: *mut clk,
    pub alarm_lock: spinlock_t,
    pub alarm_irq: c_int,
    pub alarm_clk_enabled: bool,
}

    static void pic32_rtc_alarm_clk_enable(struct pic32_rtc_dev *pdata,
    bool enable)
    {
    unsigned long flags;
    spin_lock_irqsave(&pdata.alarm_lock, flags);
    if (enable) {
    if (!pdata.alarm_clk_enabled) {
    clk_enable(pdata.clk);
    pdata.alarm_clk_enabled = true;
    }
    } else {
    if (pdata.alarm_clk_enabled) {
    clk_disable(pdata.clk);
    pdata.alarm_clk_enabled = false;
    }
    }
    spin_unlock_irqrestore(&pdata.alarm_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn pic32_rtc_alarmirq(irq: c_int, id: *mut c_void) -> irqreturn_t {
    static irqreturn_t pic32_rtc_alarmirq(int irq, void *id)
    {
    struct pic32_rtc_dev *pdata = (struct pic32_rtc_dev *)id;
    clk_enable(pdata.clk);
    rtc_update_irq(pdata.rtc, 1, RTC_AF | RTC_IRQF);
    clk_disable(pdata.clk);
    pic32_rtc_alarm_clk_enable(pdata, false);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn pic32_rtc_setaie(dev: *mut device, enabled: c_uint) -> c_int {
    static int pic32_rtc_setaie(struct device *dev, unsigned int enabled)
    {
    struct pic32_rtc_dev *pdata = dev_get_drvdata(dev);
    void __iomem *base = pdata.reg_base;
    clk_enable(pdata.clk);
    writel(PIC32_RTCALRM_ALRMEN,
    base + (enabled ? PIC32_SET(PIC32_RTCALRM) :
    PIC32_CLR(PIC32_RTCALRM)));
    clk_disable(pdata.clk);
    pic32_rtc_alarm_clk_enable(pdata, enabled);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pic32_rtc_setfreq(dev: *mut device, freq: c_int) -> c_int {
    static int pic32_rtc_setfreq(struct device *dev, int freq)
    {
    struct pic32_rtc_dev *pdata = dev_get_drvdata(dev);
    void __iomem *base = pdata.reg_base;
    clk_enable(pdata.clk);
    writel(PIC32_RTCALRM_AMASK, base + PIC32_CLR(PIC32_RTCALRM));
    writel(freq << 8, base + PIC32_SET(PIC32_RTCALRM));
    writel(PIC32_RTCALRM_CHIME, base + PIC32_SET(PIC32_RTCALRM));
    clk_disable(pdata.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pic32_rtc_gettime(dev: *mut device, rtc_tm: *mut rtc_time) -> c_int {
    static int pic32_rtc_gettime(struct device *dev, struct rtc_time *rtc_tm)
    {
    struct pic32_rtc_dev *pdata = dev_get_drvdata(dev);
    void __iomem *base = pdata.reg_base;
    let mut tries: c_uint = 0;
    clk_enable(pdata.clk);
    do {
    rtc_tm.tm_hour = readb(base + PIC32_RTCHOUR);
    rtc_tm.tm_min = readb(base + PIC32_RTCMIN);
    rtc_tm.tm_mon  = readb(base + PIC32_RTCMON);
    rtc_tm.tm_mday = readb(base + PIC32_RTCDAY);
    rtc_tm.tm_year = readb(base + PIC32_RTCYEAR);
    rtc_tm.tm_sec  = readb(base + PIC32_RTCSEC);
//
// The only way to work out whether the system was mid-update
// when we read it is to check the second counter, and if it
// is zero, then we re-try the entire read.
//
    tries += 1;
    } while (rtc_tm.tm_sec == 0 && tries < 2);
    rtc_tm.tm_sec = bcd2bin(rtc_tm.tm_sec);
    rtc_tm.tm_min = bcd2bin(rtc_tm.tm_min);
    rtc_tm.tm_hour = bcd2bin(rtc_tm.tm_hour);
    rtc_tm.tm_mday = bcd2bin(rtc_tm.tm_mday);
    rtc_tm.tm_mon = bcd2bin(rtc_tm.tm_mon) - 1;
    rtc_tm.tm_year = bcd2bin(rtc_tm.tm_year);
    rtc_tm.tm_year += 100;
    dev_dbg(dev, "read time %ptR\n", rtc_tm);
    clk_disable(pdata.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pic32_rtc_settime(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int pic32_rtc_settime(struct device *dev, struct rtc_time *tm)
    {
    struct pic32_rtc_dev *pdata = dev_get_drvdata(dev);
    void __iomem *base = pdata.reg_base;
    dev_dbg(dev, "set time %ptR\n", tm);
    clk_enable(pdata.clk);
    writeb(bin2bcd(tm.tm_sec),  base + PIC32_RTCSEC);
    writeb(bin2bcd(tm.tm_min),  base + PIC32_RTCMIN);
    writeb(bin2bcd(tm.tm_hour), base + PIC32_RTCHOUR);
    writeb(bin2bcd(tm.tm_mday), base + PIC32_RTCDAY);
    writeb(bin2bcd(tm.tm_mon + 1), base + PIC32_RTCMON);
    writeb(bin2bcd(tm.tm_year - 100), base + PIC32_RTCYEAR);
    clk_disable(pdata.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pic32_rtc_getalarm(dev: *mut device, alrm: *mut rtc_wkalrm) -> c_int {
    static int pic32_rtc_getalarm(struct device *dev, struct rtc_wkalrm *alrm)
    {
    struct pic32_rtc_dev *pdata = dev_get_drvdata(dev);
    struct rtc_time *alm_tm = &alrm.time;
    void __iomem *base = pdata.reg_base;
    unsigned int alm_en;
    clk_enable(pdata.clk);
    alm_tm.tm_sec  = readb(base + PIC32_ALRMSEC);
    alm_tm.tm_min  = readb(base + PIC32_ALRMMIN);
    alm_tm.tm_hour = readb(base + PIC32_ALRMHOUR);
    alm_tm.tm_mon  = readb(base + PIC32_ALRMMON);
    alm_tm.tm_mday = readb(base + PIC32_ALRMDAY);
    alm_tm.tm_year = readb(base + PIC32_ALRMYEAR);
    alm_en = readb(base + PIC32_RTCALRM);
    alrm.enabled = (alm_en & PIC32_RTCALRM_ALRMEN) ? 1 : 0;
    dev_dbg(dev, "getalarm: %d, %ptR\n", alm_en, alm_tm);
    alm_tm.tm_sec = bcd2bin(alm_tm.tm_sec);
    alm_tm.tm_min = bcd2bin(alm_tm.tm_min);
    alm_tm.tm_hour = bcd2bin(alm_tm.tm_hour);
    alm_tm.tm_mday = bcd2bin(alm_tm.tm_mday);
    alm_tm.tm_mon = bcd2bin(alm_tm.tm_mon) - 1;
    alm_tm.tm_year = bcd2bin(alm_tm.tm_year);
    clk_disable(pdata.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pic32_rtc_setalarm(dev: *mut device, alrm: *mut rtc_wkalrm) -> c_int {
    static int pic32_rtc_setalarm(struct device *dev, struct rtc_wkalrm *alrm)
    {
    struct pic32_rtc_dev *pdata = dev_get_drvdata(dev);
    struct rtc_time *tm = &alrm.time;
    void __iomem *base = pdata.reg_base;
    clk_enable(pdata.clk);
    dev_dbg(dev, "setalarm: %d, %ptR\n", alrm.enabled, tm);
    writel(0x00, base + PIC32_ALRMTIME);
    writel(0x00, base + PIC32_ALRMDATE);
    pic32_rtc_setaie(dev, alrm.enabled);
    clk_disable(pdata.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pic32_rtc_proc(dev: *mut device, seq: *mut seq_file) -> c_int {
    static int pic32_rtc_proc(struct device *dev, struct seq_file *seq)
    {
    struct pic32_rtc_dev *pdata = dev_get_drvdata(dev);
    void __iomem *base = pdata.reg_base;
    unsigned int repeat;
    clk_enable(pdata.clk);
    repeat = readw(base + PIC32_RTCALRM);
    repeat &= PIC32_RTCALRM_ARPT;
    seq_printf(seq, "periodic_IRQ\t: %s\n", repeat  ? "yes" : "no");
    clk_disable(pdata.clk);
    return 0;
    }
    static const struct rtc_class_ops pic32_rtcops = {
    .read_time	  = pic32_rtc_gettime,
    .set_time	  = pic32_rtc_settime,
    .read_alarm	  = pic32_rtc_getalarm,
    .set_alarm	  = pic32_rtc_setalarm,
    .proc		  = pic32_rtc_proc,
    .alarm_irq_enable = pic32_rtc_setaie,
    };
#[no_mangle]
unsafe extern "C" fn pic32_rtc_enable(pdata: *mut pic32_rtc_dev, en: c_int) {
    static void pic32_rtc_enable(struct pic32_rtc_dev *pdata, int en)
    {
    void __iomem *base = pdata.reg_base;
    if (!base)
    return;
    clk_enable(pdata.clk);
    if (!en) {
    writel(PIC32_RTCCON_ON, base + PIC32_CLR(PIC32_RTCCON));
    } else {
    pic32_syskey_unlock();
    writel(PIC32_RTCCON_RTCWREN, base + PIC32_SET(PIC32_RTCCON));
    writel(3 << 9, base + PIC32_CLR(PIC32_RTCCON));
    if (!(readl(base + PIC32_RTCCON) & PIC32_RTCCON_ON))
    writel(PIC32_RTCCON_ON, base + PIC32_SET(PIC32_RTCCON));
    }
    clk_disable(pdata.clk);
    }
#[no_mangle]
unsafe extern "C" fn pic32_rtc_remove(pdev: *mut platform_device) {
    static void pic32_rtc_remove(struct platform_device *pdev)
    {
    struct pic32_rtc_dev *pdata = platform_get_drvdata(pdev);
    pic32_rtc_setaie(&pdev.dev, 0);
    clk_unprepare(pdata.clk);
    pdata.clk = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn pic32_rtc_probe(pdev: *mut platform_device) -> c_int {
    static int pic32_rtc_probe(struct platform_device *pdev)
    {
    struct pic32_rtc_dev *pdata;
    int ret;
    pdata = devm_kzalloc(&pdev.dev, sizeof(*pdata), GFP_KERNEL);
    if (!pdata)
    return -ENOMEM;
    platform_set_drvdata(pdev, pdata);
    pdata.alarm_irq = platform_get_irq(pdev, 0);
    if (pdata.alarm_irq < 0)
    return pdata.alarm_irq;
    pdata.reg_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(pdata.reg_base))
    return PTR_ERR(pdata.reg_base);
    pdata.clk = devm_clk_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(pdata.clk)) {
    dev_err(&pdev.dev, "failed to find rtc clock source\n");
    ret = PTR_ERR(pdata.clk);
    pdata.clk = core::ptr::null_mut();
    return ret;
    }
    spin_lock_init(&pdata.alarm_lock);
    pdata.rtc = devm_rtc_allocate_device(&pdev.dev);
    if (IS_ERR(pdata.rtc))
    return PTR_ERR(pdata.rtc);
    clk_prepare_enable(pdata.clk);
    pic32_rtc_enable(pdata, 1);
    device_init_wakeup(&pdev.dev, true);
    pdata.rtc.ops = &pic32_rtcops;
    pdata.rtc.range_min = RTC_TIMESTAMP_BEGIN_2000;
    pdata.rtc.range_max = RTC_TIMESTAMP_END_2099;
    ret = devm_rtc_register_device(pdata.rtc);
    if (ret)
    goto err_nortc;
    pic32_rtc_setfreq(&pdev.dev, 1);
    ret = devm_request_irq(&pdev.dev, pdata.alarm_irq,
    pic32_rtc_alarmirq, 0,
    dev_name(&pdev.dev), pdata);
    if (ret) {
    dev_err(&pdev.dev,
    "IRQ %d error %d\n", pdata.alarm_irq, ret);
    goto err_nortc;
    }
    clk_disable(pdata.clk);
    return 0;
    err_nortc:
    pic32_rtc_enable(pdata, 0);
    clk_disable_unprepare(pdata.clk);
    return ret;
    }
    static const struct of_device_id pic32_rtc_dt_ids[] = {
    { .compatible = "microchip,pic32mzda-rtc" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, pic32_rtc_dt_ids);
    static struct platform_driver pic32_rtc_driver = {
    .probe		= pic32_rtc_probe,
    .remove		= pic32_rtc_remove,
    .driver		= {
    .name	= "pic32-rtc",
    .of_match_table	= of_match_ptr(pic32_rtc_dt_ids),
    },
    };
    module_platform_driver(pic32_rtc_driver);
    MODULE_DESCRIPTION("Microchip PIC32 RTC Driver");
    MODULE_AUTHOR("Joshua Henderson <joshua.henderson@microchip.com>");
    MODULE_LICENSE("GPL");
