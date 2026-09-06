//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/imx7ulp_wdt.c
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
// Copyright 2019 NXP.
//

pub const WDOG_CS: c_uint = 0x0;

pub const LPO_CLK: c_uint = 0x1;
pub const LPO_CLK_SHIFT: c_int = 8;

pub const WDOG_CNT: c_uint = 0x4;
pub const WDOG_TOVAL: c_uint = 0x8;
pub const REFRESH_SEQ0: c_uint = 0xA602;
pub const REFRESH_SEQ1: c_uint = 0xB480;

pub const UNLOCK_SEQ0: c_uint = 0xC520;
pub const UNLOCK_SEQ1: c_uint = 0xD928;

pub const DEFAULT_TIMEOUT: c_int = 60;
pub const MAX_TIMEOUT: c_int = 128;
pub const WDOG_CLOCK_RATE: c_int = 1000;
pub const WDOG_ULK_WAIT_TIMEOUT: c_int = 1000;
pub const WDOG_RCS_WAIT_TIMEOUT: c_int = 10000;
pub const WDOG_RCS_POST_WAIT: c_int = 3000;
pub const RETRY_MAX: c_int = 5;
    let mut nowayout: static bool = WATCHDOG_NOWAYOUT;
    module_param(nowayout, bool, 0000);
    MODULE_PARM_DESC(nowayout, "Watchdog cannot be stopped once started (default="
    __MODULE_STRING(WATCHDOG_NOWAYOUT) ")");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_wdt_hw_feature {
    pub prescaler_enable: bool,
    pub post_rcs_wait: bool,
    pub cpu_lpm_auto_cg: bool,
    pub wdog_clock_rate: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx7ulp_wdt_device {
    pub wdd: watchdog_device,
    pub base: *mut void __iomem,
    pub clk: *mut clk,
    pub ext_reset: bool,
    pub hw: *const imx_wdt_hw_feature,
}

#[no_mangle]
unsafe extern "C" fn imx7ulp_wdt_wait_ulk(base: *mut void __iomem) -> c_int {
    static int imx7ulp_wdt_wait_ulk(void __iomem *base)
    {
    let mut val: u32 = readl(base + WDOG_CS);
    if (!(val & WDOG_CS_ULK) &&
    readl_poll_timeout_atomic(base + WDOG_CS, val,
    val & WDOG_CS_ULK, 0,
    WDOG_ULK_WAIT_TIMEOUT))
    return -ETIMEDOUT;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx7ulp_wdt_wait_rcs(wdt: *mut imx7ulp_wdt_device) -> c_int {
    static int imx7ulp_wdt_wait_rcs(struct imx7ulp_wdt_device *wdt)
    {
    let mut ret: c_int = 0;
    let mut val: u32 = readl(wdt.base + WDOG_CS);
    u64 timeout = (val & WDOG_CS_PRES) ?
    WDOG_RCS_WAIT_TIMEOUT * 256 : WDOG_RCS_WAIT_TIMEOUT;
    unsigned long wait_min = (val & WDOG_CS_PRES) ?
    WDOG_RCS_POST_WAIT * 256 : WDOG_RCS_POST_WAIT;
    if (!(val & WDOG_CS_RCS) &&
    readl_poll_timeout(wdt.base + WDOG_CS, val, val & WDOG_CS_RCS, 100,
    timeout))
    ret = -ETIMEDOUT;
// Wait 2.5 clocks after RCS done
    if (wdt.hw.post_rcs_wait)
    usleep_range(wait_min, wait_min + 2000);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn _imx7ulp_wdt_enable(wdt: *mut imx7ulp_wdt_device, enable: bool) -> c_int {
    static int _imx7ulp_wdt_enable(struct imx7ulp_wdt_device *wdt, bool enable)
    {
    let mut val: u32 = readl(wdt.base + WDOG_CS);
    int ret;
    local_irq_disable();
    writel(UNLOCK, wdt.base + WDOG_CNT);
    ret = imx7ulp_wdt_wait_ulk(wdt.base);
    if (ret)
    goto enable_out;
    if (enable)
    writel(val | WDOG_CS_EN, wdt.base + WDOG_CS);
    else
    writel(val & ~WDOG_CS_EN, wdt.base + WDOG_CS);
    local_irq_enable();
    ret = imx7ulp_wdt_wait_rcs(wdt);
    return ret;
    enable_out:
    local_irq_enable();
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn imx7ulp_wdt_enable(wdog: *mut watchdog_device, enable: bool) -> c_int {
    static int imx7ulp_wdt_enable(struct watchdog_device *wdog, bool enable)
    {
    struct imx7ulp_wdt_device *wdt = watchdog_get_drvdata(wdog);
    int ret;
    u32 val;
    let mut loop: u32 = RETRY_MAX;
    do {
    ret = _imx7ulp_wdt_enable(wdt, enable);
    val = readl(wdt.base + WDOG_CS);
    } while (--loop > 0 && ((!!(val & WDOG_CS_EN)) != enable || ret));
    if (loop == 0)
    return -EBUSY;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn imx7ulp_wdt_ping(wdog: *mut watchdog_device) -> c_int {
    static int imx7ulp_wdt_ping(struct watchdog_device *wdog)
    {
    struct imx7ulp_wdt_device *wdt = watchdog_get_drvdata(wdog);
    writel(REFRESH, wdt.base + WDOG_CNT);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx7ulp_wdt_start(wdog: *mut watchdog_device) -> c_int {
    static int imx7ulp_wdt_start(struct watchdog_device *wdog)
    {
    return imx7ulp_wdt_enable(wdog, true);
    }
#[no_mangle]
unsafe extern "C" fn imx7ulp_wdt_stop(wdog: *mut watchdog_device) -> c_int {
    static int imx7ulp_wdt_stop(struct watchdog_device *wdog)
    {
    return imx7ulp_wdt_enable(wdog, false);
    }
    static int _imx7ulp_wdt_set_timeout(struct imx7ulp_wdt_device *wdt,
    unsigned int toval)
    {
    int ret;
    local_irq_disable();
    writel(UNLOCK, wdt.base + WDOG_CNT);
    ret = imx7ulp_wdt_wait_ulk(wdt.base);
    if (ret)
    goto timeout_out;
    writel(toval, wdt.base + WDOG_TOVAL);
    local_irq_enable();
    ret = imx7ulp_wdt_wait_rcs(wdt);
    return ret;
    timeout_out:
    local_irq_enable();
    return ret;
    }
    static int imx7ulp_wdt_set_timeout(struct watchdog_device *wdog,
    unsigned int timeout)
    {
    struct imx7ulp_wdt_device *wdt = watchdog_get_drvdata(wdog);
    let mut toval: u32 = wdt.hw.wdog_clock_rate * timeout;
    u32 val;
    int ret;
    let mut loop: u32 = RETRY_MAX;
    do {
    ret = _imx7ulp_wdt_set_timeout(wdt, toval);
    val = readl(wdt.base + WDOG_TOVAL);
    } while (--loop > 0 && (val != toval || ret));
    if (loop == 0)
    return -EBUSY;
    wdog.timeout = timeout;
    return ret;
    }
    static int imx7ulp_wdt_restart(struct watchdog_device *wdog,
    unsigned long action, void *data)
    {
    struct imx7ulp_wdt_device *wdt = watchdog_get_drvdata(wdog);
    int ret;
    ret = imx7ulp_wdt_enable(wdog, true);
    if (ret)
    return ret;
    ret = imx7ulp_wdt_set_timeout(&wdt.wdd, 1);
    if (ret)
    return ret;
// wait for wdog to fire
    while (true)
    ;
    return NOTIFY_DONE;
    }
    static const struct watchdog_ops imx7ulp_wdt_ops = {
    .owner = THIS_MODULE,
    .start = imx7ulp_wdt_start,
    .stop  = imx7ulp_wdt_stop,
    .ping  = imx7ulp_wdt_ping,
    .set_timeout = imx7ulp_wdt_set_timeout,
    .restart = imx7ulp_wdt_restart,
    };
    static const struct watchdog_info imx7ulp_wdt_info = {
    .identity = "i.MX7ULP watchdog timer",
    .options  = WDIOF_SETTIMEOUT | WDIOF_KEEPALIVEPING |
    WDIOF_MAGICCLOSE,
    };
#[no_mangle]
unsafe extern "C" fn _imx7ulp_wdt_init(wdt: *mut imx7ulp_wdt_device, timeout: c_uint, cs: c_uint) -> c_int {
    static int _imx7ulp_wdt_init(struct imx7ulp_wdt_device *wdt, unsigned int timeout, unsigned int cs)
    {
    u32 val;
    int ret;
    local_irq_disable();
    val = readl(wdt.base + WDOG_CS);
    if (val & WDOG_CS_CMD32EN) {
    writel(UNLOCK, wdt.base + WDOG_CNT);
    } else {
    mb();
// unlock the wdog for reconfiguration
    writel_relaxed(UNLOCK_SEQ0, wdt.base + WDOG_CNT);
    writel_relaxed(UNLOCK_SEQ1, wdt.base + WDOG_CNT);
    mb();
    }
    ret = imx7ulp_wdt_wait_ulk(wdt.base);
    if (ret)
    goto init_out;
// set an initial timeout value in TOVAL
    writel(timeout, wdt.base + WDOG_TOVAL);
    writel(cs, wdt.base + WDOG_CS);
    local_irq_enable();
    ret = imx7ulp_wdt_wait_rcs(wdt);
    return ret;
    init_out:
    local_irq_enable();
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn imx7ulp_wdt_init(wdt: *mut imx7ulp_wdt_device, timeout: c_uint) -> c_int {
    static int imx7ulp_wdt_init(struct imx7ulp_wdt_device *wdt, unsigned int timeout)
    {
// enable 32bit command sequence and reconfigure
    u32 val = WDOG_CS_CMD32EN | WDOG_CS_CLK | WDOG_CS_UPDATE |
    WDOG_CS_WAIT | WDOG_CS_STOP;
    u32 cs, toval;
    int ret;
    let mut loop: u32 = RETRY_MAX;
    if (wdt.hw.prescaler_enable)
    val |= WDOG_CS_PRES;
    if (wdt.ext_reset)
    val |= WDOG_CS_INT_EN;
    if (readl(wdt.base + WDOG_CS) & WDOG_CS_EN) {
    set_bit(WDOG_HW_RUNNING, &wdt.wdd.status);
    val |= WDOG_CS_EN;
    }
    do {
    ret = _imx7ulp_wdt_init(wdt, timeout, val);
    toval = readl(wdt.base + WDOG_TOVAL);
    cs = readl(wdt.base + WDOG_CS);
    cs &= ~(WDOG_CS_FLG | WDOG_CS_ULK | WDOG_CS_RCS);
    } while (--loop > 0 && (cs != val || toval != timeout || ret));
    if (loop == 0)
    return -EBUSY;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn imx7ulp_wdt_probe(pdev: *mut platform_device) -> c_int {
    static int imx7ulp_wdt_probe(struct platform_device *pdev)
    {
    struct imx7ulp_wdt_device *imx7ulp_wdt;
    struct device *dev = &pdev.dev;
    struct watchdog_device *wdog;
    int ret;
    imx7ulp_wdt = devm_kzalloc(dev, sizeof(*imx7ulp_wdt), GFP_KERNEL);
    if (!imx7ulp_wdt)
    return -ENOMEM;
    platform_set_drvdata(pdev, imx7ulp_wdt);
    imx7ulp_wdt.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(imx7ulp_wdt.base))
    return PTR_ERR(imx7ulp_wdt.base);
    imx7ulp_wdt.clk = devm_clk_get_enabled(dev, core::ptr::null_mut());
    if (IS_ERR(imx7ulp_wdt.clk)) {
    dev_err(dev, "Failed to get watchdog clock\n");
    return PTR_ERR(imx7ulp_wdt.clk);
    }
// The WDOG may need to do external reset through dedicated pin
    imx7ulp_wdt.ext_reset = of_property_read_bool(dev.of_node, "fsl,ext-reset-output");
    wdog = &imx7ulp_wdt.wdd;
    wdog.info = &imx7ulp_wdt_info;
    wdog.ops = &imx7ulp_wdt_ops;
    wdog.min_timeout = 1;
    wdog.max_timeout = MAX_TIMEOUT;
    wdog.parent = dev;
    wdog.timeout = DEFAULT_TIMEOUT;
    watchdog_init_timeout(wdog, 0, dev);
    watchdog_stop_on_reboot(wdog);
    watchdog_stop_on_unregister(wdog);
    watchdog_set_drvdata(wdog, imx7ulp_wdt);
    watchdog_set_nowayout(wdog, nowayout);
    imx7ulp_wdt.hw = of_device_get_match_data(dev);
    ret = imx7ulp_wdt_init(imx7ulp_wdt, wdog.timeout * imx7ulp_wdt.hw.wdog_clock_rate);
    if (ret)
    return ret;
    return devm_watchdog_register_device(dev, wdog);
    }
#[no_mangle]
unsafe extern "C" fn imx7ulp_wdt_suspend_noirq(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused imx7ulp_wdt_suspend_noirq(struct device *dev)
    {
    struct imx7ulp_wdt_device *imx7ulp_wdt = dev_get_drvdata(dev);
    if (watchdog_active(&imx7ulp_wdt.wdd) && !imx7ulp_wdt.hw.cpu_lpm_auto_cg)
    imx7ulp_wdt_stop(&imx7ulp_wdt.wdd);
    clk_disable_unprepare(imx7ulp_wdt.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx7ulp_wdt_resume_noirq(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused imx7ulp_wdt_resume_noirq(struct device *dev)
    {
    struct imx7ulp_wdt_device *imx7ulp_wdt = dev_get_drvdata(dev);
    let mut timeout: u32 = imx7ulp_wdt.wdd.timeout * imx7ulp_wdt.hw.wdog_clock_rate;
    int ret;
    ret = clk_prepare_enable(imx7ulp_wdt.clk);
    if (ret)
    return ret;
    if (watchdog_active(&imx7ulp_wdt.wdd)) {
    imx7ulp_wdt_init(imx7ulp_wdt, timeout);
    imx7ulp_wdt_start(&imx7ulp_wdt.wdd);
    imx7ulp_wdt_ping(&imx7ulp_wdt.wdd);
    }
    return 0;
    }
    static const struct dev_pm_ops imx7ulp_wdt_pm_ops = {
    SET_NOIRQ_SYSTEM_SLEEP_PM_OPS(imx7ulp_wdt_suspend_noirq,
    imx7ulp_wdt_resume_noirq)
    };
    static const struct imx_wdt_hw_feature imx7ulp_wdt_hw = {
    .prescaler_enable = false,
    .wdog_clock_rate = 1000,
    .post_rcs_wait = true,
    };
    static const struct imx_wdt_hw_feature imx8ulp_wdt_hw = {
    .prescaler_enable = false,
    .wdog_clock_rate = 1000,
    };
    static const struct imx_wdt_hw_feature imx93_wdt_hw = {
    .prescaler_enable = true,
    .wdog_clock_rate = 125,
    };
    static const struct imx_wdt_hw_feature imx94_wdt_hw = {
    .prescaler_enable = true,
    .wdog_clock_rate = 125,
    .cpu_lpm_auto_cg = true,
    };
    static const struct of_device_id imx7ulp_wdt_dt_ids[] = {
    { .compatible = "fsl,imx7ulp-wdt", .data = &imx7ulp_wdt_hw, },
    { .compatible = "fsl,imx8ulp-wdt", .data = &imx8ulp_wdt_hw, },
    { .compatible = "fsl,imx93-wdt", .data = &imx93_wdt_hw, },
    { .compatible = "fsl,imx94-wdt", .data = &imx94_wdt_hw, },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, imx7ulp_wdt_dt_ids);
    static struct platform_driver imx7ulp_wdt_driver = {
    .probe		= imx7ulp_wdt_probe,
    .driver		= {
    .name	= "imx7ulp-wdt",
    .pm	= &imx7ulp_wdt_pm_ops,
    .of_match_table = imx7ulp_wdt_dt_ids,
    },
    };
    module_platform_driver(imx7ulp_wdt_driver);
    MODULE_AUTHOR("Anson Huang <Anson.Huang@nxp.com>");
    MODULE_DESCRIPTION("Freescale i.MX7ULP watchdog driver");
    MODULE_LICENSE("GPL v2");
