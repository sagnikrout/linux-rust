//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-asm9260.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) 2016 Oleksij Rempel <linux@rempel-privat.de>
//

// Miscellaneous registers
// Interrupt Location Register
pub const HW_ILR: c_uint = 0x00;

// Clock Control Register
pub const HW_CCR: c_uint = 0x08;
// Calibration counter disable

// Reset internal oscillator divider

// Clock Enable

// Counter Increment Interrupt Register
pub const HW_CIIR: c_uint = 0x0C;

// Alarm Mask Register
pub const HW_AMR: c_uint = 0x10;

pub const BM_AMR_OFF: c_uint = 0xff;
// Consolidated time registers
pub const HW_CTIME0: c_uint = 0x14;
pub const BM_CTIME0_DOW_S: c_int = 24;
pub const BM_CTIME0_DOW_M: c_uint = 0x7;
pub const BM_CTIME0_HOUR_S: c_int = 16;
pub const BM_CTIME0_HOUR_M: c_uint = 0x1f;
pub const BM_CTIME0_MIN_S: c_int = 8;
pub const BM_CTIME0_MIN_M: c_uint = 0x3f;
pub const BM_CTIME0_SEC_S: c_int = 0;
pub const BM_CTIME0_SEC_M: c_uint = 0x3f;
pub const HW_CTIME1: c_uint = 0x18;
pub const BM_CTIME1_YEAR_S: c_int = 16;
pub const BM_CTIME1_YEAR_M: c_uint = 0xfff;
pub const BM_CTIME1_MON_S: c_int = 8;
pub const BM_CTIME1_MON_M: c_uint = 0xf;
pub const BM_CTIME1_DOM_S: c_int = 0;
pub const BM_CTIME1_DOM_M: c_uint = 0x1f;
pub const HW_CTIME2: c_uint = 0x1C;
pub const BM_CTIME2_DOY_S: c_int = 0;
pub const BM_CTIME2_DOY_M: c_uint = 0xfff;
// Time counter registers
pub const HW_SEC: c_uint = 0x20;
pub const HW_MIN: c_uint = 0x24;
pub const HW_HOUR: c_uint = 0x28;
pub const HW_DOM: c_uint = 0x2C;
pub const HW_DOW: c_uint = 0x30;
pub const HW_DOY: c_uint = 0x34;
pub const HW_MONTH: c_uint = 0x38;
pub const HW_YEAR: c_uint = 0x3C;
pub const HW_CALIBRATION: c_uint = 0x40;

pub const BM_CALVAL_M: c_uint = 0x1ffff;
// General purpose registers
pub const HW_GPREG0: c_uint = 0x44;
pub const HW_GPREG1: c_uint = 0x48;
pub const HW_GPREG2: c_uint = 0x4C;
pub const HW_GPREG3: c_uint = 0x50;
pub const HW_GPREG4: c_uint = 0x54;
// Alarm register group
pub const HW_ALSEC: c_uint = 0x60;
pub const HW_ALMIN: c_uint = 0x64;
pub const HW_ALHOUR: c_uint = 0x68;
pub const HW_ALDOM: c_uint = 0x6C;
pub const HW_ALDOW: c_uint = 0x70;
pub const HW_ALDOY: c_uint = 0x74;
pub const HW_ALMON: c_uint = 0x78;
pub const HW_ALYEAR: c_uint = 0x7C;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct asm9260_rtc_priv {
    pub dev: *mut device,
    pub iobase: *mut void __iomem,
    pub rtc: *mut rtc_device,
    pub clk: *mut clk,
}

#[no_mangle]
unsafe extern "C" fn asm9260_rtc_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t asm9260_rtc_irq(int irq, void *dev_id)
    {
    struct asm9260_rtc_priv *priv = dev_id;
    u32 isr;
    let mut events: c_ulong = 0;
    rtc_lock(priv.rtc);
    isr = ioread32(priv.iobase + HW_CIIR);
    if (!isr) {
    rtc_unlock(priv.rtc);
    return IRQ_NONE;
    }
    iowrite32(0, priv.iobase + HW_CIIR);
    rtc_unlock(priv.rtc);
    events |= RTC_AF | RTC_IRQF;
    rtc_update_irq(priv.rtc, 1, events);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn asm9260_rtc_read_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int asm9260_rtc_read_time(struct device *dev, struct rtc_time *tm)
    {
    struct asm9260_rtc_priv *priv = dev_get_drvdata(dev);
    u32 ctime0, ctime1, ctime2;
    ctime0 = ioread32(priv.iobase + HW_CTIME0);
    ctime1 = ioread32(priv.iobase + HW_CTIME1);
    ctime2 = ioread32(priv.iobase + HW_CTIME2);
    if (ctime1 != ioread32(priv.iobase + HW_CTIME1)) {
//
// woops, counter flipped right now. Now we are safe
// to reread.
//
    ctime0 = ioread32(priv.iobase + HW_CTIME0);
    ctime1 = ioread32(priv.iobase + HW_CTIME1);
    ctime2 = ioread32(priv.iobase + HW_CTIME2);
    }
    tm.tm_sec  = (ctime0 >> BM_CTIME0_SEC_S)  & BM_CTIME0_SEC_M;
    tm.tm_min  = (ctime0 >> BM_CTIME0_MIN_S)  & BM_CTIME0_MIN_M;
    tm.tm_hour = (ctime0 >> BM_CTIME0_HOUR_S) & BM_CTIME0_HOUR_M;
    tm.tm_wday = (ctime0 >> BM_CTIME0_DOW_S)  & BM_CTIME0_DOW_M;
    tm.tm_mday = (ctime1 >> BM_CTIME1_DOM_S)  & BM_CTIME1_DOM_M;
    tm.tm_mon  = (ctime1 >> BM_CTIME1_MON_S)  & BM_CTIME1_MON_M;
    tm.tm_year = (ctime1 >> BM_CTIME1_YEAR_S) & BM_CTIME1_YEAR_M;
    tm.tm_yday = (ctime2 >> BM_CTIME2_DOY_S)  & BM_CTIME2_DOY_M;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn asm9260_rtc_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int asm9260_rtc_set_time(struct device *dev, struct rtc_time *tm)
    {
    struct asm9260_rtc_priv *priv = dev_get_drvdata(dev);
//
// make sure SEC counter will not flip other counter on write time,
// real value will be written at the enf of sequence.
//
    iowrite32(0, priv.iobase + HW_SEC);
    iowrite32(tm.tm_year, priv.iobase + HW_YEAR);
    iowrite32(tm.tm_mon,  priv.iobase + HW_MONTH);
    iowrite32(tm.tm_mday, priv.iobase + HW_DOM);
    iowrite32(tm.tm_wday, priv.iobase + HW_DOW);
    iowrite32(tm.tm_yday, priv.iobase + HW_DOY);
    iowrite32(tm.tm_hour, priv.iobase + HW_HOUR);
    iowrite32(tm.tm_min,  priv.iobase + HW_MIN);
    iowrite32(tm.tm_sec,  priv.iobase + HW_SEC);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn asm9260_rtc_read_alarm(dev: *mut device, alrm: *mut rtc_wkalrm) -> c_int {
    static int asm9260_rtc_read_alarm(struct device *dev, struct rtc_wkalrm *alrm)
    {
    struct asm9260_rtc_priv *priv = dev_get_drvdata(dev);
    alrm.time.tm_year = ioread32(priv.iobase + HW_ALYEAR);
    alrm.time.tm_mon  = ioread32(priv.iobase + HW_ALMON);
    alrm.time.tm_mday = ioread32(priv.iobase + HW_ALDOM);
    alrm.time.tm_wday = ioread32(priv.iobase + HW_ALDOW);
    alrm.time.tm_yday = ioread32(priv.iobase + HW_ALDOY);
    alrm.time.tm_hour = ioread32(priv.iobase + HW_ALHOUR);
    alrm.time.tm_min  = ioread32(priv.iobase + HW_ALMIN);
    alrm.time.tm_sec  = ioread32(priv.iobase + HW_ALSEC);
    alrm.enabled = ioread32(priv.iobase + HW_AMR) ? 1 : 0;
    alrm.pending = ioread32(priv.iobase + HW_CIIR) ? 1 : 0;
    return rtc_valid_tm(&alrm.time);
    }
#[no_mangle]
unsafe extern "C" fn asm9260_rtc_set_alarm(dev: *mut device, alrm: *mut rtc_wkalrm) -> c_int {
    static int asm9260_rtc_set_alarm(struct device *dev, struct rtc_wkalrm *alrm)
    {
    struct asm9260_rtc_priv *priv = dev_get_drvdata(dev);
    iowrite32(alrm.time.tm_year, priv.iobase + HW_ALYEAR);
    iowrite32(alrm.time.tm_mon,  priv.iobase + HW_ALMON);
    iowrite32(alrm.time.tm_mday, priv.iobase + HW_ALDOM);
    iowrite32(alrm.time.tm_wday, priv.iobase + HW_ALDOW);
    iowrite32(alrm.time.tm_yday, priv.iobase + HW_ALDOY);
    iowrite32(alrm.time.tm_hour, priv.iobase + HW_ALHOUR);
    iowrite32(alrm.time.tm_min,  priv.iobase + HW_ALMIN);
    iowrite32(alrm.time.tm_sec,  priv.iobase + HW_ALSEC);
    iowrite32(alrm.enabled ? 0 : BM_AMR_OFF, priv.iobase + HW_AMR);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn asm9260_alarm_irq_enable(dev: *mut device, enabled: c_uint) -> c_int {
    static int asm9260_alarm_irq_enable(struct device *dev, unsigned int enabled)
    {
    struct asm9260_rtc_priv *priv = dev_get_drvdata(dev);
    iowrite32(enabled ? 0 : BM_AMR_OFF, priv.iobase + HW_AMR);
    return 0;
    }
    static const struct rtc_class_ops asm9260_rtc_ops = {
    .read_time		= asm9260_rtc_read_time,
    .set_time		= asm9260_rtc_set_time,
    .read_alarm		= asm9260_rtc_read_alarm,
    .set_alarm		= asm9260_rtc_set_alarm,
    .alarm_irq_enable	= asm9260_alarm_irq_enable,
    };
#[no_mangle]
unsafe extern "C" fn asm9260_rtc_probe(pdev: *mut platform_device) -> c_int {
    static int asm9260_rtc_probe(struct platform_device *pdev)
    {
    struct asm9260_rtc_priv *priv;
    struct device *dev = &pdev.dev;
    int irq_alarm, ret;
    u32 ccr;
    priv = devm_kzalloc(dev, sizeof(struct asm9260_rtc_priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.dev = &pdev.dev;
    platform_set_drvdata(pdev, priv);
    irq_alarm = platform_get_irq(pdev, 0);
    if (irq_alarm < 0)
    return irq_alarm;
    priv.iobase = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.iobase))
    return PTR_ERR(priv.iobase);
    priv.clk = devm_clk_get(dev, "ahb");
    if (IS_ERR(priv.clk))
    return PTR_ERR(priv.clk);
    ret = clk_prepare_enable(priv.clk);
    if (ret) {
    dev_err(dev, "Failed to enable clk!\n");
    return ret;
    }
    ccr = ioread32(priv.iobase + HW_CCR);
// if dev is not enabled, reset it
    if ((ccr & (BM_CLKEN | BM_CTCRST)) != BM_CLKEN) {
    iowrite32(BM_CTCRST, priv.iobase + HW_CCR);
    ccr = 0;
    }
    iowrite32(BM_CLKEN | ccr, priv.iobase + HW_CCR);
    iowrite32(0, priv.iobase + HW_CIIR);
    iowrite32(BM_AMR_OFF, priv.iobase + HW_AMR);
    priv.rtc = devm_rtc_device_register(dev, dev_name(dev),
    &asm9260_rtc_ops, THIS_MODULE);
    if (IS_ERR(priv.rtc)) {
    ret = PTR_ERR(priv.rtc);
    dev_err(dev, "Failed to register RTC device: %d\n", ret);
    goto err_return;
    }
    ret = devm_request_threaded_irq(dev, irq_alarm, core::ptr::null_mut(),
    asm9260_rtc_irq, IRQF_ONESHOT,
    dev_name(dev), priv);
    if (ret < 0) {
    dev_err(dev, "can't get irq %i, err %d\n",
    irq_alarm, ret);
    goto err_return;
    }
    return 0;
    err_return:
    clk_disable_unprepare(priv.clk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn asm9260_rtc_remove(pdev: *mut platform_device) {
    static void asm9260_rtc_remove(struct platform_device *pdev)
    {
    struct asm9260_rtc_priv *priv = platform_get_drvdata(pdev);
// Disable alarm matching
    iowrite32(BM_AMR_OFF, priv.iobase + HW_AMR);
    clk_disable_unprepare(priv.clk);
    }
    static const struct of_device_id asm9260_dt_ids[] = {
    { .compatible = "alphascale,asm9260-rtc", },
    {}
    };
    MODULE_DEVICE_TABLE(of, asm9260_dt_ids);
    static struct platform_driver asm9260_rtc_driver = {
    .probe		= asm9260_rtc_probe,
    .remove		= asm9260_rtc_remove,
    .driver		= {
    .name	= "asm9260-rtc",
    .of_match_table = asm9260_dt_ids,
    },
    };
    module_platform_driver(asm9260_rtc_driver);
    MODULE_AUTHOR("Oleksij Rempel <linux@rempel-privat.de>");
    MODULE_DESCRIPTION("Alphascale asm9260 SoC Realtime Clock Driver (RTC)");
    MODULE_LICENSE("GPL");
