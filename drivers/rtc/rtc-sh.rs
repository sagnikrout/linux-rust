//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-sh.c
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
// SuperH On-Chip RTC Support
//
// Copyright (C) 2006 - 2009  Paul Mundt
// Copyright (C) 2006  Jamie Lenehan
// Copyright (C) 2008  Angelo Castello
// Copyright (C) 2025  Wolfram Sang, Renesas Electronics Corporation
//
// Based on the old arch/sh/kernel/cpu/rtc.c by:
//
// Copyright (C) 2000  Philipp Rumpf <prumpf@tux.org>
// Copyright (C) 1999  Tetsuya Okada & Niibe Yutaka
//

// Default values for RZ/A RTC

//
// Note on RYRAR and RCR3: Up until this point most of the register
// definitions are consistent across all of the available parts. However,
// the placement of the optional RYRAR and RCR3 (the RYRAR control
// register used to control RYRCNT/RYRAR compare) varies considerably
// across various parts, occasionally being mapped in to a completely
// unrelated address space. For proper RYRAR support a separate resource
// would have to be handed off, but as this is purely optional in
// practice, we simply opt not to support it, thereby keeping the code
// quite a bit more simplified.
//
// ALARM Bits - or with BCD encoded value

// RCR1 Bits

// RCR2 Bits

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sh_rtc {
    pub regbase: *mut void __iomem,
    pub alarm_irq: c_int,
    pub clk: *mut clk,
    pub rtc_dev: *mut rtc_device,
    pub /: *mut *mut spinlock_t lock; / protecting register access,
    pub /: *mut *mut unsigned long capabilities; / See asm/rtc.h for cap bits,
}

#[no_mangle]
unsafe extern "C" fn sh_rtc_alarm(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t sh_rtc_alarm(int irq, void *dev_id)
    {
    struct sh_rtc *rtc = dev_id;
    unsigned int tmp, pending;
    spin_lock(&rtc.lock);
    tmp = readb(rtc.regbase + RCR1);
    pending = tmp & RCR1_AF;
    tmp &= ~(RCR1_AF | RCR1_AIE);
    writeb(tmp, rtc.regbase + RCR1);
    if (pending)
    rtc_update_irq(rtc.rtc_dev, 1, RTC_AF | RTC_IRQF);
    spin_unlock(&rtc.lock);
    return IRQ_RETVAL(pending);
    }
#[no_mangle]
unsafe extern "C" fn sh_rtc_alarm_irq_enable(dev: *mut device, enable: c_uint) -> c_int {
    static int sh_rtc_alarm_irq_enable(struct device *dev, unsigned int enable)
    {
    struct sh_rtc *rtc = dev_get_drvdata(dev);
    unsigned int tmp;
    spin_lock_irq(&rtc.lock);
    tmp = readb(rtc.regbase + RCR1);
    if (enable)
    tmp |= RCR1_AIE;
    else
    tmp &= ~RCR1_AIE;
    writeb(tmp, rtc.regbase + RCR1);
    spin_unlock_irq(&rtc.lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sh_rtc_read_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int sh_rtc_read_time(struct device *dev, struct rtc_time *tm)
    {
    struct sh_rtc *rtc = dev_get_drvdata(dev);
    unsigned int sec128, sec2, yr, yr100, cf_bit;
    if (!(readb(rtc.regbase + RCR2) & RCR2_RTCEN))
    return -EINVAL;
    do {
    unsigned int tmp;
    spin_lock_irq(&rtc.lock);
    tmp = readb(rtc.regbase + RCR1);
    tmp &= ~RCR1_CF; /* Clear CF-bit */
    tmp |= RCR1_CIE;
    writeb(tmp, rtc.regbase + RCR1);
    sec128 = readb(rtc.regbase + R64CNT);
    tm.tm_sec	= bcd2bin(readb(rtc.regbase + RSECCNT));
    tm.tm_min	= bcd2bin(readb(rtc.regbase + RMINCNT));
    tm.tm_hour	= bcd2bin(readb(rtc.regbase + RHRCNT));
    tm.tm_wday	= bcd2bin(readb(rtc.regbase + RWKCNT));
    tm.tm_mday	= bcd2bin(readb(rtc.regbase + RDAYCNT));
    tm.tm_mon	= bcd2bin(readb(rtc.regbase + RMONCNT)) - 1;
    if (rtc.capabilities & RTC_CAP_4_DIGIT_YEAR) {
    yr  = readw(rtc.regbase + RYRCNT);
    yr100 = bcd2bin(yr >> 8);
    yr &= 0xff;
    } else {
    yr  = readb(rtc.regbase + RYRCNT);
    yr100 = bcd2bin((yr == 0x99) ? 0x19 : 0x20);
    }
    tm.tm_year = (yr100 * 100 + bcd2bin(yr)) - 1900;
    sec2 = readb(rtc.regbase + R64CNT);
    cf_bit = readb(rtc.regbase + RCR1) & RCR1_CF;
    spin_unlock_irq(&rtc.lock);
    } while (cf_bit != 0 || ((sec128 ^ sec2) & RTC_BIT_INVERTED) != 0);

    if ((sec128 & RTC_BIT_INVERTED))
    tm.tm_sec--;

    dev_dbg(dev, "%s: tm is secs=%d, mins=%d, hours=%d, mday=%d, mon=%d, year=%d, wday=%d\n",
    __func__, tm.tm_sec, tm.tm_min, tm.tm_hour,
    tm.tm_mday, tm.tm_mon + 1, tm.tm_year, tm.tm_wday);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sh_rtc_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int sh_rtc_set_time(struct device *dev, struct rtc_time *tm)
    {
    struct sh_rtc *rtc = dev_get_drvdata(dev);
    unsigned int tmp;
    int year;
    spin_lock_irq(&rtc.lock);
// Reset pre-scaler & stop RTC
    tmp = readb(rtc.regbase + RCR2);
    tmp |= RCR2_RESET;
    tmp &= ~RCR2_START;
    writeb(tmp, rtc.regbase + RCR2);
    writeb(bin2bcd(tm.tm_sec),  rtc.regbase + RSECCNT);
    writeb(bin2bcd(tm.tm_min),  rtc.regbase + RMINCNT);
    writeb(bin2bcd(tm.tm_hour), rtc.regbase + RHRCNT);
    writeb(bin2bcd(tm.tm_wday), rtc.regbase + RWKCNT);
    writeb(bin2bcd(tm.tm_mday), rtc.regbase + RDAYCNT);
    writeb(bin2bcd(tm.tm_mon + 1), rtc.regbase + RMONCNT);
    if (rtc.capabilities & RTC_CAP_4_DIGIT_YEAR) {
    year = (bin2bcd((tm.tm_year + 1900) / 100) << 8) |
    bin2bcd(tm.tm_year % 100);
    writew(year, rtc.regbase + RYRCNT);
    } else {
    year = tm.tm_year % 100;
    writeb(bin2bcd(year), rtc.regbase + RYRCNT);
    }
// Start RTC
    tmp = readb(rtc.regbase + RCR2);
    tmp &= ~RCR2_RESET;
    tmp |= RCR2_RTCEN | RCR2_START;
    writeb(tmp, rtc.regbase + RCR2);
    spin_unlock_irq(&rtc.lock);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn sh_rtc_read_alarm_value(rtc: *mut sh_rtc, reg_off: c_int) -> c_int {
    static inline int sh_rtc_read_alarm_value(struct sh_rtc *rtc, int reg_off)
    {
    unsigned int byte;
    int value = -1;			/* return -1 for ignored values */
    byte = readb(rtc.regbase + reg_off);
    if (byte & AR_ENB) {
    byte &= ~AR_ENB;	/* strip the enable bit */
    value = bcd2bin(byte);
    }
    return value;
    }
#[no_mangle]
unsafe extern "C" fn sh_rtc_read_alarm(dev: *mut device, wkalrm: *mut rtc_wkalrm) -> c_int {
    static int sh_rtc_read_alarm(struct device *dev, struct rtc_wkalrm *wkalrm)
    {
    struct sh_rtc *rtc = dev_get_drvdata(dev);
    struct rtc_time *tm = &wkalrm.time;
    spin_lock_irq(&rtc.lock);
    tm.tm_sec	= sh_rtc_read_alarm_value(rtc, RSECAR);
    tm.tm_min	= sh_rtc_read_alarm_value(rtc, RMINAR);
    tm.tm_hour	= sh_rtc_read_alarm_value(rtc, RHRAR);
    tm.tm_wday	= sh_rtc_read_alarm_value(rtc, RWKAR);
    tm.tm_mday	= sh_rtc_read_alarm_value(rtc, RDAYAR);
    tm.tm_mon	= sh_rtc_read_alarm_value(rtc, RMONAR);
    if (tm.tm_mon > 0)
    tm.tm_mon -= 1; /* RTC is 1-12, tm_mon is 0-11 */
    wkalrm.enabled = (readb(rtc.regbase + RCR1) & RCR1_AIE) ? 1 : 0;
    spin_unlock_irq(&rtc.lock);
    return 0;
    }
    static inline void sh_rtc_write_alarm_value(struct sh_rtc *rtc,
    int value, int reg_off)
    {
// < 0 for a value that is ignored
    if (value < 0)
    writeb(0, rtc.regbase + reg_off);
    else
    writeb(bin2bcd(value) | AR_ENB,  rtc.regbase + reg_off);
    }
#[no_mangle]
unsafe extern "C" fn sh_rtc_set_alarm(dev: *mut device, wkalrm: *mut rtc_wkalrm) -> c_int {
    static int sh_rtc_set_alarm(struct device *dev, struct rtc_wkalrm *wkalrm)
    {
    struct sh_rtc *rtc = dev_get_drvdata(dev);
    unsigned int rcr1;
    struct rtc_time *tm = &wkalrm.time;
    int mon;
    spin_lock_irq(&rtc.lock);
// disable alarm interrupt and clear the alarm flag
    rcr1 = readb(rtc.regbase + RCR1);
    rcr1 &= ~(RCR1_AF | RCR1_AIE);
    writeb(rcr1, rtc.regbase + RCR1);
// set alarm time
    sh_rtc_write_alarm_value(rtc, tm.tm_sec,  RSECAR);
    sh_rtc_write_alarm_value(rtc, tm.tm_min,  RMINAR);
    sh_rtc_write_alarm_value(rtc, tm.tm_hour, RHRAR);
    sh_rtc_write_alarm_value(rtc, tm.tm_wday, RWKAR);
    sh_rtc_write_alarm_value(rtc, tm.tm_mday, RDAYAR);
    mon = tm.tm_mon;
    if (mon >= 0)
    mon += 1;
    sh_rtc_write_alarm_value(rtc, mon, RMONAR);
    if (wkalrm.enabled) {
    rcr1 |= RCR1_AIE;
    writeb(rcr1, rtc.regbase + RCR1);
    }
    spin_unlock_irq(&rtc.lock);
    return 0;
    }
    static const struct rtc_class_ops sh_rtc_ops = {
    .read_time	= sh_rtc_read_time,
    .set_time	= sh_rtc_set_time,
    .read_alarm	= sh_rtc_read_alarm,
    .set_alarm	= sh_rtc_set_alarm,
    .alarm_irq_enable = sh_rtc_alarm_irq_enable,
    };
#[no_mangle]
unsafe extern "C" fn sh_rtc_probe(pdev: *mut platform_device) -> int __init {
    static int __init sh_rtc_probe(struct platform_device *pdev)
    {
    struct sh_rtc *rtc;
    struct resource *res, *req_res;
    char clk_name[14];
    int clk_id, ret;
    unsigned int tmp;
    resource_size_t regsize;
    rtc = devm_kzalloc(&pdev.dev, sizeof(*rtc), GFP_KERNEL);
    if (unlikely(!rtc))
    return -ENOMEM;
    spin_lock_init(&rtc.lock);
    ret = platform_get_irq(pdev, 0);
    if (unlikely(ret <= 0)) {
    dev_err(&pdev.dev, "No IRQ resource\n");
    return -ENOENT;
    }
    if (!pdev.dev.of_node)
    rtc.alarm_irq = platform_get_irq(pdev, 2);
    else
    rtc.alarm_irq = ret;
    res = platform_get_resource(pdev, IORESOURCE_IO, 0);
    if (!res)
    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!res) {
    dev_err(&pdev.dev, "No IO resource\n");
    return -ENOENT;
    }
    regsize = resource_size(res);
    req_res = devm_request_mem_region(&pdev.dev, res.start, regsize, pdev.name);
    if (!req_res)
    return -EBUSY;
    rtc.regbase = devm_ioremap(&pdev.dev, req_res.start, regsize);
    if (!rtc.regbase)
    return -EINVAL;
    if (!pdev.dev.of_node) {
    clk_id = pdev.id;
// With a single device, the clock id is still "rtc0"
    if (clk_id < 0)
    clk_id = 0;
    snprintf(clk_name, sizeof(clk_name), "rtc%d", clk_id);
    } else {
    snprintf(clk_name, sizeof(clk_name), "fck");
    }
    rtc.clk = devm_clk_get(&pdev.dev, clk_name);
    if (IS_ERR(rtc.clk)) {
//
// No error handling for rtc->clk intentionally, not all
// platforms will have a unique clock for the RTC, and
// the clk API can handle the struct clk pointer being
// NULL.
//
    rtc.clk = core::ptr::null_mut();
    }
    rtc.rtc_dev = devm_rtc_allocate_device(&pdev.dev);
    if (IS_ERR(rtc.rtc_dev))
    return PTR_ERR(rtc.rtc_dev);
    clk_enable(rtc.clk);
    rtc.capabilities = RTC_DEF_CAPABILITIES;

    if (dev_get_platdata(&pdev.dev)) {
    struct sh_rtc_platform_info *pinfo =
    dev_get_platdata(&pdev.dev);
//
// Some CPUs have special capabilities in addition to the
// default set. Add those in here.
//
    rtc.capabilities |= pinfo.capabilities;
    }

    ret = devm_request_irq(&pdev.dev, rtc.alarm_irq, sh_rtc_alarm, 0, "sh-rtc", rtc);
    if (ret) {
    dev_err(&pdev.dev, "request alarm IRQ failed with %d, IRQ %d\n",
    ret, rtc.alarm_irq);
    goto err_unmap;
    }
    platform_set_drvdata(pdev, rtc);
// everything disabled by default
    tmp = readb(rtc.regbase + RCR1);
    tmp &= ~(RCR1_CIE | RCR1_AIE);
    writeb(tmp, rtc.regbase + RCR1);
    rtc.rtc_dev.ops = &sh_rtc_ops;
    if (rtc.capabilities & RTC_CAP_4_DIGIT_YEAR) {
    rtc.rtc_dev.range_min = RTC_TIMESTAMP_BEGIN_1900;
    rtc.rtc_dev.range_max = RTC_TIMESTAMP_END_9999;
    } else {
    rtc.rtc_dev.range_min = mktime64(1999, 1, 1, 0, 0, 0);
    rtc.rtc_dev.range_max = mktime64(2098, 12, 31, 23, 59, 59);
    }
    ret = devm_rtc_register_device(rtc.rtc_dev);
    if (ret)
    goto err_unmap;
    device_init_wakeup(&pdev.dev, true);
    return 0;
    err_unmap:
    clk_disable(rtc.clk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sh_rtc_remove(pdev: *mut platform_device) -> void __exit {
    static void __exit sh_rtc_remove(struct platform_device *pdev)
    {
    struct sh_rtc *rtc = platform_get_drvdata(pdev);
    sh_rtc_alarm_irq_enable(&pdev.dev, 0);
    clk_disable(rtc.clk);
    }
#[no_mangle]
unsafe extern "C" fn sh_rtc_suspend(dev: *mut device) -> c_int {
    static int sh_rtc_suspend(struct device *dev)
    {
    struct sh_rtc *rtc = dev_get_drvdata(dev);
    if (device_may_wakeup(dev))
    irq_set_irq_wake(rtc.alarm_irq, 1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sh_rtc_resume(dev: *mut device) -> c_int {
    static int sh_rtc_resume(struct device *dev)
    {
    struct sh_rtc *rtc = dev_get_drvdata(dev);
    if (device_may_wakeup(dev))
    irq_set_irq_wake(rtc.alarm_irq, 0);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(sh_rtc_pm_ops, sh_rtc_suspend, sh_rtc_resume);
    static const struct of_device_id sh_rtc_of_match[] = {
    { .compatible = "renesas,sh-rtc", },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, sh_rtc_of_match);
//
// sh_rtc_remove() lives in .exit.text. For drivers registered via
// module_platform_driver_probe() this is ok because they cannot get unbound at
// runtime. So mark the driver struct with __refdata to prevent modpost
// triggering a section mismatch warning.
//
    static struct platform_driver sh_rtc_platform_driver __refdata = {
    .driver		= {
    .name	= DRV_NAME,
    .pm	= pm_sleep_ptr(&sh_rtc_pm_ops),
    .of_match_table = sh_rtc_of_match,
    },
    .remove		= __exit_p(sh_rtc_remove),
    };
    module_platform_driver_probe(sh_rtc_platform_driver, sh_rtc_probe);
    MODULE_DESCRIPTION("SuperH on-chip RTC driver");
    MODULE_AUTHOR("Paul Mundt <lethal@linux-sh.org>");
    MODULE_AUTHOR("Jamie Lenehan <lenehan@twibble.org>");
    MODULE_AUTHOR("Angelo Castello <angelo.castello@st.com>");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:" DRV_NAME);
