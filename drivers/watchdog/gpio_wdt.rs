//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/gpio_wdt.c
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
// Driver for watchdog device controlled through GPIO-line
//
// Author: 2013, Alexander Shiyan <shc_work@mail.ru>
//

    let mut nowayout: static bool = WATCHDOG_NOWAYOUT;
    module_param(nowayout, bool, 0);
    MODULE_PARM_DESC(nowayout,
    "Watchdog cannot be stopped once started (default="
    __MODULE_STRING(WATCHDOG_NOWAYOUT) ")");
pub const SOFT_TIMEOUT_MIN: c_int = 1;
pub const SOFT_TIMEOUT_DEF: c_int = 60;
    enum {
    HW_ALGO_TOGGLE,
    HW_ALGO_LEVEL,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_wdt_priv {
    pub gpiod: *mut gpio_desc,
    pub state: bool,
    pub always_running: bool,
    pub hw_algo: c_uint,
    pub wdd: watchdog_device,
}

#[no_mangle]
unsafe extern "C" fn gpio_wdt_disable(priv: *mut gpio_wdt_priv) {
    static void gpio_wdt_disable(struct gpio_wdt_priv *priv)
    {
// Eternal ping
    gpiod_set_value_cansleep(priv.gpiod, 1);
// Put GPIO back to tristate
    if (priv.hw_algo == HW_ALGO_TOGGLE)
    gpiod_direction_input(priv.gpiod);
    }
#[no_mangle]
unsafe extern "C" fn gpio_wdt_ping(wdd: *mut watchdog_device) -> c_int {
    static int gpio_wdt_ping(struct watchdog_device *wdd)
    {
    struct gpio_wdt_priv *priv = watchdog_get_drvdata(wdd);
    switch (priv.hw_algo) {
    case HW_ALGO_TOGGLE:
// Toggle output pin
    priv.state = !priv.state;
    gpiod_set_value_cansleep(priv.gpiod, priv.state);
    break;
    case HW_ALGO_LEVEL:
// Pulse
    gpiod_set_value_cansleep(priv.gpiod, 1);
    udelay(1);
    gpiod_set_value_cansleep(priv.gpiod, 0);
    break;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gpio_wdt_start(wdd: *mut watchdog_device) -> c_int {
    static int gpio_wdt_start(struct watchdog_device *wdd)
    {
    struct gpio_wdt_priv *priv = watchdog_get_drvdata(wdd);
    priv.state = 0;
    gpiod_direction_output(priv.gpiod, priv.state);
    set_bit(WDOG_HW_RUNNING, &wdd.status);
    return gpio_wdt_ping(wdd);
    }
#[no_mangle]
unsafe extern "C" fn gpio_wdt_stop(wdd: *mut watchdog_device) -> c_int {
    static int gpio_wdt_stop(struct watchdog_device *wdd)
    {
    struct gpio_wdt_priv *priv = watchdog_get_drvdata(wdd);
    if (!priv.always_running) {
    gpio_wdt_disable(priv);
    } else {
    set_bit(WDOG_HW_RUNNING, &wdd.status);
    }
    return 0;
    }
    static const struct watchdog_info gpio_wdt_ident = {
    .options	= WDIOF_MAGICCLOSE | WDIOF_KEEPALIVEPING |
    WDIOF_SETTIMEOUT,
    .identity	= "GPIO Watchdog",
    };
    static const struct watchdog_ops gpio_wdt_ops = {
    .owner		= THIS_MODULE,
    .start		= gpio_wdt_start,
    .stop		= gpio_wdt_stop,
    .ping		= gpio_wdt_ping,
    };
#[no_mangle]
unsafe extern "C" fn gpio_wdt_probe(pdev: *mut platform_device) -> c_int {
    static int gpio_wdt_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct gpio_wdt_priv *priv;
    enum gpiod_flags gflags;
    unsigned int hw_margin;
    const char *algo;
    int ret;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    platform_set_drvdata(pdev, priv);
    ret = device_property_read_string(dev, "hw_algo", &algo);
    if (ret)
    return ret;
    if (!strcmp(algo, "toggle")) {
    priv.hw_algo = HW_ALGO_TOGGLE;
    gflags = GPIOD_IN;
    } else if (!strcmp(algo, "level")) {
    priv.hw_algo = HW_ALGO_LEVEL;
    gflags = GPIOD_OUT_LOW;
    } else {
    return -EINVAL;
    }
    priv.gpiod = devm_gpiod_get(dev, core::ptr::null_mut(), gflags);
    if (IS_ERR(priv.gpiod))
    return PTR_ERR(priv.gpiod);
    ret = device_property_read_u32(dev, "hw_margin_ms", &hw_margin);
    if (ret)
    return ret;
// Disallow values lower than 2 and higher than 65535 ms
    if (hw_margin < 2 || hw_margin > 65535)
    return -EINVAL;
    priv.always_running = device_property_read_bool(dev, "always-running");
    watchdog_set_drvdata(&priv.wdd, priv);
    priv.wdd.info		= &gpio_wdt_ident;
    priv.wdd.ops		= &gpio_wdt_ops;
    priv.wdd.min_timeout	= SOFT_TIMEOUT_MIN;
    priv.wdd.max_hw_heartbeat_ms = hw_margin;
    priv.wdd.parent	= dev;
    priv.wdd.timeout	= SOFT_TIMEOUT_DEF;
    watchdog_init_timeout(&priv.wdd, 0, dev);
    watchdog_set_nowayout(&priv.wdd, nowayout);
    watchdog_stop_on_reboot(&priv.wdd);
    if (priv.always_running)
    gpio_wdt_start(&priv.wdd);
    return devm_watchdog_register_device(dev, &priv.wdd);
    }
    static const struct of_device_id gpio_wdt_dt_ids[] = {
    { .compatible = "linux,wdt-gpio", },
    { }
    };
    MODULE_DEVICE_TABLE(of, gpio_wdt_dt_ids);
    static struct platform_driver gpio_wdt_driver = {
    .driver	= {
    .name		= "gpio-wdt",
    .of_match_table	= gpio_wdt_dt_ids,
    },
    .probe	= gpio_wdt_probe,
    };

#[no_mangle]
unsafe extern "C" fn gpio_wdt_init() -> int __init {
    static int __init gpio_wdt_init(void)
    {
    return platform_driver_register(&gpio_wdt_driver);
    }
    arch_initcall(gpio_wdt_init);

    module_platform_driver(gpio_wdt_driver);

    MODULE_AUTHOR("Alexander Shiyan <shc_work@mail.ru>");
    MODULE_DESCRIPTION("GPIO Watchdog");
    MODULE_LICENSE("GPL");
