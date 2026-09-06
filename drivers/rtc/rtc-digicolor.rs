//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-digicolor.c
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
// Real Time Clock driver for Conexant Digicolor
//
// Copyright (C) 2015 Paradox Innovation Ltd.
//
// Author: Baruch Siach <baruch@tkos.co.il>
//

pub const DC_RTC_CONTROL: c_uint = 0x0;
pub const DC_RTC_TIME: c_uint = 0x8;
pub const DC_RTC_REFERENCE: c_uint = 0xc;
pub const DC_RTC_ALARM: c_uint = 0x10;
pub const DC_RTC_INTFLAG_CLEAR: c_uint = 0x14;
pub const DC_RTC_INTENABLE: c_uint = 0x16;
pub const DC_RTC_CMD_MASK: c_uint = 0xf;

pub const CMD_NOP: c_int = 0;
pub const CMD_RESET: c_int = 1;
pub const CMD_WRITE: c_int = 3;
pub const CMD_READ: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_rtc {
    pub rtc_dev: *mut rtc_device,
    pub regs: *mut void __iomem,
}

#[no_mangle]
unsafe extern "C" fn dc_rtc_cmds(rtc: *mut dc_rtc, cmds: *const u8, len: c_int) -> c_int {
    static int dc_rtc_cmds(struct dc_rtc *rtc, const u8 *cmds, int len)
    {
    u8 val;
    int i, ret;
    for (i = 0; i < len; i++) {
    writeb_relaxed((cmds[i] & DC_RTC_CMD_MASK) | DC_RTC_GO_BUSY,
    rtc.regs + DC_RTC_CONTROL);
    ret = readb_relaxed_poll_timeout(
    rtc.regs + DC_RTC_CONTROL, val,
    !(val & DC_RTC_GO_BUSY), CMD_DELAY_US, CMD_TIMEOUT_US);
    if (ret < 0)
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dc_rtc_read(rtc: *mut dc_rtc, val: *mut c_ulong) -> c_int {
    static int dc_rtc_read(struct dc_rtc *rtc, unsigned long *val)
    {
    static const u8 read_cmds[] = {CMD_READ, CMD_NOP};
    u32 reference, time1, time2;
    int ret;
    ret = dc_rtc_cmds(rtc, read_cmds, ARRAY_SIZE(read_cmds));
    if (ret < 0)
    return ret;
    reference = readl_relaxed(rtc.regs + DC_RTC_REFERENCE);
    time1 = readl_relaxed(rtc.regs + DC_RTC_TIME);
// Read twice to ensure consistency
    while (1) {
    time2 = readl_relaxed(rtc.regs + DC_RTC_TIME);
    if (time1 == time2)
    break;
    time1 = time2;
    }
// val = reference + time1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dc_rtc_write(rtc: *mut dc_rtc, val: u32) -> c_int {
    static int dc_rtc_write(struct dc_rtc *rtc, u32 val)
    {
    static const u8 write_cmds[] = {CMD_WRITE, CMD_NOP, CMD_RESET, CMD_NOP};
    writel_relaxed(val, rtc.regs + DC_RTC_REFERENCE);
    return dc_rtc_cmds(rtc, write_cmds, ARRAY_SIZE(write_cmds));
    }
#[no_mangle]
unsafe extern "C" fn dc_rtc_read_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int dc_rtc_read_time(struct device *dev, struct rtc_time *tm)
    {
    struct dc_rtc *rtc = dev_get_drvdata(dev);
    unsigned long now;
    int ret;
    ret = dc_rtc_read(rtc, &now);
    if (ret < 0)
    return ret;
    rtc_time64_to_tm(now, tm);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dc_rtc_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int dc_rtc_set_time(struct device *dev, struct rtc_time *tm)
    {
    struct dc_rtc *rtc = dev_get_drvdata(dev);
    return dc_rtc_write(rtc, rtc_tm_to_time64(tm));
    }
#[no_mangle]
unsafe extern "C" fn dc_rtc_read_alarm(dev: *mut device, alarm: *mut rtc_wkalrm) -> c_int {
    static int dc_rtc_read_alarm(struct device *dev, struct rtc_wkalrm *alarm)
    {
    struct dc_rtc *rtc = dev_get_drvdata(dev);
    u32 alarm_reg, reference;
    unsigned long now;
    int ret;
    alarm_reg = readl_relaxed(rtc.regs + DC_RTC_ALARM);
    reference = readl_relaxed(rtc.regs + DC_RTC_REFERENCE);
    rtc_time64_to_tm(reference + alarm_reg, &alarm.time);
    ret = dc_rtc_read(rtc, &now);
    if (ret < 0)
    return ret;
    alarm.pending = alarm_reg + reference > now;
    alarm.enabled = readl_relaxed(rtc.regs + DC_RTC_INTENABLE);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dc_rtc_set_alarm(dev: *mut device, alarm: *mut rtc_wkalrm) -> c_int {
    static int dc_rtc_set_alarm(struct device *dev, struct rtc_wkalrm *alarm)
    {
    struct dc_rtc *rtc = dev_get_drvdata(dev);
    time64_t alarm_time;
    u32 reference;
    alarm_time = rtc_tm_to_time64(&alarm.time);
    reference = readl_relaxed(rtc.regs + DC_RTC_REFERENCE);
    writel_relaxed(alarm_time - reference, rtc.regs + DC_RTC_ALARM);
    writeb_relaxed(!!alarm.enabled, rtc.regs + DC_RTC_INTENABLE);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dc_rtc_alarm_irq_enable(dev: *mut device, enabled: c_uint) -> c_int {
    static int dc_rtc_alarm_irq_enable(struct device *dev, unsigned int enabled)
    {
    struct dc_rtc *rtc = dev_get_drvdata(dev);
    writeb_relaxed(!!enabled, rtc.regs + DC_RTC_INTENABLE);
    return 0;
    }
    static const struct rtc_class_ops dc_rtc_ops = {
    .read_time		= dc_rtc_read_time,
    .set_time		= dc_rtc_set_time,
    .read_alarm		= dc_rtc_read_alarm,
    .set_alarm		= dc_rtc_set_alarm,
    .alarm_irq_enable	= dc_rtc_alarm_irq_enable,
    };
#[no_mangle]
unsafe extern "C" fn dc_rtc_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t dc_rtc_irq(int irq, void *dev_id)
    {
    struct dc_rtc *rtc = dev_id;
    writeb_relaxed(1, rtc.regs + DC_RTC_INTFLAG_CLEAR);
    rtc_update_irq(rtc.rtc_dev, 1, RTC_AF | RTC_IRQF);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn dc_rtc_probe(pdev: *mut platform_device) -> int __init {
    static int __init dc_rtc_probe(struct platform_device *pdev)
    {
    struct dc_rtc *rtc;
    int irq, ret;
    rtc = devm_kzalloc(&pdev.dev, sizeof(*rtc), GFP_KERNEL);
    if (!rtc)
    return -ENOMEM;
    rtc.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(rtc.regs))
    return PTR_ERR(rtc.regs);
    rtc.rtc_dev = devm_rtc_allocate_device(&pdev.dev);
    if (IS_ERR(rtc.rtc_dev))
    return PTR_ERR(rtc.rtc_dev);
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    ret = devm_request_irq(&pdev.dev, irq, dc_rtc_irq, 0, pdev.name, rtc);
    if (ret < 0)
    return ret;
    platform_set_drvdata(pdev, rtc);
    rtc.rtc_dev.ops = &dc_rtc_ops;
    rtc.rtc_dev.range_max = U32_MAX;
    return devm_rtc_register_device(rtc.rtc_dev);
    }
    static const __maybe_unused struct of_device_id dc_dt_ids[] = {
    { .compatible = "cnxt,cx92755-rtc" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, dc_dt_ids);
    static struct platform_driver dc_rtc_driver = {
    .driver = {
    .name = "digicolor_rtc",
    .of_match_table = of_match_ptr(dc_dt_ids),
    },
    };
    module_platform_driver_probe(dc_rtc_driver, dc_rtc_probe);
    MODULE_AUTHOR("Baruch Siach <baruch@tkos.co.il>");
    MODULE_DESCRIPTION("Conexant Digicolor Realtime Clock Driver (RTC)");
    MODULE_LICENSE("GPL");
