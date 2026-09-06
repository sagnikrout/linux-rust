//! Automatically rewritten from C to Rust
//! Source: drivers/clocksource/timer-tegra186.c
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
// Copyright (c) 2019-2025 NVIDIA Corporation. All rights reserved.
//

// shared registers
pub const TKETSC0: c_uint = 0x000;
pub const TKETSC1: c_uint = 0x004;
pub const TKEUSEC: c_uint = 0x008;
pub const TKEOSC: c_uint = 0x00c;

// timer registers
pub const TMRCR: c_uint = 0x000;

pub const TMRSR: c_uint = 0x004;

pub const TMRCSSR: c_uint = 0x008;

// watchdog registers
pub const WDTCR: c_uint = 0x000;

pub const WDTCR_TIMER_SOURCE_MASK: c_uint = 0xf;

pub const WDTSR: c_uint = 0x004;

pub const WDTCMDR: c_uint = 0x008;

pub const WDTUR: c_uint = 0x00c;
pub const WDTUR_UNLOCK_PATTERN: c_uint = 0x0000c45a;
pub const TEGRA186_KERNEL_WDT_TIMEOUT: c_int = 120;
// WDT security configuration registers

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra186_timer_soc {
    pub num_timers: c_uint,
    pub num_wdts: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra186_tmr {
    pub parent: *mut tegra186_timer,
    pub regs: *mut void __iomem,
    pub index: c_uint,
    pub hwirq: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra186_wdt {
    pub base: watchdog_device,
    pub regs: *mut void __iomem,
    pub index: c_uint,
    pub locked: bool,
    pub is_kernel_wdt: bool,
    pub tmr: *mut tegra186_tmr,
}

    static inline struct tegra186_wdt *to_tegra186_wdt(struct watchdog_device *wdd)
    {
    return container_of(wdd, struct tegra186_wdt, base);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra186_timer {
    pub soc: *const tegra186_timer_soc,
    pub dev: *mut device,
    pub regs: *mut void __iomem,
    pub wdts: *mut tegra186_wdt,
    pub usec: clocksource,
    pub tsc: clocksource,
    pub osc: clocksource,
}

#[no_mangle]
unsafe extern "C" fn tmr_writel(tmr: *mut tegra186_tmr, value: u32, offset: c_uint) {
    static void tmr_writel(struct tegra186_tmr *tmr, u32 value, unsigned int offset)
    {
    writel_relaxed(value, tmr.regs + offset);
    }
#[no_mangle]
unsafe extern "C" fn wdt_writel(wdt: *mut tegra186_wdt, value: u32, offset: c_uint) {
    static void wdt_writel(struct tegra186_wdt *wdt, u32 value, unsigned int offset)
    {
    writel_relaxed(value, wdt.regs + offset);
    }
#[no_mangle]
unsafe extern "C" fn wdt_readl(wdt: *mut tegra186_wdt, offset: c_uint) -> u32 {
    static u32 wdt_readl(struct tegra186_wdt *wdt, unsigned int offset)
    {
    return readl_relaxed(wdt.regs + offset);
    }
    static struct tegra186_tmr *tegra186_tmr_create(struct tegra186_timer *tegra,
    unsigned int index)
    {
    let mut offset: c_uint = 0x10000 + index * 0x10000;
    struct tegra186_tmr *tmr;
    tmr = devm_kzalloc(tegra.dev, sizeof(*tmr), GFP_KERNEL);
    if (!tmr)
    return ERR_PTR(-ENOMEM);
    tmr.parent = tegra;
    tmr.regs = tegra.regs + offset;
    tmr.index = index;
    tmr.hwirq = 0;
    return tmr;
    }
    static const struct watchdog_info tegra186_wdt_info = {
    .options = WDIOF_SETTIMEOUT | WDIOF_MAGICCLOSE | WDIOF_KEEPALIVEPING,
    .identity = "NVIDIA Tegra186 WDT",
    };
#[no_mangle]
unsafe extern "C" fn tegra186_wdt_disable(wdt: *mut tegra186_wdt) {
    static void tegra186_wdt_disable(struct tegra186_wdt *wdt)
    {
// unlock and disable the watchdog
    wdt_writel(wdt, WDTUR_UNLOCK_PATTERN, WDTUR);
    wdt_writel(wdt, WDTCMDR_DISABLE_COUNTER, WDTCMDR);
// disable timer
    tmr_writel(wdt.tmr, 0, TMRCR);
    }
#[no_mangle]
unsafe extern "C" fn tegra186_wdt_enable(wdt: *mut tegra186_wdt) {
    static void tegra186_wdt_enable(struct tegra186_wdt *wdt)
    {
    struct tegra186_timer *tegra = wdt.tmr.parent;
    u32 value;
// unmask hardware IRQ, this may have been lost across powergate
    value = readl(tegra.regs + TKEIE(wdt.tmr.hwirq));
    value |= TKEIE_WDT_MASK(wdt.index, 1);
    writel(value, tegra.regs + TKEIE(wdt.tmr.hwirq));
// clear interrupt
    tmr_writel(wdt.tmr, TMRSR_INTR_CLR, TMRSR);
// select microsecond source
    tmr_writel(wdt.tmr, TMRCSSR_SRC_USEC, TMRCSSR);
// configure timer (system reset happens on the fifth expiration)
    value = TMRCR_PTV(wdt.base.timeout * (USEC_PER_SEC / 5)) |
    TMRCR_PERIODIC | TMRCR_ENABLE;
    tmr_writel(wdt.tmr, value, TMRCR);
    if (!wdt.locked) {
    value = wdt_readl(wdt, WDTCR);
// select the proper timer source
    value &= ~WDTCR_TIMER_SOURCE_MASK;
    value |= WDTCR_TIMER_SOURCE(wdt.tmr.index);
// single timer period since that's already configured
    value &= ~WDTCR_PERIOD_MASK;
    value |= WDTCR_PERIOD(1);
// enable local interrupt for kernel watchdog
    if (wdt.is_kernel_wdt)
    value |= WDTCR_LOCAL_INT_ENABLE;
// enable system POR reset
    value |= WDTCR_SYSTEM_POR_RESET_ENABLE;
    wdt_writel(wdt, value, WDTCR);
    }
    wdt_writel(wdt, WDTCMDR_START_COUNTER, WDTCMDR);
    }
#[no_mangle]
unsafe extern "C" fn tegra186_wdt_start(wdd: *mut watchdog_device) -> c_int {
    static int tegra186_wdt_start(struct watchdog_device *wdd)
    {
    struct tegra186_wdt *wdt = to_tegra186_wdt(wdd);
    tegra186_wdt_enable(wdt);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra186_wdt_stop(wdd: *mut watchdog_device) -> c_int {
    static int tegra186_wdt_stop(struct watchdog_device *wdd)
    {
    struct tegra186_wdt *wdt = to_tegra186_wdt(wdd);
    tegra186_wdt_disable(wdt);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra186_wdt_ping(wdd: *mut watchdog_device) -> c_int {
    static int tegra186_wdt_ping(struct watchdog_device *wdd)
    {
    struct tegra186_wdt *wdt = to_tegra186_wdt(wdd);
    tegra186_wdt_disable(wdt);
    tegra186_wdt_enable(wdt);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra186_wdt_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t tegra186_wdt_irq(int irq, void *data)
    {
    struct tegra186_wdt *wdt = data;
    tegra186_wdt_disable(wdt);
    tegra186_wdt_enable(wdt);
    return IRQ_HANDLED;
    }
    static int tegra186_wdt_set_timeout(struct watchdog_device *wdd,
    unsigned int timeout)
    {
    struct tegra186_wdt *wdt = to_tegra186_wdt(wdd);
    if (watchdog_active(&wdt.base))
    tegra186_wdt_disable(wdt);
    wdt.base.timeout = timeout;
    if (watchdog_active(&wdt.base))
    tegra186_wdt_enable(wdt);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra186_wdt_get_timeleft(wdd: *mut watchdog_device) -> c_uint {
    static unsigned int tegra186_wdt_get_timeleft(struct watchdog_device *wdd)
    {
    struct tegra186_wdt *wdt = to_tegra186_wdt(wdd);
    u32 expiration, val;
    u32 timeleft;
    if (!watchdog_active(&wdt.base)) {
// return zero if the watchdog timer is not activated.
    return 0;
    }
//
// Reset occurs on the fifth expiration of the
// watchdog timer and so when the watchdog timer is configured,
// the actual value programmed into the counter is 1/5 of the
// timeout value. Once the counter reaches 0, expiration count
// will be increased by 1 and the down counter restarts.
// Hence to get the time left before system reset we must
// combine 2 parts:
// 1. value of the current down counter
// 2. (number of counter expirations remaining) * (timeout/5)
//
// Get the current number of counter expirations. Should be a
// value between 0 and 4
//
    val = readl_relaxed(wdt.regs + WDTSR);
    expiration = FIELD_GET(WDTSR_CURRENT_EXPIRATION_COUNT, val);
    if (WARN_ON_ONCE(expiration > 4))
    return 0;
// Get the current counter value in microsecond.
    val = readl_relaxed(wdt.tmr.regs + TMRSR);
    timeleft = FIELD_GET(TMRSR_PCV, val);
//
// Calculate the time remaining by adding the time for the
// counter value to the time of the counter expirations that
// remain.
// Note: Since wdt->base.timeout is bound to 255, the maximum
// value added to timeleft is
// 255 * (1,000,000 / 5) * 4
// = 255 * 200,000 * 4
// = 204,000,000
// TMRSR_PCV is a 29-bit field.
// Its maximum value is 0x1fffffff = 536,870,911.
// 204,000,000 + 536,870,911 = 740,870,911 = 0x2C28CAFF.
// timeleft can therefore not overflow, and 64-bit calculations
// are not necessary.
//
    timeleft += (wdt.base.timeout * (USEC_PER_SEC / 5)) * (4 - expiration);
//
// Convert the current counter value to seconds,
// rounding to the nearest second.
//
    timeleft = DIV_ROUND_CLOSEST(timeleft, USEC_PER_SEC);
    return timeleft;
    }
    static const struct watchdog_ops tegra186_wdt_ops = {
    .owner = THIS_MODULE,
    .start = tegra186_wdt_start,
    .stop = tegra186_wdt_stop,
    .ping = tegra186_wdt_ping,
    .set_timeout = tegra186_wdt_set_timeout,
    .get_timeleft = tegra186_wdt_get_timeleft,
    };
#[no_mangle]
unsafe extern "C" fn tegra186_wdt_is_accessible(tegra: *mut tegra186_timer, index: c_uint) -> bool {
    static bool tegra186_wdt_is_accessible(struct tegra186_timer *tegra, unsigned int index)
    {
    u32 value;
    value = readl_relaxed(tegra.regs + WDTSCR(index));
// Check OS write access if write blocking is enabled.
    if ((value & WDTSCR_SEC_WEN) && !(value & WDTSCR_SEC_G1W))
    return false;
// Check OS read access if read blocking is enabled.
    if ((value & WDTSCR_SEC_REN) && !(value & WDTSCR_SEC_G1R))
    return false;
    return true;
    }
    static struct tegra186_wdt *tegra186_wdt_create(struct tegra186_timer *tegra,
    unsigned int index)
    {
    let mut offset: c_uint = 0x10000, source;
    struct tegra186_wdt *wdt;
    u32 value;
    int err;
    offset += tegra.soc.num_timers * 0x10000 + index * 0x10000;
    wdt = devm_kzalloc(tegra.dev, sizeof(*wdt), GFP_KERNEL);
    if (!wdt)
    return ERR_PTR(-ENOMEM);
    wdt.regs = tegra.regs + offset;
    wdt.index = index;
// read the watchdog configuration since it might be locked down
    value = wdt_readl(wdt, WDTCR);
    if (value & WDTCR_LOCAL_INT_ENABLE)
    wdt.locked = true;
    source = value & WDTCR_TIMER_SOURCE_MASK;
    wdt.tmr = tegra186_tmr_create(tegra, source);
    if (IS_ERR(wdt.tmr))
    return ERR_CAST(wdt.tmr);
    wdt.base.info = &tegra186_wdt_info;
    wdt.base.ops = &tegra186_wdt_ops;
    wdt.base.min_timeout = 1;
    wdt.base.max_timeout = 255;
    wdt.base.parent = tegra.dev;
    err = watchdog_init_timeout(&wdt.base, 5, tegra.dev);
    if (err < 0)
    return ERR_PTR(err);
    return wdt;
    }
#[no_mangle]
unsafe extern "C" fn tegra186_timer_tsc_read(cs: *mut clocksource) -> u64 {
    static u64 tegra186_timer_tsc_read(struct clocksource *cs)
    {
    struct tegra186_timer *tegra = container_of(cs, struct tegra186_timer,
    tsc);
    u32 hi, lo, ss;
    hi = readl_relaxed(tegra.regs + TKETSC1);
//
// The 56-bit value of the TSC is spread across two registers that are
// not synchronized. In order to read them atomically, ensure that the
// high 24 bits match before and after reading the low 32 bits.
//
    do {
// snapshot the high 24 bits
    ss = hi;
    lo = readl_relaxed(tegra.regs + TKETSC0);
    hi = readl_relaxed(tegra.regs + TKETSC1);
    } while (hi != ss);
    return (u64)hi << 32 | lo;
    }
#[no_mangle]
unsafe extern "C" fn tegra186_timer_tsc_init(tegra: *mut tegra186_timer) -> c_int {
    static int tegra186_timer_tsc_init(struct tegra186_timer *tegra)
    {
    tegra.tsc.name = "tsc";
    tegra.tsc.rating = 300;
    tegra.tsc.read = tegra186_timer_tsc_read;
    tegra.tsc.mask = CLOCKSOURCE_MASK(56);
    tegra.tsc.flags = CLOCK_SOURCE_IS_CONTINUOUS;
    tegra.tsc.owner = THIS_MODULE;
    return clocksource_register_hz(&tegra.tsc, 31250000);
    }
#[no_mangle]
unsafe extern "C" fn tegra186_timer_osc_read(cs: *mut clocksource) -> u64 {
    static u64 tegra186_timer_osc_read(struct clocksource *cs)
    {
    struct tegra186_timer *tegra = container_of(cs, struct tegra186_timer,
    osc);
    return readl_relaxed(tegra.regs + TKEOSC);
    }
#[no_mangle]
unsafe extern "C" fn tegra186_timer_osc_init(tegra: *mut tegra186_timer) -> c_int {
    static int tegra186_timer_osc_init(struct tegra186_timer *tegra)
    {
    tegra.osc.name = "osc";
    tegra.osc.rating = 300;
    tegra.osc.read = tegra186_timer_osc_read;
    tegra.osc.mask = CLOCKSOURCE_MASK(32);
    tegra.osc.flags = CLOCK_SOURCE_IS_CONTINUOUS;
    tegra.osc.owner = THIS_MODULE;
    return clocksource_register_hz(&tegra.osc, 38400000);
    }
#[no_mangle]
unsafe extern "C" fn tegra186_timer_usec_read(cs: *mut clocksource) -> u64 {
    static u64 tegra186_timer_usec_read(struct clocksource *cs)
    {
    struct tegra186_timer *tegra = container_of(cs, struct tegra186_timer,
    usec);
    return readl_relaxed(tegra.regs + TKEUSEC);
    }
#[no_mangle]
unsafe extern "C" fn tegra186_timer_usec_init(tegra: *mut tegra186_timer) -> c_int {
    static int tegra186_timer_usec_init(struct tegra186_timer *tegra)
    {
    tegra.usec.name = "usec";
    tegra.usec.rating = 300;
    tegra.usec.read = tegra186_timer_usec_read;
    tegra.usec.mask = CLOCKSOURCE_MASK(32);
    tegra.usec.flags = CLOCK_SOURCE_IS_CONTINUOUS;
    tegra.usec.owner = THIS_MODULE;
    return clocksource_register_hz(&tegra.usec, USEC_PER_SEC);
    }
#[no_mangle]
unsafe extern "C" fn tegra186_timer_probe(pdev: *mut platform_device) -> c_int {
    static int tegra186_timer_probe(struct platform_device *pdev)
    {
    struct tegra186_wdt *kernel_wdt = core::ptr::null_mut();
    struct device *dev = &pdev.dev;
    struct tegra186_timer *tegra;
    unsigned int i;
    int irq;
    int err;
    tegra = devm_kzalloc(dev, sizeof(*tegra), GFP_KERNEL);
    if (!tegra)
    return -ENOMEM;
    tegra.soc = of_device_get_match_data(dev);
    dev_set_drvdata(dev, tegra);
    tegra.dev = dev;
    tegra.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(tegra.regs))
    return PTR_ERR(tegra.regs);
    err = platform_get_irq(pdev, 0);
    if (err < 0)
    return err;
    irq = err;
    tegra.wdts = devm_kcalloc(dev, tegra.soc.num_wdts, sizeof(*tegra.wdts), GFP_KERNEL);
    if (!tegra.wdts)
    return -ENOMEM;
    for (i = 0; i < tegra.soc.num_wdts; i++) {
    if (!tegra186_wdt_is_accessible(tegra, i)) {
    dev_warn(dev, "WDT%u is not accessible\n", i);
    continue;
    }
    tegra.wdts[i] = tegra186_wdt_create(tegra, i);
    if (IS_ERR(tegra.wdts[i]))
    return dev_err_probe(dev, PTR_ERR(tegra.wdts[i]),
    "failed to create WDT%u\n", i);
// Reserve the first accessible WDT for the Kernel.
    if (!kernel_wdt) {
    kernel_wdt = tegra.wdts[i];
    kernel_wdt.is_kernel_wdt = true;
    } else {
    err = devm_watchdog_register_device(dev, &tegra.wdts[i].base);
    if (err < 0)
    return dev_err_probe(dev, err,
    "failed to register WDT%u\n", i);
    }
    }
    err = tegra186_timer_tsc_init(tegra);
    if (err < 0) {
    dev_err(dev, "failed to register TSC counter: %d\n", err);
    return err;
    }
    err = tegra186_timer_osc_init(tegra);
    if (err < 0) {
    dev_err(dev, "failed to register OSC counter: %d\n", err);
    goto unregister_tsc;
    }
    err = tegra186_timer_usec_init(tegra);
    if (err < 0) {
    dev_err(dev, "failed to register USEC counter: %d\n", err);
    goto unregister_osc;
    }
    if (kernel_wdt) {
    err = devm_request_irq(dev, irq, tegra186_wdt_irq, 0,
    dev_name(dev), kernel_wdt);
    if (err < 0)
    goto unregister_usec;
    tegra186_wdt_set_timeout(&kernel_wdt.base, TEGRA186_KERNEL_WDT_TIMEOUT);
    tegra186_wdt_enable(kernel_wdt);
    }
    return 0;
    unregister_usec:
    clocksource_unregister(&tegra.usec);
    unregister_osc:
    clocksource_unregister(&tegra.osc);
    unregister_tsc:
    clocksource_unregister(&tegra.tsc);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn tegra186_timer_remove(pdev: *mut platform_device) {
    static void tegra186_timer_remove(struct platform_device *pdev)
    {
    struct tegra186_timer *tegra = platform_get_drvdata(pdev);
    clocksource_unregister(&tegra.usec);
    clocksource_unregister(&tegra.osc);
    clocksource_unregister(&tegra.tsc);
    }
#[no_mangle]
unsafe extern "C" fn tegra186_timer_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused tegra186_timer_suspend(struct device *dev)
    {
    struct tegra186_timer *tegra = dev_get_drvdata(dev);
    unsigned int i;
    for (i = 0; i < tegra.soc.num_wdts; i++) {
    struct tegra186_wdt *wdt = tegra.wdts[i];
    if (wdt && (wdt.is_kernel_wdt || watchdog_active(&wdt.base)))
    tegra186_wdt_disable(wdt);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra186_timer_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused tegra186_timer_resume(struct device *dev)
    {
    struct tegra186_timer *tegra = dev_get_drvdata(dev);
    unsigned int i;
    for (i = 0; i < tegra.soc.num_wdts; i++) {
    struct tegra186_wdt *wdt = tegra.wdts[i];
    if (wdt && (wdt.is_kernel_wdt || watchdog_active(&wdt.base)))
    tegra186_wdt_enable(wdt);
    }
    return 0;
    }
    static SIMPLE_DEV_PM_OPS(tegra186_timer_pm_ops, tegra186_timer_suspend,
    tegra186_timer_resume);
    static const struct tegra186_timer_soc tegra186_timer = {
    .num_timers = 10,
    .num_wdts = 2,
    };
    static const struct tegra186_timer_soc tegra234_timer = {
    .num_timers = 16,
    .num_wdts = 2,
    };
    static const struct of_device_id tegra186_timer_of_match[] = {
    { .compatible = "nvidia,tegra186-timer", .data = &tegra186_timer },
    { .compatible = "nvidia,tegra234-timer", .data = &tegra234_timer },
    { }
    };
    MODULE_DEVICE_TABLE(of, tegra186_timer_of_match);
    static struct platform_driver tegra186_wdt_driver = {
    .driver = {
    .name = "tegra186-timer",
    .pm = &tegra186_timer_pm_ops,
    .of_match_table = tegra186_timer_of_match,
    },
    .probe = tegra186_timer_probe,
    .remove = tegra186_timer_remove,
    };
    module_platform_driver(tegra186_wdt_driver);
    MODULE_AUTHOR("Thierry Reding <treding@nvidia.com>");
    MODULE_DESCRIPTION("NVIDIA Tegra186 timers driver");
