//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/renesas_wwdt.c
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
// Driver for the Renesas Window Watchdog Timer (WWDT)
//
// The WWDT can only be setup once after boot. Because we cannot know if this
// already happened in early boot stages, it is mandated that the firmware
// configures the watchdog. Linux then adapts according to the given setup.
// Note that this watchdog reports in the default configuration an overflow to
// the Error Control Module which then decides further actions. Or the WWDT is
// configured to generate an interrupt.
//

pub const WDTA0WDTE: c_uint = 0x00;

pub const WDTA0_KEY: c_uint = 0x2c;
pub const WDTA0MD: c_uint = 0x0c;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wwdt_priv {
    pub base: *mut void __iomem,
    pub wdev: watchdog_device,
}

#[no_mangle]
unsafe extern "C" fn wwdt_start(wdev: *mut watchdog_device) -> c_int {
    static int wwdt_start(struct watchdog_device *wdev)
    {
    struct wwdt_priv *priv = container_of(wdev, struct wwdt_priv, wdev);
    writeb(WDTA0RUN | WDTA0_KEY, priv.base + WDTA0WDTE);
    return 0;
    }
    static const struct watchdog_info wwdt_ident = {
    .options = WDIOF_KEEPALIVEPING | WDIOF_ALARMONLY,
    .identity = "Renesas Window Watchdog",
    };
    static const struct watchdog_ops wwdt_ops = {
    .owner = THIS_MODULE,
    .start = wwdt_start,
    };
#[no_mangle]
unsafe extern "C" fn wwdt_error_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t wwdt_error_irq(int irq, void *dev_id)
    {
    struct device *dev = dev_id;
    dev_warn(dev, "Watchdog timed out\n");
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn wwdt_pretimeout_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t wwdt_pretimeout_irq(int irq, void *dev_id)
    {
    struct watchdog_device *wdev = dev_id;
    watchdog_notify_pretimeout(wdev);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn wwdt_probe(pdev: *mut platform_device) -> c_int {
    static int wwdt_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct wwdt_priv *priv;
    struct watchdog_device *wdev;
    struct clk *clk;
    unsigned long rate;
    unsigned int interval, window_size;
    int ret;
    u8 val;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.base))
    return PTR_ERR(priv.base);
    clk = devm_clk_get(dev, "cnt");
    if (IS_ERR(clk))
    return PTR_ERR(clk);
    rate = clk_get_rate(clk);
    if (!rate)
    return -EINVAL;
    wdev = &priv.wdev;
    val = readb(priv.base + WDTA0WDTE);
    if (val & WDTA0RUN)
    set_bit(WDOG_HW_RUNNING, &wdev.status);
    val = readb(priv.base + WDTA0MD);
    interval = 1 << (9 + WDTA0OVF(val));
// size of the closed(!) window per mille
    window_size = 250 * (3 - WDTA0WS(val));
    wdev.info = &wwdt_ident;
    wdev.ops = &wwdt_ops;
    wdev.parent = dev;
    wdev.min_hw_heartbeat_ms = window_size * interval / rate;
    wdev.max_hw_heartbeat_ms = 1000 * interval / rate;
    wdev.timeout = DIV_ROUND_UP(wdev.max_hw_heartbeat_ms, 1000);
    watchdog_set_nowayout(wdev, true);
    if (!(val & WDTA0ERM)) {
    ret = platform_get_irq_byname(pdev, "error");
    if (ret < 0)
    return ret;
    ret = devm_request_threaded_irq(dev, ret, core::ptr::null_mut(), wwdt_error_irq,
    IRQF_ONESHOT, core::ptr::null_mut(), dev);
    if (ret < 0)
    return ret;
    }
    if (val & WDTA0WIE) {
    ret = platform_get_irq_byname(pdev, "pretimeout");
    if (ret < 0)
    return ret;
    ret = devm_request_threaded_irq(dev, ret, core::ptr::null_mut(), wwdt_pretimeout_irq,
    IRQF_ONESHOT, core::ptr::null_mut(), wdev);
    if (ret < 0)
    return ret;
    }
    devm_watchdog_register_device(dev, wdev);
    return 0;
    }
    static const struct of_device_id renesas_wwdt_ids[] = {
    { .compatible = "renesas,rcar-gen3-wwdt", },
    { .compatible = "renesas,rcar-gen4-wwdt", },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, renesas_wwdt_ids);
    static struct platform_driver renesas_wwdt_driver = {
    .driver = {
    .name = "renesas_wwdt",
    .of_match_table = renesas_wwdt_ids,
    },
    .probe = wwdt_probe,
    };
    module_platform_driver(renesas_wwdt_driver);
    MODULE_DESCRIPTION("Renesas Window Watchdog (WWDT) Driver");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Wolfram Sang <wsa+renesas@sang-engineering.com>");
