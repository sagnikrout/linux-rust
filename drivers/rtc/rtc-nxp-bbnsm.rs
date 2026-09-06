//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-nxp-bbnsm.c
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
// Copyright 2022 NXP.

pub const BBNSM_CTRL: c_uint = 0x8;
pub const BBNSM_INT_EN: c_uint = 0x10;
pub const BBNSM_EVENTS: c_uint = 0x14;
pub const BBNSM_RTC_LS: c_uint = 0x40;
pub const BBNSM_RTC_MS: c_uint = 0x44;
pub const BBNSM_TA: c_uint = 0x50;
pub const RTC_EN: c_uint = 0x2;
pub const RTC_EN_MSK: c_uint = 0x3;

pub const RTC_INT_EN: c_uint = 0x2;

pub const CNTR_TO_SECS_SH: c_int = 15;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bbnsm_rtc {
    pub rtc: *mut rtc_device,
    pub regmap: *mut regmap,
    pub irq: c_int,
    pub clk: *mut clk,
}

#[no_mangle]
unsafe extern "C" fn bbnsm_read_counter(bbnsm: *mut bbnsm_rtc) -> u32 {
    static u32 bbnsm_read_counter(struct bbnsm_rtc *bbnsm)
    {
    u32 rtc_msb, rtc_lsb;
    let mut timeout: c_uint = 100;
    u32 time;
    let mut tmp: u32 = 0;
    do {
    time = tmp;
// read the msb
    regmap_read(bbnsm.regmap, BBNSM_RTC_MS, &rtc_msb);
// read the lsb
    regmap_read(bbnsm.regmap, BBNSM_RTC_LS, &rtc_lsb);
// convert to seconds
    tmp = (rtc_msb << 17) | (rtc_lsb >> 15);
    } while (tmp != time && --timeout);
    return time;
    }
#[no_mangle]
unsafe extern "C" fn bbnsm_rtc_read_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int bbnsm_rtc_read_time(struct device *dev, struct rtc_time *tm)
    {
    struct bbnsm_rtc *bbnsm = dev_get_drvdata(dev);
    unsigned long time;
    u32 val;
    regmap_read(bbnsm.regmap, BBNSM_CTRL, &val);
    if ((val & RTC_EN_MSK) != RTC_EN)
    return -EINVAL;
    time = bbnsm_read_counter(bbnsm);
    rtc_time64_to_tm(time, tm);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bbnsm_rtc_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int bbnsm_rtc_set_time(struct device *dev, struct rtc_time *tm)
    {
    struct bbnsm_rtc *bbnsm = dev_get_drvdata(dev);
    let mut time: c_ulong = rtc_tm_to_time64(tm);
// disable the RTC first
    regmap_update_bits(bbnsm.regmap, BBNSM_CTRL, RTC_EN_MSK, 0);
// write the 32bit sec time to 47 bit timer counter, leaving 15 LSBs blank
    regmap_write(bbnsm.regmap, BBNSM_RTC_LS, time << CNTR_TO_SECS_SH);
    regmap_write(bbnsm.regmap, BBNSM_RTC_MS, time >> (32 - CNTR_TO_SECS_SH));
// Enable the RTC again
    regmap_update_bits(bbnsm.regmap, BBNSM_CTRL, RTC_EN_MSK, RTC_EN);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bbnsm_rtc_read_alarm(dev: *mut device, alrm: *mut rtc_wkalrm) -> c_int {
    static int bbnsm_rtc_read_alarm(struct device *dev, struct rtc_wkalrm *alrm)
    {
    struct bbnsm_rtc *bbnsm = dev_get_drvdata(dev);
    u32 bbnsm_events, bbnsm_ta;
    regmap_read(bbnsm.regmap, BBNSM_TA, &bbnsm_ta);
    rtc_time64_to_tm(bbnsm_ta, &alrm.time);
    regmap_read(bbnsm.regmap, BBNSM_EVENTS, &bbnsm_events);
    alrm.pending = (bbnsm_events & BBNSM_EVENT_TA) ? 1 : 0;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bbnsm_rtc_alarm_irq_enable(dev: *mut device, enable: c_uint) -> c_int {
    static int bbnsm_rtc_alarm_irq_enable(struct device *dev, unsigned int enable)
    {
    struct bbnsm_rtc *bbnsm = dev_get_drvdata(dev);
// enable the alarm event
    regmap_update_bits(bbnsm.regmap, BBNSM_CTRL, TA_EN_MSK, enable ? TA_EN : TA_DIS);
// enable the alarm interrupt
    regmap_update_bits(bbnsm.regmap, BBNSM_INT_EN, TA_EN_MSK, enable ? TA_EN : TA_DIS);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bbnsm_rtc_set_alarm(dev: *mut device, alrm: *mut rtc_wkalrm) -> c_int {
    static int bbnsm_rtc_set_alarm(struct device *dev, struct rtc_wkalrm *alrm)
    {
    struct bbnsm_rtc *bbnsm = dev_get_drvdata(dev);
    let mut time: c_ulong = rtc_tm_to_time64(&alrm.time);
// disable the alarm
    regmap_update_bits(bbnsm.regmap, BBNSM_CTRL, TA_EN, TA_EN);
// write the seconds to TA
    regmap_write(bbnsm.regmap, BBNSM_TA, time);
    return bbnsm_rtc_alarm_irq_enable(dev, alrm.enabled);
    }
    static const struct rtc_class_ops bbnsm_rtc_ops = {
    .read_time = bbnsm_rtc_read_time,
    .set_time = bbnsm_rtc_set_time,
    .read_alarm = bbnsm_rtc_read_alarm,
    .set_alarm = bbnsm_rtc_set_alarm,
    .alarm_irq_enable = bbnsm_rtc_alarm_irq_enable,
    };
#[no_mangle]
unsafe extern "C" fn bbnsm_rtc_irq_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t bbnsm_rtc_irq_handler(int irq, void *dev_id)
    {
    struct device *dev = dev_id;
    struct bbnsm_rtc  *bbnsm = dev_get_drvdata(dev);
    u32 val;
    regmap_read(bbnsm.regmap, BBNSM_EVENTS, &val);
    if (val & BBNSM_EVENT_TA) {
    bbnsm_rtc_alarm_irq_enable(dev, false);
// clear the alarm event
    regmap_write_bits(bbnsm.regmap, BBNSM_EVENTS, TA_EN_MSK, BBNSM_EVENT_TA);
    rtc_update_irq(bbnsm.rtc, 1, RTC_AF | RTC_IRQF);
    return IRQ_HANDLED;
    }
    return IRQ_NONE;
    }
#[no_mangle]
unsafe extern "C" fn bbnsm_rtc_probe(pdev: *mut platform_device) -> c_int {
    static int bbnsm_rtc_probe(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    struct bbnsm_rtc *bbnsm;
    int ret;
    bbnsm = devm_kzalloc(&pdev.dev, sizeof(*bbnsm), GFP_KERNEL);
    if (!bbnsm)
    return -ENOMEM;
    bbnsm.rtc = devm_rtc_allocate_device(&pdev.dev);
    if (IS_ERR(bbnsm.rtc))
    return PTR_ERR(bbnsm.rtc);
    bbnsm.regmap = syscon_node_to_regmap(np.parent);
    if (IS_ERR(bbnsm.regmap)) {
    dev_dbg(&pdev.dev, "bbnsm get regmap failed\n");
    return PTR_ERR(bbnsm.regmap);
    }
    bbnsm.irq = platform_get_irq(pdev, 0);
    if (bbnsm.irq < 0)
    return bbnsm.irq;
    platform_set_drvdata(pdev, bbnsm);
// clear all the pending events
    regmap_write(bbnsm.regmap, BBNSM_EVENTS, 0x7A);
    ret = devm_device_init_wakeup(&pdev.dev);
    if (ret)
    dev_err(&pdev.dev, "failed to init wakeup, %d\n", ret);
    ret = devm_pm_set_wake_irq(&pdev.dev, bbnsm.irq);
    if (ret)
    dev_err(&pdev.dev, "failed to set wake irq, %d\n", ret);
    ret = devm_request_irq(&pdev.dev, bbnsm.irq, bbnsm_rtc_irq_handler,
    IRQF_SHARED, "rtc alarm", &pdev.dev);
    if (ret) {
    dev_err(&pdev.dev, "failed to request irq %d: %d\n",
    bbnsm.irq, ret);
    return ret;
    }
    bbnsm.rtc.ops = &bbnsm_rtc_ops;
    bbnsm.rtc.range_max = U32_MAX;
    return devm_rtc_register_device(bbnsm.rtc);
    }
    static const struct of_device_id bbnsm_dt_ids[] = {
    { .compatible = "nxp,imx93-bbnsm-rtc" },
    { /* sentinel */ },
    };
    MODULE_DEVICE_TABLE(of, bbnsm_dt_ids);
    static struct platform_driver bbnsm_rtc_driver = {
    .driver = {
    .name = "bbnsm_rtc",
    .of_match_table = bbnsm_dt_ids,
    },
    .probe = bbnsm_rtc_probe,
    };
    module_platform_driver(bbnsm_rtc_driver);
    MODULE_AUTHOR("Jacky Bai <ping.bai@nxp.com>");
    MODULE_DESCRIPTION("NXP BBNSM RTC Driver");
    MODULE_LICENSE("GPL");
