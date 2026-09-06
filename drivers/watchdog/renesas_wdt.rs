//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/renesas_wdt.c
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
// Watchdog driver for Renesas WDT watchdog
//
// Copyright (C) 2015-17 Wolfram Sang, Sang Engineering <wsa@sang-engineering.com>
// Copyright (C) 2015-17 Renesas Electronics Corporation
//

pub const RWTCNT: c_int = 0;
pub const RWTCSRA: c_int = 4;

pub const RWTCSRB: c_int = 8;

//
// In probe, clk_rate is checked to be not more than 16 bit * biggest clock
// divider (12 bits). d is only a factor to fully utilize the WDT counter and
// will not exceed its 16 bits. Thus, no overflow, we stay below 32 bits.
//

    DIV_ROUND_UP((d) * (p).clk_rate, clk_divs[(p).cks])
// d is 16 bit, clk_divs 12 bit -> no 32 bit overflow

    static const unsigned int clk_divs[] = { 1, 4, 16, 32, 64, 128, 1024, 4096 };
    let mut nowayout: static bool = WATCHDOG_NOWAYOUT;
    module_param(nowayout, bool, 0);
    MODULE_PARM_DESC(nowayout, "Watchdog cannot be stopped once started (default="
    __MODULE_STRING(WATCHDOG_NOWAYOUT) ")");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rwdt_priv {
    pub base: *mut void __iomem,
    pub wdev: watchdog_device,
    pub clk_rate: c_ulong,
    pub cks: u8,
    pub clk: *mut clk,
}

#[no_mangle]
unsafe extern "C" fn rwdt_write(priv: *mut rwdt_priv, val: u32, reg: c_uint) {
    static void rwdt_write(struct rwdt_priv *priv, u32 val, unsigned int reg)
    {
    if (reg == RWTCNT)
    val |= 0x5a5a0000;
    else
    val |= 0xa5a5a500;
    writel_relaxed(val, priv.base + reg);
    }
#[no_mangle]
unsafe extern "C" fn rwdt_init_timeout(wdev: *mut watchdog_device) -> c_int {
    static int rwdt_init_timeout(struct watchdog_device *wdev)
    {
    struct rwdt_priv *priv = watchdog_get_drvdata(wdev);
    rwdt_write(priv, 65536 - MUL_BY_CLKS_PER_SEC(priv, wdev.timeout), RWTCNT);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rwdt_wait_cycles(priv: *mut rwdt_priv, cycles: c_uint) {
    static void rwdt_wait_cycles(struct rwdt_priv *priv, unsigned int cycles)
    {
    unsigned int delay;
    delay = DIV_ROUND_UP(cycles * 1000000, priv.clk_rate);
    usleep_range(delay, 2 * delay);
    }
#[no_mangle]
unsafe extern "C" fn rwdt_start(wdev: *mut watchdog_device) -> c_int {
    static int rwdt_start(struct watchdog_device *wdev)
    {
    struct rwdt_priv *priv = watchdog_get_drvdata(wdev);
    u8 val;
    pm_runtime_get_sync(wdev.parent);
// Stop the timer before we modify any register
    val = readb_relaxed(priv.base + RWTCSRA) & ~RWTCSRA_TME;
    rwdt_write(priv, val, RWTCSRA);
// Delay 2 cycles before setting watchdog counter
    rwdt_wait_cycles(priv, 2);
    rwdt_init_timeout(wdev);
    rwdt_write(priv, priv.cks, RWTCSRA);
    rwdt_write(priv, 0, RWTCSRB);
    while (readb_relaxed(priv.base + RWTCSRA) & RWTCSRA_WRFLG)
    cpu_relax();
    rwdt_write(priv, priv.cks | RWTCSRA_TME, RWTCSRA);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rwdt_stop(wdev: *mut watchdog_device) -> c_int {
    static int rwdt_stop(struct watchdog_device *wdev)
    {
    struct rwdt_priv *priv = watchdog_get_drvdata(wdev);
    rwdt_write(priv, priv.cks, RWTCSRA);
// Delay 3 cycles before disabling module clock
    rwdt_wait_cycles(priv, 3);
    pm_runtime_put(wdev.parent);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rwdt_get_timeleft(wdev: *mut watchdog_device) -> c_uint {
    static unsigned int rwdt_get_timeleft(struct watchdog_device *wdev)
    {
    struct rwdt_priv *priv = watchdog_get_drvdata(wdev);
    let mut val: u16 = readw_relaxed(priv.base + RWTCNT);
    return DIV_BY_CLKS_PER_SEC(priv, 65536 - val);
    }
// needs to be atomic - no RPM, no usleep_range, no scheduling!
    static int rwdt_restart(struct watchdog_device *wdev, unsigned long action,
    void *data)
    {
    struct rwdt_priv *priv = watchdog_get_drvdata(wdev);
    u8 val;
    clk_prepare_enable(priv.clk);
// Stop the timer before we modify any register
    val = readb_relaxed(priv.base + RWTCSRA) & ~RWTCSRA_TME;
    rwdt_write(priv, val, RWTCSRA);
// Delay 2 cycles before setting watchdog counter
    udelay(DIV_ROUND_UP(2 * 1000000, priv.clk_rate));
    rwdt_write(priv, 0xffff, RWTCNT);
// smallest divider to reboot soon
    rwdt_write(priv, 0, RWTCSRA);
    readb_poll_timeout_atomic(priv.base + RWTCSRA, val,
    !(val & RWTCSRA_WRFLG), 1, 100);
    rwdt_write(priv, RWTCSRA_TME, RWTCSRA);
// wait 2 cycles, so watchdog will trigger
    udelay(DIV_ROUND_UP(2 * 1000000, priv.clk_rate));
    return 0;
    }
    static const struct watchdog_info rwdt_ident = {
    .options = WDIOF_MAGICCLOSE | WDIOF_KEEPALIVEPING | WDIOF_SETTIMEOUT |
    WDIOF_CARDRESET,
    .identity = "Renesas WDT Watchdog",
    };
    static const struct watchdog_ops rwdt_ops = {
    .owner = THIS_MODULE,
    .start = rwdt_start,
    .stop = rwdt_stop,
    .ping = rwdt_init_timeout,
    .get_timeleft = rwdt_get_timeleft,
    .restart = rwdt_restart,
    };

//
// Watchdog-reset integration is broken on early revisions of R-Car Gen2 SoCs
//
    static const struct soc_device_attribute rwdt_quirks_match[] = {
    {
    .soc_id = "r8a7790",
    .revision = "ES1.*",
    .data = (void *)1,	/* needs single CPU */
    }, {
    .soc_id = "r8a7791",
    .revision = "ES1.*",
    .data = (void *)1,	/* needs single CPU */
    }, {
    .soc_id = "r8a7792",
    .data = (void *)0,	/* needs SMP disabled */
    },
    { /* sentinel */ }
    };
#[no_mangle]
unsafe extern "C" fn rwdt_blacklisted(dev: *mut device) -> bool {
    static bool rwdt_blacklisted(struct device *dev)
    {
    const struct soc_device_attribute *attr;
    attr = soc_device_match(rwdt_quirks_match);
    if (attr && setup_max_cpus > (uintptr_t)attr.data) {
    dev_info(dev, "Watchdog blacklisted on %s %s\n", attr.soc_id,
    attr.revision);
    return true;
    }
    return false;
    }

    static inline bool rwdt_blacklisted(struct device *dev) { return false; }

#[no_mangle]
unsafe extern "C" fn rwdt_probe(pdev: *mut platform_device) -> c_int {
    static int rwdt_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct rwdt_priv *priv;
    unsigned long clks_per_sec;
    int ret, i;
    u8 csra;
    if (rwdt_blacklisted(dev))
    return -ENODEV;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.base))
    return PTR_ERR(priv.base);
    priv.clk = devm_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(priv.clk))
    return PTR_ERR(priv.clk);
    pm_runtime_enable(dev);
    pm_runtime_get_sync(dev);
    priv.clk_rate = clk_get_rate(priv.clk);
    csra = readb_relaxed(priv.base + RWTCSRA);
    priv.wdev.bootstatus = csra & RWTCSRA_WOVF ? WDIOF_CARDRESET : 0;
    pm_runtime_put(dev);
    if (!priv.clk_rate) {
    ret = -ENOENT;
    goto out_pm_disable;
    }
    for (i = ARRAY_SIZE(clk_divs) - 1; i >= 0; i--) {
    clks_per_sec = priv.clk_rate / clk_divs[i];
    if (clks_per_sec && clks_per_sec < 65536) {
    priv.cks = i;
    break;
    }
    }
    if (i < 0) {
    dev_err(dev, "Can't find suitable clock divider\n");
    ret = -ERANGE;
    goto out_pm_disable;
    }
    priv.wdev.info = &rwdt_ident;
    priv.wdev.ops = &rwdt_ops;
    priv.wdev.parent = dev;
    priv.wdev.min_timeout = 1;
    priv.wdev.max_timeout = DIV_BY_CLKS_PER_SEC(priv, 65536);
    priv.wdev.timeout = min(priv.wdev.max_timeout, RWDT_DEFAULT_TIMEOUT);
    platform_set_drvdata(pdev, priv);
    watchdog_set_drvdata(&priv.wdev, priv);
    watchdog_set_nowayout(&priv.wdev, nowayout);
    watchdog_set_restart_priority(&priv.wdev, 0);
    watchdog_stop_on_unregister(&priv.wdev);
// This overrides the default timeout only if DT configuration was found
    watchdog_init_timeout(&priv.wdev, 0, dev);
// Check if FW enabled the watchdog
    if (csra & RWTCSRA_TME) {
// Ensure properly initialized dividers
    rwdt_start(&priv.wdev);
    set_bit(WDOG_HW_RUNNING, &priv.wdev.status);
    }
    ret = watchdog_register_device(&priv.wdev);
    if (ret < 0)
    goto out_pm_disable;
    return 0;
    out_pm_disable:
    pm_runtime_disable(dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rwdt_remove(pdev: *mut platform_device) {
    static void rwdt_remove(struct platform_device *pdev)
    {
    struct rwdt_priv *priv = platform_get_drvdata(pdev);
    watchdog_unregister_device(&priv.wdev);
    pm_runtime_disable(&pdev.dev);
    }
#[no_mangle]
unsafe extern "C" fn rwdt_suspend(dev: *mut device) -> c_int {
    static int rwdt_suspend(struct device *dev)
    {
    struct rwdt_priv *priv = dev_get_drvdata(dev);
    if (watchdog_active(&priv.wdev))
    rwdt_stop(&priv.wdev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rwdt_resume(dev: *mut device) -> c_int {
    static int rwdt_resume(struct device *dev)
    {
    struct rwdt_priv *priv = dev_get_drvdata(dev);
    if (watchdog_active(&priv.wdev))
    rwdt_start(&priv.wdev);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(rwdt_pm_ops, rwdt_suspend, rwdt_resume);
    static const struct of_device_id rwdt_ids[] = {
    { .compatible = "renesas,rcar-gen2-wdt", },
    { .compatible = "renesas,rcar-gen3-wdt", },
    { .compatible = "renesas,rcar-gen4-wdt", },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, rwdt_ids);
    static struct platform_driver rwdt_driver = {
    .driver = {
    .name = "renesas_wdt",
    .of_match_table = rwdt_ids,
    .pm = pm_sleep_ptr(&rwdt_pm_ops),
    },
    .probe = rwdt_probe,
    .remove = rwdt_remove,
    };
    module_platform_driver(rwdt_driver);
    MODULE_DESCRIPTION("Renesas WDT Watchdog Driver");
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Wolfram Sang <wsa@sang-engineering.com>");
