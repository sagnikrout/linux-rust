//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-sc27xx.c
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
// Copyright (C) 2017 Spreadtrum Communications Inc.
//

pub const SPRD_RTC_SEC_CNT_VALUE: c_uint = 0x0;
pub const SPRD_RTC_MIN_CNT_VALUE: c_uint = 0x4;
pub const SPRD_RTC_HOUR_CNT_VALUE: c_uint = 0x8;
pub const SPRD_RTC_DAY_CNT_VALUE: c_uint = 0xc;
pub const SPRD_RTC_SEC_CNT_UPD: c_uint = 0x10;
pub const SPRD_RTC_MIN_CNT_UPD: c_uint = 0x14;
pub const SPRD_RTC_HOUR_CNT_UPD: c_uint = 0x18;
pub const SPRD_RTC_DAY_CNT_UPD: c_uint = 0x1c;
pub const SPRD_RTC_SEC_ALM_UPD: c_uint = 0x20;
pub const SPRD_RTC_MIN_ALM_UPD: c_uint = 0x24;
pub const SPRD_RTC_HOUR_ALM_UPD: c_uint = 0x28;
pub const SPRD_RTC_DAY_ALM_UPD: c_uint = 0x2c;
pub const SPRD_RTC_INT_EN: c_uint = 0x30;
pub const SPRD_RTC_INT_RAW_STS: c_uint = 0x34;
pub const SPRD_RTC_INT_CLR: c_uint = 0x38;
pub const SPRD_RTC_INT_MASK_STS: c_uint = 0x3C;
pub const SPRD_RTC_SEC_ALM_VALUE: c_uint = 0x40;
pub const SPRD_RTC_MIN_ALM_VALUE: c_uint = 0x44;
pub const SPRD_RTC_HOUR_ALM_VALUE: c_uint = 0x48;
pub const SPRD_RTC_DAY_ALM_VALUE: c_uint = 0x4c;
pub const SPRD_RTC_SPG_VALUE: c_uint = 0x50;
pub const SPRD_RTC_SPG_UPD: c_uint = 0x54;
pub const SPRD_RTC_PWR_CTRL: c_uint = 0x58;
pub const SPRD_RTC_PWR_STS: c_uint = 0x5c;
pub const SPRD_RTC_SEC_AUXALM_UPD: c_uint = 0x60;
pub const SPRD_RTC_MIN_AUXALM_UPD: c_uint = 0x64;
pub const SPRD_RTC_HOUR_AUXALM_UPD: c_uint = 0x68;
pub const SPRD_RTC_DAY_AUXALM_UPD: c_uint = 0x6c;
// BIT & MASK definition for SPRD_RTC_INT_* registers

    (SPRD_RTC_SEC_UPD_EN | SPRD_RTC_MIN_UPD_EN |	\
    SPRD_RTC_HOUR_UPD_EN | SPRD_RTC_DAY_UPD_EN)

    (SPRD_RTC_ALMSEC_UPD_EN | SPRD_RTC_ALMMIN_UPD_EN |	\
    SPRD_RTC_ALMHOUR_UPD_EN | SPRD_RTC_ALMDAY_UPD_EN)

    (SPRD_RTC_SEC_EN | SPRD_RTC_MIN_EN |	\
    SPRD_RTC_HOUR_EN | SPRD_RTC_DAY_EN |	\
    SPRD_RTC_ALARM_EN | SPRD_RTC_AUXALM_EN)
// second/minute/hour/day values mask definition

// alarm lock definition for SPRD_RTC_SPG_UPD register

pub const SPRD_RTC_ALM_UNLOCK: c_uint = 0xa5;

    SPRD_RTC_ALMLOCK_MASK)
// SPG values definition for SPRD_RTC_SPG_UPD register

// power control/status definition
pub const SPRD_RTC_POWER_RESET_VALUE: c_uint = 0x96;

pub const SPRD_RTC_POWER_STS_SHIFT: c_int = 8;

    (~SPRD_RTC_POWER_RESET_VALUE << SPRD_RTC_POWER_STS_SHIFT)
// timeout of synchronizing time and alarm registers (us)
pub const SPRD_RTC_POLL_TIMEOUT: c_int = 200000;
pub const SPRD_RTC_POLL_DELAY_US: c_int = 20000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sprd_rtc {
    pub rtc: *mut rtc_device,
    pub regmap: *mut regmap,
    pub dev: *mut device,
    pub base: u32,
    pub irq: c_int,
    pub valid: bool,
}

//
// The Spreadtrum RTC controller has 3 groups registers, including time, normal
// alarm and auxiliary alarm. The time group registers are used to set RTC time,
// the normal alarm registers are used to set normal alarm, and the auxiliary
// alarm registers are used to set auxiliary alarm. Both alarm event and
// auxiliary alarm event can wake up system from deep sleep, but only alarm
// event can power up system from power down status.
//
    enum sprd_rtc_reg_types {
    SPRD_RTC_TIME,
    SPRD_RTC_ALARM,
    SPRD_RTC_AUX_ALARM,
    };
#[no_mangle]
unsafe extern "C" fn sprd_rtc_clear_alarm_ints(rtc: *mut sprd_rtc) -> c_int {
    static int sprd_rtc_clear_alarm_ints(struct sprd_rtc *rtc)
    {
    return regmap_write(rtc.regmap, rtc.base + SPRD_RTC_INT_CLR,
    SPRD_RTC_ALM_INT_MASK);
    }
#[no_mangle]
unsafe extern "C" fn sprd_rtc_lock_alarm(rtc: *mut sprd_rtc, lock: bool) -> c_int {
    static int sprd_rtc_lock_alarm(struct sprd_rtc *rtc, bool lock)
    {
    int ret;
    u32 val;
    ret = regmap_read(rtc.regmap, rtc.base + SPRD_RTC_SPG_VALUE, &val);
    if (ret)
    return ret;
    val &= ~SPRD_RTC_ALMLOCK_MASK;
    if (lock)
    val |= SPRD_RTC_ALM_LOCK;
    else
    val |= SPRD_RTC_ALM_UNLOCK | SPRD_RTC_POWEROFF_ALM_FLAG;
    ret = regmap_write(rtc.regmap, rtc.base + SPRD_RTC_SPG_UPD, val);
    if (ret)
    return ret;
// wait until the SPG value is updated successfully
    ret = regmap_read_poll_timeout(rtc.regmap,
    rtc.base + SPRD_RTC_INT_RAW_STS, val,
    (val & SPRD_RTC_SPG_UPD_EN),
    SPRD_RTC_POLL_DELAY_US,
    SPRD_RTC_POLL_TIMEOUT);
    if (ret) {
    dev_err(rtc.dev, "failed to update SPG value:%d\n", ret);
    return ret;
    }
    return regmap_write(rtc.regmap, rtc.base + SPRD_RTC_INT_CLR,
    SPRD_RTC_SPG_UPD_EN);
    }
    static int sprd_rtc_get_secs(struct sprd_rtc *rtc, enum sprd_rtc_reg_types type,
    time64_t *secs)
    {
    u32 sec_reg, min_reg, hour_reg, day_reg;
    u32 val, sec, min, hour, day;
    int ret;
    switch (type) {
    case SPRD_RTC_TIME:
    sec_reg = SPRD_RTC_SEC_CNT_VALUE;
    min_reg = SPRD_RTC_MIN_CNT_VALUE;
    hour_reg = SPRD_RTC_HOUR_CNT_VALUE;
    day_reg = SPRD_RTC_DAY_CNT_VALUE;
    break;
    case SPRD_RTC_ALARM:
    sec_reg = SPRD_RTC_SEC_ALM_VALUE;
    min_reg = SPRD_RTC_MIN_ALM_VALUE;
    hour_reg = SPRD_RTC_HOUR_ALM_VALUE;
    day_reg = SPRD_RTC_DAY_ALM_VALUE;
    break;
    case SPRD_RTC_AUX_ALARM:
    sec_reg = SPRD_RTC_SEC_AUXALM_UPD;
    min_reg = SPRD_RTC_MIN_AUXALM_UPD;
    hour_reg = SPRD_RTC_HOUR_AUXALM_UPD;
    day_reg = SPRD_RTC_DAY_AUXALM_UPD;
    break;
    default:
    return -EINVAL;
    }
    ret = regmap_read(rtc.regmap, rtc.base + sec_reg, &val);
    if (ret)
    return ret;
    sec = val & SPRD_RTC_SEC_MASK;
    ret = regmap_read(rtc.regmap, rtc.base + min_reg, &val);
    if (ret)
    return ret;
    min = val & SPRD_RTC_MIN_MASK;
    ret = regmap_read(rtc.regmap, rtc.base + hour_reg, &val);
    if (ret)
    return ret;
    hour = val & SPRD_RTC_HOUR_MASK;
    ret = regmap_read(rtc.regmap, rtc.base + day_reg, &val);
    if (ret)
    return ret;
    day = val & SPRD_RTC_DAY_MASK;
// secs = (((time64_t)(day * 24) + hour) * 60 + min) * 60 + sec;
    return 0;
    }
    static int sprd_rtc_set_secs(struct sprd_rtc *rtc, enum sprd_rtc_reg_types type,
    time64_t secs)
    {
    u32 sec_reg, min_reg, hour_reg, day_reg, sts_mask;
    u32 sec, min, hour, day, val;
    int ret, rem;
// convert seconds to RTC time format
    day = div_s64_rem(secs, 86400, &rem);
    hour = rem / 3600;
    rem -= hour * 3600;
    min = rem / 60;
    sec = rem - min * 60;
    switch (type) {
    case SPRD_RTC_TIME:
    sec_reg = SPRD_RTC_SEC_CNT_UPD;
    min_reg = SPRD_RTC_MIN_CNT_UPD;
    hour_reg = SPRD_RTC_HOUR_CNT_UPD;
    day_reg = SPRD_RTC_DAY_CNT_UPD;
    sts_mask = SPRD_RTC_TIME_INT_MASK;
    break;
    case SPRD_RTC_ALARM:
    sec_reg = SPRD_RTC_SEC_ALM_UPD;
    min_reg = SPRD_RTC_MIN_ALM_UPD;
    hour_reg = SPRD_RTC_HOUR_ALM_UPD;
    day_reg = SPRD_RTC_DAY_ALM_UPD;
    sts_mask = SPRD_RTC_ALMTIME_INT_MASK;
    break;
    case SPRD_RTC_AUX_ALARM:
    sec_reg = SPRD_RTC_SEC_AUXALM_UPD;
    min_reg = SPRD_RTC_MIN_AUXALM_UPD;
    hour_reg = SPRD_RTC_HOUR_AUXALM_UPD;
    day_reg = SPRD_RTC_DAY_AUXALM_UPD;
    sts_mask = 0;
    break;
    default:
    return -EINVAL;
    }
    ret = regmap_write(rtc.regmap, rtc.base + sec_reg, sec);
    if (ret)
    return ret;
    ret = regmap_write(rtc.regmap, rtc.base + min_reg, min);
    if (ret)
    return ret;
    ret = regmap_write(rtc.regmap, rtc.base + hour_reg, hour);
    if (ret)
    return ret;
    ret = regmap_write(rtc.regmap, rtc.base + day_reg, day);
    if (ret)
    return ret;
    if (type == SPRD_RTC_AUX_ALARM)
    return 0;
//
// Since the time and normal alarm registers are put in always-power-on
// region supplied by VDDRTC, then these registers changing time will
// be very long, about 125ms. Thus here we should wait until all
// values are updated successfully.
//
    ret = regmap_read_poll_timeout(rtc.regmap,
    rtc.base + SPRD_RTC_INT_RAW_STS, val,
    ((val & sts_mask) == sts_mask),
    SPRD_RTC_POLL_DELAY_US,
    SPRD_RTC_POLL_TIMEOUT);
    if (ret < 0) {
    dev_err(rtc.dev, "set time/alarm values timeout\n");
    return ret;
    }
    return regmap_write(rtc.regmap, rtc.base + SPRD_RTC_INT_CLR,
    sts_mask);
    }
#[no_mangle]
unsafe extern "C" fn sprd_rtc_set_aux_alarm(dev: *mut device, alrm: *mut rtc_wkalrm) -> c_int {
    static int sprd_rtc_set_aux_alarm(struct device *dev, struct rtc_wkalrm *alrm)
    {
    struct sprd_rtc *rtc = dev_get_drvdata(dev);
    let mut secs: time64_t = rtc_tm_to_time64(&alrm.time);
    int ret;
// clear the auxiliary alarm interrupt status
    ret = regmap_write(rtc.regmap, rtc.base + SPRD_RTC_INT_CLR,
    SPRD_RTC_AUXALM_EN);
    if (ret)
    return ret;
    ret = sprd_rtc_set_secs(rtc, SPRD_RTC_AUX_ALARM, secs);
    if (ret)
    return ret;
    if (alrm.enabled) {
    ret = regmap_update_bits(rtc.regmap,
    rtc.base + SPRD_RTC_INT_EN,
    SPRD_RTC_AUXALM_EN,
    SPRD_RTC_AUXALM_EN);
    } else {
    ret = regmap_update_bits(rtc.regmap,
    rtc.base + SPRD_RTC_INT_EN,
    SPRD_RTC_AUXALM_EN, 0);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sprd_rtc_read_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int sprd_rtc_read_time(struct device *dev, struct rtc_time *tm)
    {
    struct sprd_rtc *rtc = dev_get_drvdata(dev);
    time64_t secs;
    int ret;
    if (!rtc.valid) {
    dev_warn(dev, "RTC values are invalid\n");
    return -EINVAL;
    }
    ret = sprd_rtc_get_secs(rtc, SPRD_RTC_TIME, &secs);
    if (ret)
    return ret;
    rtc_time64_to_tm(secs, tm);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sprd_rtc_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int sprd_rtc_set_time(struct device *dev, struct rtc_time *tm)
    {
    struct sprd_rtc *rtc = dev_get_drvdata(dev);
    let mut secs: time64_t = rtc_tm_to_time64(tm);
    int ret;
    ret = sprd_rtc_set_secs(rtc, SPRD_RTC_TIME, secs);
    if (ret)
    return ret;
    if (!rtc.valid) {
// Clear RTC power status firstly
    ret = regmap_write(rtc.regmap, rtc.base + SPRD_RTC_PWR_CTRL,
    SPRD_RTC_POWER_STS_CLEAR);
    if (ret)
    return ret;
//
// Set RTC power status to indicate now RTC has valid time
// values.
//
    ret = regmap_write(rtc.regmap, rtc.base + SPRD_RTC_PWR_CTRL,
    SPRD_RTC_POWER_STS_VALID);
    if (ret)
    return ret;
    rtc.valid = true;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sprd_rtc_read_alarm(dev: *mut device, alrm: *mut rtc_wkalrm) -> c_int {
    static int sprd_rtc_read_alarm(struct device *dev, struct rtc_wkalrm *alrm)
    {
    struct sprd_rtc *rtc = dev_get_drvdata(dev);
    time64_t secs;
    int ret;
    u32 val;
//
// The RTC core checks to see if there is an alarm already set in RTC
// hardware, and we always read the normal alarm at this time.
//
    ret = sprd_rtc_get_secs(rtc, SPRD_RTC_ALARM, &secs);
    if (ret)
    return ret;
    rtc_time64_to_tm(secs, &alrm.time);
    ret = regmap_read(rtc.regmap, rtc.base + SPRD_RTC_INT_EN, &val);
    if (ret)
    return ret;
    alrm.enabled = !!(val & SPRD_RTC_ALARM_EN);
    ret = regmap_read(rtc.regmap, rtc.base + SPRD_RTC_INT_RAW_STS, &val);
    if (ret)
    return ret;
    alrm.pending = !!(val & SPRD_RTC_ALARM_EN);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sprd_rtc_set_alarm(dev: *mut device, alrm: *mut rtc_wkalrm) -> c_int {
    static int sprd_rtc_set_alarm(struct device *dev, struct rtc_wkalrm *alrm)
    {
    struct sprd_rtc *rtc = dev_get_drvdata(dev);
    let mut secs: time64_t = rtc_tm_to_time64(&alrm.time);
    struct rtc_time aie_time =
    rtc_ktime_to_tm(rtc.rtc.aie_timer.node.expires);
    int ret;
//
// We have 2 groups alarms: normal alarm and auxiliary alarm. Since
// both normal alarm event and auxiliary alarm event can wake up system
// from deep sleep, but only alarm event can power up system from power
// down status. Moreover we do not need to poll about 125ms when
// updating auxiliary alarm registers. Thus we usually set auxiliary
// alarm when wake up system from deep sleep, and for other scenarios,
// we should set normal alarm with polling status.
//
// So here we check if the alarm time is set by aie_timer, if yes, we
// should set normal alarm, if not, we should set auxiliary alarm which
// means it is just a wake event.
//
    if (!rtc.rtc.aie_timer.enabled || rtc_tm_sub(&aie_time, &alrm.time))
    return sprd_rtc_set_aux_alarm(dev, alrm);
// clear the alarm interrupt status firstly
    ret = regmap_write(rtc.regmap, rtc.base + SPRD_RTC_INT_CLR,
    SPRD_RTC_ALARM_EN);
    if (ret)
    return ret;
    ret = sprd_rtc_set_secs(rtc, SPRD_RTC_ALARM, secs);
    if (ret)
    return ret;
    if (alrm.enabled) {
    ret = regmap_update_bits(rtc.regmap,
    rtc.base + SPRD_RTC_INT_EN,
    SPRD_RTC_ALARM_EN,
    SPRD_RTC_ALARM_EN);
    if (ret)
    return ret;
// unlock the alarm to enable the alarm function.
    ret = sprd_rtc_lock_alarm(rtc, false);
    } else {
    regmap_update_bits(rtc.regmap,
    rtc.base + SPRD_RTC_INT_EN,
    SPRD_RTC_ALARM_EN, 0);
//
// Lock the alarm function in case fake alarm event will power
// up systems.
//
    ret = sprd_rtc_lock_alarm(rtc, true);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sprd_rtc_alarm_irq_enable(dev: *mut device, enabled: c_uint) -> c_int {
    static int sprd_rtc_alarm_irq_enable(struct device *dev, unsigned int enabled)
    {
    struct sprd_rtc *rtc = dev_get_drvdata(dev);
    int ret;
    if (enabled) {
    ret = regmap_update_bits(rtc.regmap,
    rtc.base + SPRD_RTC_INT_EN,
    SPRD_RTC_ALARM_EN | SPRD_RTC_AUXALM_EN,
    SPRD_RTC_ALARM_EN | SPRD_RTC_AUXALM_EN);
    if (ret)
    return ret;
    ret = sprd_rtc_lock_alarm(rtc, false);
    } else {
    regmap_update_bits(rtc.regmap, rtc.base + SPRD_RTC_INT_EN,
    SPRD_RTC_ALARM_EN | SPRD_RTC_AUXALM_EN, 0);
    ret = sprd_rtc_lock_alarm(rtc, true);
    }
    return ret;
    }
    static const struct rtc_class_ops sprd_rtc_ops = {
    .read_time = sprd_rtc_read_time,
    .set_time = sprd_rtc_set_time,
    .read_alarm = sprd_rtc_read_alarm,
    .set_alarm = sprd_rtc_set_alarm,
    .alarm_irq_enable = sprd_rtc_alarm_irq_enable,
    };
#[no_mangle]
unsafe extern "C" fn sprd_rtc_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t sprd_rtc_handler(int irq, void *dev_id)
    {
    struct sprd_rtc *rtc = dev_id;
    int ret;
    ret = sprd_rtc_clear_alarm_ints(rtc);
    if (ret)
    return IRQ_RETVAL(ret);
    rtc_update_irq(rtc.rtc, 1, RTC_AF | RTC_IRQF);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn sprd_rtc_check_power_down(rtc: *mut sprd_rtc) -> c_int {
    static int sprd_rtc_check_power_down(struct sprd_rtc *rtc)
    {
    u32 val;
    int ret;
    ret = regmap_read(rtc.regmap, rtc.base + SPRD_RTC_PWR_STS, &val);
    if (ret)
    return ret;
//
// If the RTC power status value is SPRD_RTC_POWER_RESET_VALUE, which
// means the RTC has been powered down, so the RTC time values are
// invalid.
//
    rtc.valid = val != SPRD_RTC_POWER_RESET_VALUE;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sprd_rtc_check_alarm_int(rtc: *mut sprd_rtc) -> c_int {
    static int sprd_rtc_check_alarm_int(struct sprd_rtc *rtc)
    {
    u32 val;
    int ret;
    ret = regmap_read(rtc.regmap, rtc.base + SPRD_RTC_SPG_VALUE, &val);
    if (ret)
    return ret;
//
// The SPRD_RTC_INT_EN register is not put in always-power-on region
// supplied by VDDRTC, so we should check if we need enable the alarm
// interrupt when system booting.
//
// If we have set SPRD_RTC_POWEROFF_ALM_FLAG which is saved in
// always-power-on region, that means we have set one alarm last time,
// so we should enable the alarm interrupt to help RTC core to see if
// there is an alarm already set in RTC hardware.
//
    if (!(val & SPRD_RTC_POWEROFF_ALM_FLAG))
    return 0;
    return regmap_update_bits(rtc.regmap, rtc.base + SPRD_RTC_INT_EN,
    SPRD_RTC_ALARM_EN, SPRD_RTC_ALARM_EN);
    }
#[no_mangle]
unsafe extern "C" fn sprd_rtc_probe(pdev: *mut platform_device) -> c_int {
    static int sprd_rtc_probe(struct platform_device *pdev)
    {
    struct device_node *node = pdev.dev.of_node;
    struct sprd_rtc *rtc;
    int ret;
    rtc = devm_kzalloc(&pdev.dev, sizeof(*rtc), GFP_KERNEL);
    if (!rtc)
    return -ENOMEM;
    rtc.regmap = dev_get_regmap(pdev.dev.parent, core::ptr::null_mut());
    if (!rtc.regmap)
    return -ENODEV;
    ret = of_property_read_u32(node, "reg", &rtc.base);
    if (ret) {
    dev_err(&pdev.dev, "failed to get RTC base address\n");
    return ret;
    }
    rtc.irq = platform_get_irq(pdev, 0);
    if (rtc.irq < 0)
    return rtc.irq;
    rtc.rtc = devm_rtc_allocate_device(&pdev.dev);
    if (IS_ERR(rtc.rtc))
    return PTR_ERR(rtc.rtc);
    rtc.dev = &pdev.dev;
    platform_set_drvdata(pdev, rtc);
// check if we need set the alarm interrupt
    ret = sprd_rtc_check_alarm_int(rtc);
    if (ret) {
    dev_err(&pdev.dev, "failed to check RTC alarm interrupt\n");
    return ret;
    }
// check if RTC time values are valid
    ret = sprd_rtc_check_power_down(rtc);
    if (ret) {
    dev_err(&pdev.dev, "failed to check RTC time values\n");
    return ret;
    }
    ret = devm_request_threaded_irq(&pdev.dev, rtc.irq, core::ptr::null_mut(),
    sprd_rtc_handler,
    IRQF_ONESHOT | IRQF_EARLY_RESUME,
    pdev.name, rtc);
    if (ret < 0) {
    dev_err(&pdev.dev, "failed to request RTC irq\n");
    return ret;
    }
    device_init_wakeup(&pdev.dev, true);
    rtc.rtc.ops = &sprd_rtc_ops;
    rtc.rtc.range_min = 0;
    rtc.rtc.range_max = 5662310399LL;
    ret = devm_rtc_register_device(rtc.rtc);
    if (ret) {
    device_init_wakeup(&pdev.dev, false);
    return ret;
    }
    return 0;
    }
    static const struct of_device_id sprd_rtc_of_match[] = {
    { .compatible = "sprd,sc2731-rtc", },
    { },
    };
    MODULE_DEVICE_TABLE(of, sprd_rtc_of_match);
    static struct platform_driver sprd_rtc_driver = {
    .driver = {
    .name = "sprd-rtc",
    .of_match_table = sprd_rtc_of_match,
    },
    .probe	= sprd_rtc_probe,
    };
    module_platform_driver(sprd_rtc_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("Spreadtrum RTC Device Driver");
    MODULE_AUTHOR("Baolin Wang <baolin.wang@spreadtrum.com>");
