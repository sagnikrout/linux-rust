//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-spear.c
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
// drivers/rtc/rtc-spear.c
//
// Copyright (C) 2010 ST Microelectronics
// Rajeev Kumar<rajeev-dlh.kumar@st.com>
//

// RTC registers
pub const TIME_REG: c_uint = 0x00;
pub const DATE_REG: c_uint = 0x04;
pub const ALARM_TIME_REG: c_uint = 0x08;
pub const ALARM_DATE_REG: c_uint = 0x0C;
pub const CTRL_REG: c_uint = 0x10;
pub const STATUS_REG: c_uint = 0x14;
// TIME_REG & ALARM_TIME_REG

// DATE_REG & ALARM_DATE_REG

// MASK SHIFT TIME_REG & ALARM_TIME_REG
pub const SECOND_SHIFT: c_uint = 0x00		/* seconds units */;
pub const MINUTE_SHIFT: c_uint = 0x08		/* minutes units position */;
pub const HOUR_SHIFT: c_uint = 0x10		/* hours units position */;
pub const MDAY_SHIFT: c_uint = 0x00		/* Month day shift */;
pub const MONTH_SHIFT: c_uint = 0x08		/* Month shift */;
pub const YEAR_SHIFT: c_uint = 0x10		/* Year shift */;
pub const SECOND_MASK: c_uint = 0x7F;
pub const MIN_MASK: c_uint = 0x7F;
pub const HOUR_MASK: c_uint = 0x3F;
pub const DAY_MASK: c_uint = 0x3F;
pub const MONTH_MASK: c_uint = 0x7F;
pub const YEAR_MASK: c_uint = 0xFFFF;
// date reg equal to time reg, for debug only

// STATUS_REG

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spear_rtc_config {
    pub rtc: *mut rtc_device,
    pub clk: *mut clk,
    pub lock: spinlock_t,
    pub ioaddr: *mut void __iomem,
    pub irq_wake: c_uint,
}

#[no_mangle]
pub unsafe extern "C" fn spear_rtc_clear_interrupt(config: *mut spear_rtc_config) {
    static inline void spear_rtc_clear_interrupt(struct spear_rtc_config *config)
    {
    unsigned int val;
    unsigned long flags;
    spin_lock_irqsave(&config.lock, flags);
    val = readl(config.ioaddr + STATUS_REG);
    val |= RTC_INT_MASK;
    writel(val, config.ioaddr + STATUS_REG);
    spin_unlock_irqrestore(&config.lock, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn spear_rtc_enable_interrupt(config: *mut spear_rtc_config) {
    static inline void spear_rtc_enable_interrupt(struct spear_rtc_config *config)
    {
    unsigned int val;
    val = readl(config.ioaddr + CTRL_REG);
    if (!(val & INT_ENABLE)) {
    spear_rtc_clear_interrupt(config);
    val |= INT_ENABLE;
    writel(val, config.ioaddr + CTRL_REG);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn spear_rtc_disable_interrupt(config: *mut spear_rtc_config) {
    static inline void spear_rtc_disable_interrupt(struct spear_rtc_config *config)
    {
    unsigned int val;
    val = readl(config.ioaddr + CTRL_REG);
    if (val & INT_ENABLE) {
    val &= ~INT_ENABLE;
    writel(val, config.ioaddr + CTRL_REG);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn is_write_complete(config: *mut spear_rtc_config) -> c_int {
    static inline int is_write_complete(struct spear_rtc_config *config)
    {
    let mut ret: c_int = 0;
    unsigned long flags;
    spin_lock_irqsave(&config.lock, flags);
    if ((readl(config.ioaddr + STATUS_REG)) & STATUS_FAIL)
    ret = -EIO;
    spin_unlock_irqrestore(&config.lock, flags);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rtc_wait_not_busy(config: *mut spear_rtc_config) {
    static void rtc_wait_not_busy(struct spear_rtc_config *config)
    {
    int status, count = 0;
    unsigned long flags;
// Assuming BUSY may stay active for 80 msec)
    for (count = 0; count < 80; count++) {
    spin_lock_irqsave(&config.lock, flags);
    status = readl(config.ioaddr + STATUS_REG);
    spin_unlock_irqrestore(&config.lock, flags);
    if ((status & STATUS_BUSY) == 0)
    break;
// check status busy, after each msec
    msleep(1);
    }
    }
#[no_mangle]
unsafe extern "C" fn spear_rtc_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t spear_rtc_irq(int irq, void *dev_id)
    {
    struct spear_rtc_config *config = dev_id;
    let mut events: c_ulong = 0;
    unsigned int irq_data;
    spin_lock(&config.lock);
    irq_data = readl(config.ioaddr + STATUS_REG);
    spin_unlock(&config.lock);
    if ((irq_data & RTC_INT_MASK)) {
    spear_rtc_clear_interrupt(config);
    events = RTC_IRQF | RTC_AF;
    rtc_update_irq(config.rtc, 1, events);
    return IRQ_HANDLED;
    } else
    return IRQ_NONE;
    }
#[no_mangle]
unsafe extern "C" fn tm2bcd(tm: *mut rtc_time) {
    static void tm2bcd(struct rtc_time *tm)
    {
    tm.tm_sec = bin2bcd(tm.tm_sec);
    tm.tm_min = bin2bcd(tm.tm_min);
    tm.tm_hour = bin2bcd(tm.tm_hour);
    tm.tm_mday = bin2bcd(tm.tm_mday);
    tm.tm_mon = bin2bcd(tm.tm_mon + 1);
    tm.tm_year = bin2bcd(tm.tm_year);
    }
#[no_mangle]
unsafe extern "C" fn bcd2tm(tm: *mut rtc_time) {
    static void bcd2tm(struct rtc_time *tm)
    {
    tm.tm_sec = bcd2bin(tm.tm_sec);
    tm.tm_min = bcd2bin(tm.tm_min);
    tm.tm_hour = bcd2bin(tm.tm_hour);
    tm.tm_mday = bcd2bin(tm.tm_mday);
    tm.tm_mon = bcd2bin(tm.tm_mon) - 1;
// epoch == 1900
    tm.tm_year = bcd2bin(tm.tm_year);
    }
//
// spear_rtc_read_time - set the time
// @dev: rtc device in use
// @tm: holds date and time
//
// This function read time and date. On success it will return 0
// otherwise -ve error is returned.
//
#[no_mangle]
unsafe extern "C" fn spear_rtc_read_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int spear_rtc_read_time(struct device *dev, struct rtc_time *tm)
    {
    struct spear_rtc_config *config = dev_get_drvdata(dev);
    unsigned int time, date;
// we don't report wday/yday/isdst ...
    rtc_wait_not_busy(config);
    do {
    time = readl(config.ioaddr + TIME_REG);
    date = readl(config.ioaddr + DATE_REG);
    } while (time == readl(config.ioaddr + TIME_REG));
    tm.tm_sec = (time >> SECOND_SHIFT) & SECOND_MASK;
    tm.tm_min = (time >> MINUTE_SHIFT) & MIN_MASK;
    tm.tm_hour = (time >> HOUR_SHIFT) & HOUR_MASK;
    tm.tm_mday = (date >> MDAY_SHIFT) & DAY_MASK;
    tm.tm_mon = (date >> MONTH_SHIFT) & MONTH_MASK;
    tm.tm_year = (date >> YEAR_SHIFT) & YEAR_MASK;
    bcd2tm(tm);
    return 0;
    }
//
// spear_rtc_set_time - set the time
// @dev: rtc device in use
// @tm: holds date and time
//
// This function set time and date. On success it will return 0
// otherwise -ve error is returned.
//
#[no_mangle]
unsafe extern "C" fn spear_rtc_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int spear_rtc_set_time(struct device *dev, struct rtc_time *tm)
    {
    struct spear_rtc_config *config = dev_get_drvdata(dev);
    unsigned int time, date;
    tm2bcd(tm);
    rtc_wait_not_busy(config);
    time = (tm.tm_sec << SECOND_SHIFT) | (tm.tm_min << MINUTE_SHIFT) |
    (tm.tm_hour << HOUR_SHIFT);
    date = (tm.tm_mday << MDAY_SHIFT) | (tm.tm_mon << MONTH_SHIFT) |
    (tm.tm_year << YEAR_SHIFT);
    writel(time, config.ioaddr + TIME_REG);
    writel(date, config.ioaddr + DATE_REG);
    return is_write_complete(config);
    }
//
// spear_rtc_read_alarm - read the alarm time
// @dev: rtc device in use
// @alm: holds alarm date and time
//
// This function read alarm time and date. On success it will return 0
// otherwise -ve error is returned.
//
#[no_mangle]
unsafe extern "C" fn spear_rtc_read_alarm(dev: *mut device, alm: *mut rtc_wkalrm) -> c_int {
    static int spear_rtc_read_alarm(struct device *dev, struct rtc_wkalrm *alm)
    {
    struct spear_rtc_config *config = dev_get_drvdata(dev);
    unsigned int time, date;
    rtc_wait_not_busy(config);
    time = readl(config.ioaddr + ALARM_TIME_REG);
    date = readl(config.ioaddr + ALARM_DATE_REG);
    alm.time.tm_sec = (time >> SECOND_SHIFT) & SECOND_MASK;
    alm.time.tm_min = (time >> MINUTE_SHIFT) & MIN_MASK;
    alm.time.tm_hour = (time >> HOUR_SHIFT) & HOUR_MASK;
    alm.time.tm_mday = (date >> MDAY_SHIFT) & DAY_MASK;
    alm.time.tm_mon = (date >> MONTH_SHIFT) & MONTH_MASK;
    alm.time.tm_year = (date >> YEAR_SHIFT) & YEAR_MASK;
    bcd2tm(&alm.time);
    alm.enabled = readl(config.ioaddr + CTRL_REG) & INT_ENABLE;
    return 0;
    }
//
// spear_rtc_set_alarm - set the alarm time
// @dev: rtc device in use
// @alm: holds alarm date and time
//
// This function set alarm time and date. On success it will return 0
// otherwise -ve error is returned.
//
#[no_mangle]
unsafe extern "C" fn spear_rtc_set_alarm(dev: *mut device, alm: *mut rtc_wkalrm) -> c_int {
    static int spear_rtc_set_alarm(struct device *dev, struct rtc_wkalrm *alm)
    {
    struct spear_rtc_config *config = dev_get_drvdata(dev);
    unsigned int time, date;
    int err;
    tm2bcd(&alm.time);
    rtc_wait_not_busy(config);
    time = (alm.time.tm_sec << SECOND_SHIFT) | (alm.time.tm_min <<
    MINUTE_SHIFT) |	(alm.time.tm_hour << HOUR_SHIFT);
    date = (alm.time.tm_mday << MDAY_SHIFT) | (alm.time.tm_mon <<
    MONTH_SHIFT) | (alm.time.tm_year << YEAR_SHIFT);
    writel(time, config.ioaddr + ALARM_TIME_REG);
    writel(date, config.ioaddr + ALARM_DATE_REG);
    err = is_write_complete(config);
    if (err < 0)
    return err;
    if (alm.enabled)
    spear_rtc_enable_interrupt(config);
    else
    spear_rtc_disable_interrupt(config);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn spear_alarm_irq_enable(dev: *mut device, enabled: c_uint) -> c_int {
    static int spear_alarm_irq_enable(struct device *dev, unsigned int enabled)
    {
    struct spear_rtc_config *config = dev_get_drvdata(dev);
    let mut ret: c_int = 0;
    spear_rtc_clear_interrupt(config);
    switch (enabled) {
    case 0:
// alarm off
    spear_rtc_disable_interrupt(config);
    break;
    case 1:
// alarm on
    spear_rtc_enable_interrupt(config);
    break;
    default:
    ret = -EINVAL;
    break;
    }
    return ret;
    }
    static const struct rtc_class_ops spear_rtc_ops = {
    .read_time = spear_rtc_read_time,
    .set_time = spear_rtc_set_time,
    .read_alarm = spear_rtc_read_alarm,
    .set_alarm = spear_rtc_set_alarm,
    .alarm_irq_enable = spear_alarm_irq_enable,
    };
#[no_mangle]
unsafe extern "C" fn spear_rtc_probe(pdev: *mut platform_device) -> c_int {
    static int spear_rtc_probe(struct platform_device *pdev)
    {
    struct spear_rtc_config *config;
    let mut status: c_int = 0;
    int irq;
    config = devm_kzalloc(&pdev.dev, sizeof(*config), GFP_KERNEL);
    if (!config)
    return -ENOMEM;
    config.rtc = devm_rtc_allocate_device(&pdev.dev);
    if (IS_ERR(config.rtc))
    return PTR_ERR(config.rtc);
// alarm irqs
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    status = devm_request_irq(&pdev.dev, irq, spear_rtc_irq, 0, pdev.name,
    config);
    if (status) {
    dev_err(&pdev.dev, "Alarm interrupt IRQ%d already claimed\n",
    irq);
    return status;
    }
    config.ioaddr = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(config.ioaddr))
    return PTR_ERR(config.ioaddr);
    config.clk = devm_clk_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(config.clk))
    return PTR_ERR(config.clk);
    status = clk_prepare_enable(config.clk);
    if (status < 0)
    return status;
    spin_lock_init(&config.lock);
    platform_set_drvdata(pdev, config);
    config.rtc.ops = &spear_rtc_ops;
    config.rtc.range_min = RTC_TIMESTAMP_BEGIN_0000;
    config.rtc.range_max = RTC_TIMESTAMP_END_9999;
    status = devm_rtc_register_device(config.rtc);
    if (status)
    goto err_disable_clock;
    if (!device_can_wakeup(&pdev.dev))
    device_init_wakeup(&pdev.dev, true);
    return 0;
    err_disable_clock:
    clk_disable_unprepare(config.clk);
    return status;
    }
#[no_mangle]
unsafe extern "C" fn spear_rtc_remove(pdev: *mut platform_device) {
    static void spear_rtc_remove(struct platform_device *pdev)
    {
    struct spear_rtc_config *config = platform_get_drvdata(pdev);
    spear_rtc_disable_interrupt(config);
    clk_disable_unprepare(config.clk);
    device_init_wakeup(&pdev.dev, false);
    }

#[no_mangle]
unsafe extern "C" fn spear_rtc_suspend(dev: *mut device) -> c_int {
    static int spear_rtc_suspend(struct device *dev)
    {
    struct platform_device *pdev = to_platform_device(dev);
    struct spear_rtc_config *config = platform_get_drvdata(pdev);
    int irq;
    irq = platform_get_irq(pdev, 0);
    if (device_may_wakeup(&pdev.dev)) {
    if (!enable_irq_wake(irq))
    config.irq_wake = 1;
    } else {
    spear_rtc_disable_interrupt(config);
    clk_disable(config.clk);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn spear_rtc_resume(dev: *mut device) -> c_int {
    static int spear_rtc_resume(struct device *dev)
    {
    struct platform_device *pdev = to_platform_device(dev);
    struct spear_rtc_config *config = platform_get_drvdata(pdev);
    int irq;
    irq = platform_get_irq(pdev, 0);
    if (device_may_wakeup(&pdev.dev)) {
    if (config.irq_wake) {
    disable_irq_wake(irq);
    config.irq_wake = 0;
    }
    } else {
    clk_enable(config.clk);
    spear_rtc_enable_interrupt(config);
    }
    return 0;
    }

    static SIMPLE_DEV_PM_OPS(spear_rtc_pm_ops, spear_rtc_suspend, spear_rtc_resume);
#[no_mangle]
unsafe extern "C" fn spear_rtc_shutdown(pdev: *mut platform_device) {
    static void spear_rtc_shutdown(struct platform_device *pdev)
    {
    struct spear_rtc_config *config = platform_get_drvdata(pdev);
    spear_rtc_disable_interrupt(config);
    clk_disable(config.clk);
    }

    static const struct of_device_id spear_rtc_id_table[] = {
    { .compatible = "st,spear600-rtc" },
    {}
    };
    MODULE_DEVICE_TABLE(of, spear_rtc_id_table);

    static struct platform_driver spear_rtc_driver = {
    .probe = spear_rtc_probe,
    .remove = spear_rtc_remove,
    .shutdown = spear_rtc_shutdown,
    .driver = {
    .name = "rtc-spear",
    .pm = &spear_rtc_pm_ops,
    .of_match_table = of_match_ptr(spear_rtc_id_table),
    },
    };
    module_platform_driver(spear_rtc_driver);
    MODULE_ALIAS("platform:rtc-spear");
    MODULE_AUTHOR("Rajeev Kumar <rajeev-dlh.kumar@st.com>");
    MODULE_DESCRIPTION("ST SPEAr Realtime Clock Driver (RTC)");
    MODULE_LICENSE("GPL");
