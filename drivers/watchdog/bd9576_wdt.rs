//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/bd9576_wdt.c
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
// Copyright (C) 2020 ROHM Semiconductors
//
// ROHM BD9576MUF and BD9573MUF Watchdog driver
//

    static bool nowayout;
    module_param(nowayout, bool, 0);
    MODULE_PARM_DESC(nowayout,
    "Watchdog cannot be stopped once started (default=\"false\")");
pub const HW_MARGIN_MIN: c_int = 2;
pub const HW_MARGIN_MAX: c_int = 4416;
pub const BD957X_WDT_DEFAULT_MARGIN: c_int = 4416;
pub const WATCHDOG_TIMEOUT: c_int = 30;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bd9576_wdt_priv {
    pub gpiod_ping: *mut gpio_desc,
    pub gpiod_en: *mut gpio_desc,
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub wdd: watchdog_device,
}

#[no_mangle]
unsafe extern "C" fn bd9576_wdt_disable(priv: *mut bd9576_wdt_priv) {
    static void bd9576_wdt_disable(struct bd9576_wdt_priv *priv)
    {
    gpiod_set_value_cansleep(priv.gpiod_en, 0);
    }
#[no_mangle]
unsafe extern "C" fn bd9576_wdt_ping(wdd: *mut watchdog_device) -> c_int {
    static int bd9576_wdt_ping(struct watchdog_device *wdd)
    {
    struct bd9576_wdt_priv *priv = watchdog_get_drvdata(wdd);
// Pulse
    gpiod_set_value_cansleep(priv.gpiod_ping, 1);
    gpiod_set_value_cansleep(priv.gpiod_ping, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bd9576_wdt_start(wdd: *mut watchdog_device) -> c_int {
    static int bd9576_wdt_start(struct watchdog_device *wdd)
    {
    struct bd9576_wdt_priv *priv = watchdog_get_drvdata(wdd);
    gpiod_set_value_cansleep(priv.gpiod_en, 1);
    return bd9576_wdt_ping(wdd);
    }
#[no_mangle]
unsafe extern "C" fn bd9576_wdt_stop(wdd: *mut watchdog_device) -> c_int {
    static int bd9576_wdt_stop(struct watchdog_device *wdd)
    {
    struct bd9576_wdt_priv *priv = watchdog_get_drvdata(wdd);
    bd9576_wdt_disable(priv);
    return 0;
    }
    static const struct watchdog_info bd957x_wdt_ident = {
    .options	= WDIOF_MAGICCLOSE | WDIOF_KEEPALIVEPING |
    WDIOF_SETTIMEOUT,
    .identity	= "BD957x Watchdog",
    };
    static const struct watchdog_ops bd957x_wdt_ops = {
    .owner		= THIS_MODULE,
    .start		= bd9576_wdt_start,
    .stop		= bd9576_wdt_stop,
    .ping		= bd9576_wdt_ping,
    };
// Unit is hundreds of uS
pub const FASTNG_MIN: c_int = 23;
#[no_mangle]
unsafe extern "C" fn find_closest_fast(target: c_int, sel: *mut c_int, val: *mut c_int) -> c_int {
    static int find_closest_fast(int target, int *sel, int *val)
    {
    int i;
    let mut window: c_int = FASTNG_MIN;
    for (i = 0; i < 8 && window < target; i++)
    window <<= 1;
// val = window;
// sel = i;
    if (i == 8)
    return -EINVAL;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn find_closest_slow_by_fast(fast_val: c_int, target: c_int, slowsel: *mut c_int) -> c_int {
    static int find_closest_slow_by_fast(int fast_val, int target, int *slowsel)
    {
    int sel;
    static const int multipliers[] = {2, 3, 7, 15};
    for (sel = 0; sel < ARRAY_SIZE(multipliers) &&
    multipliers[sel] * fast_val < target; sel++)
    ;
    if (sel == ARRAY_SIZE(multipliers))
    return -EINVAL;
// slowsel = sel;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn find_closest_slow(target: c_int, slow_sel: *mut c_int, fast_sel: *mut c_int) -> c_int {
    static int find_closest_slow(int target, int *slow_sel, int *fast_sel)
    {
    static const int multipliers[] = {2, 3, 7, 15};
    int i, j;
    let mut val: c_int = 0;
    let mut window: c_int = FASTNG_MIN;
    for (i = 0; i < 8; i++) {
    for (j = 0; j < ARRAY_SIZE(multipliers); j++) {
    int slow;
    slow = window * multipliers[j];
    if (slow >= target && (!val || slow < val)) {
    val = slow;
// fast_sel = i;
// slow_sel = j;
    }
    }
    window <<= 1;
    }
    if (!val)
    return -EINVAL;
    return 0;
    }

pub const BD957X_WDG_TYPE_SLOW: c_int = 0;

pub const BD957X_WDG_NG_RATIO_MASK: c_uint = 0x18;
pub const BD957X_WDG_FASTNG_MASK: c_uint = 0x7;
    static int bd957x_set_wdt_mode(struct bd9576_wdt_priv *priv, int hw_margin,
    int hw_margin_min)
    {
    int ret, fastng, slowng, type, reg, mask;
    struct device *dev = priv.dev;
// convert to 100uS
    hw_margin *= 10;
    hw_margin_min *= 10;
    if (hw_margin_min) {
    int min;
    type = BD957X_WDG_TYPE_WINDOW;
    dev_dbg(dev, "Setting type WINDOW 0x%x\n", type);
    ret = find_closest_fast(hw_margin_min, &fastng, &min);
    if (ret) {
    dev_err(dev, "bad WDT window for fast timeout\n");
    return ret;
    }
    ret = find_closest_slow_by_fast(min, hw_margin, &slowng);
    if (ret) {
    dev_err(dev, "bad WDT window\n");
    return ret;
    }
    } else {
    type = BD957X_WDG_TYPE_SLOW;
    dev_dbg(dev, "Setting type SLOW 0x%x\n", type);
    ret = find_closest_slow(hw_margin, &slowng, &fastng);
    if (ret) {
    dev_err(dev, "bad WDT window\n");
    return ret;
    }
    }
    slowng <<= ffs(BD957X_WDG_NG_RATIO_MASK) - 1;
    reg = type | slowng | fastng;
    mask = BD957X_WDG_TYPE_MASK | BD957X_WDG_NG_RATIO_MASK |
    BD957X_WDG_FASTNG_MASK;
    ret = regmap_update_bits(priv.regmap, BD957X_REG_WDT_CONF,
    mask, reg);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn bd9576_wdt_probe(pdev: *mut platform_device) -> c_int {
    static int bd9576_wdt_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct bd9576_wdt_priv *priv;
    u32 hw_margin[2];
    let mut hw_margin_max: u32 = BD957X_WDT_DEFAULT_MARGIN, hw_margin_min = 0;
    int count;
    int ret;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    platform_set_drvdata(pdev, priv);
    priv.dev = dev;
    priv.regmap = dev_get_regmap(dev.parent, core::ptr::null_mut());
    if (!priv.regmap) {
    dev_err(dev, "No regmap found\n");
    return -ENODEV;
    }
    priv.gpiod_en = devm_fwnode_gpiod_get(dev, dev_fwnode(dev.parent),
    "rohm,watchdog-enable",
    GPIOD_OUT_LOW,
    "watchdog-enable");
    if (IS_ERR(priv.gpiod_en))
    return dev_err_probe(dev, PTR_ERR(priv.gpiod_en),
    "getting watchdog-enable GPIO failed\n");
    priv.gpiod_ping = devm_fwnode_gpiod_get(dev, dev_fwnode(dev.parent),
    "rohm,watchdog-ping",
    GPIOD_OUT_LOW,
    "watchdog-ping");
    if (IS_ERR(priv.gpiod_ping))
    return dev_err_probe(dev, PTR_ERR(priv.gpiod_ping),
    "getting watchdog-ping GPIO failed\n");
    count = device_property_count_u32(dev.parent, "rohm,hw-timeout-ms");
    if (count < 0 && count != -EINVAL)
    return count;
    if (count > 0) {
    if (count > ARRAY_SIZE(hw_margin))
    return -EINVAL;
    ret = device_property_read_u32_array(dev.parent,
    "rohm,hw-timeout-ms",
    hw_margin, count);
    if (ret < 0)
    return ret;
    if (count == 1)
    hw_margin_max = hw_margin[0];
    if (count == 2) {
    hw_margin_max = hw_margin[1];
    hw_margin_min = hw_margin[0];
    }
    }
    ret = bd957x_set_wdt_mode(priv, hw_margin_max, hw_margin_min);
    if (ret)
    return ret;
    watchdog_set_drvdata(&priv.wdd, priv);
    priv.wdd.info			= &bd957x_wdt_ident;
    priv.wdd.ops			= &bd957x_wdt_ops;
    priv.wdd.min_hw_heartbeat_ms	= hw_margin_min;
    priv.wdd.max_hw_heartbeat_ms	= hw_margin_max;
    priv.wdd.parent		= dev;
    priv.wdd.timeout		= WATCHDOG_TIMEOUT;
    watchdog_init_timeout(&priv.wdd, 0, dev);
    watchdog_set_nowayout(&priv.wdd, nowayout);
    watchdog_stop_on_reboot(&priv.wdd);
    return devm_watchdog_register_device(dev, &priv.wdd);
    }
    static struct platform_driver bd9576_wdt_driver = {
    .driver	= {
    .name = "bd9576-wdt",
    },
    .probe	= bd9576_wdt_probe,
    };
    module_platform_driver(bd9576_wdt_driver);
    MODULE_AUTHOR("Matti Vaittinen <matti.vaittinen@fi.rohmeurope.com>");
    MODULE_DESCRIPTION("ROHM BD9576/BD9573 Watchdog driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:bd9576-wdt");
