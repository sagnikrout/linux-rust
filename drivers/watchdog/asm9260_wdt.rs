//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/asm9260_wdt.c
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
// Watchdog driver for Alphascale ASM9260.
//
// Copyright (c) 2014 Oleksij Rempel <linux@rempel-privat.de>
//

pub const CLOCK_FREQ: c_int = 1000000;
// Watchdog Mode register
pub const HW_WDMOD: c_uint = 0x00;
// Wake interrupt. Set by HW, can't be cleared.

// This bit set if timeout reached. Cleared by SW.

// HW Reset on timeout

// WD enable

//
// Watchdog Timer Constant register
// Minimal value is 0xff, the meaning of this value
// depends on used clock: T = WDCLK * (0xff + 1) * 4
//
pub const HW_WDTC: c_uint = 0x04;

// Watchdog Feed register
pub const HW_WDFEED: c_uint = 0x08;
// Watchdog Timer Value register
pub const HW_WDTV: c_uint = 0x0c;
pub const ASM9260_WDT_DEFAULT_TIMEOUT: c_int = 30;
    enum asm9260_wdt_mode {
    HW_RESET,
    SW_RESET,
    DEBUG,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct asm9260_wdt_priv {
    pub dev: *mut device,
    pub wdd: watchdog_device,
    pub clk: *mut clk,
    pub clk_ahb: *mut clk,
    pub rst: *mut reset_control,
    pub iobase: *mut void __iomem,
    pub irq: c_int,
    pub wdt_freq: c_ulong,
    pub mode: enum asm9260_wdt_mode,
}

#[no_mangle]
unsafe extern "C" fn asm9260_wdt_feed(wdd: *mut watchdog_device) -> c_int {
    static int asm9260_wdt_feed(struct watchdog_device *wdd)
    {
    struct asm9260_wdt_priv *priv = watchdog_get_drvdata(wdd);
    iowrite32(0xaa, priv.iobase + HW_WDFEED);
    iowrite32(0x55, priv.iobase + HW_WDFEED);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn asm9260_wdt_gettimeleft(wdd: *mut watchdog_device) -> c_uint {
    static unsigned int asm9260_wdt_gettimeleft(struct watchdog_device *wdd)
    {
    struct asm9260_wdt_priv *priv = watchdog_get_drvdata(wdd);
    u32 counter;
    counter = ioread32(priv.iobase + HW_WDTV);
    return counter / priv.wdt_freq;
    }
#[no_mangle]
unsafe extern "C" fn asm9260_wdt_updatetimeout(wdd: *mut watchdog_device) -> c_int {
    static int asm9260_wdt_updatetimeout(struct watchdog_device *wdd)
    {
    struct asm9260_wdt_priv *priv = watchdog_get_drvdata(wdd);
    u32 counter;
    counter = wdd.timeout * priv.wdt_freq;
    iowrite32(counter, priv.iobase + HW_WDTC);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn asm9260_wdt_enable(wdd: *mut watchdog_device) -> c_int {
    static int asm9260_wdt_enable(struct watchdog_device *wdd)
    {
    struct asm9260_wdt_priv *priv = watchdog_get_drvdata(wdd);
    let mut mode: u32 = 0;
    if (priv.mode == HW_RESET)
    mode = BM_MOD_WDRESET;
    iowrite32(BM_MOD_WDEN | mode, priv.iobase + HW_WDMOD);
    asm9260_wdt_updatetimeout(wdd);
    asm9260_wdt_feed(wdd);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn asm9260_wdt_disable(wdd: *mut watchdog_device) -> c_int {
    static int asm9260_wdt_disable(struct watchdog_device *wdd)
    {
    struct asm9260_wdt_priv *priv = watchdog_get_drvdata(wdd);
// The only way to disable WD is to reset it.
    reset_control_assert(priv.rst);
    reset_control_deassert(priv.rst);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn asm9260_wdt_settimeout(wdd: *mut watchdog_device, to: c_uint) -> c_int {
    static int asm9260_wdt_settimeout(struct watchdog_device *wdd, unsigned int to)
    {
    wdd.timeout = to;
    asm9260_wdt_updatetimeout(wdd);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn asm9260_wdt_sys_reset(priv: *mut asm9260_wdt_priv) {
    static void asm9260_wdt_sys_reset(struct asm9260_wdt_priv *priv)
    {
// init WD if it was not started
    iowrite32(BM_MOD_WDEN | BM_MOD_WDRESET, priv.iobase + HW_WDMOD);
    iowrite32(0xff, priv.iobase + HW_WDTC);
// first pass correct sequence
    asm9260_wdt_feed(&priv.wdd);
//
// Then write wrong pattern to the feed to trigger reset
// ASAP.
//
    iowrite32(0xff, priv.iobase + HW_WDFEED);
    mdelay(1000);
    }
#[no_mangle]
unsafe extern "C" fn asm9260_wdt_irq(irq: c_int, devid: *mut c_void) -> irqreturn_t {
    static irqreturn_t asm9260_wdt_irq(int irq, void *devid)
    {
    struct asm9260_wdt_priv *priv = devid;
    u32 stat;
    stat = ioread32(priv.iobase + HW_WDMOD);
    if (!(stat & BM_MOD_WDINT))
    return IRQ_NONE;
    if (priv.mode == DEBUG) {
    dev_info(priv.dev, "Watchdog Timeout. Do nothing.\n");
    } else {
    dev_info(priv.dev, "Watchdog Timeout. Doing SW Reset.\n");
    asm9260_wdt_sys_reset(priv);
    }
    return IRQ_HANDLED;
    }
    static int asm9260_restart(struct watchdog_device *wdd, unsigned long action,
    void *data)
    {
    struct asm9260_wdt_priv *priv = watchdog_get_drvdata(wdd);
    asm9260_wdt_sys_reset(priv);
    return 0;
    }
    static const struct watchdog_info asm9260_wdt_ident = {
    .options          =     WDIOF_SETTIMEOUT | WDIOF_KEEPALIVEPING
    | WDIOF_MAGICCLOSE,
    .identity         =	"Alphascale asm9260 Watchdog",
    };
    static const struct watchdog_ops asm9260_wdt_ops = {
    .owner		= THIS_MODULE,
    .start		= asm9260_wdt_enable,
    .stop		= asm9260_wdt_disable,
    .get_timeleft	= asm9260_wdt_gettimeleft,
    .ping		= asm9260_wdt_feed,
    .set_timeout	= asm9260_wdt_settimeout,
    .restart	= asm9260_restart,
    };
#[no_mangle]
unsafe extern "C" fn asm9260_clk_disable_unprepare(data: *mut c_void) {
    static void asm9260_clk_disable_unprepare(void *data)
    {
    clk_disable_unprepare(data);
    }
#[no_mangle]
unsafe extern "C" fn asm9260_wdt_get_dt_clks(priv: *mut asm9260_wdt_priv) -> c_int {
    static int asm9260_wdt_get_dt_clks(struct asm9260_wdt_priv *priv)
    {
    int err;
    unsigned long clk;
    priv.clk = devm_clk_get(priv.dev, "mod");
    if (IS_ERR(priv.clk)) {
    dev_err(priv.dev, "Failed to get \"mod\" clk\n");
    return PTR_ERR(priv.clk);
    }
// configure AHB clock
    priv.clk_ahb = devm_clk_get(priv.dev, "ahb");
    if (IS_ERR(priv.clk_ahb)) {
    dev_err(priv.dev, "Failed to get \"ahb\" clk\n");
    return PTR_ERR(priv.clk_ahb);
    }
    err = clk_prepare_enable(priv.clk_ahb);
    if (err) {
    dev_err(priv.dev, "Failed to enable ahb_clk!\n");
    return err;
    }
    err = devm_add_action_or_reset(priv.dev,
    asm9260_clk_disable_unprepare,
    priv.clk_ahb);
    if (err)
    return err;
    err = clk_set_rate(priv.clk, CLOCK_FREQ);
    if (err) {
    dev_err(priv.dev, "Failed to set rate!\n");
    return err;
    }
    err = clk_prepare_enable(priv.clk);
    if (err) {
    dev_err(priv.dev, "Failed to enable clk!\n");
    return err;
    }
    err = devm_add_action_or_reset(priv.dev,
    asm9260_clk_disable_unprepare,
    priv.clk);
    if (err)
    return err;
// wdt has internal divider
    clk = clk_get_rate(priv.clk);
    if (!clk) {
    dev_err(priv.dev, "Failed, clk is 0!\n");
    return -EINVAL;
    }
    priv.wdt_freq = clk / 2;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn asm9260_wdt_get_dt_mode(priv: *mut asm9260_wdt_priv) {
    static void asm9260_wdt_get_dt_mode(struct asm9260_wdt_priv *priv)
    {
    const char *tmp;
    int ret;
// default mode
    priv.mode = HW_RESET;
    ret = of_property_read_string(priv.dev.of_node,
    "alphascale,mode", &tmp);
    if (ret < 0)
    return;
    if (!strcmp(tmp, "hw"))
    priv.mode = HW_RESET;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(tmp, _arg: "sw")) -> else {
    else if (!strcmp(tmp, "sw"))
    priv.mode = SW_RESET;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(tmp, _arg: "debug")) -> else {
    else if (!strcmp(tmp, "debug"))
    priv.mode = DEBUG;
    else
    dev_warn(priv.dev, "unknown reset-type: %s. Using default \"hw\" mode.",
    tmp);
    }
#[no_mangle]
unsafe extern "C" fn asm9260_wdt_probe(pdev: *mut platform_device) -> c_int {
    static int asm9260_wdt_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct asm9260_wdt_priv *priv;
    struct watchdog_device *wdd;
    int ret;
    static const char * const mode_name[] = { "hw", "sw", "debug", };
    priv = devm_kzalloc(dev, sizeof(struct asm9260_wdt_priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.dev = dev;
    priv.iobase = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.iobase))
    return PTR_ERR(priv.iobase);
    priv.rst = devm_reset_control_get_exclusive(dev, "wdt_rst");
    if (IS_ERR(priv.rst))
    return PTR_ERR(priv.rst);
    ret = asm9260_wdt_get_dt_clks(priv);
    if (ret)
    return ret;
    wdd = &priv.wdd;
    wdd.info = &asm9260_wdt_ident;
    wdd.ops = &asm9260_wdt_ops;
    wdd.min_timeout = 1;
    wdd.max_timeout = BM_WDTC_MAX(priv.wdt_freq);
    wdd.parent = dev;
    watchdog_set_drvdata(wdd, priv);
//
// If 'timeout-sec' unspecified in devicetree, assume a 30 second
// default, unless the max timeout is less than 30 seconds, then use
// the max instead.
//
    wdd.timeout = ASM9260_WDT_DEFAULT_TIMEOUT;
    watchdog_init_timeout(wdd, 0, dev);
    asm9260_wdt_get_dt_mode(priv);
    if (priv.mode != HW_RESET)
    priv.irq = platform_get_irq(pdev, 0);
    if (priv.irq > 0) {
//
// Not all supported platforms specify an interrupt for the
// watchdog, so let's make it optional.
//
    ret = devm_request_irq(dev, priv.irq, asm9260_wdt_irq, 0,
    pdev.name, priv);
    if (ret < 0)
    dev_warn(dev, "failed to request IRQ\n");
    }
    watchdog_set_restart_priority(wdd, 128);
    watchdog_stop_on_reboot(wdd);
    watchdog_stop_on_unregister(wdd);
    ret = devm_watchdog_register_device(dev, wdd);
    if (ret)
    return ret;
    platform_set_drvdata(pdev, priv);
    dev_info(dev, "Watchdog enabled (timeout: %d sec, mode: %s)\n",
    wdd.timeout, mode_name[priv.mode]);
    return 0;
    }
    static const struct of_device_id asm9260_wdt_of_match[] = {
    { .compatible = "alphascale,asm9260-wdt"},
    {},
    };
    MODULE_DEVICE_TABLE(of, asm9260_wdt_of_match);
    static struct platform_driver asm9260_wdt_driver = {
    .driver = {
    .name = "asm9260-wdt",
    .of_match_table	= asm9260_wdt_of_match,
    },
    .probe = asm9260_wdt_probe,
    };
    module_platform_driver(asm9260_wdt_driver);
    MODULE_DESCRIPTION("asm9260 WatchDog Timer Driver");
    MODULE_AUTHOR("Oleksij Rempel <linux@rempel-privat.de>");
    MODULE_LICENSE("GPL");
