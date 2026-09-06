//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-brcmstb-waketimer.c
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
// Copyright © 2014-2023 Broadcom
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmstb_waketmr {
    pub rtc: *mut rtc_device,
    pub dev: *mut device,
    pub base: *mut void __iomem,
    pub wake_irq: c_uint,
    pub alarm_irq: c_uint,
    pub reboot_notifier: notifier_block,
    pub clk: *mut clk,
    pub rate: u32,
    pub rtc_alarm: c_ulong,
    pub alarm_en: bool,
    pub alarm_expired: bool,
}

pub const BRCMSTB_WKTMR_EVENT: c_uint = 0x00;

pub const BRCMSTB_WKTMR_COUNTER: c_uint = 0x04;
pub const BRCMSTB_WKTMR_ALARM: c_uint = 0x08;
pub const BRCMSTB_WKTMR_PRESCALER: c_uint = 0x0C;
pub const BRCMSTB_WKTMR_PRESCALER_VAL: c_uint = 0x10;
pub const BRCMSTB_WKTMR_DEFAULT_FREQ: c_int = 27000000;
#[no_mangle]
pub unsafe extern "C" fn brcmstb_waketmr_is_pending(timer: *mut brcmstb_waketmr) -> bool {
    static inline bool brcmstb_waketmr_is_pending(struct brcmstb_waketmr *timer)
    {
    u32 reg;
    reg = readl_relaxed(timer.base + BRCMSTB_WKTMR_EVENT);
    return !!(reg & WKTMR_ALARM_EVENT);
    }
#[no_mangle]
pub unsafe extern "C" fn brcmstb_waketmr_clear_alarm(timer: *mut brcmstb_waketmr) {
    static inline void brcmstb_waketmr_clear_alarm(struct brcmstb_waketmr *timer)
    {
    u32 reg;
    if (timer.alarm_en && timer.alarm_irq)
    disable_irq(timer.alarm_irq);
    timer.alarm_en = false;
    reg = readl_relaxed(timer.base + BRCMSTB_WKTMR_COUNTER);
    writel_relaxed(reg - 1, timer.base + BRCMSTB_WKTMR_ALARM);
    writel_relaxed(WKTMR_ALARM_EVENT, timer.base + BRCMSTB_WKTMR_EVENT);
    (void)readl_relaxed(timer.base + BRCMSTB_WKTMR_EVENT);
    if (timer.alarm_expired) {
    timer.alarm_expired = false;
// maintain call balance
    enable_irq(timer.alarm_irq);
    }
    }
    static void brcmstb_waketmr_set_alarm(struct brcmstb_waketmr *timer,
    unsigned int secs)
    {
    unsigned int now;
    brcmstb_waketmr_clear_alarm(timer);
// Make sure we are actually counting in seconds
    writel_relaxed(timer.rate, timer.base + BRCMSTB_WKTMR_PRESCALER);
    writel_relaxed(secs, timer.base + BRCMSTB_WKTMR_ALARM);
    now = readl_relaxed(timer.base + BRCMSTB_WKTMR_COUNTER);
    while ((int)(secs - now) <= 0 &&
    !brcmstb_waketmr_is_pending(timer)) {
    secs = now + 1;
    writel_relaxed(secs, timer.base + BRCMSTB_WKTMR_ALARM);
    now = readl_relaxed(timer.base + BRCMSTB_WKTMR_COUNTER);
    }
    }
#[no_mangle]
unsafe extern "C" fn brcmstb_waketmr_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t brcmstb_waketmr_irq(int irq, void *data)
    {
    struct brcmstb_waketmr *timer = data;
    if (!timer.alarm_irq)
    pm_wakeup_event(timer.dev, 0);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn brcmstb_alarm_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t brcmstb_alarm_irq(int irq, void *data)
    {
    struct brcmstb_waketmr *timer = data;
// Ignore spurious interrupts
    if (!brcmstb_waketmr_is_pending(timer))
    return IRQ_HANDLED;
    if (timer.alarm_en) {
    if (device_may_wakeup(timer.dev)) {
    disable_irq_nosync(irq);
    timer.alarm_expired = true;
    } else {
    writel_relaxed(WKTMR_ALARM_EVENT,
    timer.base + BRCMSTB_WKTMR_EVENT);
    }
    rtc_update_irq(timer.rtc, 1, RTC_IRQF | RTC_AF);
    } else {
    writel_relaxed(WKTMR_ALARM_EVENT,
    timer.base + BRCMSTB_WKTMR_EVENT);
    }
    return IRQ_HANDLED;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wktmr_time {
    pub sec: u32,
    pub pre: u32,
}

    static void wktmr_read(struct brcmstb_waketmr *timer,
    struct wktmr_time *t)
    {
    u32 tmp;
    do {
    t.sec = readl_relaxed(timer.base + BRCMSTB_WKTMR_COUNTER);
    tmp = readl_relaxed(timer.base + BRCMSTB_WKTMR_PRESCALER_VAL);
    } while (tmp >= timer.rate);
    t.pre = timer.rate - tmp;
    }
#[no_mangle]
unsafe extern "C" fn brcmstb_waketmr_prepare_suspend(timer: *mut brcmstb_waketmr) -> c_int {
    static int brcmstb_waketmr_prepare_suspend(struct brcmstb_waketmr *timer)
    {
    struct device *dev = timer.dev;
    int ret;
    if (device_may_wakeup(dev)) {
    ret = enable_irq_wake(timer.wake_irq);
    if (ret) {
    dev_err(dev, "failed to enable wake-up interrupt\n");
    return ret;
    }
    if (timer.alarm_en && timer.alarm_irq) {
    ret = enable_irq_wake(timer.alarm_irq);
    if (ret) {
    dev_err(dev, "failed to enable rtc interrupt\n");
    disable_irq_wake(timer.wake_irq);
    return ret;
    }
    }
    }
    return 0;
    }
// If enabled as a wakeup-source, arm the timer when powering off
    static int brcmstb_waketmr_reboot(struct notifier_block *nb,
    unsigned long action, void *data)
    {
    struct brcmstb_waketmr *timer;
    timer = container_of(nb, struct brcmstb_waketmr, reboot_notifier);
// Set timer for cold boot
    if (action == SYS_POWER_OFF)
    brcmstb_waketmr_prepare_suspend(timer);
    return NOTIFY_DONE;
    }
    static int brcmstb_waketmr_gettime(struct device *dev,
    struct rtc_time *tm)
    {
    struct brcmstb_waketmr *timer = dev_get_drvdata(dev);
    struct wktmr_time now;
    wktmr_read(timer, &now);
    rtc_time64_to_tm(now.sec, tm);
    return 0;
    }
    static int brcmstb_waketmr_settime(struct device *dev,
    struct rtc_time *tm)
    {
    struct brcmstb_waketmr *timer = dev_get_drvdata(dev);
    time64_t sec;
    sec = rtc_tm_to_time64(tm);
    writel_relaxed(sec, timer.base + BRCMSTB_WKTMR_COUNTER);
    return 0;
    }
    static int brcmstb_waketmr_getalarm(struct device *dev,
    struct rtc_wkalrm *alarm)
    {
    struct brcmstb_waketmr *timer = dev_get_drvdata(dev);
    alarm.enabled = timer.alarm_en;
    rtc_time64_to_tm(timer.rtc_alarm, &alarm.time);
    alarm.pending = brcmstb_waketmr_is_pending(timer);
    return 0;
    }
    static int brcmstb_waketmr_alarm_enable(struct device *dev,
    unsigned int enabled)
    {
    struct brcmstb_waketmr *timer = dev_get_drvdata(dev);
    if (enabled && !timer.alarm_en) {
    if ((int)(readl_relaxed(timer.base + BRCMSTB_WKTMR_COUNTER) -
    readl_relaxed(timer.base + BRCMSTB_WKTMR_ALARM)) >= 0 &&
    !brcmstb_waketmr_is_pending(timer))
    return -EINVAL;
    timer.alarm_en = true;
    if (timer.alarm_irq) {
    if (timer.alarm_expired) {
    timer.alarm_expired = false;
// maintain call balance
    enable_irq(timer.alarm_irq);
    }
    enable_irq(timer.alarm_irq);
    }
    } else if (!enabled && timer.alarm_en) {
    if (timer.alarm_irq)
    disable_irq(timer.alarm_irq);
    timer.alarm_en = false;
    }
    return 0;
    }
    static int brcmstb_waketmr_setalarm(struct device *dev,
    struct rtc_wkalrm *alarm)
    {
    struct brcmstb_waketmr *timer = dev_get_drvdata(dev);
    timer.rtc_alarm = rtc_tm_to_time64(&alarm.time);
    brcmstb_waketmr_set_alarm(timer, timer.rtc_alarm);
    return brcmstb_waketmr_alarm_enable(dev, alarm.enabled);
    }
    static const struct rtc_class_ops brcmstb_waketmr_ops = {
    .read_time	= brcmstb_waketmr_gettime,
    .set_time	= brcmstb_waketmr_settime,
    .read_alarm	= brcmstb_waketmr_getalarm,
    .set_alarm	= brcmstb_waketmr_setalarm,
    .alarm_irq_enable = brcmstb_waketmr_alarm_enable,
    };
#[no_mangle]
unsafe extern "C" fn brcmstb_waketmr_probe(pdev: *mut platform_device) -> c_int {
    static int brcmstb_waketmr_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct brcmstb_waketmr *timer;
    int ret;
    timer = devm_kzalloc(dev, sizeof(*timer), GFP_KERNEL);
    if (!timer)
    return -ENOMEM;
    platform_set_drvdata(pdev, timer);
    timer.dev = dev;
    timer.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(timer.base))
    return PTR_ERR(timer.base);
    timer.rtc = devm_rtc_allocate_device(dev);
    if (IS_ERR(timer.rtc))
    return PTR_ERR(timer.rtc);
//
// Set wakeup capability before requesting wakeup interrupt, so we can
// process boot-time "wakeups" (e.g., from S5 soft-off)
//
    device_init_wakeup(dev, true);
    ret = platform_get_irq(pdev, 0);
    if (ret < 0)
    return -ENODEV;
    timer.wake_irq = (unsigned int)ret;
    timer.clk = devm_clk_get(dev, core::ptr::null_mut());
    if (!IS_ERR(timer.clk)) {
    ret = clk_prepare_enable(timer.clk);
    if (ret)
    return ret;
    timer.rate = clk_get_rate(timer.clk);
    if (!timer.rate)
    timer.rate = BRCMSTB_WKTMR_DEFAULT_FREQ;
    } else {
    timer.rate = BRCMSTB_WKTMR_DEFAULT_FREQ;
    timer.clk = core::ptr::null_mut();
    }
    ret = devm_request_irq(dev, timer.wake_irq, brcmstb_waketmr_irq, 0,
    "brcmstb-waketimer", timer);
    if (ret < 0)
    goto err_clk;
    brcmstb_waketmr_clear_alarm(timer);
// Attempt to initialize non-wake irq
    ret = platform_get_irq(pdev, 1);
    if (ret > 0) {
    timer.alarm_irq = (unsigned int)ret;
    ret = devm_request_irq(dev, timer.alarm_irq, brcmstb_alarm_irq,
    IRQF_NO_AUTOEN, "brcmstb-waketimer-rtc",
    timer);
    if (ret < 0)
    timer.alarm_irq = 0;
    }
    timer.reboot_notifier.notifier_call = brcmstb_waketmr_reboot;
    register_reboot_notifier(&timer.reboot_notifier);
    timer.rtc.ops = &brcmstb_waketmr_ops;
    timer.rtc.range_max = U32_MAX;
    ret = devm_rtc_register_device(timer.rtc);
    if (ret)
    goto err_notifier;
    return 0;
    err_notifier:
    unregister_reboot_notifier(&timer.reboot_notifier);
    err_clk:
    clk_disable_unprepare(timer.clk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn brcmstb_waketmr_remove(pdev: *mut platform_device) {
    static void brcmstb_waketmr_remove(struct platform_device *pdev)
    {
    struct brcmstb_waketmr *timer = dev_get_drvdata(&pdev.dev);
    unregister_reboot_notifier(&timer.reboot_notifier);
    clk_disable_unprepare(timer.clk);
    }

#[no_mangle]
unsafe extern "C" fn brcmstb_waketmr_suspend(dev: *mut device) -> c_int {
    static int brcmstb_waketmr_suspend(struct device *dev)
    {
    struct brcmstb_waketmr *timer = dev_get_drvdata(dev);
    return brcmstb_waketmr_prepare_suspend(timer);
    }
#[no_mangle]
unsafe extern "C" fn brcmstb_waketmr_suspend_noirq(dev: *mut device) -> c_int {
    static int brcmstb_waketmr_suspend_noirq(struct device *dev)
    {
    struct brcmstb_waketmr *timer = dev_get_drvdata(dev);
// Catch any alarms occurring prior to noirq
    if (timer.alarm_expired && device_may_wakeup(dev))
    return -EBUSY;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn brcmstb_waketmr_resume(dev: *mut device) -> c_int {
    static int brcmstb_waketmr_resume(struct device *dev)
    {
    struct brcmstb_waketmr *timer = dev_get_drvdata(dev);
    int ret;
    if (!device_may_wakeup(dev))
    return 0;
    ret = disable_irq_wake(timer.wake_irq);
    if (timer.alarm_en && timer.alarm_irq)
    disable_irq_wake(timer.alarm_irq);
    brcmstb_waketmr_clear_alarm(timer);
    return ret;
    }

    static const struct dev_pm_ops brcmstb_waketmr_pm_ops = {
    .suspend	= brcmstb_waketmr_suspend,
    .suspend_noirq	= brcmstb_waketmr_suspend_noirq,
    .resume		= brcmstb_waketmr_resume,
    };
    static const __maybe_unused struct of_device_id brcmstb_waketmr_of_match[] = {
    { .compatible = "brcm,brcmstb-waketimer" },
    { /* sentinel */ },
    };
    MODULE_DEVICE_TABLE(of, brcmstb_waketmr_of_match);
    static struct platform_driver brcmstb_waketmr_driver = {
    .probe			= brcmstb_waketmr_probe,
    .remove			= brcmstb_waketmr_remove,
    .driver = {
    .name		= "brcmstb-waketimer",
    .pm		= &brcmstb_waketmr_pm_ops,
    .of_match_table	= of_match_ptr(brcmstb_waketmr_of_match),
    }
    };
    module_platform_driver(brcmstb_waketmr_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Brian Norris");
    MODULE_AUTHOR("Markus Mayer");
    MODULE_AUTHOR("Doug Berger");
    MODULE_DESCRIPTION("Wake-up timer driver for STB chips");
