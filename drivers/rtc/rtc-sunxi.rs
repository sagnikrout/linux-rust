//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-sunxi.c
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
// An RTC driver for Allwinner A10/A20
//
// Copyright (c) 2013, Carlo Caione <carlo.caione@gmail.com>
//

pub const SUNXI_LOSC_CTRL: c_uint = 0x0000;

pub const SUNXI_RTC_YMD: c_uint = 0x0004;
pub const SUNXI_RTC_HMS: c_uint = 0x0008;
pub const SUNXI_ALRM_DHMS: c_uint = 0x000c;
pub const SUNXI_ALRM_EN: c_uint = 0x0014;

pub const SUNXI_ALRM_IRQ_EN: c_uint = 0x0018;

pub const SUNXI_ALRM_IRQ_STA: c_uint = 0x001c;

pub const SUNXI_MASK_DH: c_uint = 0x0000001f;
pub const SUNXI_MASK_SM: c_uint = 0x0000003f;
pub const SUNXI_MASK_M: c_uint = 0x0000000f;
pub const SUNXI_MASK_LY: c_uint = 0x00000001;
pub const SUNXI_MASK_D: c_uint = 0x00000ffe;
pub const SUNXI_MASK_M: c_uint = 0x0000000f;

    >> (shift))

//
// Get date values
//

//
// Get time values
//

//
// Get alarm values
//

//
// Set date values
//

//
// Set time values
//

//
// Set alarm values
//

//
// Time unit conversions
//
pub const SEC_IN_MIN: c_int = 60;

//
// The year parameter passed to the driver is usually an offset relative to
// the year 1900. This macro is used to convert this offset to another one
// relative to the minimum year allowed by the hardware.
//

//
// min and max year are arbitrary set considering the limited range of the
// hardware register field
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sunxi_rtc_data_year {
    pub /: *mut *mut unsigned int min; / min year allowed,
    pub /: *mut *mut unsigned int max; / max year allowed,
    pub /: *mut *mut unsigned int mask; / mask for the year field,
    pub /: *mut *mut unsigned char leap_shift; / bit shift to get the leap year,
}

    static const struct sunxi_rtc_data_year data_year_param[] = {
    [0] = {
    .min		= 2010,
    .max		= 2073,
    .mask		= 0x3f,
    .leap_shift	= 22,
    },
    [1] = {
    .min		= 1970,
    .max		= 2225,
    .mask		= 0xff,
    .leap_shift	= 24,
    },
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sunxi_rtc_dev {
    pub rtc: *mut rtc_device,
    pub dev: *mut device,
    pub data_year: *const sunxi_rtc_data_year,
    pub base: *mut void __iomem,
    pub irq: c_int,
}

#[no_mangle]
unsafe extern "C" fn sunxi_rtc_alarmirq(irq: c_int, id: *mut c_void) -> irqreturn_t {
    static irqreturn_t sunxi_rtc_alarmirq(int irq, void *id)
    {
    struct sunxi_rtc_dev *chip = (struct sunxi_rtc_dev *) id;
    u32 val;
    val = readl(chip.base + SUNXI_ALRM_IRQ_STA);
    if (val & SUNXI_ALRM_IRQ_STA_CNT_IRQ_PEND) {
    val |= SUNXI_ALRM_IRQ_STA_CNT_IRQ_PEND;
    writel(val, chip.base + SUNXI_ALRM_IRQ_STA);
    rtc_update_irq(chip.rtc, 1, RTC_AF | RTC_IRQF);
    return IRQ_HANDLED;
    }
    return IRQ_NONE;
    }
#[no_mangle]
unsafe extern "C" fn sunxi_rtc_setaie(to: c_uint, chip: *mut sunxi_rtc_dev) {
    static void sunxi_rtc_setaie(unsigned int to, struct sunxi_rtc_dev *chip)
    {
    let mut alrm_val: u32 = 0;
    let mut alrm_irq_val: u32 = 0;
    if (to) {
    alrm_val = readl(chip.base + SUNXI_ALRM_EN);
    alrm_val |= SUNXI_ALRM_EN_CNT_EN;
    alrm_irq_val = readl(chip.base + SUNXI_ALRM_IRQ_EN);
    alrm_irq_val |= SUNXI_ALRM_IRQ_EN_CNT_IRQ_EN;
    } else {
    writel(SUNXI_ALRM_IRQ_STA_CNT_IRQ_PEND,
    chip.base + SUNXI_ALRM_IRQ_STA);
    }
    writel(alrm_val, chip.base + SUNXI_ALRM_EN);
    writel(alrm_irq_val, chip.base + SUNXI_ALRM_IRQ_EN);
    }
#[no_mangle]
unsafe extern "C" fn sunxi_rtc_getalarm(dev: *mut device, wkalrm: *mut rtc_wkalrm) -> c_int {
    static int sunxi_rtc_getalarm(struct device *dev, struct rtc_wkalrm *wkalrm)
    {
    struct sunxi_rtc_dev *chip = dev_get_drvdata(dev);
    struct rtc_time *alrm_tm = &wkalrm.time;
    u32 alrm;
    u32 alrm_en;
    u32 date;
    alrm = readl(chip.base + SUNXI_ALRM_DHMS);
    date = readl(chip.base + SUNXI_RTC_YMD);
    alrm_tm.tm_sec = SUNXI_ALRM_GET_SEC_VALUE(alrm);
    alrm_tm.tm_min = SUNXI_ALRM_GET_MIN_VALUE(alrm);
    alrm_tm.tm_hour = SUNXI_ALRM_GET_HOUR_VALUE(alrm);
    alrm_tm.tm_mday = SUNXI_DATE_GET_DAY_VALUE(date);
    alrm_tm.tm_mon = SUNXI_DATE_GET_MON_VALUE(date);
    alrm_tm.tm_year = SUNXI_DATE_GET_YEAR_VALUE(date,
    chip.data_year.mask);
    alrm_tm.tm_mon -= 1;
//
// switch from (data_year->min)-relative offset to
// a (1900)-relative one
//
    alrm_tm.tm_year += SUNXI_YEAR_OFF(chip.data_year);
    alrm_en = readl(chip.base + SUNXI_ALRM_IRQ_EN);
    if (alrm_en & SUNXI_ALRM_EN_CNT_EN)
    wkalrm.enabled = 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sunxi_rtc_gettime(dev: *mut device, rtc_tm: *mut rtc_time) -> c_int {
    static int sunxi_rtc_gettime(struct device *dev, struct rtc_time *rtc_tm)
    {
    struct sunxi_rtc_dev *chip = dev_get_drvdata(dev);
    u32 date, time;
//
// read again in case it changes
//
    do {
    date = readl(chip.base + SUNXI_RTC_YMD);
    time = readl(chip.base + SUNXI_RTC_HMS);
    } while ((date != readl(chip.base + SUNXI_RTC_YMD)) ||
    (time != readl(chip.base + SUNXI_RTC_HMS)));
    rtc_tm.tm_sec  = SUNXI_TIME_GET_SEC_VALUE(time);
    rtc_tm.tm_min  = SUNXI_TIME_GET_MIN_VALUE(time);
    rtc_tm.tm_hour = SUNXI_TIME_GET_HOUR_VALUE(time);
    rtc_tm.tm_mday = SUNXI_DATE_GET_DAY_VALUE(date);
    rtc_tm.tm_mon  = SUNXI_DATE_GET_MON_VALUE(date);
    rtc_tm.tm_year = SUNXI_DATE_GET_YEAR_VALUE(date,
    chip.data_year.mask);
    rtc_tm.tm_mon  -= 1;
//
// switch from (data_year->min)-relative offset to
// a (1900)-relative one
//
    rtc_tm.tm_year += SUNXI_YEAR_OFF(chip.data_year);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sunxi_rtc_setalarm(dev: *mut device, wkalrm: *mut rtc_wkalrm) -> c_int {
    static int sunxi_rtc_setalarm(struct device *dev, struct rtc_wkalrm *wkalrm)
    {
    struct sunxi_rtc_dev *chip = dev_get_drvdata(dev);
    struct rtc_time *alrm_tm = &wkalrm.time;
    struct rtc_time tm_now;
    u32 alrm;
    time64_t diff;
    unsigned long time_gap;
    unsigned long time_gap_day;
    unsigned long time_gap_hour;
    unsigned long time_gap_min;
    int ret;
    ret = sunxi_rtc_gettime(dev, &tm_now);
    if (ret < 0) {
    dev_err(dev, "Error in getting time\n");
    return -EINVAL;
    }
    diff = rtc_tm_sub(alrm_tm, &tm_now);
    if (diff <= 0) {
    dev_err(dev, "Date to set in the past\n");
    return -EINVAL;
    }
    if (diff > 255 * SEC_IN_DAY) {
    dev_err(dev, "Day must be in the range 0 - 255\n");
    return -EINVAL;
    }
    time_gap = diff;
    time_gap_day = time_gap / SEC_IN_DAY;
    time_gap -= time_gap_day * SEC_IN_DAY;
    time_gap_hour = time_gap / SEC_IN_HOUR;
    time_gap -= time_gap_hour * SEC_IN_HOUR;
    time_gap_min = time_gap / SEC_IN_MIN;
    time_gap -= time_gap_min * SEC_IN_MIN;
    sunxi_rtc_setaie(0, chip);
    writel(0, chip.base + SUNXI_ALRM_DHMS);
    usleep_range(100, 300);
    alrm = SUNXI_ALRM_SET_SEC_VALUE(time_gap) |
    SUNXI_ALRM_SET_MIN_VALUE(time_gap_min) |
    SUNXI_ALRM_SET_HOUR_VALUE(time_gap_hour) |
    SUNXI_ALRM_SET_DAY_VALUE(time_gap_day);
    writel(alrm, chip.base + SUNXI_ALRM_DHMS);
    writel(0, chip.base + SUNXI_ALRM_IRQ_EN);
    writel(SUNXI_ALRM_IRQ_EN_CNT_IRQ_EN, chip.base + SUNXI_ALRM_IRQ_EN);
    sunxi_rtc_setaie(wkalrm.enabled, chip);
    return 0;
    }
    static int sunxi_rtc_wait(struct sunxi_rtc_dev *chip, int offset,
    unsigned int mask, unsigned int ms_timeout)
    {
    let mut timeout: c_ulong = jiffies + msecs_to_jiffies(ms_timeout);
    u32 reg;
    do {
    reg = readl(chip.base + offset);
    reg &= mask;
    if (reg == mask)
    return 0;
    } while (time_before(jiffies, timeout));
    return -ETIMEDOUT;
    }
#[no_mangle]
unsafe extern "C" fn sunxi_rtc_settime(dev: *mut device, rtc_tm: *mut rtc_time) -> c_int {
    static int sunxi_rtc_settime(struct device *dev, struct rtc_time *rtc_tm)
    {
    struct sunxi_rtc_dev *chip = dev_get_drvdata(dev);
    let mut date: u32 = 0;
    let mut time: u32 = 0;
    unsigned int year;
//
// the input rtc_tm->tm_year is the offset relative to 1900. We use
// the SUNXI_YEAR_OFF macro to rebase it with respect to the min year
// allowed by the hardware
//
    year = rtc_tm.tm_year + 1900;
    if (year < chip.data_year.min || year > chip.data_year.max) {
    dev_err(dev, "rtc only supports year in range %u - %u\n",
    chip.data_year.min, chip.data_year.max);
    return -EINVAL;
    }
    rtc_tm.tm_year -= SUNXI_YEAR_OFF(chip.data_year);
    rtc_tm.tm_mon += 1;
    date = SUNXI_DATE_SET_DAY_VALUE(rtc_tm.tm_mday) |
    SUNXI_DATE_SET_MON_VALUE(rtc_tm.tm_mon)  |
    SUNXI_DATE_SET_YEAR_VALUE(rtc_tm.tm_year,
    chip.data_year.mask);
    if (is_leap_year(year))
    date |= SUNXI_LEAP_SET_VALUE(1, chip.data_year.leap_shift);
    time = SUNXI_TIME_SET_SEC_VALUE(rtc_tm.tm_sec)  |
    SUNXI_TIME_SET_MIN_VALUE(rtc_tm.tm_min)  |
    SUNXI_TIME_SET_HOUR_VALUE(rtc_tm.tm_hour);
    writel(0, chip.base + SUNXI_RTC_HMS);
    writel(0, chip.base + SUNXI_RTC_YMD);
    writel(time, chip.base + SUNXI_RTC_HMS);
//
// After writing the RTC HH-MM-SS register, the
// SUNXI_LOSC_CTRL_RTC_HMS_ACC bit is set and it will not
// be cleared until the real writing operation is finished
//
    if (sunxi_rtc_wait(chip, SUNXI_LOSC_CTRL,
    SUNXI_LOSC_CTRL_RTC_HMS_ACC, 50)) {
    dev_err(dev, "Failed to set rtc time.\n");
    return -1;
    }
    writel(date, chip.base + SUNXI_RTC_YMD);
//
// After writing the RTC YY-MM-DD register, the
// SUNXI_LOSC_CTRL_RTC_YMD_ACC bit is set and it will not
// be cleared until the real writing operation is finished
//
    if (sunxi_rtc_wait(chip, SUNXI_LOSC_CTRL,
    SUNXI_LOSC_CTRL_RTC_YMD_ACC, 50)) {
    dev_err(dev, "Failed to set rtc time.\n");
    return -1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sunxi_rtc_alarm_irq_enable(dev: *mut device, enabled: c_uint) -> c_int {
    static int sunxi_rtc_alarm_irq_enable(struct device *dev, unsigned int enabled)
    {
    struct sunxi_rtc_dev *chip = dev_get_drvdata(dev);
    if (!enabled)
    sunxi_rtc_setaie(enabled, chip);
    return 0;
    }
    static const struct rtc_class_ops sunxi_rtc_ops = {
    .read_time		= sunxi_rtc_gettime,
    .set_time		= sunxi_rtc_settime,
    .read_alarm		= sunxi_rtc_getalarm,
    .set_alarm		= sunxi_rtc_setalarm,
    .alarm_irq_enable	= sunxi_rtc_alarm_irq_enable
    };
    static const struct of_device_id sunxi_rtc_dt_ids[] = {
    { .compatible = "allwinner,sun4i-a10-rtc", .data = &data_year_param[0] },
    { .compatible = "allwinner,sun7i-a20-rtc", .data = &data_year_param[1] },
    { /* sentinel */ },
    };
    MODULE_DEVICE_TABLE(of, sunxi_rtc_dt_ids);
#[no_mangle]
unsafe extern "C" fn sunxi_rtc_probe(pdev: *mut platform_device) -> c_int {
    static int sunxi_rtc_probe(struct platform_device *pdev)
    {
    struct sunxi_rtc_dev *chip;
    int ret;
    chip = devm_kzalloc(&pdev.dev, sizeof(*chip), GFP_KERNEL);
    if (!chip)
    return -ENOMEM;
    platform_set_drvdata(pdev, chip);
    chip.dev = &pdev.dev;
    chip.rtc = devm_rtc_allocate_device(&pdev.dev);
    if (IS_ERR(chip.rtc))
    return PTR_ERR(chip.rtc);
    chip.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(chip.base))
    return PTR_ERR(chip.base);
    chip.irq = platform_get_irq(pdev, 0);
    if (chip.irq < 0)
    return chip.irq;
    ret = devm_request_irq(&pdev.dev, chip.irq, sunxi_rtc_alarmirq,
    0, dev_name(&pdev.dev), chip);
    if (ret) {
    dev_err(&pdev.dev, "Could not request IRQ\n");
    return ret;
    }
    chip.data_year = of_device_get_match_data(&pdev.dev);
    if (!chip.data_year) {
    dev_err(&pdev.dev, "Unable to setup RTC data\n");
    return -ENODEV;
    }
// clear the alarm count value
    writel(0, chip.base + SUNXI_ALRM_DHMS);
// disable alarm, not generate irq pending
    writel(0, chip.base + SUNXI_ALRM_EN);
// disable alarm week/cnt irq, unset to cpu
    writel(0, chip.base + SUNXI_ALRM_IRQ_EN);
// clear alarm week/cnt irq pending
    writel(SUNXI_ALRM_IRQ_STA_CNT_IRQ_PEND, chip.base +
    SUNXI_ALRM_IRQ_STA);
    chip.rtc.ops = &sunxi_rtc_ops;
    return devm_rtc_register_device(chip.rtc);
    }
    static struct platform_driver sunxi_rtc_driver = {
    .probe		= sunxi_rtc_probe,
    .driver		= {
    .name		= "sunxi-rtc",
    .of_match_table = sunxi_rtc_dt_ids,
    },
    };
    module_platform_driver(sunxi_rtc_driver);
    MODULE_DESCRIPTION("sunxi RTC driver");
    MODULE_AUTHOR("Carlo Caione <carlo.caione@gmail.com>");
    MODULE_LICENSE("GPL");
