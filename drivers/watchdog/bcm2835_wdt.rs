//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/bcm2835_wdt.c
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
// Watchdog driver for Broadcom BCM2835
//
// "bcm2708_wdog" driver written by Luke Diamand that was obtained from
// branch "rpi-3.6.y" of git://github.com/raspberrypi/linux.git was used
// as a hardware reference for the Broadcom BCM2835 watchdog timer.
//
// Copyright (C) 2013 Lubomir Rintel <lkundrak@v3.sk>
//

pub const PM_RSTC: c_uint = 0x1c;
pub const PM_RSTS: c_uint = 0x20;
pub const PM_WDOG: c_uint = 0x24;
pub const PM_PASSWORD: c_uint = 0x5a000000;
pub const PM_WDOG_TIME_SET: c_uint = 0x000fffff;
pub const PM_RSTC_WRCFG_CLR: c_uint = 0xffffffcf;
pub const PM_RSTS_HADWRH_SET: c_uint = 0x00000040;
pub const PM_RSTC_WRCFG_SET: c_uint = 0x00000030;
pub const PM_RSTC_WRCFG_FULL_RESET: c_uint = 0x00000020;
pub const PM_RSTC_RESET: c_uint = 0x00000102;
//
// The Raspberry Pi firmware uses the RSTS register to know which partition
// to boot from. The partition value is spread into bits 0, 2, 4, 6, 8, 10.
// Partition 63 is a special partition used by the firmware to indicate halt.
//
pub const PM_RSTS_RASPBERRYPI_HALT: c_uint = 0x555;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm2835_wdt {
    pub base: *mut void __iomem,
    pub lock: spinlock_t,
}

    static unsigned int heartbeat;
    let mut nowayout: static bool = WATCHDOG_NOWAYOUT;
#[no_mangle]
unsafe extern "C" fn bcm2835_wdt_is_running(wdt: *mut bcm2835_wdt) -> bool {
    static bool bcm2835_wdt_is_running(struct bcm2835_wdt *wdt)
    {
    uint32_t cur;
    cur = readl(wdt.base + PM_RSTC);
    return !!(cur & PM_RSTC_WRCFG_FULL_RESET);
    }
#[no_mangle]
unsafe extern "C" fn bcm2835_wdt_start(wdog: *mut watchdog_device) -> c_int {
    static int bcm2835_wdt_start(struct watchdog_device *wdog)
    {
    struct bcm2835_wdt *wdt = watchdog_get_drvdata(wdog);
    uint32_t cur;
    unsigned long flags;
    spin_lock_irqsave(&wdt.lock, flags);
    writel_relaxed(PM_PASSWORD | (SECS_TO_WDOG_TICKS(wdog.timeout) &
    PM_WDOG_TIME_SET), wdt.base + PM_WDOG);
    cur = readl_relaxed(wdt.base + PM_RSTC);
    writel_relaxed(PM_PASSWORD | (cur & PM_RSTC_WRCFG_CLR) |
    PM_RSTC_WRCFG_FULL_RESET, wdt.base + PM_RSTC);
    spin_unlock_irqrestore(&wdt.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcm2835_wdt_stop(wdog: *mut watchdog_device) -> c_int {
    static int bcm2835_wdt_stop(struct watchdog_device *wdog)
    {
    struct bcm2835_wdt *wdt = watchdog_get_drvdata(wdog);
    writel_relaxed(PM_PASSWORD | PM_RSTC_RESET, wdt.base + PM_RSTC);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcm2835_wdt_get_timeleft(wdog: *mut watchdog_device) -> c_uint {
    static unsigned int bcm2835_wdt_get_timeleft(struct watchdog_device *wdog)
    {
    struct bcm2835_wdt *wdt = watchdog_get_drvdata(wdog);
    let mut ret: u32 = readl_relaxed(wdt.base + PM_WDOG);
    return WDOG_TICKS_TO_SECS(ret & PM_WDOG_TIME_SET);
    }
#[no_mangle]
unsafe extern "C" fn __bcm2835_restart(wdt: *mut bcm2835_wdt) {
    static void __bcm2835_restart(struct bcm2835_wdt *wdt)
    {
    u32 val;
// use a timeout of 10 ticks (~150us)
    writel_relaxed(10 | PM_PASSWORD, wdt.base + PM_WDOG);
    val = readl_relaxed(wdt.base + PM_RSTC);
    val &= PM_RSTC_WRCFG_CLR;
    val |= PM_PASSWORD | PM_RSTC_WRCFG_FULL_RESET;
    writel_relaxed(val, wdt.base + PM_RSTC);
// No sleeping, possibly atomic.
    mdelay(1);
    }
    static int bcm2835_restart(struct watchdog_device *wdog,
    unsigned long action, void *data)
    {
    struct bcm2835_wdt *wdt = watchdog_get_drvdata(wdog);
    __bcm2835_restart(wdt);
    return 0;
    }
    static const struct watchdog_ops bcm2835_wdt_ops = {
    .owner =	THIS_MODULE,
    .start =	bcm2835_wdt_start,
    .stop =		bcm2835_wdt_stop,
    .get_timeleft =	bcm2835_wdt_get_timeleft,
    .restart =	bcm2835_restart,
    };
    static const struct watchdog_info bcm2835_wdt_info = {
    .options =	WDIOF_SETTIMEOUT | WDIOF_MAGICCLOSE |
    WDIOF_KEEPALIVEPING,
    .identity =	"Broadcom BCM2835 Watchdog timer",
    };
    static struct watchdog_device bcm2835_wdt_wdd = {
    .info =		&bcm2835_wdt_info,
    .ops =		&bcm2835_wdt_ops,
    .min_timeout =	1,
    .max_hw_heartbeat_ms =	WDOG_TICKS_TO_MSECS(PM_WDOG_TIME_SET),
    .timeout =	WDOG_TICKS_TO_SECS(PM_WDOG_TIME_SET),
    };
//
// We can't really power off, but if we do the normal reset scheme, and
// indicate to bootcode.bin not to reboot, then most of the chip will be
// powered off.
//
#[no_mangle]
unsafe extern "C" fn bcm2835_power_off(data: *mut sys_off_data) -> c_int {
    static int bcm2835_power_off(struct sys_off_data *data)
    {
    struct bcm2835_wdt *wdt = data.cb_data;
    u32 val;
//
// We set the watchdog hard reset bit here to distinguish this reset
// from the normal (full) reset. bootcode.bin will not reboot after a
// hard reset.
//
    val = readl_relaxed(wdt.base + PM_RSTS);
    val |= PM_PASSWORD | PM_RSTS_RASPBERRYPI_HALT;
    writel_relaxed(val, wdt.base + PM_RSTS);
// Continue with normal reset mechanism
    __bcm2835_restart(wdt);
    return NOTIFY_DONE;
    }
#[no_mangle]
unsafe extern "C" fn bcm2835_wdt_probe(pdev: *mut platform_device) -> c_int {
    static int bcm2835_wdt_probe(struct platform_device *pdev)
    {
    struct bcm2835_pm *pm = dev_get_drvdata(pdev.dev.parent);
    struct device *dev = &pdev.dev;
    struct bcm2835_wdt *wdt;
    int err;
    wdt = devm_kzalloc(dev, sizeof(struct bcm2835_wdt), GFP_KERNEL);
    if (!wdt)
    return -ENOMEM;
    spin_lock_init(&wdt.lock);
    wdt.base = pm.base;
    watchdog_set_drvdata(&bcm2835_wdt_wdd, wdt);
    watchdog_init_timeout(&bcm2835_wdt_wdd, heartbeat, dev);
    watchdog_set_nowayout(&bcm2835_wdt_wdd, nowayout);
    bcm2835_wdt_wdd.parent = dev;
    if (bcm2835_wdt_is_running(wdt)) {
//
// The currently active timeout value (set by the
// bootloader) may be different from the module
// heartbeat parameter or the value in device
// tree. But we just need to set WDOG_HW_RUNNING,
// because then the framework will "immediately" ping
// the device, updating the timeout.
//
    set_bit(WDOG_HW_RUNNING, &bcm2835_wdt_wdd.status);
    }
    watchdog_set_restart_priority(&bcm2835_wdt_wdd, 128);
    watchdog_stop_on_reboot(&bcm2835_wdt_wdd);
    err = devm_watchdog_register_device(dev, &bcm2835_wdt_wdd);
    if (err)
    return err;
    if (of_device_is_system_power_controller(pdev.dev.parent.of_node))
    devm_register_sys_off_handler(dev, SYS_OFF_MODE_POWER_OFF,
    SYS_OFF_PRIO_DEFAULT,
    bcm2835_power_off, wdt);
    dev_info(dev, "Broadcom BCM2835 watchdog timer");
    return 0;
    }
    static struct platform_driver bcm2835_wdt_driver = {
    .probe		= bcm2835_wdt_probe,
    .driver = {
    .name =		"bcm2835-wdt",
    },
    };
    module_platform_driver(bcm2835_wdt_driver);
    module_param(heartbeat, uint, 0);
    MODULE_PARM_DESC(heartbeat, "Initial watchdog heartbeat in seconds");
    module_param(nowayout, bool, 0);
    MODULE_PARM_DESC(nowayout, "Watchdog cannot be stopped once started (default="
    __MODULE_STRING(WATCHDOG_NOWAYOUT) ")");
    MODULE_ALIAS("platform:bcm2835-wdt");
    MODULE_AUTHOR("Lubomir Rintel <lkundrak@v3.sk>");
    MODULE_DESCRIPTION("Driver for Broadcom BCM2835 watchdog timer");
    MODULE_LICENSE("GPL");
