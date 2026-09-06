//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/ie6xx_wdt.c
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
// Intel Atom E6xx Watchdog driver
//
// Copyright (C) 2011 Alexander Stein
// <alexander.stein@systec-electronic.com>
//

pub const PV1: c_uint = 0x00;
pub const PV2: c_uint = 0x04;
pub const RR0: c_uint = 0x0c;
pub const RR1: c_uint = 0x0d;
pub const WDT_RELOAD: c_uint = 0x01;
pub const WDT_TOUT: c_uint = 0x02;
pub const WDTCR: c_uint = 0x10;
pub const WDT_PRE_SEL: c_uint = 0x04;
pub const WDT_RESET_SEL: c_uint = 0x08;
pub const WDT_RESET_EN: c_uint = 0x10;
pub const WDT_TOUT_EN: c_uint = 0x20;
pub const DCR: c_uint = 0x14;
pub const WDTLR: c_uint = 0x18;
pub const WDT_LOCK: c_uint = 0x01;
pub const WDT_ENABLE: c_uint = 0x02;
pub const WDT_TOUT_CNF: c_uint = 0x03;
pub const MIN_TIME: c_int = 1;

pub const DEFAULT_TIME: c_int = 60;
    let mut timeout: static unsigned int = DEFAULT_TIME;
    module_param(timeout, uint, 0);
    MODULE_PARM_DESC(timeout,
    "Default Watchdog timer setting ("
    __MODULE_STRING(DEFAULT_TIME) "s)."
    "The range is from 1 to 600");
    let mut nowayout: static bool = WATCHDOG_NOWAYOUT;
    module_param(nowayout, bool, 0);
    MODULE_PARM_DESC(nowayout,
    "Watchdog cannot be stopped once started (default="
    __MODULE_STRING(WATCHDOG_NOWAYOUT) ")");
    let mut resetmode: static u8 = 0x10;
    module_param(resetmode, byte, 0);
    MODULE_PARM_DESC(resetmode,
    "Resetmode bits: 0x08 warm reset (cold reset otherwise), "
    "0x10 reset enable, 0x20 disable toggle GPIO[4] (default=0x10)");
    static struct {
    unsigned short sch_wdtba;
    spinlock_t unlock_sequence;

    struct dentry *debugfs;

    } ie6xx_wdt_data;
//
// This is needed to write to preload and reload registers
// struct ie6xx_wdt_data.unlock_sequence must be used
// to prevent sequence interrupts
//
#[no_mangle]
unsafe extern "C" fn ie6xx_wdt_unlock_registers() {
    static void ie6xx_wdt_unlock_registers(void)
    {
    outb(0x80, ie6xx_wdt_data.sch_wdtba + RR0);
    outb(0x86, ie6xx_wdt_data.sch_wdtba + RR0);
    }
#[no_mangle]
unsafe extern "C" fn ie6xx_wdt_ping(wdd: *mut watchdog_device) -> c_int {
    static int ie6xx_wdt_ping(struct watchdog_device *wdd)
    {
    spin_lock(&ie6xx_wdt_data.unlock_sequence);
    ie6xx_wdt_unlock_registers();
    outb(WDT_RELOAD, ie6xx_wdt_data.sch_wdtba + RR1);
    spin_unlock(&ie6xx_wdt_data.unlock_sequence);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ie6xx_wdt_set_timeout(wdd: *mut watchdog_device, t: c_uint) -> c_int {
    static int ie6xx_wdt_set_timeout(struct watchdog_device *wdd, unsigned int t)
    {
    u32 preload;
    u64 clock;
    u8 wdtcr;
// Watchdog clock is PCI Clock (33MHz)
    clock = 33000000;
// and the preload value is loaded into [34:15] of the down counter
    preload = (t * clock) >> 15;
//
// Manual states preload must be one less.
// Does not wrap as t is at least 1
//
    preload -= 1;
    spin_lock(&ie6xx_wdt_data.unlock_sequence);
// Set ResetMode & Enable prescaler for range 10ms to 10 min
    wdtcr = resetmode & 0x38;
    outb(wdtcr, ie6xx_wdt_data.sch_wdtba + WDTCR);
    ie6xx_wdt_unlock_registers();
    outl(0, ie6xx_wdt_data.sch_wdtba + PV1);
    ie6xx_wdt_unlock_registers();
    outl(preload, ie6xx_wdt_data.sch_wdtba + PV2);
    ie6xx_wdt_unlock_registers();
    outb(WDT_RELOAD | WDT_TOUT, ie6xx_wdt_data.sch_wdtba + RR1);
    spin_unlock(&ie6xx_wdt_data.unlock_sequence);
    wdd.timeout = t;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ie6xx_wdt_start(wdd: *mut watchdog_device) -> c_int {
    static int ie6xx_wdt_start(struct watchdog_device *wdd)
    {
    ie6xx_wdt_set_timeout(wdd, wdd.timeout);
// Enable the watchdog timer
    spin_lock(&ie6xx_wdt_data.unlock_sequence);
    outb(WDT_ENABLE, ie6xx_wdt_data.sch_wdtba + WDTLR);
    spin_unlock(&ie6xx_wdt_data.unlock_sequence);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ie6xx_wdt_stop(wdd: *mut watchdog_device) -> c_int {
    static int ie6xx_wdt_stop(struct watchdog_device *wdd)
    {
    if (inb(ie6xx_wdt_data.sch_wdtba + WDTLR) & WDT_LOCK)
    return -1;
// Disable the watchdog timer
    spin_lock(&ie6xx_wdt_data.unlock_sequence);
    outb(0, ie6xx_wdt_data.sch_wdtba + WDTLR);
    spin_unlock(&ie6xx_wdt_data.unlock_sequence);
    return 0;
    }
    static const struct watchdog_info ie6xx_wdt_info = {
    .identity =	"Intel Atom E6xx Watchdog",
    .options =	WDIOF_SETTIMEOUT |
    WDIOF_MAGICCLOSE |
    WDIOF_KEEPALIVEPING,
    };
    static const struct watchdog_ops ie6xx_wdt_ops = {
    .owner =	THIS_MODULE,
    .start =	ie6xx_wdt_start,
    .stop =		ie6xx_wdt_stop,
    .ping =		ie6xx_wdt_ping,
    .set_timeout =	ie6xx_wdt_set_timeout,
    };
    static struct watchdog_device ie6xx_wdt_dev = {
    .info =		&ie6xx_wdt_info,
    .ops =		&ie6xx_wdt_ops,
    .min_timeout =	MIN_TIME,
    .max_timeout =	MAX_TIME,
    };

#[no_mangle]
unsafe extern "C" fn ie6xx_wdt_show(s: *mut seq_file, unused: *mut c_void) -> c_int {
    static int ie6xx_wdt_show(struct seq_file *s, void *unused)
    {
    seq_printf(s, "PV1   = 0x%08x\n",
    inl(ie6xx_wdt_data.sch_wdtba + PV1));
    seq_printf(s, "PV2   = 0x%08x\n",
    inl(ie6xx_wdt_data.sch_wdtba + PV2));
    seq_printf(s, "RR    = 0x%08x\n",
    inw(ie6xx_wdt_data.sch_wdtba + RR0));
    seq_printf(s, "WDTCR = 0x%08x\n",
    inw(ie6xx_wdt_data.sch_wdtba + WDTCR));
    seq_printf(s, "DCR   = 0x%08x\n",
    inl(ie6xx_wdt_data.sch_wdtba + DCR));
    seq_printf(s, "WDTLR = 0x%08x\n",
    inw(ie6xx_wdt_data.sch_wdtba + WDTLR));
    seq_printf(s, "\n");
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(ie6xx_wdt);
#[no_mangle]
unsafe extern "C" fn ie6xx_wdt_debugfs_init() {
    static void ie6xx_wdt_debugfs_init(void)
    {
// /sys/kernel/debug/ie6xx_wdt
    ie6xx_wdt_data.debugfs = debugfs_create_file("ie6xx_wdt",
    S_IFREG | S_IRUGO, core::ptr::null_mut(), core::ptr::null_mut(), &ie6xx_wdt_fops);
    }
#[no_mangle]
unsafe extern "C" fn ie6xx_wdt_debugfs_exit() {
    static void ie6xx_wdt_debugfs_exit(void)
    {
    debugfs_remove(ie6xx_wdt_data.debugfs);
    }

#[no_mangle]
unsafe extern "C" fn ie6xx_wdt_debugfs_init() {
    static void ie6xx_wdt_debugfs_init(void)
    {
    }
#[no_mangle]
unsafe extern "C" fn ie6xx_wdt_debugfs_exit() {
    static void ie6xx_wdt_debugfs_exit(void)
    {
    }

#[no_mangle]
unsafe extern "C" fn ie6xx_wdt_probe(pdev: *mut platform_device) -> c_int {
    static int ie6xx_wdt_probe(struct platform_device *pdev)
    {
    struct resource *res;
    u8 wdtlr;
    int ret;
    res = platform_get_resource(pdev, IORESOURCE_IO, 0);
    if (!res)
    return -ENODEV;
    if (!request_region(res.start, resource_size(res), pdev.name)) {
    dev_err(&pdev.dev, "Watchdog region 0x%llx already in use!\n",
    (u64)res.start);
    return -EBUSY;
    }
    ie6xx_wdt_data.sch_wdtba = res.start;
    dev_dbg(&pdev.dev, "WDT = 0x%X\n", ie6xx_wdt_data.sch_wdtba);
    ie6xx_wdt_dev.timeout = timeout;
    watchdog_set_nowayout(&ie6xx_wdt_dev, nowayout);
    ie6xx_wdt_dev.parent = &pdev.dev;
    spin_lock_init(&ie6xx_wdt_data.unlock_sequence);
    wdtlr = inb(ie6xx_wdt_data.sch_wdtba + WDTLR);
    if (wdtlr & WDT_LOCK)
    dev_warn(&pdev.dev,
    "Watchdog Timer is Locked (Reg=0x%x)\n", wdtlr);
    ie6xx_wdt_debugfs_init();
    ret = watchdog_register_device(&ie6xx_wdt_dev);
    if (ret)
    goto misc_register_error;
    return 0;
    misc_register_error:
    ie6xx_wdt_debugfs_exit();
    release_region(res.start, resource_size(res));
    ie6xx_wdt_data.sch_wdtba = 0;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ie6xx_wdt_remove(pdev: *mut platform_device) {
    static void ie6xx_wdt_remove(struct platform_device *pdev)
    {
    struct resource *res;
    res = platform_get_resource(pdev, IORESOURCE_IO, 0);
    ie6xx_wdt_stop(core::ptr::null_mut());
    watchdog_unregister_device(&ie6xx_wdt_dev);
    ie6xx_wdt_debugfs_exit();
    release_region(res.start, resource_size(res));
    ie6xx_wdt_data.sch_wdtba = 0;
    }
    static struct platform_driver ie6xx_wdt_driver = {
    .probe		= ie6xx_wdt_probe,
    .remove		= ie6xx_wdt_remove,
    .driver		= {
    .name	= DRIVER_NAME,
    },
    };
#[no_mangle]
unsafe extern "C" fn ie6xx_wdt_init() -> int __init {
    static int __init ie6xx_wdt_init(void)
    {
// Check boot parameters to verify that their initial values
// are in range.
    if ((timeout < MIN_TIME) ||
    (timeout > MAX_TIME)) {
    pr_err("Watchdog timer: value of timeout %d (dec) "
    "is out of range from %d to %d (dec)\n",
    timeout, MIN_TIME, MAX_TIME);
    return -EINVAL;
    }
    return platform_driver_register(&ie6xx_wdt_driver);
    }
#[no_mangle]
unsafe extern "C" fn ie6xx_wdt_exit() -> void __exit {
    static void __exit ie6xx_wdt_exit(void)
    {
    platform_driver_unregister(&ie6xx_wdt_driver);
    }
    late_initcall(ie6xx_wdt_init);
    module_exit(ie6xx_wdt_exit);
    MODULE_AUTHOR("Alexander Stein <alexander.stein@systec-electronic.com>");
    MODULE_DESCRIPTION("Intel Atom E6xx Watchdog Device Driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:" DRIVER_NAME);
