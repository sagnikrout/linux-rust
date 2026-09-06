//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/lantiq_wdt.c
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
// Copyright (C) 2010 John Crispin <john@phrozen.org>
// Copyright (C) 2017 Hauke Mehrtens <hauke@hauke-m.de>
// Based on EP93xx wdt driver
//

pub const LTQ_XRX_RCU_RST_STAT: c_uint = 0x0014;

// CPU0 Reset Source Register
pub const LTQ_FALCON_SYS1_CPU0RS: c_uint = 0x0060;
// reset cause mask
pub const LTQ_FALCON_SYS1_CPU0RS_MASK: c_uint = 0x0007;
pub const LTQ_FALCON_SYS1_CPU0RS_WDT: c_uint = 0x02;
//
// Section 3.4 of the datasheet
// The password sequence protects the WDT control register from unintended
// write actions, which might cause malfunction of the WDT.
//
// essentially the following two magic passwords need to be written to allow
// IO access to the WDT core
//
pub const LTQ_WDT_CR_PW1: c_uint = 0x00BE0000;
pub const LTQ_WDT_CR_PW2: c_uint = 0x00DC0000;
pub const LTQ_WDT_CR: c_uint = 0x0		/* watchdog control register */;

// Pre-warning limit set to 1/16 of max WDT period

// set clock divider to 0x40000

pub const LTQ_WDT_SR: c_uint = 0x8		/* watchdog status register */;

pub const LTQ_WDT_DIVIDER: c_uint = 0x40000;
    let mut nowayout: static bool = WATCHDOG_NOWAYOUT;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ltq_wdt_hw {
    pub dev): *mut *mut int (bootstatus_get)(struct device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ltq_wdt_priv {
    pub wdt: watchdog_device,
    pub membase: *mut void __iomem,
    pub clk_rate: c_ulong,
}

#[no_mangle]
unsafe extern "C" fn ltq_wdt_r32(priv: *mut ltq_wdt_priv, offset: u32) -> u32 {
    static u32 ltq_wdt_r32(struct ltq_wdt_priv *priv, u32 offset)
    {
    return __raw_readl(priv.membase + offset);
    }
#[no_mangle]
unsafe extern "C" fn ltq_wdt_w32(priv: *mut ltq_wdt_priv, val: u32, offset: u32) {
    static void ltq_wdt_w32(struct ltq_wdt_priv *priv, u32 val, u32 offset)
    {
    __raw_writel(val, priv.membase + offset);
    }
    static void ltq_wdt_mask(struct ltq_wdt_priv *priv, u32 clear, u32 set,
    u32 offset)
    {
    let mut val: u32 = ltq_wdt_r32(priv, offset);
    val &= ~(clear);
    val |= set;
    ltq_wdt_w32(priv, val, offset);
    }
    static struct ltq_wdt_priv *ltq_wdt_get_priv(struct watchdog_device *wdt)
    {
    return container_of(wdt, struct ltq_wdt_priv, wdt);
    }
    static struct watchdog_info ltq_wdt_info = {
    .options = WDIOF_MAGICCLOSE | WDIOF_SETTIMEOUT | WDIOF_KEEPALIVEPING |
    WDIOF_CARDRESET,
    .identity = "ltq_wdt",
    };
#[no_mangle]
unsafe extern "C" fn ltq_wdt_start(wdt: *mut watchdog_device) -> c_int {
    static int ltq_wdt_start(struct watchdog_device *wdt)
    {
    struct ltq_wdt_priv *priv = ltq_wdt_get_priv(wdt);
    u32 timeout;
    timeout = wdt.timeout * priv.clk_rate;
    ltq_wdt_mask(priv, LTQ_WDT_CR_PW_MASK, LTQ_WDT_CR_PW1, LTQ_WDT_CR);
// write the second magic plus the configuration and new timeout
    ltq_wdt_mask(priv, LTQ_WDT_CR_PW_MASK | LTQ_WDT_CR_MAX_TIMEOUT,
    LTQ_WDT_CR_GEN | LTQ_WDT_CR_PWL | LTQ_WDT_CR_CLKDIV |
    LTQ_WDT_CR_PW2 | timeout,
    LTQ_WDT_CR);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ltq_wdt_stop(wdt: *mut watchdog_device) -> c_int {
    static int ltq_wdt_stop(struct watchdog_device *wdt)
    {
    struct ltq_wdt_priv *priv = ltq_wdt_get_priv(wdt);
    ltq_wdt_mask(priv, LTQ_WDT_CR_PW_MASK, LTQ_WDT_CR_PW1, LTQ_WDT_CR);
    ltq_wdt_mask(priv, LTQ_WDT_CR_GEN | LTQ_WDT_CR_PW_MASK,
    LTQ_WDT_CR_PW2, LTQ_WDT_CR);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ltq_wdt_ping(wdt: *mut watchdog_device) -> c_int {
    static int ltq_wdt_ping(struct watchdog_device *wdt)
    {
    struct ltq_wdt_priv *priv = ltq_wdt_get_priv(wdt);
    u32 timeout;
    timeout = wdt.timeout * priv.clk_rate;
    ltq_wdt_mask(priv, LTQ_WDT_CR_PW_MASK, LTQ_WDT_CR_PW1, LTQ_WDT_CR);
// write the second magic plus the configuration and new timeout
    ltq_wdt_mask(priv, LTQ_WDT_CR_PW_MASK | LTQ_WDT_CR_MAX_TIMEOUT,
    LTQ_WDT_CR_PW2 | timeout, LTQ_WDT_CR);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ltq_wdt_get_timeleft(wdt: *mut watchdog_device) -> c_uint {
    static unsigned int ltq_wdt_get_timeleft(struct watchdog_device *wdt)
    {
    struct ltq_wdt_priv *priv = ltq_wdt_get_priv(wdt);
    u64 timeout;
    timeout = ltq_wdt_r32(priv, LTQ_WDT_SR) & LTQ_WDT_SR_VALUE_MASK;
    return do_div(timeout, priv.clk_rate);
    }
    static const struct watchdog_ops ltq_wdt_ops = {
    .owner		= THIS_MODULE,
    .start		= ltq_wdt_start,
    .stop		= ltq_wdt_stop,
    .ping		= ltq_wdt_ping,
    .get_timeleft	= ltq_wdt_get_timeleft,
    };
#[no_mangle]
unsafe extern "C" fn ltq_wdt_xrx_bootstatus_get(dev: *mut device) -> c_int {
    static int ltq_wdt_xrx_bootstatus_get(struct device *dev)
    {
    struct regmap *rcu_regmap;
    u32 val;
    int err;
    rcu_regmap = syscon_regmap_lookup_by_phandle(dev.of_node, "regmap");
    if (IS_ERR(rcu_regmap))
    return PTR_ERR(rcu_regmap);
    err = regmap_read(rcu_regmap, LTQ_XRX_RCU_RST_STAT, &val);
    if (err)
    return err;
    if (val & LTQ_XRX_RCU_RST_STAT_WDT)
    return WDIOF_CARDRESET;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ltq_wdt_falcon_bootstatus_get(dev: *mut device) -> c_int {
    static int ltq_wdt_falcon_bootstatus_get(struct device *dev)
    {
    struct regmap *rcu_regmap;
    u32 val;
    int err;
    rcu_regmap = syscon_regmap_lookup_by_phandle(dev.of_node,
    "lantiq,rcu");
    if (IS_ERR(rcu_regmap))
    return PTR_ERR(rcu_regmap);
    err = regmap_read(rcu_regmap, LTQ_FALCON_SYS1_CPU0RS, &val);
    if (err)
    return err;
    if ((val & LTQ_FALCON_SYS1_CPU0RS_MASK) == LTQ_FALCON_SYS1_CPU0RS_WDT)
    return WDIOF_CARDRESET;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ltq_wdt_probe(pdev: *mut platform_device) -> c_int {
    static int ltq_wdt_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct ltq_wdt_priv *priv;
    struct watchdog_device *wdt;
    struct clk *clk;
    const struct ltq_wdt_hw *ltq_wdt_hw;
    int ret;
    u32 status;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.membase = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.membase))
    return PTR_ERR(priv.membase);
// we do not need to enable the clock as it is always running
    clk = clk_get_io();
    priv.clk_rate = clk_get_rate(clk) / LTQ_WDT_DIVIDER;
    if (!priv.clk_rate) {
    dev_err(dev, "clock rate less than divider %i\n",
    LTQ_WDT_DIVIDER);
    return -EINVAL;
    }
    wdt = &priv.wdt;
    wdt.info		= &ltq_wdt_info;
    wdt.ops		= &ltq_wdt_ops;
    wdt.min_timeout	= 1;
    wdt.max_timeout	= LTQ_WDT_CR_MAX_TIMEOUT / priv.clk_rate;
    wdt.timeout		= wdt.max_timeout;
    wdt.parent		= dev;
    ltq_wdt_hw = of_device_get_match_data(dev);
    if (ltq_wdt_hw && ltq_wdt_hw.bootstatus_get) {
    ret = ltq_wdt_hw.bootstatus_get(dev);
    if (ret >= 0)
    wdt.bootstatus = ret;
    }
    watchdog_set_nowayout(wdt, nowayout);
    watchdog_init_timeout(wdt, 0, dev);
    status = ltq_wdt_r32(priv, LTQ_WDT_SR);
    if (status & LTQ_WDT_SR_EN) {
//
// If the watchdog is already running overwrite it with our
// new settings. Stop is not needed as the start call will
// replace all settings anyway.
//
    ltq_wdt_start(wdt);
    set_bit(WDOG_HW_RUNNING, &wdt.status);
    }
    return devm_watchdog_register_device(dev, wdt);
    }
    static const struct ltq_wdt_hw ltq_wdt_xrx100 = {
    .bootstatus_get = ltq_wdt_xrx_bootstatus_get,
    };
    static const struct ltq_wdt_hw ltq_wdt_falcon = {
    .bootstatus_get = ltq_wdt_falcon_bootstatus_get,
    };
    static const struct of_device_id ltq_wdt_match[] = {
    { .compatible = "lantiq,wdt", .data = core::ptr::null_mut() },
    { .compatible = "lantiq,xrx100-wdt", .data = &ltq_wdt_xrx100 },
    { .compatible = "lantiq,falcon-wdt", .data = &ltq_wdt_falcon },
    {},
    };
    MODULE_DEVICE_TABLE(of, ltq_wdt_match);
    static struct platform_driver ltq_wdt_driver = {
    .probe = ltq_wdt_probe,
    .driver = {
    .name = "wdt",
    .of_match_table = ltq_wdt_match,
    },
    };
    module_platform_driver(ltq_wdt_driver);
    module_param(nowayout, bool, 0);
    MODULE_PARM_DESC(nowayout, "Watchdog cannot be stopped once started");
    MODULE_AUTHOR("John Crispin <john@phrozen.org>");
    MODULE_DESCRIPTION("Lantiq SoC Watchdog");
    MODULE_LICENSE("GPL");
