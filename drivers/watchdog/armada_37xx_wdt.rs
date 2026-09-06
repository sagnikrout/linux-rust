//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/armada_37xx_wdt.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Watchdog driver for Marvell Armada 37xx SoCs
//
// Author: Marek Behún <kabel@kernel.org>
//

//
// There are four counters that can be used for watchdog on Armada 37xx.
// The addresses for counter control registers are register base plus ID*0x10,
// where ID is 0, 1, 2 or 3.
//
// In this driver we use IDs 0 and 1. Counter ID 1 is used as watchdog counter,
// while counter ID 0 is used to implement pinging the watchdog: counter ID 1 is
// set to restart counting from initial value on counter ID 0 end count event.
// Pinging is done by forcing immediate end count event on counter ID 0.
// If only one counter was used, pinging would have to be implemented by
// disabling and enabling the counter, leaving the system in a vulnerable state
// for a (really) short period of time.
//
// Counters ID 2 and 3 are enabled by default even before U-Boot loads,
// therefore this driver does not provide a way to use them, eg. by setting a
// property in device tree.
//
pub const CNTR_ID_RETRIGGER: c_int = 0;
pub const CNTR_ID_WDOG: c_int = 1;
// relative to cpu_misc
pub const WDT_TIMER_SELECT: c_uint = 0x64;
pub const WDT_TIMER_SELECT_MASK: c_uint = 0xf;

// relative to reg

pub const CNTR_CTRL_ENABLE: c_uint = 0x0001;
pub const CNTR_CTRL_ACTIVE: c_uint = 0x0002;
pub const CNTR_CTRL_MODE_MASK: c_uint = 0x000c;
pub const CNTR_CTRL_MODE_ONESHOT: c_uint = 0x0000;
pub const CNTR_CTRL_MODE_HWSIG: c_uint = 0x000c;
pub const CNTR_CTRL_TRIG_SRC_MASK: c_uint = 0x00f0;
pub const CNTR_CTRL_TRIG_SRC_PREV_CNTR: c_uint = 0x0050;
pub const CNTR_CTRL_PRESCALE_MASK: c_uint = 0xff00;
pub const CNTR_CTRL_PRESCALE_MIN: c_int = 2;
pub const CNTR_CTRL_PRESCALE_SHIFT: c_int = 8;

pub const WATCHDOG_TIMEOUT: c_int = 120;
    static unsigned int timeout;
    module_param(timeout, int, 0);
    MODULE_PARM_DESC(timeout, "Watchdog timeout in seconds");
    let mut nowayout: static bool = WATCHDOG_NOWAYOUT;
    module_param(nowayout, bool, 0);
    MODULE_PARM_DESC(nowayout, "Watchdog cannot be stopped once started (default="
    __MODULE_STRING(WATCHDOG_NOWAYOUT) ")");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct armada_37xx_watchdog {
    pub wdt: watchdog_device,
    pub cpu_misc: *mut regmap,
    pub reg: *mut void __iomem,
    pub /: *mut *mut u64 timeout; / in clock ticks,
    pub clk_rate: c_ulong,
    pub clk: *mut clk,
}

#[no_mangle]
unsafe extern "C" fn get_counter_value(dev: *mut armada_37xx_watchdog, id: c_int) -> u64 {
    static u64 get_counter_value(struct armada_37xx_watchdog *dev, int id)
    {
    u64 val;
//
// when low is read, high is latched into flip-flops so that it can be
// read consistently without using software debouncing
//
    val = readl(dev.reg + CNTR_COUNT_LOW(id));
    val |= ((u64)readl(dev.reg + CNTR_COUNT_HIGH(id))) << 32;
    return val;
    }
#[no_mangle]
unsafe extern "C" fn set_counter_value(dev: *mut armada_37xx_watchdog, id: c_int, val: u64) {
    static void set_counter_value(struct armada_37xx_watchdog *dev, int id, u64 val)
    {
    writel(val & 0xffffffff, dev.reg + CNTR_COUNT_LOW(id));
    writel(val >> 32, dev.reg + CNTR_COUNT_HIGH(id));
    }
#[no_mangle]
unsafe extern "C" fn counter_enable(dev: *mut armada_37xx_watchdog, id: c_int) {
    static void counter_enable(struct armada_37xx_watchdog *dev, int id)
    {
    u32 reg;
    reg = readl(dev.reg + CNTR_CTRL(id));
    reg |= CNTR_CTRL_ENABLE;
    writel(reg, dev.reg + CNTR_CTRL(id));
    }
#[no_mangle]
unsafe extern "C" fn counter_disable(dev: *mut armada_37xx_watchdog, id: c_int) {
    static void counter_disable(struct armada_37xx_watchdog *dev, int id)
    {
    u32 reg;
    reg = readl(dev.reg + CNTR_CTRL(id));
    reg &= ~CNTR_CTRL_ENABLE;
    writel(reg, dev.reg + CNTR_CTRL(id));
    }
    static void init_counter(struct armada_37xx_watchdog *dev, int id, u32 mode,
    u32 trig_src)
    {
    u32 reg;
    reg = readl(dev.reg + CNTR_CTRL(id));
    reg &= ~(CNTR_CTRL_MODE_MASK | CNTR_CTRL_PRESCALE_MASK |
    CNTR_CTRL_TRIG_SRC_MASK);
// set mode
    reg |= mode & CNTR_CTRL_MODE_MASK;
// set prescaler to the min value
    reg |= CNTR_CTRL_PRESCALE_MIN << CNTR_CTRL_PRESCALE_SHIFT;
// set trigger source
    reg |= trig_src & CNTR_CTRL_TRIG_SRC_MASK;
    writel(reg, dev.reg + CNTR_CTRL(id));
    }
#[no_mangle]
unsafe extern "C" fn armada_37xx_wdt_ping(wdt: *mut watchdog_device) -> c_int {
    static int armada_37xx_wdt_ping(struct watchdog_device *wdt)
    {
    struct armada_37xx_watchdog *dev = watchdog_get_drvdata(wdt);
// counter 1 is retriggered by forcing end count on counter 0
    counter_disable(dev, CNTR_ID_RETRIGGER);
    counter_enable(dev, CNTR_ID_RETRIGGER);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn armada_37xx_wdt_get_timeleft(wdt: *mut watchdog_device) -> c_uint {
    static unsigned int armada_37xx_wdt_get_timeleft(struct watchdog_device *wdt)
    {
    struct armada_37xx_watchdog *dev = watchdog_get_drvdata(wdt);
    u64 res;
    res = get_counter_value(dev, CNTR_ID_WDOG) * CNTR_CTRL_PRESCALE_MIN;
    do_div(res, dev.clk_rate);
    return res;
    }
    static int armada_37xx_wdt_set_timeout(struct watchdog_device *wdt,
    unsigned int timeout)
    {
    struct armada_37xx_watchdog *dev = watchdog_get_drvdata(wdt);
    wdt.timeout = timeout;
//
// Compute the timeout in clock rate. We use smallest possible
// prescaler, which divides the clock rate by 2
// (CNTR_CTRL_PRESCALE_MIN).
//
    dev.timeout = (u64)dev.clk_rate * timeout;
    do_div(dev.timeout, CNTR_CTRL_PRESCALE_MIN);
    set_counter_value(dev, CNTR_ID_WDOG, dev.timeout);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn armada_37xx_wdt_is_running(dev: *mut armada_37xx_watchdog) -> bool {
    static bool armada_37xx_wdt_is_running(struct armada_37xx_watchdog *dev)
    {
    u32 reg;
    regmap_read(dev.cpu_misc, WDT_TIMER_SELECT, &reg);
    if ((reg & WDT_TIMER_SELECT_MASK) != WDT_TIMER_SELECT_VAL)
    return false;
    reg = readl(dev.reg + CNTR_CTRL(CNTR_ID_WDOG));
    return !!(reg & CNTR_CTRL_ACTIVE);
    }
#[no_mangle]
unsafe extern "C" fn armada_37xx_wdt_start(wdt: *mut watchdog_device) -> c_int {
    static int armada_37xx_wdt_start(struct watchdog_device *wdt)
    {
    struct armada_37xx_watchdog *dev = watchdog_get_drvdata(wdt);
// select counter 1 as watchdog counter
    regmap_write(dev.cpu_misc, WDT_TIMER_SELECT, WDT_TIMER_SELECT_VAL);
// init counter 0 as retrigger counter for counter 1
    init_counter(dev, CNTR_ID_RETRIGGER, CNTR_CTRL_MODE_ONESHOT, 0);
    set_counter_value(dev, CNTR_ID_RETRIGGER, 0);
// init counter 1 to be retriggerable by counter 0 end count
    init_counter(dev, CNTR_ID_WDOG, CNTR_CTRL_MODE_HWSIG,
    CNTR_CTRL_TRIG_SRC_PREV_CNTR);
    set_counter_value(dev, CNTR_ID_WDOG, dev.timeout);
// enable counter 1
    counter_enable(dev, CNTR_ID_WDOG);
// start counter 1 by forcing immediate end count on counter 0
    counter_enable(dev, CNTR_ID_RETRIGGER);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn armada_37xx_wdt_stop(wdt: *mut watchdog_device) -> c_int {
    static int armada_37xx_wdt_stop(struct watchdog_device *wdt)
    {
    struct armada_37xx_watchdog *dev = watchdog_get_drvdata(wdt);
    counter_disable(dev, CNTR_ID_WDOG);
    counter_disable(dev, CNTR_ID_RETRIGGER);
    regmap_write(dev.cpu_misc, WDT_TIMER_SELECT, 0);
    return 0;
    }
    static const struct watchdog_info armada_37xx_wdt_info = {
    .options = WDIOF_SETTIMEOUT | WDIOF_KEEPALIVEPING | WDIOF_MAGICCLOSE,
    .identity = "Armada 37xx Watchdog",
    };
    static const struct watchdog_ops armada_37xx_wdt_ops = {
    .owner = THIS_MODULE,
    .start = armada_37xx_wdt_start,
    .stop = armada_37xx_wdt_stop,
    .ping = armada_37xx_wdt_ping,
    .set_timeout = armada_37xx_wdt_set_timeout,
    .get_timeleft = armada_37xx_wdt_get_timeleft,
    };
#[no_mangle]
unsafe extern "C" fn armada_37xx_wdt_probe(pdev: *mut platform_device) -> c_int {
    static int armada_37xx_wdt_probe(struct platform_device *pdev)
    {
    struct armada_37xx_watchdog *dev;
    struct regmap *regmap;
    int ret;
    dev = devm_kzalloc(&pdev.dev, sizeof(struct armada_37xx_watchdog),
    GFP_KERNEL);
    if (!dev)
    return -ENOMEM;
    dev.wdt.info = &armada_37xx_wdt_info;
    dev.wdt.ops = &armada_37xx_wdt_ops;
    regmap = syscon_regmap_lookup_by_phandle(pdev.dev.of_node,
    "marvell,system-controller");
    if (IS_ERR(regmap))
    return PTR_ERR(regmap);
    dev.cpu_misc = regmap;
    dev.reg = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(dev.reg))
    return PTR_ERR(dev.reg);
// init clock
    dev.clk = devm_clk_get_enabled(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(dev.clk))
    return PTR_ERR(dev.clk);
    dev.clk_rate = clk_get_rate(dev.clk);
    if (!dev.clk_rate)
    return -EINVAL;
//
// Since the timeout in seconds is given as 32 bit unsigned int, and
// the counters hold 64 bit values, even after multiplication by clock
// rate the counter can hold timeout of UINT_MAX seconds.
//
    dev.wdt.min_timeout = 1;
    dev.wdt.max_timeout = UINT_MAX;
    dev.wdt.parent = &pdev.dev;
// default value, possibly override by module parameter or dtb
    dev.wdt.timeout = WATCHDOG_TIMEOUT;
    watchdog_init_timeout(&dev.wdt, timeout, &pdev.dev);
    platform_set_drvdata(pdev, &dev.wdt);
    watchdog_set_drvdata(&dev.wdt, dev);
    armada_37xx_wdt_set_timeout(&dev.wdt, dev.wdt.timeout);
    if (armada_37xx_wdt_is_running(dev))
    set_bit(WDOG_HW_RUNNING, &dev.wdt.status);
    watchdog_set_nowayout(&dev.wdt, nowayout);
    watchdog_stop_on_reboot(&dev.wdt);
    ret = devm_watchdog_register_device(&pdev.dev, &dev.wdt);
    if (ret)
    return ret;
    dev_info(&pdev.dev, "Initial timeout %d sec%s\n",
    dev.wdt.timeout, nowayout ? ", nowayout" : "");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn armada_37xx_wdt_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused armada_37xx_wdt_suspend(struct device *dev)
    {
    struct watchdog_device *wdt = dev_get_drvdata(dev);
    return armada_37xx_wdt_stop(wdt);
    }
#[no_mangle]
unsafe extern "C" fn armada_37xx_wdt_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused armada_37xx_wdt_resume(struct device *dev)
    {
    struct watchdog_device *wdt = dev_get_drvdata(dev);
    if (watchdog_active(wdt))
    return armada_37xx_wdt_start(wdt);
    return 0;
    }
    static const struct dev_pm_ops armada_37xx_wdt_dev_pm_ops = {
    SET_SYSTEM_SLEEP_PM_OPS(armada_37xx_wdt_suspend,
    armada_37xx_wdt_resume)
    };

    static const struct of_device_id armada_37xx_wdt_match[] = {
    { .compatible = "marvell,armada-3700-wdt", },
    {},
    };
    MODULE_DEVICE_TABLE(of, armada_37xx_wdt_match);

    static struct platform_driver armada_37xx_wdt_driver = {
    .probe		= armada_37xx_wdt_probe,
    .driver		= {
    .name	= "armada_37xx_wdt",
    .of_match_table = of_match_ptr(armada_37xx_wdt_match),
    .pm = &armada_37xx_wdt_dev_pm_ops,
    },
    };
    module_platform_driver(armada_37xx_wdt_driver);
    MODULE_AUTHOR("Marek Behun <kabel@kernel.org>");
    MODULE_DESCRIPTION("Armada 37xx CPU Watchdog");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:armada_37xx_wdt");
