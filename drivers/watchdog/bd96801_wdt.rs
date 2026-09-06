//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/bd96801_wdt.c
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
// Copyright (C) 2024 ROHM Semiconductors
//
// ROHM BD96801 watchdog driver
//

    static bool nowayout;
    module_param(nowayout, bool, 0);
    MODULE_PARM_DESC(nowayout,
    "Watchdog cannot be stopped once started (default=\"false\")");
pub const BD96801_WD_TMO_SHORT_MASK: c_uint = 0x70;
pub const BD96801_WD_RATIO_MASK: c_uint = 0x3;
pub const BD96801_WD_TYPE_MASK: c_uint = 0x4;
pub const BD96801_WD_TYPE_SLOW: c_uint = 0x4;
pub const BD96801_WD_TYPE_WIN: c_uint = 0x0;
pub const BD96801_WD_EN_MASK: c_uint = 0x3;
pub const BD96801_WD_IF_EN: c_uint = 0x1;
pub const BD96801_WD_QA_EN: c_uint = 0x2;
pub const BD96801_WD_DISABLE: c_uint = 0x0;
pub const BD96801_WD_ASSERT_MASK: c_uint = 0x8;
pub const BD96801_WD_ASSERT_RST: c_uint = 0x8;
pub const BD96801_WD_ASSERT_IRQ: c_uint = 0x0;
pub const BD96801_WD_FEED_MASK: c_uint = 0x1;
pub const BD96801_WD_FEED: c_uint = 0x1;
// 1.1 mS
pub const FASTNG_MIN: c_int = 11;

pub const BD96801_WDT_DEFAULT_MARGIN_MS: c_int = 1843;
// Unit is seconds
pub const DEFAULT_TIMEOUT: c_int = 30;
//
// BD96801 WDG supports window mode so the TMO consists of SHORT and LONG
// timeout values. SHORT time is meaningful only in window mode where feeding
// period shorter than SHORT would be an error. LONG time is used to detect if
// feeding is not occurring within given time limit (SoC SW hangs). The LONG
// timeout time is a multiple of (2, 4, 8 or 16 times) the SHORT timeout.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wdtbd96801 {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub wdt: watchdog_device,
}

#[no_mangle]
unsafe extern "C" fn bd96801_wdt_ping(wdt: *mut watchdog_device) -> c_int {
    static int bd96801_wdt_ping(struct watchdog_device *wdt)
    {
    struct wdtbd96801 *w = watchdog_get_drvdata(wdt);
    return regmap_update_bits(w.regmap, BD96801_REG_WD_FEED,
    BD96801_WD_FEED_MASK, BD96801_WD_FEED);
    }
#[no_mangle]
unsafe extern "C" fn bd96801_wdt_start(wdt: *mut watchdog_device) -> c_int {
    static int bd96801_wdt_start(struct watchdog_device *wdt)
    {
    struct wdtbd96801 *w = watchdog_get_drvdata(wdt);
    return regmap_update_bits(w.regmap, BD96801_REG_WD_CONF,
    BD96801_WD_EN_MASK, BD96801_WD_IF_EN);
    }
#[no_mangle]
unsafe extern "C" fn bd96801_wdt_stop(wdt: *mut watchdog_device) -> c_int {
    static int bd96801_wdt_stop(struct watchdog_device *wdt)
    {
    struct wdtbd96801 *w = watchdog_get_drvdata(wdt);
    return regmap_update_bits(w.regmap, BD96801_REG_WD_CONF,
    BD96801_WD_EN_MASK, BD96801_WD_DISABLE);
    }
    static const struct watchdog_info bd96801_wdt_info = {
    .options	= WDIOF_MAGICCLOSE | WDIOF_KEEPALIVEPING |
    WDIOF_SETTIMEOUT,
    .identity	= "BD96801 Watchdog",
    };
    static const struct watchdog_ops bd96801_wdt_ops = {
    .start		= bd96801_wdt_start,
    .stop		= bd96801_wdt_stop,
    .ping		= bd96801_wdt_ping,
    };
#[no_mangle]
unsafe extern "C" fn find_closest_fast(target: c_uint, sel: *mut c_int, val: *mut c_uint) -> c_int {
    static int find_closest_fast(unsigned int target, int *sel, unsigned int *val)
    {
    let mut window: c_uint = FASTNG_MIN;
    int i;
    for (i = 0; i < 8 && window < target; i++)
    window <<= 1;
    if (i == 8)
    return -EINVAL;
// val = window;
// sel = i;
    return 0;
    }
    static int find_closest_slow_by_fast(unsigned int fast_val, unsigned int *target,
    int *slowsel)
    {
    static const int multipliers[] = {2, 4, 8, 16};
    int sel;
    for (sel = 0; sel < ARRAY_SIZE(multipliers) &&
    multipliers[sel] * fast_val < *target; sel++)
    ;
    if (sel == ARRAY_SIZE(multipliers))
    return -EINVAL;
// slowsel = sel;
// target = multipliers[sel] * fast_val;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn find_closest_slow(target: *mut c_uint, slow_sel: *mut c_int, fast_sel: *mut c_int) -> c_int {
    static int find_closest_slow(unsigned int *target, int *slow_sel, int *fast_sel)
    {
    static const int multipliers[] = {2, 4, 8, 16};
    let mut window: c_uint = FASTNG_MIN;
    let mut val: c_uint = 0;
    int i, j;
    for (i = 0; i < 8; i++) {
    for (j = 0; j < ARRAY_SIZE(multipliers); j++) {
    unsigned int slow;
    slow = window * multipliers[j];
    if (slow >= *target && (!val || slow < val)) {
    val = slow;
// fast_sel = i;
// slow_sel = j;
    }
    }
    window <<= 1;
    }
    if (!val)
    return -EINVAL;
// target = val;
    return 0;
    }
    static int bd96801_set_wdt_mode(struct wdtbd96801 *w, unsigned int hw_margin,
    unsigned int hw_margin_min)
    {
    int fastng, slowng, type, ret, reg, mask;
    struct device *dev = w.dev;
    if (hw_margin_min * 1000 > FASTNG_MAX_US) {
    dev_err(dev, "Unsupported fast timeout %u uS [max %u]\n",
    hw_margin_min * 1000, FASTNG_MAX_US);
    return -EINVAL;
    }
    if (hw_margin * 1000 > SLOWNG_MAX_US) {
    dev_err(dev, "Unsupported slow timeout %u uS [max %u]\n",
    hw_margin * 1000, SLOWNG_MAX_US);
    return -EINVAL;
    }
//
// Convert to 100uS to guarantee reasonable timeouts fit in
// 32bit maintaining also a decent accuracy.
//
    hw_margin *= 10;
    hw_margin_min *= 10;
    if (hw_margin_min) {
    unsigned int min;
    type = BD96801_WD_TYPE_WIN;
    dev_dbg(dev, "Setting type WINDOW 0x%x\n", type);
    ret = find_closest_fast(hw_margin_min, &fastng, &min);
    if (ret)
    return ret;
    ret = find_closest_slow_by_fast(min, &hw_margin, &slowng);
    if (ret) {
    dev_err(dev,
    "can't support slow timeout %u uS using fast %u uS. [max slow %u uS]\n",
    hw_margin * 100, min * 100, min * 100 * 16);
    return ret;
    }
    w.wdt.min_hw_heartbeat_ms = min / 10;
    } else {
    type = BD96801_WD_TYPE_SLOW;
    dev_dbg(dev, "Setting type SLOW 0x%x\n", type);
    ret = find_closest_slow(&hw_margin, &slowng, &fastng);
    if (ret)
    return ret;
    }
    w.wdt.max_hw_heartbeat_ms = hw_margin / 10;
    fastng = FIELD_PREP(BD96801_WD_TMO_SHORT_MASK, fastng);
    reg = slowng | fastng;
    mask = BD96801_WD_RATIO_MASK | BD96801_WD_TMO_SHORT_MASK;
    ret = regmap_update_bits(w.regmap, BD96801_REG_WD_TMO,
    mask, reg);
    if (ret)
    return ret;
    ret = regmap_update_bits(w.regmap, BD96801_REG_WD_CONF,
    BD96801_WD_TYPE_MASK, type);
    return ret;
    }
    static int bd96801_set_heartbeat_from_hw(struct wdtbd96801 *w,
    unsigned int conf_reg)
    {
    int ret;
    unsigned int val, sel, fast;
//
// The BD96801 supports a somewhat peculiar QA-mode, which we do not
// support in this driver. If the QA-mode is enabled then we just
// warn and bail-out.
//
    if ((conf_reg & BD96801_WD_EN_MASK) != BD96801_WD_IF_EN) {
    dev_err(w.dev, "watchdog set to Q&A mode - exiting\n");
    return -EINVAL;
    }
    ret = regmap_read(w.regmap, BD96801_REG_WD_TMO, &val);
    if (ret)
    return ret;
    sel = FIELD_GET(BD96801_WD_TMO_SHORT_MASK, val);
    fast = FASTNG_MIN << sel;
    sel = (val & BD96801_WD_RATIO_MASK) + 1;
    w.wdt.max_hw_heartbeat_ms = (fast << sel) / 10;
    if ((conf_reg & BD96801_WD_TYPE_MASK) == BD96801_WD_TYPE_WIN)
    w.wdt.min_hw_heartbeat_ms = fast / 10;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn init_wdg_hw(w: *mut wdtbd96801) -> c_int {
    static int init_wdg_hw(struct wdtbd96801 *w)
    {
    u32 hw_margin[2];
    int count, ret;
    let mut hw_margin_max: u32 = BD96801_WDT_DEFAULT_MARGIN_MS, hw_margin_min = 0;
    count = device_property_count_u32(w.dev.parent, "rohm,hw-timeout-ms");
    if (count < 0 && count != -EINVAL)
    return count;
    if (count > 0) {
    if (count > ARRAY_SIZE(hw_margin))
    return -EINVAL;
    ret = device_property_read_u32_array(w.dev.parent,
    "rohm,hw-timeout-ms",
    &hw_margin[0], count);
    if (ret < 0)
    return ret;
    if (count == 1)
    hw_margin_max = hw_margin[0];
    if (count == 2) {
    if (hw_margin[1] > hw_margin[0]) {
    hw_margin_max = hw_margin[1];
    hw_margin_min = hw_margin[0];
    } else {
    hw_margin_max = hw_margin[0];
    hw_margin_min = hw_margin[1];
    }
    }
    }
    ret = bd96801_set_wdt_mode(w, hw_margin_max, hw_margin_min);
    if (ret)
    return ret;
    ret = device_property_match_string(w.dev.parent, "rohm,wdg-action",
    "prstb");
    if (ret >= 0) {
    ret = regmap_update_bits(w.regmap, BD96801_REG_WD_CONF,
    BD96801_WD_ASSERT_MASK,
    BD96801_WD_ASSERT_RST);
    return ret;
    }
    ret = device_property_match_string(w.dev.parent, "rohm,wdg-action",
    "intb-only");
    if (ret >= 0) {
    ret = regmap_update_bits(w.regmap, BD96801_REG_WD_CONF,
    BD96801_WD_ASSERT_MASK,
    BD96801_WD_ASSERT_IRQ);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bd96801_irq_hnd(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t bd96801_irq_hnd(int irq, void *data)
    {
    emergency_restart();
    return IRQ_NONE;
    }
#[no_mangle]
unsafe extern "C" fn bd96801_wdt_probe(pdev: *mut platform_device) -> c_int {
    static int bd96801_wdt_probe(struct platform_device *pdev)
    {
    struct wdtbd96801 *w;
    int ret, irq;
    unsigned int val;
    w = devm_kzalloc(&pdev.dev, sizeof(*w), GFP_KERNEL);
    if (!w)
    return -ENOMEM;
    w.regmap = dev_get_regmap(pdev.dev.parent, core::ptr::null_mut());
    w.dev = &pdev.dev;
    w.wdt.info = &bd96801_wdt_info;
    w.wdt.ops =  &bd96801_wdt_ops;
    w.wdt.parent = pdev.dev.parent;
    w.wdt.timeout = DEFAULT_TIMEOUT;
    watchdog_set_drvdata(&w.wdt, w);
    ret = regmap_read(w.regmap, BD96801_REG_WD_CONF, &val);
    if (ret)
    return dev_err_probe(&pdev.dev, ret,
    "Failed to get the watchdog state\n");
//
// If the WDG is already enabled we assume it is configured by boot.
// In this case we just update the hw-timeout based on values set to
// the timeout / mode registers and leave the hardware configs
// untouched.
//
    if ((val & BD96801_WD_EN_MASK) != BD96801_WD_DISABLE) {
    dev_dbg(&pdev.dev, "watchdog was running during probe\n");
    ret = bd96801_set_heartbeat_from_hw(w, val);
    if (ret)
    return ret;
    set_bit(WDOG_HW_RUNNING, &w.wdt.status);
    } else {
// If WDG is not running so we will initializate it
    ret = init_wdg_hw(w);
    if (ret)
    return ret;
    }
    dev_dbg(w.dev, "heartbeat set to %u - %u\n",
    w.wdt.min_hw_heartbeat_ms, w.wdt.max_hw_heartbeat_ms);
    watchdog_init_timeout(&w.wdt, 0, pdev.dev.parent);
    watchdog_set_nowayout(&w.wdt, nowayout);
    watchdog_stop_on_reboot(&w.wdt);
    irq = platform_get_irq_byname(pdev, "bd96801-wdg");
    if (irq > 0) {
    ret = devm_request_threaded_irq(&pdev.dev, irq, core::ptr::null_mut(),
    bd96801_irq_hnd,
    IRQF_ONESHOT,  "bd96801-wdg",
    core::ptr::null_mut());
    if (ret)
    return ret;
    }
    return devm_watchdog_register_device(&pdev.dev, &w.wdt);
    }
    static const struct platform_device_id bd96801_wdt_id[] = {
    { "bd96801-wdt", },
    { }
    };
    MODULE_DEVICE_TABLE(platform, bd96801_wdt_id);
    static struct platform_driver bd96801_wdt = {
    .driver = {
    .name = "bd96801-wdt"
    },
    .probe = bd96801_wdt_probe,
    .id_table = bd96801_wdt_id,
    };
    module_platform_driver(bd96801_wdt);
    MODULE_AUTHOR("Matti Vaittinen <matti.vaittinen@fi.rohmeurope.com>");
    MODULE_DESCRIPTION("BD96801 watchdog driver");
    MODULE_LICENSE("GPL");
