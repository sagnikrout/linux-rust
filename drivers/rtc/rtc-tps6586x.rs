//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-tps6586x.c
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
// rtc-tps6586x.c: RTC driver for TI PMIC TPS6586X
//
// Copyright (c) 2012, NVIDIA Corporation.
//
// Author: Laxman Dewangan <ldewangan@nvidia.com>
//

pub const RTC_CTRL: c_uint = 0xc0;

pub const CL_SEL_POS: c_int = 1;
pub const RTC_ALARM1_HI: c_uint = 0xc1;
pub const RTC_COUNT4: c_uint = 0xc6;
// start a PMU RTC access by reading the register prior to the RTC_COUNT4
pub const RTC_COUNT4_DUMMYREAD: c_uint = 0xc5;
// only 14-bits width in second
pub const ALM1_VALID_RANGE_IN_SEC: c_uint = 0x3FFF;
pub const TPS6586X_RTC_CL_SEL_1_5PF: c_uint = 0x0;
pub const TPS6586X_RTC_CL_SEL_6_5PF: c_uint = 0x1;
pub const TPS6586X_RTC_CL_SEL_7_5PF: c_uint = 0x2;
pub const TPS6586X_RTC_CL_SEL_12_5PF: c_uint = 0x3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tps6586x_rtc {
    pub dev: *mut device,
    pub rtc: *mut rtc_device,
    pub irq: c_int,
    pub irq_en: bool,
}

    static inline struct device *to_tps6586x_dev(struct device *dev)
    {
    return dev.parent;
    }
#[no_mangle]
unsafe extern "C" fn tps6586x_rtc_read_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int tps6586x_rtc_read_time(struct device *dev, struct rtc_time *tm)
    {
    struct device *tps_dev = to_tps6586x_dev(dev);
    let mut ticks: c_ulonglong = 0;
    time64_t seconds;
    u8 buff[6];
    int ret;
    int i;
    ret = tps6586x_reads(tps_dev, RTC_COUNT4_DUMMYREAD, sizeof(buff), buff);
    if (ret < 0) {
    dev_err(dev, "read counter failed with err %d\n", ret);
    return ret;
    }
    for (i = 1; i < sizeof(buff); i++) {
    ticks <<= 8;
    ticks |= buff[i];
    }
    seconds = ticks >> 10;
    rtc_time64_to_tm(seconds, tm);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tps6586x_rtc_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int tps6586x_rtc_set_time(struct device *dev, struct rtc_time *tm)
    {
    struct device *tps_dev = to_tps6586x_dev(dev);
    unsigned long long ticks;
    time64_t seconds;
    u8 buff[5];
    int ret;
    seconds = rtc_tm_to_time64(tm);
    ticks = (unsigned long long)seconds << 10;
    buff[0] = (ticks >> 32) & 0xff;
    buff[1] = (ticks >> 24) & 0xff;
    buff[2] = (ticks >> 16) & 0xff;
    buff[3] = (ticks >> 8) & 0xff;
    buff[4] = ticks & 0xff;
// Disable RTC before changing time
    ret = tps6586x_clr_bits(tps_dev, RTC_CTRL, RTC_ENABLE);
    if (ret < 0) {
    dev_err(dev, "failed to clear RTC_ENABLE\n");
    return ret;
    }
    ret = tps6586x_writes(tps_dev, RTC_COUNT4, sizeof(buff), buff);
    if (ret < 0) {
    dev_err(dev, "failed to program new time\n");
    return ret;
    }
// Enable RTC
    ret = tps6586x_set_bits(tps_dev, RTC_CTRL, RTC_ENABLE);
    if (ret < 0) {
    dev_err(dev, "failed to set RTC_ENABLE\n");
    return ret;
    }
    return 0;
    }
    static int tps6586x_rtc_alarm_irq_enable(struct device *dev,
    unsigned int enabled)
    {
    struct tps6586x_rtc *rtc = dev_get_drvdata(dev);
    if (enabled && !rtc.irq_en) {
    enable_irq(rtc.irq);
    rtc.irq_en = true;
    } else if (!enabled && rtc.irq_en)  {
    disable_irq(rtc.irq);
    rtc.irq_en = false;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tps6586x_rtc_set_alarm(dev: *mut device, alrm: *mut rtc_wkalrm) -> c_int {
    static int tps6586x_rtc_set_alarm(struct device *dev, struct rtc_wkalrm *alrm)
    {
    struct device *tps_dev = to_tps6586x_dev(dev);
    time64_t seconds;
    unsigned long ticks;
    unsigned long rtc_current_time;
    let mut rticks: c_ulonglong = 0;
    u8 buff[3];
    u8 rbuff[6];
    int ret;
    int i;
    seconds = rtc_tm_to_time64(&alrm.time);
    ret = tps6586x_rtc_alarm_irq_enable(dev, alrm.enabled);
    if (ret < 0) {
    dev_err(dev, "can't set alarm irq, err %d\n", ret);
    return ret;
    }
    ret = tps6586x_reads(tps_dev, RTC_COUNT4_DUMMYREAD,
    sizeof(rbuff), rbuff);
    if (ret < 0) {
    dev_err(dev, "read counter failed with err %d\n", ret);
    return ret;
    }
    for (i = 1; i < sizeof(rbuff); i++) {
    rticks <<= 8;
    rticks |= rbuff[i];
    }
    rtc_current_time = rticks >> 10;
    if ((seconds - rtc_current_time) > ALM1_VALID_RANGE_IN_SEC)
    seconds = rtc_current_time - 1;
    ticks = (unsigned long long)seconds << 10;
    buff[0] = (ticks >> 16) & 0xff;
    buff[1] = (ticks >> 8) & 0xff;
    buff[2] = ticks & 0xff;
    ret = tps6586x_writes(tps_dev, RTC_ALARM1_HI, sizeof(buff), buff);
    if (ret)
    dev_err(dev, "programming alarm failed with err %d\n", ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn tps6586x_rtc_read_alarm(dev: *mut device, alrm: *mut rtc_wkalrm) -> c_int {
    static int tps6586x_rtc_read_alarm(struct device *dev, struct rtc_wkalrm *alrm)
    {
    struct device *tps_dev = to_tps6586x_dev(dev);
    unsigned long ticks;
    time64_t seconds;
    u8 buff[3];
    int ret;
    ret = tps6586x_reads(tps_dev, RTC_ALARM1_HI, sizeof(buff), buff);
    if (ret) {
    dev_err(dev, "read RTC_ALARM1_HI failed with err %d\n", ret);
    return ret;
    }
    ticks = (buff[0] << 16) | (buff[1] << 8) | buff[2];
    seconds = ticks >> 10;
    rtc_time64_to_tm(seconds, &alrm.time);
    return 0;
    }
    static const struct rtc_class_ops tps6586x_rtc_ops = {
    .read_time	= tps6586x_rtc_read_time,
    .set_time	= tps6586x_rtc_set_time,
    .set_alarm	= tps6586x_rtc_set_alarm,
    .read_alarm	= tps6586x_rtc_read_alarm,
    .alarm_irq_enable = tps6586x_rtc_alarm_irq_enable,
    };
#[no_mangle]
unsafe extern "C" fn tps6586x_rtc_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t tps6586x_rtc_irq(int irq, void *data)
    {
    struct tps6586x_rtc *rtc = data;
    rtc_update_irq(rtc.rtc, 1, RTC_IRQF | RTC_AF);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn tps6586x_rtc_probe(pdev: *mut platform_device) -> c_int {
    static int tps6586x_rtc_probe(struct platform_device *pdev)
    {
    struct device *tps_dev = to_tps6586x_dev(&pdev.dev);
    struct tps6586x_rtc *rtc;
    int ret;
    rtc = devm_kzalloc(&pdev.dev, sizeof(*rtc), GFP_KERNEL);
    if (!rtc)
    return -ENOMEM;
    rtc.dev = &pdev.dev;
    rtc.irq = platform_get_irq(pdev, 0);
// 1 kHz tick mode, enable tick counting
    ret = tps6586x_update(tps_dev, RTC_CTRL,
    RTC_ENABLE | OSC_SRC_SEL |
    ((TPS6586X_RTC_CL_SEL_1_5PF << CL_SEL_POS) & CL_SEL_MASK),
    RTC_ENABLE | OSC_SRC_SEL | PRE_BYPASS | CL_SEL_MASK);
    if (ret < 0) {
    dev_err(&pdev.dev, "unable to start counter\n");
    return ret;
    }
    device_init_wakeup(&pdev.dev, true);
    platform_set_drvdata(pdev, rtc);
    rtc.rtc = devm_rtc_allocate_device(&pdev.dev);
    if (IS_ERR(rtc.rtc)) {
    ret = PTR_ERR(rtc.rtc);
    goto fail_rtc_register;
    }
    rtc.rtc.ops = &tps6586x_rtc_ops;
    rtc.rtc.range_max = (1ULL << 30) - 1; /* 30-bit seconds */
    rtc.rtc.alarm_offset_max = ALM1_VALID_RANGE_IN_SEC;
    rtc.rtc.start_secs = mktime64(2009, 1, 1, 0, 0, 0);
    rtc.rtc.set_start_time = true;
    irq_set_status_flags(rtc.irq, IRQ_NOAUTOEN);
    ret = devm_request_threaded_irq(&pdev.dev, rtc.irq, core::ptr::null_mut(),
    tps6586x_rtc_irq,
    IRQF_ONESHOT,
    dev_name(&pdev.dev), rtc);
    if (ret < 0) {
    dev_err(&pdev.dev, "request IRQ(%d) failed with ret %d\n",
    rtc.irq, ret);
    goto fail_rtc_register;
    }
    ret = devm_rtc_register_device(rtc.rtc);
    if (ret)
    goto fail_rtc_register;
    return 0;
    fail_rtc_register:
    tps6586x_update(tps_dev, RTC_CTRL, 0,
    RTC_ENABLE | OSC_SRC_SEL | PRE_BYPASS | CL_SEL_MASK);
    return ret;
    };
#[no_mangle]
unsafe extern "C" fn tps6586x_rtc_remove(pdev: *mut platform_device) {
    static void tps6586x_rtc_remove(struct platform_device *pdev)
    {
    struct device *tps_dev = to_tps6586x_dev(&pdev.dev);
    tps6586x_update(tps_dev, RTC_CTRL, 0,
    RTC_ENABLE | OSC_SRC_SEL | PRE_BYPASS | CL_SEL_MASK);
    }

#[no_mangle]
unsafe extern "C" fn tps6586x_rtc_suspend(dev: *mut device) -> c_int {
    static int tps6586x_rtc_suspend(struct device *dev)
    {
    struct tps6586x_rtc *rtc = dev_get_drvdata(dev);
    if (device_may_wakeup(dev))
    enable_irq_wake(rtc.irq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tps6586x_rtc_resume(dev: *mut device) -> c_int {
    static int tps6586x_rtc_resume(struct device *dev)
    {
    struct tps6586x_rtc *rtc = dev_get_drvdata(dev);
    if (device_may_wakeup(dev))
    disable_irq_wake(rtc.irq);
    return 0;
    }

    static SIMPLE_DEV_PM_OPS(tps6586x_pm_ops, tps6586x_rtc_suspend,
    tps6586x_rtc_resume);
    static struct platform_driver tps6586x_rtc_driver = {
    .driver	= {
    .name	= "tps6586x-rtc",
    .pm	= &tps6586x_pm_ops,
    },
    .probe	= tps6586x_rtc_probe,
    .remove = tps6586x_rtc_remove,
    };
    module_platform_driver(tps6586x_rtc_driver);
    MODULE_ALIAS("platform:tps6586x-rtc");
    MODULE_DESCRIPTION("TI TPS6586x RTC driver");
    MODULE_AUTHOR("Laxman dewangan <ldewangan@nvidia.com>");
    MODULE_LICENSE("GPL v2");
