//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-ab8500.c
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
// Copyright (C) ST-Ericsson SA 2010
//
// Author: Virupax Sadashivpetimath <virupax.sadashivpetimath@stericsson.com>
//
// RTC clock driver for the RTC part of the AB8500 Power management chip.
// Based on RTC clock driver for the AB3100 Analog Baseband Chip by
// Linus Walleij <linus.walleij@stericsson.com>
//

pub const AB8500_RTC_SOFF_STAT_REG: c_uint = 0x00;
pub const AB8500_RTC_CC_CONF_REG: c_uint = 0x01;
pub const AB8500_RTC_READ_REQ_REG: c_uint = 0x02;
pub const AB8500_RTC_WATCH_TSECMID_REG: c_uint = 0x03;
pub const AB8500_RTC_WATCH_TSECHI_REG: c_uint = 0x04;
pub const AB8500_RTC_WATCH_TMIN_LOW_REG: c_uint = 0x05;
pub const AB8500_RTC_WATCH_TMIN_MID_REG: c_uint = 0x06;
pub const AB8500_RTC_WATCH_TMIN_HI_REG: c_uint = 0x07;
pub const AB8500_RTC_ALRM_MIN_LOW_REG: c_uint = 0x08;
pub const AB8500_RTC_ALRM_MIN_MID_REG: c_uint = 0x09;
pub const AB8500_RTC_ALRM_MIN_HI_REG: c_uint = 0x0A;
pub const AB8500_RTC_STAT_REG: c_uint = 0x0B;
pub const AB8500_RTC_BKUP_CHG_REG: c_uint = 0x0C;
pub const AB8500_RTC_FORCE_BKUP_REG: c_uint = 0x0D;
pub const AB8500_RTC_CALIB_REG: c_uint = 0x0E;
pub const AB8500_RTC_SWITCH_STAT_REG: c_uint = 0x0F;
// RtcReadRequest bits
pub const RTC_READ_REQUEST: c_uint = 0x01;
pub const RTC_WRITE_REQUEST: c_uint = 0x02;
// RtcCtrl bits
pub const RTC_ALARM_ENA: c_uint = 0x04;
pub const RTC_STATUS_DATA: c_uint = 0x01;

    static const u8 ab8500_rtc_time_regs[] = {
    AB8500_RTC_WATCH_TMIN_HI_REG, AB8500_RTC_WATCH_TMIN_MID_REG,
    AB8500_RTC_WATCH_TMIN_LOW_REG, AB8500_RTC_WATCH_TSECHI_REG,
    AB8500_RTC_WATCH_TSECMID_REG
    };
    static const u8 ab8500_rtc_alarm_regs[] = {
    AB8500_RTC_ALRM_MIN_HI_REG, AB8500_RTC_ALRM_MIN_MID_REG,
    AB8500_RTC_ALRM_MIN_LOW_REG
    };
#[no_mangle]
unsafe extern "C" fn ab8500_rtc_read_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int ab8500_rtc_read_time(struct device *dev, struct rtc_time *tm)
    {
    let mut timeout: c_ulong = jiffies + HZ;
    int retval, i;
    unsigned long mins, secs;
    unsigned char buf[ARRAY_SIZE(ab8500_rtc_time_regs)];
    u8 value;
// Request a data read
    retval = abx500_set_register_interruptible(dev,
    AB8500_RTC, AB8500_RTC_READ_REQ_REG, RTC_READ_REQUEST);
    if (retval < 0)
    return retval;
// Wait for some cycles after enabling the rtc read in ab8500
    while (time_before(jiffies, timeout)) {
    retval = abx500_get_register_interruptible(dev,
    AB8500_RTC, AB8500_RTC_READ_REQ_REG, &value);
    if (retval < 0)
    return retval;
    if (!(value & RTC_READ_REQUEST))
    break;
    usleep_range(1000, 5000);
    }
// Read the Watchtime registers
    for (i = 0; i < ARRAY_SIZE(ab8500_rtc_time_regs); i++) {
    retval = abx500_get_register_interruptible(dev,
    AB8500_RTC, ab8500_rtc_time_regs[i], &value);
    if (retval < 0)
    return retval;
    buf[i] = value;
    }
    mins = (buf[0] << 16) | (buf[1] << 8) | buf[2];
    secs =	(buf[3] << 8) | buf[4];
    secs =	secs / COUNTS_PER_SEC;
    secs =	secs + (mins * 60);
    rtc_time64_to_tm(secs, tm);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ab8500_rtc_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int ab8500_rtc_set_time(struct device *dev, struct rtc_time *tm)
    {
    int retval, i;
    unsigned char buf[ARRAY_SIZE(ab8500_rtc_time_regs)];
    unsigned long no_secs, no_mins, secs = 0;
    secs = rtc_tm_to_time64(tm);
    no_mins = secs / 60;
    no_secs = secs % 60;
// Make the seconds count as per the RTC resolution
    no_secs = no_secs * COUNTS_PER_SEC;
    buf[4] = no_secs & 0xFF;
    buf[3] = (no_secs >> 8) & 0xFF;
    buf[2] = no_mins & 0xFF;
    buf[1] = (no_mins >> 8) & 0xFF;
    buf[0] = (no_mins >> 16) & 0xFF;
    for (i = 0; i < ARRAY_SIZE(ab8500_rtc_time_regs); i++) {
    retval = abx500_set_register_interruptible(dev, AB8500_RTC,
    ab8500_rtc_time_regs[i], buf[i]);
    if (retval < 0)
    return retval;
    }
// Request a data write
    return abx500_set_register_interruptible(dev, AB8500_RTC,
    AB8500_RTC_READ_REQ_REG, RTC_WRITE_REQUEST);
    }
#[no_mangle]
unsafe extern "C" fn ab8500_rtc_read_alarm(dev: *mut device, alarm: *mut rtc_wkalrm) -> c_int {
    static int ab8500_rtc_read_alarm(struct device *dev, struct rtc_wkalrm *alarm)
    {
    int retval, i;
    u8 rtc_ctrl, value;
    unsigned char buf[ARRAY_SIZE(ab8500_rtc_alarm_regs)];
    unsigned long secs, mins;
// Check if the alarm is enabled or not
    retval = abx500_get_register_interruptible(dev, AB8500_RTC,
    AB8500_RTC_STAT_REG, &rtc_ctrl);
    if (retval < 0)
    return retval;
    if (rtc_ctrl & RTC_ALARM_ENA)
    alarm.enabled = 1;
    else
    alarm.enabled = 0;
    alarm.pending = 0;
    for (i = 0; i < ARRAY_SIZE(ab8500_rtc_alarm_regs); i++) {
    retval = abx500_get_register_interruptible(dev, AB8500_RTC,
    ab8500_rtc_alarm_regs[i], &value);
    if (retval < 0)
    return retval;
    buf[i] = value;
    }
    mins = (buf[0] << 16) | (buf[1] << 8) | (buf[2]);
    secs = mins * 60;
    rtc_time64_to_tm(secs, &alarm.time);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ab8500_rtc_irq_enable(dev: *mut device, enabled: c_uint) -> c_int {
    static int ab8500_rtc_irq_enable(struct device *dev, unsigned int enabled)
    {
    return abx500_mask_and_set_register_interruptible(dev, AB8500_RTC,
    AB8500_RTC_STAT_REG, RTC_ALARM_ENA,
    enabled ? RTC_ALARM_ENA : 0);
    }
#[no_mangle]
unsafe extern "C" fn ab8500_rtc_set_alarm(dev: *mut device, alarm: *mut rtc_wkalrm) -> c_int {
    static int ab8500_rtc_set_alarm(struct device *dev, struct rtc_wkalrm *alarm)
    {
    int retval, i;
    unsigned char buf[ARRAY_SIZE(ab8500_rtc_alarm_regs)];
    unsigned long mins;
    mins = (unsigned long)rtc_tm_to_time64(&alarm.time) / 60;
    buf[2] = mins & 0xFF;
    buf[1] = (mins >> 8) & 0xFF;
    buf[0] = (mins >> 16) & 0xFF;
// Set the alarm time
    for (i = 0; i < ARRAY_SIZE(ab8500_rtc_alarm_regs); i++) {
    retval = abx500_set_register_interruptible(dev, AB8500_RTC,
    ab8500_rtc_alarm_regs[i], buf[i]);
    if (retval < 0)
    return retval;
    }
    return ab8500_rtc_irq_enable(dev, alarm.enabled);
    }
#[no_mangle]
unsafe extern "C" fn ab8500_rtc_set_calibration(dev: *mut device, calibration: c_int) -> c_int {
    static int ab8500_rtc_set_calibration(struct device *dev, int calibration)
    {
    int retval;
    let mut rtccal: u8 = 0;
//
// Check that the calibration value (which is in units of 0.5
// parts-per-million) is in the AB8500's range for RtcCalibration
// register. -128 (0x80) is not permitted because the AB8500 uses
// a sign-bit rather than two's complement, so 0x80 is just another
// representation of zero.
//
    if ((calibration < -127) || (calibration > 127)) {
    dev_err(dev, "RtcCalibration value outside permitted range\n");
    return -EINVAL;
    }
//
// The AB8500 uses sign (in bit7) and magnitude (in bits0-7)
// so need to convert to this sort of representation before writing
// into RtcCalibration register...
//
    if (calibration >= 0)
    rtccal = 0x7F & calibration;
    else
    rtccal = ~(calibration - 1) | 0x80;
    retval = abx500_set_register_interruptible(dev, AB8500_RTC,
    AB8500_RTC_CALIB_REG, rtccal);
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn ab8500_rtc_get_calibration(dev: *mut device, calibration: *mut c_int) -> c_int {
    static int ab8500_rtc_get_calibration(struct device *dev, int *calibration)
    {
    int retval;
    let mut rtccal: u8 = 0;
    retval =  abx500_get_register_interruptible(dev, AB8500_RTC,
    AB8500_RTC_CALIB_REG, &rtccal);
    if (retval >= 0) {
//
// The AB8500 uses sign (in bit7) and magnitude (in bits0-7)
// so need to convert value from RtcCalibration register into
// a two's complement signed value...
//
    if (rtccal & 0x80)
// calibration = 0 - (rtccal & 0x7F);
    else
// calibration = 0x7F & rtccal;
    }
    return retval;
    }
    static ssize_t ab8500_sysfs_store_rtc_calibration(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    int retval;
    let mut calibration: c_int = 0;
    if (sscanf(buf, " %i ", &calibration) != 1) {
    dev_err(dev, "Failed to store RTC calibration attribute\n");
    return -EINVAL;
    }
    retval = ab8500_rtc_set_calibration(dev, calibration);
    return retval ? retval : count;
    }
    static ssize_t ab8500_sysfs_show_rtc_calibration(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    let mut retval: c_int = 0;
    let mut calibration: c_int = 0;
    retval = ab8500_rtc_get_calibration(dev, &calibration);
    if (retval < 0) {
    dev_err(dev, "Failed to read RTC calibration attribute\n");
    return retval;
    }
    return sysfs_emit(buf, "%d\n", calibration);
    }
    static DEVICE_ATTR(rtc_calibration, S_IRUGO | S_IWUSR,
    ab8500_sysfs_show_rtc_calibration,
    ab8500_sysfs_store_rtc_calibration);
    static struct attribute *ab8500_rtc_attrs[] = {
    &dev_attr_rtc_calibration.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group ab8500_rtc_sysfs_files = {
    .attrs	= ab8500_rtc_attrs,
    };
#[no_mangle]
unsafe extern "C" fn rtc_alarm_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t rtc_alarm_handler(int irq, void *data)
    {
    struct rtc_device *rtc = data;
    let mut events: c_ulong = RTC_IRQF | RTC_AF;
    dev_dbg(&rtc.dev, "%s\n", __func__);
    rtc_update_irq(rtc, 1, events);
    return IRQ_HANDLED;
    }
    static const struct rtc_class_ops ab8500_rtc_ops = {
    .read_time		= ab8500_rtc_read_time,
    .set_time		= ab8500_rtc_set_time,
    .read_alarm		= ab8500_rtc_read_alarm,
    .set_alarm		= ab8500_rtc_set_alarm,
    .alarm_irq_enable	= ab8500_rtc_irq_enable,
    };
    static const struct platform_device_id ab85xx_rtc_ids[] = {
    { .name = "ab8500-rtc" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(platform, ab85xx_rtc_ids);
#[no_mangle]
unsafe extern "C" fn ab8500_rtc_probe(pdev: *mut platform_device) -> c_int {
    static int ab8500_rtc_probe(struct platform_device *pdev)
    {
    int err;
    struct rtc_device *rtc;
    u8 rtc_ctrl;
    int irq;
    irq = platform_get_irq_byname(pdev, "ALARM");
    if (irq < 0)
    return irq;
// For RTC supply test
    err = abx500_mask_and_set_register_interruptible(&pdev.dev, AB8500_RTC,
    AB8500_RTC_STAT_REG, RTC_STATUS_DATA, RTC_STATUS_DATA);
    if (err < 0)
    return err;
// Wait for reset by the PorRtc
    usleep_range(1000, 5000);
    err = abx500_get_register_interruptible(&pdev.dev, AB8500_RTC,
    AB8500_RTC_STAT_REG, &rtc_ctrl);
    if (err < 0)
    return err;
// Check if the RTC Supply fails
    if (!(rtc_ctrl & RTC_STATUS_DATA)) {
    dev_err(&pdev.dev, "RTC supply failure\n");
    return -ENODEV;
    }
    devm_device_init_wakeup(&pdev.dev);
    rtc = devm_rtc_allocate_device(&pdev.dev);
    if (IS_ERR(rtc))
    return PTR_ERR(rtc);
    rtc.ops = &ab8500_rtc_ops;
    err = devm_request_threaded_irq(&pdev.dev, irq, core::ptr::null_mut(),
    rtc_alarm_handler, IRQF_ONESHOT,
    "ab8500-rtc", rtc);
    if (err < 0)
    return err;
    devm_pm_set_wake_irq(&pdev.dev, irq);
    platform_set_drvdata(pdev, rtc);
    set_bit(RTC_FEATURE_ALARM_RES_MINUTE, rtc.features);
    clear_bit(RTC_FEATURE_UPDATE_INTERRUPT, rtc.features);
    rtc.range_max = (1ULL << 24) * 60 - 1; // 24-bit minutes + 59 secs
    rtc.start_secs = RTC_TIMESTAMP_BEGIN_2000;
    rtc.set_start_time = true;
    err = rtc_add_group(rtc, &ab8500_rtc_sysfs_files);
    if (err)
    return err;
    return devm_rtc_register_device(rtc);
    }
    static struct platform_driver ab8500_rtc_driver = {
    .driver = {
    .name = "ab8500-rtc",
    },
    .probe	= ab8500_rtc_probe,
    .id_table = ab85xx_rtc_ids,
    };
    module_platform_driver(ab8500_rtc_driver);
    MODULE_AUTHOR("Virupax Sadashivpetimath <virupax.sadashivpetimath@stericsson.com>");
    MODULE_DESCRIPTION("AB8500 RTC Driver");
    MODULE_LICENSE("GPL v2");
