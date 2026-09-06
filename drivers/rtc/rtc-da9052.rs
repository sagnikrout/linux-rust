//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-da9052.c
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
// Real time clock driver for DA9052
//
// Copyright(c) 2012 Dialog Semiconductor Ltd.
//
// Author: Dajun Dajun Chen <dajun.chen@diasemi.com>
//

    dev_err(rtc.da9052.dev, "%s: " fmt, __func__, ##__VA_ARGS__)
pub const DA9052_GET_TIME_RETRIES: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct da9052_rtc {
    pub rtc: *mut rtc_device,
    pub da9052: *mut da9052,
}

#[no_mangle]
unsafe extern "C" fn da9052_rtc_enable_alarm(rtc: *mut da9052_rtc, enable: bool) -> c_int {
    static int da9052_rtc_enable_alarm(struct da9052_rtc *rtc, bool enable)
    {
    int ret;
    if (enable) {
    ret = da9052_reg_update(rtc.da9052, DA9052_ALARM_Y_REG,
    DA9052_ALARM_Y_ALARM_ON|DA9052_ALARM_Y_TICK_ON,
    DA9052_ALARM_Y_ALARM_ON);
    if (ret != 0)
    rtc_err(rtc, "Failed to enable ALM: %d\n", ret);
    } else {
    ret = da9052_reg_update(rtc.da9052, DA9052_ALARM_Y_REG,
    DA9052_ALARM_Y_ALARM_ON|DA9052_ALARM_Y_TICK_ON, 0);
    if (ret != 0)
    rtc_err(rtc, "Write error: %d\n", ret);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn da9052_rtc_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t da9052_rtc_irq(int irq, void *data)
    {
    struct da9052_rtc *rtc = data;
    rtc_update_irq(rtc.rtc, 1, RTC_IRQF | RTC_AF);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn da9052_read_alarm(rtc: *mut da9052_rtc, rtc_tm: *mut rtc_time) -> c_int {
    static int da9052_read_alarm(struct da9052_rtc *rtc, struct rtc_time *rtc_tm)
    {
    int ret;
    uint8_t v[2][5];
    let mut idx: c_int = 1;
    let mut timeout: c_int = DA9052_GET_TIME_RETRIES;
    ret = da9052_group_read(rtc.da9052, DA9052_ALARM_MI_REG, 5, &v[0][0]);
    if (ret) {
    rtc_err(rtc, "Failed to group read ALM: %d\n", ret);
    return ret;
    }
    do {
    ret = da9052_group_read(rtc.da9052,
    DA9052_ALARM_MI_REG, 5, &v[idx][0]);
    if (ret) {
    rtc_err(rtc, "Failed to group read ALM: %d\n", ret);
    return ret;
    }
    if (memcmp(&v[0][0], &v[1][0], 5) == 0) {
    rtc_tm.tm_year = (v[0][4] & DA9052_RTC_YEAR) + 100;
    rtc_tm.tm_mon  = (v[0][3] & DA9052_RTC_MONTH) - 1;
    rtc_tm.tm_mday = v[0][2] & DA9052_RTC_DAY;
    rtc_tm.tm_hour = v[0][1] & DA9052_RTC_HOUR;
    rtc_tm.tm_min  = v[0][0] & DA9052_RTC_MIN;
    rtc_tm.tm_sec = 0;
    ret = rtc_valid_tm(rtc_tm);
    return ret;
    }
    idx = (1-idx);
    msleep(20);
    } while (timeout--);
    rtc_err(rtc, "Timed out reading alarm time\n");
    return -EIO;
    }
#[no_mangle]
unsafe extern "C" fn da9052_set_alarm(rtc: *mut da9052_rtc, rtc_tm: *mut rtc_time) -> c_int {
    static int da9052_set_alarm(struct da9052_rtc *rtc, struct rtc_time *rtc_tm)
    {
    struct da9052 *da9052 = rtc.da9052;
    unsigned long alm_time;
    int ret;
    uint8_t v[3];
    alm_time = rtc_tm_to_time64(rtc_tm);
    if (rtc_tm.tm_sec > 0) {
    alm_time += 60 - rtc_tm.tm_sec;
    rtc_time64_to_tm(alm_time, rtc_tm);
    }
    BUG_ON(rtc_tm.tm_sec); /* it will cause repeated irqs if not zero */
    rtc_tm.tm_year -= 100;
    rtc_tm.tm_mon += 1;
    ret = da9052_reg_update(da9052, DA9052_ALARM_MI_REG,
    DA9052_RTC_MIN, rtc_tm.tm_min);
    if (ret != 0) {
    rtc_err(rtc, "Failed to write ALRM MIN: %d\n", ret);
    return ret;
    }
    v[0] = rtc_tm.tm_hour;
    v[1] = rtc_tm.tm_mday;
    v[2] = rtc_tm.tm_mon;
    ret = da9052_group_write(da9052, DA9052_ALARM_H_REG, 3, v);
    if (ret < 0)
    return ret;
    ret = da9052_reg_update(da9052, DA9052_ALARM_Y_REG,
    DA9052_RTC_YEAR, rtc_tm.tm_year);
    if (ret != 0)
    rtc_err(rtc, "Failed to write ALRM YEAR: %d\n", ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn da9052_rtc_get_alarm_status(rtc: *mut da9052_rtc) -> c_int {
    static int da9052_rtc_get_alarm_status(struct da9052_rtc *rtc)
    {
    int ret;
    ret = da9052_reg_read(rtc.da9052, DA9052_ALARM_Y_REG);
    if (ret < 0) {
    rtc_err(rtc, "Failed to read ALM: %d\n", ret);
    return ret;
    }
    return !!(ret&DA9052_ALARM_Y_ALARM_ON);
    }
#[no_mangle]
unsafe extern "C" fn da9052_rtc_read_time(dev: *mut device, rtc_tm: *mut rtc_time) -> c_int {
    static int da9052_rtc_read_time(struct device *dev, struct rtc_time *rtc_tm)
    {
    struct da9052_rtc *rtc = dev_get_drvdata(dev);
    int ret;
    uint8_t v[2][6];
    let mut idx: c_int = 1;
    let mut timeout: c_int = DA9052_GET_TIME_RETRIES;
    ret = da9052_group_read(rtc.da9052, DA9052_COUNT_S_REG, 6, &v[0][0]);
    if (ret) {
    rtc_err(rtc, "Failed to read RTC time : %d\n", ret);
    return ret;
    }
    do {
    ret = da9052_group_read(rtc.da9052,
    DA9052_COUNT_S_REG, 6, &v[idx][0]);
    if (ret) {
    rtc_err(rtc, "Failed to read RTC time : %d\n", ret);
    return ret;
    }
    if (memcmp(&v[0][0], &v[1][0], 6) == 0) {
    rtc_tm.tm_year = (v[0][5] & DA9052_RTC_YEAR) + 100;
    rtc_tm.tm_mon  = (v[0][4] & DA9052_RTC_MONTH) - 1;
    rtc_tm.tm_mday = v[0][3] & DA9052_RTC_DAY;
    rtc_tm.tm_hour = v[0][2] & DA9052_RTC_HOUR;
    rtc_tm.tm_min  = v[0][1] & DA9052_RTC_MIN;
    rtc_tm.tm_sec  = v[0][0] & DA9052_RTC_SEC;
    return 0;
    }
    idx = (1-idx);
    msleep(20);
    } while (timeout--);
    rtc_err(rtc, "Timed out reading time\n");
    return -EIO;
    }
#[no_mangle]
unsafe extern "C" fn da9052_rtc_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int da9052_rtc_set_time(struct device *dev, struct rtc_time *tm)
    {
    struct da9052_rtc *rtc;
    uint8_t v[6];
    int ret;
// DA9052 only has 6 bits for year - to represent 2000-2063
    if ((tm.tm_year < 100) || (tm.tm_year > 163))
    return -EINVAL;
    rtc = dev_get_drvdata(dev);
    v[0] = tm.tm_sec;
    v[1] = tm.tm_min;
    v[2] = tm.tm_hour;
    v[3] = tm.tm_mday;
    v[4] = tm.tm_mon + 1;
    v[5] = tm.tm_year - 100;
    ret = da9052_group_write(rtc.da9052, DA9052_COUNT_S_REG, 6, v);
    if (ret < 0)
    rtc_err(rtc, "failed to set RTC time: %d\n", ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn da9052_rtc_read_alarm(dev: *mut device, alrm: *mut rtc_wkalrm) -> c_int {
    static int da9052_rtc_read_alarm(struct device *dev, struct rtc_wkalrm *alrm)
    {
    int ret;
    struct rtc_time *tm = &alrm.time;
    struct da9052_rtc *rtc = dev_get_drvdata(dev);
    ret = da9052_read_alarm(rtc, tm);
    if (ret < 0) {
    rtc_err(rtc, "failed to read RTC alarm: %d\n", ret);
    return ret;
    }
    alrm.enabled = da9052_rtc_get_alarm_status(rtc);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn da9052_rtc_set_alarm(dev: *mut device, alrm: *mut rtc_wkalrm) -> c_int {
    static int da9052_rtc_set_alarm(struct device *dev, struct rtc_wkalrm *alrm)
    {
    int ret;
    struct rtc_time *tm = &alrm.time;
    struct da9052_rtc *rtc = dev_get_drvdata(dev);
// DA9052 only has 6 bits for year - to represent 2000-2063
    if ((tm.tm_year < 100) || (tm.tm_year > 163))
    return -EINVAL;
    ret = da9052_rtc_enable_alarm(rtc, 0);
    if (ret < 0)
    return ret;
    ret = da9052_set_alarm(rtc, tm);
    if (ret < 0)
    return ret;
    ret = da9052_rtc_enable_alarm(rtc, 1);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn da9052_rtc_alarm_irq_enable(dev: *mut device, enabled: c_uint) -> c_int {
    static int da9052_rtc_alarm_irq_enable(struct device *dev, unsigned int enabled)
    {
    struct da9052_rtc *rtc = dev_get_drvdata(dev);
    return da9052_rtc_enable_alarm(rtc, enabled);
    }
    static const struct rtc_class_ops da9052_rtc_ops = {
    .read_time	= da9052_rtc_read_time,
    .set_time	= da9052_rtc_set_time,
    .read_alarm	= da9052_rtc_read_alarm,
    .set_alarm	= da9052_rtc_set_alarm,
    .alarm_irq_enable = da9052_rtc_alarm_irq_enable,
    };
#[no_mangle]
unsafe extern "C" fn da9052_rtc_probe(pdev: *mut platform_device) -> c_int {
    static int da9052_rtc_probe(struct platform_device *pdev)
    {
    struct da9052_rtc *rtc;
    int ret;
    rtc = devm_kzalloc(&pdev.dev, sizeof(struct da9052_rtc), GFP_KERNEL);
    if (!rtc)
    return -ENOMEM;
    rtc.da9052 = dev_get_drvdata(pdev.dev.parent);
    platform_set_drvdata(pdev, rtc);
    ret = da9052_reg_write(rtc.da9052, DA9052_BBAT_CONT_REG, 0xFE);
    if (ret < 0) {
    rtc_err(rtc,
    "Failed to setup RTC battery charging: %d\n", ret);
    return ret;
    }
    ret = da9052_reg_update(rtc.da9052, DA9052_ALARM_Y_REG,
    DA9052_ALARM_Y_TICK_ON, 0);
    if (ret != 0)
    rtc_err(rtc, "Failed to disable TICKS: %d\n", ret);
    device_init_wakeup(&pdev.dev, true);
    rtc.rtc = devm_rtc_allocate_device(&pdev.dev);
    if (IS_ERR(rtc.rtc))
    return PTR_ERR(rtc.rtc);
    rtc.rtc.ops = &da9052_rtc_ops;
    rtc.rtc.range_min = RTC_TIMESTAMP_BEGIN_2000;
    rtc.rtc.range_max = RTC_TIMESTAMP_END_2063;
    ret = devm_rtc_register_device(rtc.rtc);
    if (ret)
    return ret;
    ret = da9052_request_irq(rtc.da9052, DA9052_IRQ_ALARM, "ALM",
    da9052_rtc_irq, rtc);
    if (ret != 0) {
    rtc_err(rtc, "irq registration failed: %d\n", ret);
    return ret;
    }
    return 0;
    }
    static struct platform_driver da9052_rtc_driver = {
    .probe	= da9052_rtc_probe,
    .driver = {
    .name	= "da9052-rtc",
    },
    };
    module_platform_driver(da9052_rtc_driver);
    MODULE_AUTHOR("Anthony Olech <Anthony.Olech@diasemi.com>");
    MODULE_DESCRIPTION("RTC driver for Dialog DA9052 PMIC");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:da9052-rtc");
