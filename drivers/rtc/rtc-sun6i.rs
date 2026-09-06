//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-sun6i.c
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
// An RTC driver for Allwinner A31/A23
//
// Copyright (c) 2014, Chen-Yu Tsai <wens@csie.org>
//
// based on rtc-sunxi.c
//
// An RTC driver for Allwinner A10/A20
//
// Copyright (c) 2013, Carlo Caione <carlo.caione@gmail.com>
//

// Control register
pub const SUN6I_LOSC_CTRL: c_uint = 0x0000;

pub const SUN6I_LOSC_CLK_PRESCAL: c_uint = 0x0008;
// RTC
pub const SUN6I_RTC_YMD: c_uint = 0x0010;
pub const SUN6I_RTC_HMS: c_uint = 0x0014;
// Alarm 0 (counter)
pub const SUN6I_ALRM_COUNTER: c_uint = 0x0020;
// This holds the remaining alarm seconds on older SoCs (current value)
pub const SUN6I_ALRM_COUNTER_HMS: c_uint = 0x0024;
pub const SUN6I_ALRM_EN: c_uint = 0x0028;

pub const SUN6I_ALRM_IRQ_EN: c_uint = 0x002c;

pub const SUN6I_ALRM_IRQ_STA: c_uint = 0x0030;

// Alarm 1 (wall clock)
pub const SUN6I_ALRM1_EN: c_uint = 0x0044;
pub const SUN6I_ALRM1_IRQ_EN: c_uint = 0x0048;
pub const SUN6I_ALRM1_IRQ_STA: c_uint = 0x004c;

// Alarm config
pub const SUN6I_ALARM_CONFIG: c_uint = 0x0050;

pub const SUN6I_LOSC_OUT_GATING: c_uint = 0x0060;
pub const SUN6I_LOSC_OUT_GATING_EN_OFFSET: c_int = 0;
// General-purpose data
pub const SUN6I_GP_DATA: c_uint = 0x0100;
pub const SUN6I_GP_DATA_SIZE: c_uint = 0x20;
//
// Get date values
//

//
// Get time values
//

//
// Set date values
//

//
// Set time values
//

//
// The year parameter passed to the driver is usually an offset relative to
// the year 1900. This macro is used to convert this offset to another one
// relative to the minimum year allowed by the hardware.
//
// The year range is 1970 - 2033. This range is selected to match Allwinner's
// driver, even though it is somewhat limited.
//
pub const SUN6I_YEAR_MIN: c_int = 1970;

//
// There are other differences between models, including:
//
// - number of GPIO pins that can be configured to hold a certain level
// - crypto-key related registers (H5, H6)
// - boot process related (super standby, secondary processor entry address)
// registers (R40, H6)
// - SYS power domain controls (R40)
// - DCXO controls (H6)
// - RC oscillator calibration (H6)
//
// These functions are not covered by this driver.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun6i_rtc_clk_data {
    pub rc_osc_rate: c_ulong,
    pub 16: unsigned int fixed_prescaler :,
    pub 1: unsigned int has_prescaler :,
    pub 1: unsigned int has_out_clk :,
    pub 1: unsigned int has_losc_en :,
    pub 1: unsigned int has_auto_swt :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun6i_rtc_dev {
    pub rtc: *mut rtc_device,
    pub data: *const sun6i_rtc_clk_data,
    pub base: *mut void __iomem,
    pub irq: c_int,
    pub alarm: time64_t,
    pub flags: c_ulong,
    pub hw: clk_hw,
    pub int_osc: *mut clk_hw,
    pub losc: *mut clk,
    pub ext_losc: *mut clk,
    pub lock: spinlock_t,
}

    static struct sun6i_rtc_dev *sun6i_rtc;
    static unsigned long sun6i_rtc_osc_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct sun6i_rtc_dev *rtc = container_of(hw, struct sun6i_rtc_dev, hw);
    let mut val: u32 = 0;
    val = readl(rtc.base + SUN6I_LOSC_CTRL);
    if (val & SUN6I_LOSC_CTRL_EXT_OSC)
    return parent_rate;
    if (rtc.data.fixed_prescaler)
    parent_rate /= rtc.data.fixed_prescaler;
    if (rtc.data.has_prescaler) {
    val = readl(rtc.base + SUN6I_LOSC_CLK_PRESCAL);
    val &= GENMASK(4, 0);
    }
    return parent_rate / (val + 1);
    }
#[no_mangle]
unsafe extern "C" fn sun6i_rtc_osc_get_parent(hw: *mut clk_hw) -> u8 {
    static u8 sun6i_rtc_osc_get_parent(struct clk_hw *hw)
    {
    struct sun6i_rtc_dev *rtc = container_of(hw, struct sun6i_rtc_dev, hw);
    return readl(rtc.base + SUN6I_LOSC_CTRL) & SUN6I_LOSC_CTRL_EXT_OSC;
    }
#[no_mangle]
unsafe extern "C" fn sun6i_rtc_osc_set_parent(hw: *mut clk_hw, index: u8) -> c_int {
    static int sun6i_rtc_osc_set_parent(struct clk_hw *hw, u8 index)
    {
    struct sun6i_rtc_dev *rtc = container_of(hw, struct sun6i_rtc_dev, hw);
    unsigned long flags;
    u32 val;
    if (index > 1)
    return -EINVAL;
    spin_lock_irqsave(&rtc.lock, flags);
    val = readl(rtc.base + SUN6I_LOSC_CTRL);
    val &= ~SUN6I_LOSC_CTRL_EXT_OSC;
    val |= SUN6I_LOSC_CTRL_KEY;
    val |= index ? SUN6I_LOSC_CTRL_EXT_OSC : 0;
    if (rtc.data.has_losc_en) {
    val &= ~SUN6I_LOSC_CTRL_EXT_LOSC_EN;
    val |= index ? SUN6I_LOSC_CTRL_EXT_LOSC_EN : 0;
    }
    writel(val, rtc.base + SUN6I_LOSC_CTRL);
    spin_unlock_irqrestore(&rtc.lock, flags);
    return 0;
    }
    static const struct clk_ops sun6i_rtc_osc_ops = {
    .recalc_rate	= sun6i_rtc_osc_recalc_rate,
    .determine_rate	= clk_hw_determine_rate_no_reparent,
    .get_parent	= sun6i_rtc_osc_get_parent,
    .set_parent	= sun6i_rtc_osc_set_parent,
    };
    static void __init sun6i_rtc_clk_init(struct device_node *node,
    const struct sun6i_rtc_clk_data *data)
    {
    struct clk_hw_onecell_data *clk_data;
    struct sun6i_rtc_dev *rtc;
    struct clk_init_data init = {
    .ops		= &sun6i_rtc_osc_ops,
    .name		= "losc",
    };
    const char *iosc_name = "rtc-int-osc";
    const char *clkout_name = "osc32k-out";
    const char *parents[2];
    u32 reg;
    rtc = kzalloc_obj(*rtc);
    if (!rtc)
    return;
    rtc.data = data;
    clk_data = kzalloc_flex(*clk_data, hws, 3);
    if (!clk_data) {
    kfree(rtc);
    return;
    }
    spin_lock_init(&rtc.lock);
    rtc.base = of_io_request_and_map(node, 0, of_node_full_name(node));
    if (IS_ERR(rtc.base)) {
    pr_crit("Can't map RTC registers");
    goto err;
    }
    reg = SUN6I_LOSC_CTRL_KEY;
    if (rtc.data.has_auto_swt) {
// Bypass auto-switch to int osc, on ext losc failure
    reg |= SUN6I_LOSC_CTRL_AUTO_SWT_BYPASS;
    writel(reg, rtc.base + SUN6I_LOSC_CTRL);
    }
// Switch to the external, more precise, oscillator, if present
    if (of_property_present(node, "clocks")) {
    reg |= SUN6I_LOSC_CTRL_EXT_OSC;
    if (rtc.data.has_losc_en)
    reg |= SUN6I_LOSC_CTRL_EXT_LOSC_EN;
    }
    writel(reg, rtc.base + SUN6I_LOSC_CTRL);
// Yes, I know, this is ugly.
    sun6i_rtc = rtc;
    of_property_read_string_index(node, "clock-output-names", 2,
    &iosc_name);
    rtc.int_osc = clk_hw_register_fixed_rate_with_accuracy(core::ptr::null_mut(),
    iosc_name,
    core::ptr::null_mut(), 0,
    rtc.data.rc_osc_rate,
    300000000);
    if (IS_ERR(rtc.int_osc)) {
    pr_crit("Couldn't register the internal oscillator\n");
    goto err;
    }
    parents[0] = clk_hw_get_name(rtc.int_osc);
// If there is no external oscillator, this will be NULL and ...
    parents[1] = of_clk_get_parent_name(node, 0);
    rtc.hw.init = &init;
    init.parent_names = parents;
// ... number of clock parents will be 1.
    init.num_parents = of_clk_get_parent_count(node) + 1;
    of_property_read_string_index(node, "clock-output-names", 0,
    &init.name);
    rtc.losc = clk_register(core::ptr::null_mut(), &rtc.hw);
    if (IS_ERR(rtc.losc)) {
    pr_crit("Couldn't register the LOSC clock\n");
    goto err_register;
    }
    of_property_read_string_index(node, "clock-output-names", 1,
    &clkout_name);
    rtc.ext_losc = clk_register_gate(core::ptr::null_mut(), clkout_name, init.name,
    0, rtc.base + SUN6I_LOSC_OUT_GATING,
    SUN6I_LOSC_OUT_GATING_EN_OFFSET, 0,
    &rtc.lock);
    if (IS_ERR(rtc.ext_losc)) {
    pr_crit("Couldn't register the LOSC external gate\n");
    goto err_register;
    }
    clk_data.num = 3;
    clk_data.hws[0] = &rtc.hw;
    clk_data.hws[1] = __clk_get_hw(rtc.ext_losc);
    clk_data.hws[2] = rtc.int_osc;
    of_clk_add_hw_provider(node, of_clk_hw_onecell_get, clk_data);
    return;
    err_register:
    clk_hw_unregister_fixed_rate(rtc.int_osc);
    err:
    kfree(clk_data);
    }
    static const struct sun6i_rtc_clk_data sun6i_a31_rtc_data = {
    .rc_osc_rate = 667000, /* datasheet says 600 ~ 700 KHz */
    .has_prescaler = 1,
    };
#[no_mangle]
unsafe extern "C" fn sun6i_a31_rtc_clk_init(node: *mut device_node) -> void __init {
    static void __init sun6i_a31_rtc_clk_init(struct device_node *node)
    {
    sun6i_rtc_clk_init(node, &sun6i_a31_rtc_data);
    }
    CLK_OF_DECLARE_DRIVER(sun6i_a31_rtc_clk, "allwinner,sun6i-a31-rtc",
    sun6i_a31_rtc_clk_init);
    static const struct sun6i_rtc_clk_data sun8i_a23_rtc_data = {
    .rc_osc_rate = 667000, /* datasheet says 600 ~ 700 KHz */
    .has_prescaler = 1,
    .has_out_clk = 1,
    };
#[no_mangle]
unsafe extern "C" fn sun8i_a23_rtc_clk_init(node: *mut device_node) -> void __init {
    static void __init sun8i_a23_rtc_clk_init(struct device_node *node)
    {
    sun6i_rtc_clk_init(node, &sun8i_a23_rtc_data);
    }
    CLK_OF_DECLARE_DRIVER(sun8i_a23_rtc_clk, "allwinner,sun8i-a23-rtc",
    sun8i_a23_rtc_clk_init);
    static const struct sun6i_rtc_clk_data sun8i_h3_rtc_data = {
    .rc_osc_rate = 16000000,
    .fixed_prescaler = 32,
    .has_prescaler = 1,
    .has_out_clk = 1,
    };
#[no_mangle]
unsafe extern "C" fn sun8i_h3_rtc_clk_init(node: *mut device_node) -> void __init {
    static void __init sun8i_h3_rtc_clk_init(struct device_node *node)
    {
    sun6i_rtc_clk_init(node, &sun8i_h3_rtc_data);
    }
    CLK_OF_DECLARE_DRIVER(sun8i_h3_rtc_clk, "allwinner,sun8i-h3-rtc",
    sun8i_h3_rtc_clk_init);
// As far as we are concerned, clocks for H5 are the same as H3
    CLK_OF_DECLARE_DRIVER(sun50i_h5_rtc_clk, "allwinner,sun50i-h5-rtc",
    sun8i_h3_rtc_clk_init);
    static const struct sun6i_rtc_clk_data sun50i_h6_rtc_data = {
    .rc_osc_rate = 16000000,
    .fixed_prescaler = 32,
    .has_prescaler = 1,
    .has_out_clk = 1,
    .has_losc_en = 1,
    .has_auto_swt = 1,
    };
#[no_mangle]
unsafe extern "C" fn sun50i_h6_rtc_clk_init(node: *mut device_node) -> void __init {
    static void __init sun50i_h6_rtc_clk_init(struct device_node *node)
    {
    sun6i_rtc_clk_init(node, &sun50i_h6_rtc_data);
    }
    CLK_OF_DECLARE_DRIVER(sun50i_h6_rtc_clk, "allwinner,sun50i-h6-rtc",
    sun50i_h6_rtc_clk_init);
//
// The R40 user manual is self-conflicting on whether the prescaler is
// fixed or configurable. The clock diagram shows it as fixed, but there
// is also a configurable divider in the RTC block.
//
    static const struct sun6i_rtc_clk_data sun8i_r40_rtc_data = {
    .rc_osc_rate = 16000000,
    .fixed_prescaler = 512,
    };
#[no_mangle]
unsafe extern "C" fn sun8i_r40_rtc_clk_init(node: *mut device_node) -> void __init {
    static void __init sun8i_r40_rtc_clk_init(struct device_node *node)
    {
    sun6i_rtc_clk_init(node, &sun8i_r40_rtc_data);
    }
    CLK_OF_DECLARE_DRIVER(sun8i_r40_rtc_clk, "allwinner,sun8i-r40-rtc",
    sun8i_r40_rtc_clk_init);
    static const struct sun6i_rtc_clk_data sun8i_v3_rtc_data = {
    .rc_osc_rate = 32000,
    .has_out_clk = 1,
    .has_auto_swt = 1,
    };
#[no_mangle]
unsafe extern "C" fn sun8i_v3_rtc_clk_init(node: *mut device_node) -> void __init {
    static void __init sun8i_v3_rtc_clk_init(struct device_node *node)
    {
    sun6i_rtc_clk_init(node, &sun8i_v3_rtc_data);
    }
    CLK_OF_DECLARE_DRIVER(sun8i_v3_rtc_clk, "allwinner,sun8i-v3-rtc",
    sun8i_v3_rtc_clk_init);
#[no_mangle]
unsafe extern "C" fn sun6i_rtc_alarmirq(irq: c_int, id: *mut c_void) -> irqreturn_t {
    static irqreturn_t sun6i_rtc_alarmirq(int irq, void *id)
    {
    struct sun6i_rtc_dev *chip = (struct sun6i_rtc_dev *) id;
    let mut ret: irqreturn_t = IRQ_NONE;
    u32 val;
    spin_lock(&chip.lock);
    val = readl(chip.base + SUN6I_ALRM_IRQ_STA);
    if (val & SUN6I_ALRM_IRQ_STA_CNT_IRQ_PEND) {
    val |= SUN6I_ALRM_IRQ_STA_CNT_IRQ_PEND;
    writel(val, chip.base + SUN6I_ALRM_IRQ_STA);
    rtc_update_irq(chip.rtc, 1, RTC_AF | RTC_IRQF);
    ret = IRQ_HANDLED;
    }
    spin_unlock(&chip.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sun6i_rtc_setaie(to: c_int, chip: *mut sun6i_rtc_dev) {
    static void sun6i_rtc_setaie(int to, struct sun6i_rtc_dev *chip)
    {
    let mut alrm_val: u32 = 0;
    let mut alrm_irq_val: u32 = 0;
    let mut alrm_wake_val: u32 = 0;
    unsigned long flags;
    if (to) {
    alrm_val = SUN6I_ALRM_EN_CNT_EN;
    alrm_irq_val = SUN6I_ALRM_IRQ_EN_CNT_IRQ_EN;
    alrm_wake_val = SUN6I_ALARM_CONFIG_WAKEUP;
    } else {
    writel(SUN6I_ALRM_IRQ_STA_CNT_IRQ_PEND,
    chip.base + SUN6I_ALRM_IRQ_STA);
    }
    spin_lock_irqsave(&chip.lock, flags);
    writel(alrm_val, chip.base + SUN6I_ALRM_EN);
    writel(alrm_irq_val, chip.base + SUN6I_ALRM_IRQ_EN);
    writel(alrm_wake_val, chip.base + SUN6I_ALARM_CONFIG);
    spin_unlock_irqrestore(&chip.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn sun6i_rtc_gettime(dev: *mut device, rtc_tm: *mut rtc_time) -> c_int {
    static int sun6i_rtc_gettime(struct device *dev, struct rtc_time *rtc_tm)
    {
    struct sun6i_rtc_dev *chip = dev_get_drvdata(dev);
    u32 date, time;
//
// read again in case it changes
//
    do {
    date = readl(chip.base + SUN6I_RTC_YMD);
    time = readl(chip.base + SUN6I_RTC_HMS);
    } while ((date != readl(chip.base + SUN6I_RTC_YMD)) ||
    (time != readl(chip.base + SUN6I_RTC_HMS)));
    if (chip.flags & RTC_LINEAR_DAY) {
//
// Newer chips store a linear day number, the manual
// does not mandate any epoch base. The BSP driver uses
// the UNIX epoch, let's just copy that, as it's the
// easiest anyway.
//
    rtc_time64_to_tm((date & 0xffff) * SECS_PER_DAY, rtc_tm);
    } else {
    rtc_tm.tm_mday = SUN6I_DATE_GET_DAY_VALUE(date);
    rtc_tm.tm_mon  = SUN6I_DATE_GET_MON_VALUE(date) - 1;
    rtc_tm.tm_year = SUN6I_DATE_GET_YEAR_VALUE(date);
//
// switch from (data_year->min)-relative offset to
// a (1900)-relative one
//
    rtc_tm.tm_year += SUN6I_YEAR_OFF;
    }
    rtc_tm.tm_sec  = SUN6I_TIME_GET_SEC_VALUE(time);
    rtc_tm.tm_min  = SUN6I_TIME_GET_MIN_VALUE(time);
    rtc_tm.tm_hour = SUN6I_TIME_GET_HOUR_VALUE(time);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sun6i_rtc_getalarm(dev: *mut device, wkalrm: *mut rtc_wkalrm) -> c_int {
    static int sun6i_rtc_getalarm(struct device *dev, struct rtc_wkalrm *wkalrm)
    {
    struct sun6i_rtc_dev *chip = dev_get_drvdata(dev);
    unsigned long flags;
    u32 alrm_st;
    u32 alrm_en;
    spin_lock_irqsave(&chip.lock, flags);
    alrm_en = readl(chip.base + SUN6I_ALRM_IRQ_EN);
    alrm_st = readl(chip.base + SUN6I_ALRM_IRQ_STA);
    spin_unlock_irqrestore(&chip.lock, flags);
    wkalrm.enabled = !!(alrm_en & SUN6I_ALRM_EN_CNT_EN);
    wkalrm.pending = !!(alrm_st & SUN6I_ALRM_EN_CNT_EN);
    rtc_time64_to_tm(chip.alarm, &wkalrm.time);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sun6i_rtc_setalarm(dev: *mut device, wkalrm: *mut rtc_wkalrm) -> c_int {
    static int sun6i_rtc_setalarm(struct device *dev, struct rtc_wkalrm *wkalrm)
    {
    struct sun6i_rtc_dev *chip = dev_get_drvdata(dev);
    struct rtc_time *alrm_tm = &wkalrm.time;
    struct rtc_time tm_now;
    time64_t time_set;
    u32 counter_val, counter_val_hms;
    int ret;
    time_set = rtc_tm_to_time64(alrm_tm);
    if (chip.flags & RTC_LINEAR_DAY) {
//
// The alarm registers hold the actual alarm time, encoded
// in the same way (linear day + HMS) as the current time.
//
    counter_val_hms = SUN6I_TIME_SET_SEC_VALUE(alrm_tm.tm_sec)  |
    SUN6I_TIME_SET_MIN_VALUE(alrm_tm.tm_min)  |
    SUN6I_TIME_SET_HOUR_VALUE(alrm_tm.tm_hour);
// The division will cut off the H:M:S part of alrm_tm.
    counter_val = div_u64(rtc_tm_to_time64(alrm_tm), SECS_PER_DAY);
    } else {
// The alarm register holds the number of seconds left.
    time64_t time_now;
    ret = sun6i_rtc_gettime(dev, &tm_now);
    if (ret < 0) {
    dev_err(dev, "Error in getting time\n");
    return -EINVAL;
    }
    time_now = rtc_tm_to_time64(&tm_now);
    if (time_set <= time_now) {
    dev_err(dev, "Date to set in the past\n");
    return -EINVAL;
    }
    if ((time_set - time_now) > U32_MAX) {
    dev_err(dev, "Date too far in the future\n");
    return -EINVAL;
    }
    counter_val = time_set - time_now;
    }
    sun6i_rtc_setaie(0, chip);
    writel(0, chip.base + SUN6I_ALRM_COUNTER);
    if (chip.flags & RTC_LINEAR_DAY)
    writel(0, chip.base + SUN6I_ALRM_COUNTER_HMS);
    usleep_range(100, 300);
    writel(counter_val, chip.base + SUN6I_ALRM_COUNTER);
    if (chip.flags & RTC_LINEAR_DAY)
    writel(counter_val_hms, chip.base + SUN6I_ALRM_COUNTER_HMS);
    chip.alarm = time_set;
    sun6i_rtc_setaie(wkalrm.enabled, chip);
    return 0;
    }
    static int sun6i_rtc_wait(struct sun6i_rtc_dev *chip, int offset,
    unsigned int mask, unsigned int ms_timeout)
    {
    let mut timeout: c_ulong = jiffies + msecs_to_jiffies(ms_timeout);
    u32 reg;
    do {
    reg = readl(chip.base + offset);
    reg &= mask;
    if (!reg)
    return 0;
    } while (time_before(jiffies, timeout));
    return -ETIMEDOUT;
    }
#[no_mangle]
unsafe extern "C" fn sun6i_rtc_settime(dev: *mut device, rtc_tm: *mut rtc_time) -> c_int {
    static int sun6i_rtc_settime(struct device *dev, struct rtc_time *rtc_tm)
    {
    struct sun6i_rtc_dev *chip = dev_get_drvdata(dev);
    let mut date: u32 = 0;
    let mut time: u32 = 0;
    time = SUN6I_TIME_SET_SEC_VALUE(rtc_tm.tm_sec)  |
    SUN6I_TIME_SET_MIN_VALUE(rtc_tm.tm_min)  |
    SUN6I_TIME_SET_HOUR_VALUE(rtc_tm.tm_hour);
    if (chip.flags & RTC_LINEAR_DAY) {
// The division will cut off the H:M:S part of rtc_tm.
    date = div_u64(rtc_tm_to_time64(rtc_tm), SECS_PER_DAY);
    } else {
    rtc_tm.tm_year -= SUN6I_YEAR_OFF;
    rtc_tm.tm_mon += 1;
    date = SUN6I_DATE_SET_DAY_VALUE(rtc_tm.tm_mday) |
    SUN6I_DATE_SET_MON_VALUE(rtc_tm.tm_mon)  |
    SUN6I_DATE_SET_YEAR_VALUE(rtc_tm.tm_year);
    if (is_leap_year(rtc_tm.tm_year + SUN6I_YEAR_MIN))
    date |= SUN6I_LEAP_SET_VALUE(1);
    }
// Check whether registers are writable
    if (sun6i_rtc_wait(chip, SUN6I_LOSC_CTRL,
    SUN6I_LOSC_CTRL_ACC_MASK, 50)) {
    dev_err(dev, "rtc is still busy.\n");
    return -EBUSY;
    }
    writel(time, chip.base + SUN6I_RTC_HMS);
//
// After writing the RTC HH-MM-SS register, the
// SUN6I_LOSC_CTRL_RTC_HMS_ACC bit is set and it will not
// be cleared until the real writing operation is finished
//
    if (sun6i_rtc_wait(chip, SUN6I_LOSC_CTRL,
    SUN6I_LOSC_CTRL_RTC_HMS_ACC, 50)) {
    dev_err(dev, "Failed to set rtc time.\n");
    return -ETIMEDOUT;
    }
    writel(date, chip.base + SUN6I_RTC_YMD);
//
// After writing the RTC YY-MM-DD register, the
// SUN6I_LOSC_CTRL_RTC_YMD_ACC bit is set and it will not
// be cleared until the real writing operation is finished
//
    if (sun6i_rtc_wait(chip, SUN6I_LOSC_CTRL,
    SUN6I_LOSC_CTRL_RTC_YMD_ACC, 50)) {
    dev_err(dev, "Failed to set rtc time.\n");
    return -ETIMEDOUT;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sun6i_rtc_alarm_irq_enable(dev: *mut device, enabled: c_uint) -> c_int {
    static int sun6i_rtc_alarm_irq_enable(struct device *dev, unsigned int enabled)
    {
    struct sun6i_rtc_dev *chip = dev_get_drvdata(dev);
    if (!enabled)
    sun6i_rtc_setaie(enabled, chip);
    return 0;
    }
    static const struct rtc_class_ops sun6i_rtc_ops = {
    .read_time		= sun6i_rtc_gettime,
    .set_time		= sun6i_rtc_settime,
    .read_alarm		= sun6i_rtc_getalarm,
    .set_alarm		= sun6i_rtc_setalarm,
    .alarm_irq_enable	= sun6i_rtc_alarm_irq_enable
    };
#[no_mangle]
unsafe extern "C" fn sun6i_rtc_nvmem_read(priv: *mut c_void, offset: c_uint, _val: *mut c_void, bytes: usize) -> c_int {
    static int sun6i_rtc_nvmem_read(void *priv, unsigned int offset, void *_val, size_t bytes)
    {
    struct sun6i_rtc_dev *chip = priv;
    u32 *val = _val;
    int i;
    for (i = 0; i < bytes / 4; ++i)
    val[i] = readl(chip.base + SUN6I_GP_DATA + offset + 4 * i);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sun6i_rtc_nvmem_write(priv: *mut c_void, offset: c_uint, _val: *mut c_void, bytes: usize) -> c_int {
    static int sun6i_rtc_nvmem_write(void *priv, unsigned int offset, void *_val, size_t bytes)
    {
    struct sun6i_rtc_dev *chip = priv;
    u32 *val = _val;
    int i;
    for (i = 0; i < bytes / 4; ++i)
    writel(val[i], chip.base + SUN6I_GP_DATA + offset + 4 * i);
    return 0;
    }
    static struct nvmem_config sun6i_rtc_nvmem_cfg = {
    .type		= NVMEM_TYPE_BATTERY_BACKED,
    .reg_read	= sun6i_rtc_nvmem_read,
    .reg_write	= sun6i_rtc_nvmem_write,
    .size		= SUN6I_GP_DATA_SIZE,
    .word_size	= 4,
    .stride		= 4,
    };

// Enable IRQ wake on suspend, to wake up from RTC.
#[no_mangle]
unsafe extern "C" fn sun6i_rtc_suspend(dev: *mut device) -> c_int {
    static int sun6i_rtc_suspend(struct device *dev)
    {
    struct sun6i_rtc_dev *chip = dev_get_drvdata(dev);
    if (device_may_wakeup(dev))
    enable_irq_wake(chip.irq);
    return 0;
    }
// Disable IRQ wake on resume.
#[no_mangle]
unsafe extern "C" fn sun6i_rtc_resume(dev: *mut device) -> c_int {
    static int sun6i_rtc_resume(struct device *dev)
    {
    struct sun6i_rtc_dev *chip = dev_get_drvdata(dev);
    if (device_may_wakeup(dev))
    disable_irq_wake(chip.irq);
    return 0;
    }

    static SIMPLE_DEV_PM_OPS(sun6i_rtc_pm_ops,
    sun6i_rtc_suspend, sun6i_rtc_resume);
#[no_mangle]
unsafe extern "C" fn sun6i_rtc_bus_clk_cleanup(data: *mut c_void) {
    static void sun6i_rtc_bus_clk_cleanup(void *data)
    {
    struct clk *bus_clk = data;
    clk_disable_unprepare(bus_clk);
    }
#[no_mangle]
unsafe extern "C" fn sun6i_rtc_probe(pdev: *mut platform_device) -> c_int {
    static int sun6i_rtc_probe(struct platform_device *pdev)
    {
    struct sun6i_rtc_dev *chip = sun6i_rtc;
    struct device *dev = &pdev.dev;
    struct clk *bus_clk;
    int ret;
    bus_clk = devm_clk_get_optional(dev, "bus");
    if (IS_ERR(bus_clk))
    return PTR_ERR(bus_clk);
    if (bus_clk) {
    ret = clk_prepare_enable(bus_clk);
    if (ret)
    return ret;
    ret = devm_add_action_or_reset(dev, sun6i_rtc_bus_clk_cleanup,
    bus_clk);
    if (ret)
    return ret;
    }
    if (!chip) {
    chip = devm_kzalloc(&pdev.dev, sizeof(*chip), GFP_KERNEL);
    if (!chip)
    return -ENOMEM;
    spin_lock_init(&chip.lock);
    chip.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(chip.base))
    return PTR_ERR(chip.base);
    if (IS_REACHABLE(CONFIG_SUN6I_RTC_CCU)) {
    ret = sun6i_rtc_ccu_probe(dev, chip.base);
    if (ret)
    return ret;
    }
    }
    platform_set_drvdata(pdev, chip);
    chip.flags = (unsigned long)of_device_get_match_data(&pdev.dev);
    chip.irq = platform_get_irq(pdev, 0);
    if (chip.irq < 0)
    return chip.irq;
    ret = devm_request_irq(&pdev.dev, chip.irq, sun6i_rtc_alarmirq,
    0, dev_name(&pdev.dev), chip);
    if (ret) {
    dev_err(&pdev.dev, "Could not request IRQ\n");
    return ret;
    }
// clear the alarm counter value
    writel(0, chip.base + SUN6I_ALRM_COUNTER);
// disable counter alarm
    writel(0, chip.base + SUN6I_ALRM_EN);
// disable counter alarm interrupt
    writel(0, chip.base + SUN6I_ALRM_IRQ_EN);
// disable week alarm
    writel(0, chip.base + SUN6I_ALRM1_EN);
// disable week alarm interrupt
    writel(0, chip.base + SUN6I_ALRM1_IRQ_EN);
// clear counter alarm pending interrupts
    writel(SUN6I_ALRM_IRQ_STA_CNT_IRQ_PEND,
    chip.base + SUN6I_ALRM_IRQ_STA);
// clear week alarm pending interrupts
    writel(SUN6I_ALRM1_IRQ_STA_WEEK_IRQ_PEND,
    chip.base + SUN6I_ALRM1_IRQ_STA);
// disable alarm wakeup
    writel(0, chip.base + SUN6I_ALARM_CONFIG);
    clk_prepare_enable(chip.losc);
    device_init_wakeup(&pdev.dev, true);
    chip.rtc = devm_rtc_allocate_device(&pdev.dev);
    if (IS_ERR(chip.rtc))
    return PTR_ERR(chip.rtc);
    chip.rtc.ops = &sun6i_rtc_ops;
    if (chip.flags & RTC_LINEAR_DAY)
    chip.rtc.range_max = (65536 * SECS_PER_DAY) - 1;
    else
    chip.rtc.range_max = 2019686399LL; /* 2033-12-31 23:59:59 */
    ret = devm_rtc_register_device(chip.rtc);
    if (ret)
    return ret;
    sun6i_rtc_nvmem_cfg.priv = chip;
    ret = devm_rtc_nvmem_register(chip.rtc, &sun6i_rtc_nvmem_cfg);
    if (ret)
    return ret;
    return 0;
    }
//
// As far as RTC functionality goes, all models are the same. The
// datasheets claim that different models have different number of
// registers available for non-volatile storage, but experiments show
// that all SoCs have 16 registers available for this purpose.
//
    static const struct of_device_id sun6i_rtc_dt_ids[] = {
    { .compatible = "allwinner,sun6i-a31-rtc" },
    { .compatible = "allwinner,sun8i-a23-rtc" },
    { .compatible = "allwinner,sun8i-h3-rtc" },
    { .compatible = "allwinner,sun8i-r40-rtc" },
    { .compatible = "allwinner,sun8i-v3-rtc" },
    { .compatible = "allwinner,sun50i-h5-rtc" },
    { .compatible = "allwinner,sun50i-h6-rtc" },
    { .compatible = "allwinner,sun50i-h616-rtc",
    .data = (void *)RTC_LINEAR_DAY },
    { .compatible = "allwinner,sun50i-r329-rtc",
    .data = (void *)RTC_LINEAR_DAY },
    { /* sentinel */ },
    };
    MODULE_DEVICE_TABLE(of, sun6i_rtc_dt_ids);
    static struct platform_driver sun6i_rtc_driver = {
    .probe		= sun6i_rtc_probe,
    .driver		= {
    .name		= "sun6i-rtc",
    .of_match_table = sun6i_rtc_dt_ids,
    .pm = &sun6i_rtc_pm_ops,
    },
    };
    builtin_platform_driver(sun6i_rtc_driver);
