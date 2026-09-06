//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/imx2_wdt.c
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
// Watchdog driver for IMX2 and later processors
//
// Copyright (C) 2010 Wolfram Sang, Pengutronix e.K. <kernel@pengutronix.de>
// Copyright (C) 2014 Freescale Semiconductor, Inc.
//
// some parts adapted by similar drivers from Darius Augulis and Vladimir
// Zapolskiy, additional improvements by Wim Van Sebroeck.
//
// NOTE: MX1 has a slightly different Watchdog than MX2 and later:
//
// MX1:		MX2+:
// ----		-----
// Registers:		32-bit		16-bit
// Stopable timer:	Yes		No
// Need to enable clk:	No		Yes
// Halt on suspend:	Manual		Can be automatic
//

pub const IMX2_WDT_WCR: c_uint = 0x00		/* Control Register */;

pub const IMX2_WDT_WSR: c_uint = 0x02		/* Service Register */;
pub const IMX2_WDT_SEQ1: c_uint = 0x5555		/* -> service sequence 1 */;
pub const IMX2_WDT_SEQ2: c_uint = 0xAAAA		/* -> service sequence 2 */;
pub const IMX2_WDT_WRSR: c_uint = 0x04		/* Reset Status Register */;

pub const IMX2_WDT_WICR: c_uint = 0x06		/* Interrupt Control Register */;

pub const IMX2_WDT_WICR_WICT: c_uint = 0xFF		/* -> Interrupt Count Timeout */;
pub const IMX2_WDT_WMCR: c_uint = 0x08		/* Misc Register */;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx2_wdt_data {
    pub wdw_supported: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx2_wdt_device {
    pub clk: *mut clk,
    pub regmap: *mut regmap,
    pub wdog: watchdog_device,
    pub data: *const imx2_wdt_data,
    pub ext_reset: bool,
    pub clk_is_on: bool,
    pub no_ping: bool,
    pub sleep_wait: bool,
}

    let mut nowayout: static bool = WATCHDOG_NOWAYOUT;
    module_param(nowayout, bool, 0);
    MODULE_PARM_DESC(nowayout, "Watchdog cannot be stopped once started (default="
    __MODULE_STRING(WATCHDOG_NOWAYOUT) ")");
    static unsigned timeout;
    module_param(timeout, uint, 0);
    MODULE_PARM_DESC(timeout, "Watchdog timeout in seconds (default="
    __MODULE_STRING(IMX2_WDT_DEFAULT_TIME) ")");
    static const struct watchdog_info imx2_wdt_info = {
    .identity = "imx2+ watchdog",
    .options = WDIOF_KEEPALIVEPING | WDIOF_SETTIMEOUT | WDIOF_MAGICCLOSE,
    };
    static const struct watchdog_info imx2_wdt_pretimeout_info = {
    .identity = "imx2+ watchdog",
    .options = WDIOF_KEEPALIVEPING | WDIOF_SETTIMEOUT | WDIOF_MAGICCLOSE |
    WDIOF_PRETIMEOUT,
    };
    static int imx2_wdt_restart(struct watchdog_device *wdog, unsigned long action,
    void *data)
    {
    struct imx2_wdt_device *wdev = watchdog_get_drvdata(wdog);
    let mut wcr_enable: c_uint = IMX2_WDT_WCR_WDE;
// Use internal reset or external - not both
    if (wdev.ext_reset)
    wcr_enable |= IMX2_WDT_WCR_SRS; /* do not assert int reset */
    else
    wcr_enable |= IMX2_WDT_WCR_WDA; /* do not assert ext-reset */
// Assert SRS signal
    regmap_write(wdev.regmap, IMX2_WDT_WCR, wcr_enable);
//
// Due to imx6q errata ERR004346 (WDOG: WDOG SRS bit requires to be
// written twice), we add another two writes to ensure there must be at
// least two writes happen in the same one 32kHz clock period.  We save
// the target check here, since the writes shouldn't be a huge burden
// for other platforms.
//
    regmap_write(wdev.regmap, IMX2_WDT_WCR, wcr_enable);
    regmap_write(wdev.regmap, IMX2_WDT_WCR, wcr_enable);
// wait for reset to assert...
    mdelay(500);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn imx2_wdt_setup(wdog: *mut watchdog_device) {
    static inline void imx2_wdt_setup(struct watchdog_device *wdog)
    {
    struct imx2_wdt_device *wdev = watchdog_get_drvdata(wdog);
    u32 val;
    regmap_read(wdev.regmap, IMX2_WDT_WCR, &val);
// Suspend timer in low power mode, write once-only
    val |= IMX2_WDT_WCR_WDZST;
// Suspend timer in low power WAIT mode, write once-only
    if (wdev.sleep_wait)
    val |= IMX2_WDT_WCR_WDW;
// Strip the old watchdog Time-Out value
    val &= ~IMX2_WDT_WCR_WT;
// Generate internal chip-level reset if WDOG times out
    if (!wdev.ext_reset)
    val &= ~IMX2_WDT_WCR_WRE;
// Or if external-reset assert WDOG_B reset only on time-out
    else
    val |= IMX2_WDT_WCR_WRE;
// Keep Watchdog Disabled
    val &= ~IMX2_WDT_WCR_WDE;
// Set the watchdog's Time-Out value
    val |= WDOG_SEC_TO_COUNT(wdog.timeout);
    regmap_write(wdev.regmap, IMX2_WDT_WCR, val);
// enable the watchdog
    val |= IMX2_WDT_WCR_WDE;
    regmap_write(wdev.regmap, IMX2_WDT_WCR, val);
    }
#[no_mangle]
pub unsafe extern "C" fn imx2_wdt_is_running(wdev: *mut imx2_wdt_device) -> bool {
    static inline bool imx2_wdt_is_running(struct imx2_wdt_device *wdev)
    {
    u32 val;
    regmap_read(wdev.regmap, IMX2_WDT_WCR, &val);
    return val & IMX2_WDT_WCR_WDE;
    }
#[no_mangle]
unsafe extern "C" fn imx2_wdt_ping(wdog: *mut watchdog_device) -> c_int {
    static int imx2_wdt_ping(struct watchdog_device *wdog)
    {
    struct imx2_wdt_device *wdev = watchdog_get_drvdata(wdog);
    if (!wdev.clk_is_on)
    return 0;
    regmap_write(wdev.regmap, IMX2_WDT_WSR, IMX2_WDT_SEQ1);
    regmap_write(wdev.regmap, IMX2_WDT_WSR, IMX2_WDT_SEQ2);
    return 0;
    }
    static void __imx2_wdt_set_timeout(struct watchdog_device *wdog,
    unsigned int new_timeout)
    {
    struct imx2_wdt_device *wdev = watchdog_get_drvdata(wdog);
    regmap_update_bits(wdev.regmap, IMX2_WDT_WCR, IMX2_WDT_WCR_WT,
    WDOG_SEC_TO_COUNT(new_timeout));
    }
    static int imx2_wdt_set_timeout(struct watchdog_device *wdog,
    unsigned int new_timeout)
    {
    unsigned int actual;
    actual = min(new_timeout, IMX2_WDT_MAX_TIME);
    __imx2_wdt_set_timeout(wdog, actual);
    wdog.timeout = new_timeout;
    return 0;
    }
    static int imx2_wdt_set_pretimeout(struct watchdog_device *wdog,
    unsigned int new_pretimeout)
    {
    struct imx2_wdt_device *wdev = watchdog_get_drvdata(wdog);
    if (new_pretimeout >= IMX2_WDT_MAX_TIME)
    return -EINVAL;
    wdog.pretimeout = new_pretimeout;
    regmap_update_bits(wdev.regmap, IMX2_WDT_WICR,
    IMX2_WDT_WICR_WIE | IMX2_WDT_WICR_WICT,
    IMX2_WDT_WICR_WIE | (new_pretimeout << 1));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx2_wdt_isr(irq: c_int, wdog_arg: *mut c_void) -> irqreturn_t {
    static irqreturn_t imx2_wdt_isr(int irq, void *wdog_arg)
    {
    struct watchdog_device *wdog = wdog_arg;
    struct imx2_wdt_device *wdev = watchdog_get_drvdata(wdog);
    regmap_write_bits(wdev.regmap, IMX2_WDT_WICR,
    IMX2_WDT_WICR_WTIS, IMX2_WDT_WICR_WTIS);
    watchdog_notify_pretimeout(wdog);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn imx2_wdt_start(wdog: *mut watchdog_device) -> c_int {
    static int imx2_wdt_start(struct watchdog_device *wdog)
    {
    struct imx2_wdt_device *wdev = watchdog_get_drvdata(wdog);
    if (imx2_wdt_is_running(wdev))
    imx2_wdt_set_timeout(wdog, wdog.timeout);
    else
    imx2_wdt_setup(wdog);
    set_bit(WDOG_HW_RUNNING, &wdog.status);
    return imx2_wdt_ping(wdog);
    }
    static const struct watchdog_ops imx2_wdt_ops = {
    .owner = THIS_MODULE,
    .start = imx2_wdt_start,
    .ping = imx2_wdt_ping,
    .set_timeout = imx2_wdt_set_timeout,
    .set_pretimeout = imx2_wdt_set_pretimeout,
    .restart = imx2_wdt_restart,
    };
    static const struct regmap_config imx2_wdt_regmap_config = {
    .reg_bits = 16,
    .reg_stride = 2,
    .val_bits = 16,
    .max_register = 0x8,
    };
#[no_mangle]
unsafe extern "C" fn imx2_wdt_action(data: *mut c_void) {
    static void imx2_wdt_action(void *data)
    {
    clk_disable_unprepare(data);
    }
#[no_mangle]
unsafe extern "C" fn imx2_wdt_probe(pdev: *mut platform_device) -> int __init {
    static int __init imx2_wdt_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct imx2_wdt_device *wdev;
    struct watchdog_device *wdog;
    void __iomem *base;
    int ret;
    u32 val;
    wdev = devm_kzalloc(dev, sizeof(*wdev), GFP_KERNEL);
    if (!wdev)
    return -ENOMEM;
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base))
    return PTR_ERR(base);
    wdev.regmap = devm_regmap_init_mmio_clk(dev, core::ptr::null_mut(), base,
    &imx2_wdt_regmap_config);
    if (IS_ERR(wdev.regmap)) {
    dev_err(dev, "regmap init failed\n");
    return PTR_ERR(wdev.regmap);
    }
    wdev.clk = devm_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(wdev.clk)) {
    dev_err(dev, "can't get Watchdog clock\n");
    return PTR_ERR(wdev.clk);
    }
    wdog			= &wdev.wdog;
    wdog.info		= &imx2_wdt_info;
    wdog.ops		= &imx2_wdt_ops;
    wdog.min_timeout	= 1;
    wdog.timeout		= IMX2_WDT_DEFAULT_TIME;
    wdog.max_hw_heartbeat_ms = IMX2_WDT_MAX_TIME * 1000;
    wdog.parent		= dev;
    wdev.data = of_device_get_match_data(dev);
    ret = platform_get_irq(pdev, 0);
    if (ret > 0)
    if (!devm_request_irq(dev, ret, imx2_wdt_isr, 0,
    dev_name(dev), wdog))
    wdog.info = &imx2_wdt_pretimeout_info;
    ret = clk_prepare_enable(wdev.clk);
    if (ret)
    return ret;
    ret = devm_add_action_or_reset(dev, imx2_wdt_action, wdev.clk);
    if (ret)
    return ret;
    wdev.clk_is_on = true;
    regmap_read(wdev.regmap, IMX2_WDT_WRSR, &val);
    wdog.bootstatus = val & IMX2_WDT_WRSR_TOUT ? WDIOF_CARDRESET : 0;
    wdev.ext_reset = of_property_read_bool(dev.of_node,
    "fsl,ext-reset-output");
    if (of_property_read_bool(dev.of_node, "fsl,suspend-in-wait")) {
    if (!wdev.data.wdw_supported) {
    dev_err(dev, "suspend-in-wait not supported\n");
    return -EINVAL;
    }
    wdev.sleep_wait = true;
    }
//
// The i.MX7D doesn't support low power mode, so we need to ping the watchdog
// during suspend. Interaction with "fsl,suspend-in-wait" is unknown!
//
    wdev.no_ping = !of_device_is_compatible(dev.of_node, "fsl,imx7d-wdt");
    platform_set_drvdata(pdev, wdog);
    watchdog_set_drvdata(wdog, wdev);
    watchdog_set_nowayout(wdog, nowayout);
    watchdog_set_restart_priority(wdog, 128);
    watchdog_init_timeout(wdog, timeout, dev);
    if (wdev.no_ping)
    watchdog_stop_ping_on_suspend(wdog);
    if (imx2_wdt_is_running(wdev)) {
    imx2_wdt_set_timeout(wdog, wdog.timeout);
    set_bit(WDOG_HW_RUNNING, &wdog.status);
    }
//
// Disable the watchdog power down counter at boot. Otherwise the power
// down counter will pull down the #WDOG interrupt line for one clock
// cycle.
//
    regmap_write(wdev.regmap, IMX2_WDT_WMCR, 0);
    return devm_watchdog_register_device(dev, wdog);
    }
#[no_mangle]
unsafe extern "C" fn imx2_wdt_shutdown(pdev: *mut platform_device) {
    static void imx2_wdt_shutdown(struct platform_device *pdev)
    {
    struct watchdog_device *wdog = platform_get_drvdata(pdev);
    struct imx2_wdt_device *wdev = watchdog_get_drvdata(wdog);
    if (imx2_wdt_is_running(wdev)) {
//
// We are running, configure max timeout before reboot
// will take place.
//
    imx2_wdt_set_timeout(wdog, IMX2_WDT_MAX_TIME);
    imx2_wdt_ping(wdog);
    dev_crit(&pdev.dev, "Device shutdown.\n");
    }
    }
// Disable watchdog if it is active or non-active but still running
#[no_mangle]
unsafe extern "C" fn imx2_wdt_suspend(dev: *mut device) -> c_int {
    static int imx2_wdt_suspend(struct device *dev)
    {
    struct watchdog_device *wdog = dev_get_drvdata(dev);
    struct imx2_wdt_device *wdev = watchdog_get_drvdata(wdog);
// The watchdog IP block is running
    if (imx2_wdt_is_running(wdev)) {
//
// Don't update wdog->timeout, we'll restore the current value
// during resume.
//
    __imx2_wdt_set_timeout(wdog, IMX2_WDT_MAX_TIME);
    imx2_wdt_ping(wdog);
    }
    if (wdev.no_ping) {
    clk_disable_unprepare(wdev.clk);
    wdev.clk_is_on = false;
    }
    return 0;
    }
// Enable watchdog and configure it if necessary
#[no_mangle]
unsafe extern "C" fn imx2_wdt_resume(dev: *mut device) -> c_int {
    static int imx2_wdt_resume(struct device *dev)
    {
    struct watchdog_device *wdog = dev_get_drvdata(dev);
    struct imx2_wdt_device *wdev = watchdog_get_drvdata(wdog);
    int ret;
    if (wdev.no_ping) {
    ret = clk_prepare_enable(wdev.clk);
    if (ret)
    return ret;
    wdev.clk_is_on = true;
    }
    if (watchdog_active(wdog) && !imx2_wdt_is_running(wdev)) {
//
// If the watchdog is still active and resumes
// from deep sleep state, need to restart the
// watchdog again.
//
    imx2_wdt_setup(wdog);
    }
    if (imx2_wdt_is_running(wdev)) {
    imx2_wdt_set_timeout(wdog, wdog.timeout);
    imx2_wdt_ping(wdog);
    }
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(imx2_wdt_pm_ops, imx2_wdt_suspend,
    imx2_wdt_resume);
    static struct imx2_wdt_data imx_wdt = {
    .wdw_supported = true,
    };
    static struct imx2_wdt_data imx_wdt_legacy = {
    .wdw_supported = false,
    };
    static const struct of_device_id imx2_wdt_dt_ids[] = {
    { .compatible = "fsl,imx21-wdt", .data = &imx_wdt_legacy },
    { .compatible = "fsl,imx25-wdt", .data = &imx_wdt },
    { .compatible = "fsl,imx27-wdt", .data = &imx_wdt_legacy },
    { .compatible = "fsl,imx31-wdt", .data = &imx_wdt_legacy },
    { .compatible = "fsl,imx35-wdt", .data = &imx_wdt },
    { .compatible = "fsl,imx50-wdt", .data = &imx_wdt },
    { .compatible = "fsl,imx51-wdt", .data = &imx_wdt },
    { .compatible = "fsl,imx53-wdt", .data = &imx_wdt },
    { .compatible = "fsl,imx6q-wdt", .data = &imx_wdt },
    { .compatible = "fsl,imx6sl-wdt", .data = &imx_wdt },
    { .compatible = "fsl,imx6sll-wdt", .data = &imx_wdt },
    { .compatible = "fsl,imx6sx-wdt", .data = &imx_wdt },
    { .compatible = "fsl,imx6ul-wdt", .data = &imx_wdt },
    { .compatible = "fsl,imx7d-wdt", .data = &imx_wdt },
    { .compatible = "fsl,imx8mm-wdt", .data = &imx_wdt },
    { .compatible = "fsl,imx8mn-wdt", .data = &imx_wdt },
    { .compatible = "fsl,imx8mp-wdt", .data = &imx_wdt },
    { .compatible = "fsl,imx8mq-wdt", .data = &imx_wdt },
    { .compatible = "fsl,ls1012a-wdt", .data = &imx_wdt_legacy },
    { .compatible = "fsl,ls1043a-wdt", .data = &imx_wdt_legacy },
    { .compatible = "fsl,vf610-wdt", .data = &imx_wdt },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, imx2_wdt_dt_ids);
    static struct platform_driver imx2_wdt_driver = {
    .shutdown	= imx2_wdt_shutdown,
    .driver		= {
    .name	= DRIVER_NAME,
    .pm     = pm_sleep_ptr(&imx2_wdt_pm_ops),
    .of_match_table = imx2_wdt_dt_ids,
    },
    };
    module_platform_driver_probe(imx2_wdt_driver, imx2_wdt_probe);
    MODULE_AUTHOR("Wolfram Sang");
    MODULE_DESCRIPTION("Watchdog driver for IMX2 and later");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:" DRIVER_NAME);
