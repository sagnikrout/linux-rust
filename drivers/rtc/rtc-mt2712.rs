//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-mt2712.c
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
// Copyright (c) 2019 MediaTek Inc.
// Author: Ran Bi <ran.bi@mediatek.com>
//

pub const MT2712_BBPU: c_uint = 0x0000;

pub const MT2712_IRQ_STA: c_uint = 0x0004;

pub const MT2712_IRQ_EN: c_uint = 0x0008;

pub const MT2712_CII_EN: c_uint = 0x000c;
pub const MT2712_AL_MASK: c_uint = 0x0010;

pub const MT2712_TC_SEC: c_uint = 0x0014;
pub const MT2712_TC_MIN: c_uint = 0x0018;
pub const MT2712_TC_HOU: c_uint = 0x001c;
pub const MT2712_TC_DOM: c_uint = 0x0020;
pub const MT2712_TC_DOW: c_uint = 0x0024;
pub const MT2712_TC_MTH: c_uint = 0x0028;
pub const MT2712_TC_YEA: c_uint = 0x002c;
pub const MT2712_AL_SEC: c_uint = 0x0030;
pub const MT2712_AL_MIN: c_uint = 0x0034;
pub const MT2712_AL_HOU: c_uint = 0x0038;
pub const MT2712_AL_DOM: c_uint = 0x003c;
pub const MT2712_AL_DOW: c_uint = 0x0040;
pub const MT2712_AL_MTH: c_uint = 0x0044;
pub const MT2712_AL_YEA: c_uint = 0x0048;
pub const MT2712_SEC_MASK: c_uint = 0x003f;
pub const MT2712_MIN_MASK: c_uint = 0x003f;
pub const MT2712_HOU_MASK: c_uint = 0x001f;
pub const MT2712_DOM_MASK: c_uint = 0x001f;
pub const MT2712_DOW_MASK: c_uint = 0x0007;
pub const MT2712_MTH_MASK: c_uint = 0x000f;
pub const MT2712_YEA_MASK: c_uint = 0x007f;
pub const MT2712_POWERKEY1: c_uint = 0x004c;
pub const MT2712_POWERKEY2: c_uint = 0x0050;
pub const MT2712_POWERKEY1_KEY: c_uint = 0xa357;
pub const MT2712_POWERKEY2_KEY: c_uint = 0x67d2;
pub const MT2712_CON0: c_uint = 0x005c;
pub const MT2712_CON1: c_uint = 0x0060;
pub const MT2712_PROT: c_uint = 0x0070;
pub const MT2712_PROT_UNLOCK1: c_uint = 0x9136;
pub const MT2712_PROT_UNLOCK2: c_uint = 0x586a;
pub const MT2712_WRTGR: c_uint = 0x0078;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt2712_rtc {
    pub rtc: *mut rtc_device,
    pub base: *mut void __iomem,
    pub irq: c_int,
    pub irq_wake_enabled: u8,
    pub powerlost: u8,
}

#[no_mangle]
pub unsafe extern "C" fn mt2712_readl(mt2712_rtc: *mut mt2712_rtc, reg: u32) -> u32 {
    static inline u32 mt2712_readl(struct mt2712_rtc *mt2712_rtc, u32 reg)
    {
    return readl(mt2712_rtc.base + reg);
    }
    static inline void mt2712_writel(struct mt2712_rtc *mt2712_rtc,
    u32 reg, u32 val)
    {
    writel(val, mt2712_rtc.base + reg);
    }
#[no_mangle]
unsafe extern "C" fn mt2712_rtc_write_trigger(mt2712_rtc: *mut mt2712_rtc) {
    static void mt2712_rtc_write_trigger(struct mt2712_rtc *mt2712_rtc)
    {
    let mut timeout: c_ulong = jiffies + HZ / 10;
    mt2712_writel(mt2712_rtc, MT2712_WRTGR, 1);
    while (1) {
    if (!(mt2712_readl(mt2712_rtc, MT2712_BBPU)
    & MT2712_BBPU_CBUSY))
    break;
    if (time_after(jiffies, timeout)) {
    dev_err(&mt2712_rtc.rtc.dev,
    "%s time out!\n", __func__);
    break;
    }
    cpu_relax();
    }
    }
#[no_mangle]
unsafe extern "C" fn mt2712_rtc_writeif_unlock(mt2712_rtc: *mut mt2712_rtc) {
    static void mt2712_rtc_writeif_unlock(struct mt2712_rtc *mt2712_rtc)
    {
    mt2712_writel(mt2712_rtc, MT2712_PROT, MT2712_PROT_UNLOCK1);
    mt2712_rtc_write_trigger(mt2712_rtc);
    mt2712_writel(mt2712_rtc, MT2712_PROT, MT2712_PROT_UNLOCK2);
    mt2712_rtc_write_trigger(mt2712_rtc);
    }
#[no_mangle]
unsafe extern "C" fn rtc_irq_handler_thread(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t rtc_irq_handler_thread(int irq, void *data)
    {
    struct mt2712_rtc *mt2712_rtc = data;
    u16 irqsta;
// Clear interrupt
    irqsta = mt2712_readl(mt2712_rtc, MT2712_IRQ_STA);
    if (irqsta & MT2712_IRQ_STA_AL) {
    rtc_update_irq(mt2712_rtc.rtc, 1, RTC_IRQF | RTC_AF);
    return IRQ_HANDLED;
    }
    return IRQ_NONE;
    }
    static void __mt2712_rtc_read_time(struct mt2712_rtc *mt2712_rtc,
    struct rtc_time *tm, int *sec)
    {
    tm.tm_sec  = mt2712_readl(mt2712_rtc, MT2712_TC_SEC)
    & MT2712_SEC_MASK;
    tm.tm_min  = mt2712_readl(mt2712_rtc, MT2712_TC_MIN)
    & MT2712_MIN_MASK;
    tm.tm_hour = mt2712_readl(mt2712_rtc, MT2712_TC_HOU)
    & MT2712_HOU_MASK;
    tm.tm_mday = mt2712_readl(mt2712_rtc, MT2712_TC_DOM)
    & MT2712_DOM_MASK;
    tm.tm_mon  = (mt2712_readl(mt2712_rtc, MT2712_TC_MTH) - 1)
    & MT2712_MTH_MASK;
    tm.tm_year = (mt2712_readl(mt2712_rtc, MT2712_TC_YEA) + 100)
    & MT2712_YEA_MASK;
// sec = mt2712_readl(mt2712_rtc, MT2712_TC_SEC) & MT2712_SEC_MASK;
    }
#[no_mangle]
unsafe extern "C" fn mt2712_rtc_read_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int mt2712_rtc_read_time(struct device *dev, struct rtc_time *tm)
    {
    struct mt2712_rtc *mt2712_rtc = dev_get_drvdata(dev);
    int sec;
    if (mt2712_rtc.powerlost)
    return -EINVAL;
    do {
    __mt2712_rtc_read_time(mt2712_rtc, tm, &sec);
    } while (sec < tm.tm_sec);	/* SEC has carried */
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mt2712_rtc_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int mt2712_rtc_set_time(struct device *dev, struct rtc_time *tm)
    {
    struct mt2712_rtc *mt2712_rtc = dev_get_drvdata(dev);
    mt2712_writel(mt2712_rtc, MT2712_TC_SEC, tm.tm_sec  & MT2712_SEC_MASK);
    mt2712_writel(mt2712_rtc, MT2712_TC_MIN, tm.tm_min  & MT2712_MIN_MASK);
    mt2712_writel(mt2712_rtc, MT2712_TC_HOU, tm.tm_hour & MT2712_HOU_MASK);
    mt2712_writel(mt2712_rtc, MT2712_TC_DOM, tm.tm_mday & MT2712_DOM_MASK);
    mt2712_writel(mt2712_rtc, MT2712_TC_MTH,
    (tm.tm_mon + 1) & MT2712_MTH_MASK);
    mt2712_writel(mt2712_rtc, MT2712_TC_YEA,
    (tm.tm_year - 100) & MT2712_YEA_MASK);
    mt2712_rtc_write_trigger(mt2712_rtc);
    if (mt2712_rtc.powerlost)
    mt2712_rtc.powerlost = false;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mt2712_rtc_read_alarm(dev: *mut device, alm: *mut rtc_wkalrm) -> c_int {
    static int mt2712_rtc_read_alarm(struct device *dev, struct rtc_wkalrm *alm)
    {
    struct mt2712_rtc *mt2712_rtc = dev_get_drvdata(dev);
    struct rtc_time *tm = &alm.time;
    u16 irqen;
    irqen = mt2712_readl(mt2712_rtc, MT2712_IRQ_EN);
    alm.enabled = !!(irqen & MT2712_IRQ_EN_AL);
    tm.tm_sec  = mt2712_readl(mt2712_rtc, MT2712_AL_SEC) & MT2712_SEC_MASK;
    tm.tm_min  = mt2712_readl(mt2712_rtc, MT2712_AL_MIN) & MT2712_MIN_MASK;
    tm.tm_hour = mt2712_readl(mt2712_rtc, MT2712_AL_HOU) & MT2712_HOU_MASK;
    tm.tm_mday = mt2712_readl(mt2712_rtc, MT2712_AL_DOM) & MT2712_DOM_MASK;
    tm.tm_mon  = (mt2712_readl(mt2712_rtc, MT2712_AL_MTH) - 1)
    & MT2712_MTH_MASK;
    tm.tm_year = (mt2712_readl(mt2712_rtc, MT2712_AL_YEA) + 100)
    & MT2712_YEA_MASK;
    return 0;
    }
    static int mt2712_rtc_alarm_irq_enable(struct device *dev,
    unsigned int enabled)
    {
    struct mt2712_rtc *mt2712_rtc = dev_get_drvdata(dev);
    u16 irqen;
    irqen = mt2712_readl(mt2712_rtc, MT2712_IRQ_EN);
    if (enabled)
    irqen |= MT2712_IRQ_EN_AL;
    else
    irqen &= ~MT2712_IRQ_EN_AL;
    mt2712_writel(mt2712_rtc, MT2712_IRQ_EN, irqen);
    mt2712_rtc_write_trigger(mt2712_rtc);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mt2712_rtc_set_alarm(dev: *mut device, alm: *mut rtc_wkalrm) -> c_int {
    static int mt2712_rtc_set_alarm(struct device *dev, struct rtc_wkalrm *alm)
    {
    struct mt2712_rtc *mt2712_rtc = dev_get_drvdata(dev);
    struct rtc_time *tm = &alm.time;
    dev_dbg(&mt2712_rtc.rtc.dev, "set al time: %ptR, alm en: %d\n",
    tm, alm.enabled);
    mt2712_writel(mt2712_rtc, MT2712_AL_SEC,
    (mt2712_readl(mt2712_rtc, MT2712_AL_SEC)
    & ~(MT2712_SEC_MASK)) | (tm.tm_sec  & MT2712_SEC_MASK));
    mt2712_writel(mt2712_rtc, MT2712_AL_MIN,
    (mt2712_readl(mt2712_rtc, MT2712_AL_MIN)
    & ~(MT2712_MIN_MASK)) | (tm.tm_min  & MT2712_MIN_MASK));
    mt2712_writel(mt2712_rtc, MT2712_AL_HOU,
    (mt2712_readl(mt2712_rtc, MT2712_AL_HOU)
    & ~(MT2712_HOU_MASK)) | (tm.tm_hour & MT2712_HOU_MASK));
    mt2712_writel(mt2712_rtc, MT2712_AL_DOM,
    (mt2712_readl(mt2712_rtc, MT2712_AL_DOM)
    & ~(MT2712_DOM_MASK)) | (tm.tm_mday & MT2712_DOM_MASK));
    mt2712_writel(mt2712_rtc, MT2712_AL_MTH,
    (mt2712_readl(mt2712_rtc, MT2712_AL_MTH)
    & ~(MT2712_MTH_MASK))
    | ((tm.tm_mon + 1) & MT2712_MTH_MASK));
    mt2712_writel(mt2712_rtc, MT2712_AL_YEA,
    (mt2712_readl(mt2712_rtc, MT2712_AL_YEA)
    & ~(MT2712_YEA_MASK))
    | ((tm.tm_year - 100) & MT2712_YEA_MASK));
// mask day of week
    mt2712_writel(mt2712_rtc, MT2712_AL_MASK, MT2712_AL_MASK_DOW);
    mt2712_rtc_write_trigger(mt2712_rtc);
    mt2712_rtc_alarm_irq_enable(dev, alm.enabled);
    return 0;
    }
// Init RTC register
#[no_mangle]
unsafe extern "C" fn mt2712_rtc_hw_init(mt2712_rtc: *mut mt2712_rtc) {
    static void mt2712_rtc_hw_init(struct mt2712_rtc *mt2712_rtc)
    {
    u32 p1, p2;
    mt2712_writel(mt2712_rtc, MT2712_BBPU,
    MT2712_BBPU_KEY | MT2712_BBPU_RELOAD);
    mt2712_writel(mt2712_rtc, MT2712_CII_EN, 0);
    mt2712_writel(mt2712_rtc, MT2712_AL_MASK, 0);
// necessary before set MT2712_POWERKEY
    mt2712_writel(mt2712_rtc, MT2712_CON0, 0x4848);
    mt2712_writel(mt2712_rtc, MT2712_CON1, 0x0048);
    mt2712_rtc_write_trigger(mt2712_rtc);
    p1 = mt2712_readl(mt2712_rtc, MT2712_POWERKEY1);
    p2 = mt2712_readl(mt2712_rtc, MT2712_POWERKEY2);
    if (p1 != MT2712_POWERKEY1_KEY || p2 != MT2712_POWERKEY2_KEY) {
    mt2712_rtc.powerlost = true;
    dev_dbg(&mt2712_rtc.rtc.dev,
    "powerkey not set (lost power)\n");
    } else {
    mt2712_rtc.powerlost = false;
    }
// RTC need POWERKEY1/2 match, then goto normal work mode
    mt2712_writel(mt2712_rtc, MT2712_POWERKEY1, MT2712_POWERKEY1_KEY);
    mt2712_writel(mt2712_rtc, MT2712_POWERKEY2, MT2712_POWERKEY2_KEY);
    mt2712_rtc_write_trigger(mt2712_rtc);
    mt2712_rtc_writeif_unlock(mt2712_rtc);
    }
    static const struct rtc_class_ops mt2712_rtc_ops = {
    .read_time	= mt2712_rtc_read_time,
    .set_time	= mt2712_rtc_set_time,
    .read_alarm	= mt2712_rtc_read_alarm,
    .set_alarm	= mt2712_rtc_set_alarm,
    .alarm_irq_enable = mt2712_rtc_alarm_irq_enable,
    };
#[no_mangle]
unsafe extern "C" fn mt2712_rtc_probe(pdev: *mut platform_device) -> c_int {
    static int mt2712_rtc_probe(struct platform_device *pdev)
    {
    struct mt2712_rtc *mt2712_rtc;
    int ret;
    mt2712_rtc = devm_kzalloc(&pdev.dev,
    sizeof(struct mt2712_rtc), GFP_KERNEL);
    if (!mt2712_rtc)
    return -ENOMEM;
    mt2712_rtc.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(mt2712_rtc.base))
    return PTR_ERR(mt2712_rtc.base);
// rtc hw init
    mt2712_rtc_hw_init(mt2712_rtc);
    mt2712_rtc.irq = platform_get_irq(pdev, 0);
    if (mt2712_rtc.irq < 0)
    return mt2712_rtc.irq;
    platform_set_drvdata(pdev, mt2712_rtc);
    mt2712_rtc.rtc = devm_rtc_allocate_device(&pdev.dev);
    if (IS_ERR(mt2712_rtc.rtc))
    return PTR_ERR(mt2712_rtc.rtc);
    ret = devm_request_threaded_irq(&pdev.dev, mt2712_rtc.irq, core::ptr::null_mut(),
    rtc_irq_handler_thread,
    IRQF_ONESHOT | IRQF_TRIGGER_LOW,
    dev_name(&mt2712_rtc.rtc.dev),
    mt2712_rtc);
    if (ret) {
    dev_err(&pdev.dev, "Failed to request alarm IRQ: %d: %d\n",
    mt2712_rtc.irq, ret);
    return ret;
    }
    device_init_wakeup(&pdev.dev, true);
    mt2712_rtc.rtc.ops = &mt2712_rtc_ops;
    mt2712_rtc.rtc.range_min = RTC_TIMESTAMP_BEGIN_2000;
    mt2712_rtc.rtc.range_max = MT2712_RTC_TIMESTAMP_END_2127;
    return devm_rtc_register_device(mt2712_rtc.rtc);
    }

#[no_mangle]
unsafe extern "C" fn mt2712_rtc_suspend(dev: *mut device) -> c_int {
    static int mt2712_rtc_suspend(struct device *dev)
    {
    let mut wake_status: c_int = 0;
    struct mt2712_rtc *mt2712_rtc = dev_get_drvdata(dev);
    if (device_may_wakeup(dev)) {
    wake_status = enable_irq_wake(mt2712_rtc.irq);
    if (!wake_status)
    mt2712_rtc.irq_wake_enabled = true;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mt2712_rtc_resume(dev: *mut device) -> c_int {
    static int mt2712_rtc_resume(struct device *dev)
    {
    let mut wake_status: c_int = 0;
    struct mt2712_rtc *mt2712_rtc = dev_get_drvdata(dev);
    if (device_may_wakeup(dev) && mt2712_rtc.irq_wake_enabled) {
    wake_status = disable_irq_wake(mt2712_rtc.irq);
    if (!wake_status)
    mt2712_rtc.irq_wake_enabled = false;
    }
    return 0;
    }
    static SIMPLE_DEV_PM_OPS(mt2712_pm_ops, mt2712_rtc_suspend,
    mt2712_rtc_resume);

    static const struct of_device_id mt2712_rtc_of_match[] = {
    { .compatible = "mediatek,mt2712-rtc", },
    { },
    };
    MODULE_DEVICE_TABLE(of, mt2712_rtc_of_match);
    static struct platform_driver mt2712_rtc_driver = {
    .driver = {
    .name = "mt2712-rtc",
    .of_match_table = mt2712_rtc_of_match,

    .pm = &mt2712_pm_ops,

    },
    .probe  = mt2712_rtc_probe,
    };
    module_platform_driver(mt2712_rtc_driver);
    MODULE_DESCRIPTION("MediaTek MT2712 SoC based RTC Driver");
    MODULE_AUTHOR("Ran Bi <ran.bi@mediatek.com>");
    MODULE_LICENSE("GPL");
