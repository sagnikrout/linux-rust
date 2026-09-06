//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/sama5d4_wdt.c
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
// Driver for Atmel SAMA5D4 Watchdog Timer
//
// Copyright (C) 2015-2019 Microchip Technology Inc. and its subsidiaries
//

// minimum and maximum watchdog timeout, in seconds
pub const MIN_WDT_TIMEOUT: c_int = 1;
pub const MAX_WDT_TIMEOUT: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sama5d4_wdt {
    pub wdd: watchdog_device,
    pub reg_base: *mut void __iomem,
    pub mr: u32,
    pub ir: u32,
    pub wddis_mask: u32,
    pub last_ping: c_ulong,
    pub need_irq: bool,
    pub sam9x60_support: bool,
}

    static int wdt_timeout;
    let mut nowayout: static bool = WATCHDOG_NOWAYOUT;
    module_param(wdt_timeout, int, 0);
    MODULE_PARM_DESC(wdt_timeout,
    "Watchdog timeout in seconds. (default = "
    __MODULE_STRING(WDT_DEFAULT_TIMEOUT) ")");
    module_param(nowayout, bool, 0);
    MODULE_PARM_DESC(nowayout,
    "Watchdog cannot be stopped once started (default="
    __MODULE_STRING(WATCHDOG_NOWAYOUT) ")");
#[no_mangle]
pub unsafe extern "C" fn wdt_enabled(wdt: *mut sama5d4_wdt) -> bool {
    static inline bool wdt_enabled(struct sama5d4_wdt *wdt)
    {
    return !(wdt.mr & wdt.wddis_mask);
    }

    readl_relaxed((wdt).reg_base + (field))
// 4 slow clock periods is 4/32768 = 122.07µs

#[no_mangle]
unsafe extern "C" fn wdt_write(wdt: *mut sama5d4_wdt, field: u32, val: u32) {
    static void wdt_write(struct sama5d4_wdt *wdt, u32 field, u32 val)
    {
//
// WDT_CR and WDT_MR must not be modified within three slow clock
// periods following a restart of the watchdog performed by a write
// access in WDT_CR.
//
    while (time_before(jiffies, wdt.last_ping + WDT_DELAY))
    usleep_range(30, 125);
    writel_relaxed(val, wdt.reg_base + field);
    wdt.last_ping = jiffies;
    }
#[no_mangle]
unsafe extern "C" fn wdt_write_nosleep(wdt: *mut sama5d4_wdt, field: u32, val: u32) {
    static void wdt_write_nosleep(struct sama5d4_wdt *wdt, u32 field, u32 val)
    {
    if (time_before(jiffies, wdt.last_ping + WDT_DELAY))
    udelay(123);
    writel_relaxed(val, wdt.reg_base + field);
    wdt.last_ping = jiffies;
    }
#[no_mangle]
unsafe extern "C" fn sama5d4_wdt_start(wdd: *mut watchdog_device) -> c_int {
    static int sama5d4_wdt_start(struct watchdog_device *wdd)
    {
    struct sama5d4_wdt *wdt = watchdog_get_drvdata(wdd);
    if (wdt.sam9x60_support)
    writel_relaxed(wdt.ir, wdt.reg_base + AT91_SAM9X60_IER);
    wdt.mr &= ~wdt.wddis_mask;
    wdt_write(wdt, AT91_WDT_MR, wdt.mr);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sama5d4_wdt_stop(wdd: *mut watchdog_device) -> c_int {
    static int sama5d4_wdt_stop(struct watchdog_device *wdd)
    {
    struct sama5d4_wdt *wdt = watchdog_get_drvdata(wdd);
    if (wdt.sam9x60_support)
    writel_relaxed(wdt.ir, wdt.reg_base + AT91_SAM9X60_IDR);
    wdt.mr |= wdt.wddis_mask;
    wdt_write(wdt, AT91_WDT_MR, wdt.mr);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sama5d4_wdt_ping(wdd: *mut watchdog_device) -> c_int {
    static int sama5d4_wdt_ping(struct watchdog_device *wdd)
    {
    struct sama5d4_wdt *wdt = watchdog_get_drvdata(wdd);
    wdt_write(wdt, AT91_WDT_CR, AT91_WDT_KEY | AT91_WDT_WDRSTT);
    return 0;
    }
    static int sama5d4_wdt_set_timeout(struct watchdog_device *wdd,
    unsigned int timeout)
    {
    struct sama5d4_wdt *wdt = watchdog_get_drvdata(wdd);
    let mut value: u32 = WDT_SEC2TICKS(timeout);
    if (wdt.sam9x60_support) {
    wdt_write(wdt, AT91_SAM9X60_WLR,
    AT91_SAM9X60_SET_COUNTER(value));
    wdd.timeout = timeout;
    return 0;
    }
    wdt.mr &= ~AT91_WDT_WDV;
    wdt.mr |= AT91_WDT_SET_WDV(value);
//
// WDDIS has to be 0 when updating WDD/WDV. The datasheet states: When
// setting the WDDIS bit, and while it is set, the fields WDV and WDD
// must not be modified.
// If the watchdog is enabled, then the timeout can be updated. Else,
// wait that the user enables it.
//
    if (wdt_enabled(wdt))
    wdt_write(wdt, AT91_WDT_MR, wdt.mr & ~wdt.wddis_mask);
    wdd.timeout = timeout;
    return 0;
    }
    static const struct watchdog_info sama5d4_wdt_info = {
    .options = WDIOF_SETTIMEOUT | WDIOF_MAGICCLOSE | WDIOF_KEEPALIVEPING,
    .identity = "Atmel SAMA5D4 Watchdog",
    };
    static const struct watchdog_ops sama5d4_wdt_ops = {
    .owner = THIS_MODULE,
    .start = sama5d4_wdt_start,
    .stop = sama5d4_wdt_stop,
    .ping = sama5d4_wdt_ping,
    .set_timeout = sama5d4_wdt_set_timeout,
    };
#[no_mangle]
unsafe extern "C" fn sama5d4_wdt_irq_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t sama5d4_wdt_irq_handler(int irq, void *dev_id)
    {
    struct sama5d4_wdt *wdt = platform_get_drvdata(dev_id);
    u32 reg;
    if (wdt.sam9x60_support)
    reg = wdt_read(wdt, AT91_SAM9X60_ISR);
    else
    reg = wdt_read(wdt, AT91_WDT_SR);
    if (reg) {
    pr_crit("Atmel Watchdog Software Reset\n");
    emergency_restart();
    pr_crit("Reboot didn't succeed\n");
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn of_sama5d4_wdt_init(np: *mut device_node, wdt: *mut sama5d4_wdt) -> c_int {
    static int of_sama5d4_wdt_init(struct device_node *np, struct sama5d4_wdt *wdt)
    {
    const char *tmp;
    wdt.mr = wdt.wddis_mask;
    if (!of_property_read_string(np, "atmel,watchdog-type", &tmp) &&
    !strcmp(tmp, "software"))
    wdt.need_irq = true;
    if (of_property_read_bool(np, "atmel,idle-halt"))
    wdt.mr |= AT91_WDT_WDIDLEHLT;
    if (of_property_read_bool(np, "atmel,dbg-halt"))
    wdt.mr |= AT91_WDT_WDDBGHLT;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sama5d4_wdt_init(wdt: *mut sama5d4_wdt) -> c_int {
    static int sama5d4_wdt_init(struct sama5d4_wdt *wdt)
    {
    u32 reg, val;
    val = WDT_SEC2TICKS(WDT_DEFAULT_TIMEOUT);
//
// When booting and resuming, the bootloader may have changed the
// watchdog configuration.
// If the watchdog is already running, we can safely update it.
// Else, we have to disable it properly.
//
    if (!wdt_enabled(wdt)) {
    reg = wdt_read(wdt, AT91_WDT_MR);
    if (!(reg & wdt.wddis_mask))
    wdt_write_nosleep(wdt, AT91_WDT_MR,
    reg | wdt.wddis_mask);
    }
    if (wdt.sam9x60_support) {
    if (wdt.need_irq)
    wdt.ir = AT91_SAM9X60_PERINT;
    else
    wdt.mr |= AT91_SAM9X60_PERIODRST;
    wdt_write(wdt, AT91_SAM9X60_IER, wdt.ir);
    wdt_write(wdt, AT91_SAM9X60_WLR, AT91_SAM9X60_SET_COUNTER(val));
    } else {
    wdt.mr |= AT91_WDT_SET_WDD(WDT_SEC2TICKS(MAX_WDT_TIMEOUT));
    wdt.mr |= AT91_WDT_SET_WDV(val);
    if (wdt.need_irq)
    wdt.mr |= AT91_WDT_WDFIEN;
    else
    wdt.mr |= AT91_WDT_WDRSTEN;
    }
    wdt_write_nosleep(wdt, AT91_WDT_MR, wdt.mr);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sama5d4_wdt_probe(pdev: *mut platform_device) -> c_int {
    static int sama5d4_wdt_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct watchdog_device *wdd;
    struct sama5d4_wdt *wdt;
    void __iomem *regs;
    let mut irq: u32 = 0;
    u32 reg;
    int ret;
    wdt = devm_kzalloc(dev, sizeof(*wdt), GFP_KERNEL);
    if (!wdt)
    return -ENOMEM;
    wdd = &wdt.wdd;
    wdd.timeout = WDT_DEFAULT_TIMEOUT;
    wdd.info = &sama5d4_wdt_info;
    wdd.ops = &sama5d4_wdt_ops;
    wdd.min_timeout = MIN_WDT_TIMEOUT;
    wdd.max_timeout = MAX_WDT_TIMEOUT;
    wdt.last_ping = jiffies;
    if (of_device_is_compatible(dev.of_node, "microchip,sam9x60-wdt") ||
    of_device_is_compatible(dev.of_node, "microchip,sama7g5-wdt"))
    wdt.sam9x60_support = true;
    wdt.wddis_mask = wdt.sam9x60_support ? AT91_SAM9X60_WDDIS
    : AT91_WDT_WDDIS;
    watchdog_set_drvdata(wdd, wdt);
    regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(regs))
    return PTR_ERR(regs);
    wdt.reg_base = regs;
    ret = of_sama5d4_wdt_init(dev.of_node, wdt);
    if (ret)
    return ret;
    if (wdt.need_irq) {
    irq = irq_of_parse_and_map(dev.of_node, 0);
    if (!irq) {
    dev_warn(dev, "failed to get IRQ from DT\n");
    wdt.need_irq = false;
    }
    }
    if (wdt.need_irq) {
    ret = devm_request_irq(dev, irq, sama5d4_wdt_irq_handler,
    IRQF_SHARED | IRQF_IRQPOLL |
    IRQF_NO_SUSPEND, pdev.name, pdev);
    if (ret)
    return ret;
    }
    watchdog_init_timeout(wdd, wdt_timeout, dev);
    reg = wdt_read(wdt, AT91_WDT_MR);
    if (!(reg & wdt.wddis_mask)) {
    wdt.mr &= ~wdt.wddis_mask;
    set_bit(WDOG_HW_RUNNING, &wdd.status);
    }
    ret = sama5d4_wdt_init(wdt);
    if (ret)
    return ret;
    watchdog_set_nowayout(wdd, nowayout);
    watchdog_stop_on_unregister(wdd);
    ret = devm_watchdog_register_device(dev, wdd);
    if (ret)
    return ret;
    platform_set_drvdata(pdev, wdt);
    dev_info(dev, "initialized (timeout = %d sec, nowayout = %d)\n",
    wdd.timeout, nowayout);
    return 0;
    }
    static const struct of_device_id sama5d4_wdt_of_match[] = {
    {
    .compatible = "atmel,sama5d4-wdt",
    },
    {
    .compatible = "microchip,sam9x60-wdt",
    },
    {
    .compatible = "microchip,sama7g5-wdt",
    },
    { }
    };
    MODULE_DEVICE_TABLE(of, sama5d4_wdt_of_match);
#[no_mangle]
unsafe extern "C" fn sama5d4_wdt_suspend_late(dev: *mut device) -> c_int {
    static int sama5d4_wdt_suspend_late(struct device *dev)
    {
    struct sama5d4_wdt *wdt = dev_get_drvdata(dev);
    if (watchdog_active(&wdt.wdd))
    sama5d4_wdt_stop(&wdt.wdd);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sama5d4_wdt_resume_early(dev: *mut device) -> c_int {
    static int sama5d4_wdt_resume_early(struct device *dev)
    {
    struct sama5d4_wdt *wdt = dev_get_drvdata(dev);
//
// FIXME: writing MR also pings the watchdog which may not be desired.
// This should only be done when the registers are lost on suspend but
// there is no way to get this information right now.
//
    sama5d4_wdt_init(wdt);
    if (watchdog_active(&wdt.wdd))
    sama5d4_wdt_start(&wdt.wdd);
    return 0;
    }
    static const struct dev_pm_ops sama5d4_wdt_pm_ops = {
    LATE_SYSTEM_SLEEP_PM_OPS(sama5d4_wdt_suspend_late,
    sama5d4_wdt_resume_early)
    };
    static struct platform_driver sama5d4_wdt_driver = {
    .probe		= sama5d4_wdt_probe,
    .driver		= {
    .name	= "sama5d4_wdt",
    .pm	= pm_sleep_ptr(&sama5d4_wdt_pm_ops),
    .of_match_table = sama5d4_wdt_of_match,
    }
    };
    module_platform_driver(sama5d4_wdt_driver);
    MODULE_AUTHOR("Atmel Corporation");
    MODULE_DESCRIPTION("Atmel SAMA5D4 Watchdog Timer driver");
    MODULE_LICENSE("GPL v2");
