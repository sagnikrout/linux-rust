//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/cadence_wdt.c
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
// Cadence WDT driver - Used by Xilinx Zynq
//
// Copyright (C) 2010 - 2014 Xilinx, Inc.
//

pub const CDNS_WDT_DEFAULT_TIMEOUT: c_int = 10;
// Supports 1 - 516 sec
pub const CDNS_WDT_MIN_TIMEOUT: c_int = 1;
pub const CDNS_WDT_MAX_TIMEOUT: c_int = 516;
// Restart key
pub const CDNS_WDT_RESTART_KEY: c_uint = 0x00001999;
// Counter register access key
pub const CDNS_WDT_REGISTER_ACCESS_KEY: c_uint = 0x00920000;
// Counter value divisor
pub const CDNS_WDT_COUNTER_VALUE_DIVISOR: c_uint = 0x1000;
// Clock prescaler value and selection
pub const CDNS_WDT_PRESCALE_64: c_int = 64;
pub const CDNS_WDT_PRESCALE_512: c_int = 512;
pub const CDNS_WDT_PRESCALE_4096: c_int = 4096;
pub const CDNS_WDT_PRESCALE_SELECT_64: c_int = 1;
pub const CDNS_WDT_PRESCALE_SELECT_512: c_int = 2;
pub const CDNS_WDT_PRESCALE_SELECT_4096: c_int = 3;
// Input clock frequency
pub const CDNS_WDT_CLK_10MHZ: c_int = 10000000;
pub const CDNS_WDT_CLK_75MHZ: c_int = 75000000;
// Counter maximum value
pub const CDNS_WDT_COUNTER_MAX: c_uint = 0xFFF;
    static int wdt_timeout;
    let mut nowayout: static int = WATCHDOG_NOWAYOUT;
    module_param(wdt_timeout, int, 0644);
    MODULE_PARM_DESC(wdt_timeout,
    "Watchdog time in seconds. (default="
    __MODULE_STRING(CDNS_WDT_DEFAULT_TIMEOUT) ")");
    module_param(nowayout, int, 0644);
    MODULE_PARM_DESC(nowayout,
    "Watchdog cannot be stopped once started (default="
    __MODULE_STRING(WATCHDOG_NOWAYOUT) ")");
//
// struct cdns_wdt - Watchdog device structure
// @regs: baseaddress of device
// @rst: reset flag
// @clk: struct clk * of a clock source
// @prescaler: for saving prescaler value
// @ctrl_clksel: counter clock prescaler selection
// @io_lock: spinlock for IO register access
// @cdns_wdt_device: watchdog device structure
//
// Structure containing parameters specific to cadence watchdog.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns_wdt {
    pub regs: *mut void __iomem,
    pub rst: bool,
    pub clk: *mut clk,
    pub prescaler: u32,
    pub ctrl_clksel: u32,
    pub io_lock: spinlock_t,
    pub cdns_wdt_device: watchdog_device,
}

// Write access to Registers
#[no_mangle]
pub unsafe extern "C" fn cdns_wdt_writereg(wdt: *mut cdns_wdt, offset: u32, val: u32) {
    static inline void cdns_wdt_writereg(struct cdns_wdt *wdt, u32 offset, u32 val)
    {
    writel_relaxed(val, wdt.regs + offset);
    }
// Register Map
// Register Offsets for the WDT
pub const CDNS_WDT_ZMR_OFFSET: c_uint = 0x0	/* Zero Mode Register */;
pub const CDNS_WDT_CCR_OFFSET: c_uint = 0x4	/* Counter Control Register */;
pub const CDNS_WDT_RESTART_OFFSET: c_uint = 0x8	/* Restart Register */;
pub const CDNS_WDT_SR_OFFSET: c_uint = 0xC	/* Status Register */;
//
// Zero Mode Register - This register controls how the time out is indicated
// and also contains the access code to allow writes to the register (0xABC).
//
pub const CDNS_WDT_ZMR_WDEN_MASK: c_uint = 0x00000001 /* Enable the WDT */;
pub const CDNS_WDT_ZMR_RSTEN_MASK: c_uint = 0x00000002 /* Enable the reset output */;
pub const CDNS_WDT_ZMR_IRQEN_MASK: c_uint = 0x00000004 /* Enable IRQ output */;
pub const CDNS_WDT_ZMR_RSTLEN_16: c_uint = 0x00000030 /* Reset pulse of 16 pclk cycles */;
pub const CDNS_WDT_ZMR_ZKEY_VAL: c_uint = 0x00ABC000 /* Access key, 0xABC << 12 */;
//
// Counter Control register - This register controls how fast the timer runs
// and the reset value and also contains the access code to allow writes to
// the register.
//
pub const CDNS_WDT_CCR_CRV_MASK: c_uint = 0x00003FFC /* Counter reset value */;
//
// cdns_wdt_stop - Stop the watchdog.
//
// @wdd: watchdog device
//
// Read the contents of the ZMR register, clear the WDEN bit
// in the register and set the access key for successful write.
//
// Return: always 0
//
#[no_mangle]
unsafe extern "C" fn cdns_wdt_stop(wdd: *mut watchdog_device) -> c_int {
    static int cdns_wdt_stop(struct watchdog_device *wdd)
    {
    struct cdns_wdt *wdt = watchdog_get_drvdata(wdd);
    spin_lock(&wdt.io_lock);
    cdns_wdt_writereg(wdt, CDNS_WDT_ZMR_OFFSET,
    CDNS_WDT_ZMR_ZKEY_VAL & (~CDNS_WDT_ZMR_WDEN_MASK));
    spin_unlock(&wdt.io_lock);
    return 0;
    }
//
// cdns_wdt_reload - Reload the watchdog timer (i.e. pat the watchdog).
//
// @wdd: watchdog device
//
// Write the restart key value (0x00001999) to the restart register.
//
// Return: always 0
//
#[no_mangle]
unsafe extern "C" fn cdns_wdt_reload(wdd: *mut watchdog_device) -> c_int {
    static int cdns_wdt_reload(struct watchdog_device *wdd)
    {
    struct cdns_wdt *wdt = watchdog_get_drvdata(wdd);
    spin_lock(&wdt.io_lock);
    cdns_wdt_writereg(wdt, CDNS_WDT_RESTART_OFFSET,
    CDNS_WDT_RESTART_KEY);
    spin_unlock(&wdt.io_lock);
    return 0;
    }
//
// cdns_wdt_start - Enable and start the watchdog.
//
// @wdd: watchdog device
//
// The counter value is calculated according to the formula:
// calculated count = (timeout * clock) / prescaler + 1.
// The calculated count is divided by 0x1000 to obtain the field value
// to write to counter control register.
// Clears the contents of prescaler and counter reset value. Sets the
// prescaler to 4096 and the calculated count and access key
// to write to CCR Register.
// Sets the WDT (WDEN bit) and either the Reset signal(RSTEN bit)
// or Interrupt signal(IRQEN) with a specified cycles and the access
// key to write to ZMR Register.
//
// Return: always 0
//
#[no_mangle]
unsafe extern "C" fn cdns_wdt_start(wdd: *mut watchdog_device) -> c_int {
    static int cdns_wdt_start(struct watchdog_device *wdd)
    {
    struct cdns_wdt *wdt = watchdog_get_drvdata(wdd);
    let mut data: c_uint = 0;
    unsigned short count;
    let mut clock_f: c_ulong = clk_get_rate(wdt.clk);
//
// Counter value divisor to obtain the value of
// counter reset to be written to control register.
//
    count = (wdd.timeout * (clock_f / wdt.prescaler)) /
    CDNS_WDT_COUNTER_VALUE_DIVISOR + 1;
    if (count > CDNS_WDT_COUNTER_MAX)
    count = CDNS_WDT_COUNTER_MAX;
    spin_lock(&wdt.io_lock);
    cdns_wdt_writereg(wdt, CDNS_WDT_ZMR_OFFSET,
    CDNS_WDT_ZMR_ZKEY_VAL);
    count = (count << 2) & CDNS_WDT_CCR_CRV_MASK;
// Write counter access key first to be able write to register
    data = count | CDNS_WDT_REGISTER_ACCESS_KEY | wdt.ctrl_clksel;
    cdns_wdt_writereg(wdt, CDNS_WDT_CCR_OFFSET, data);
    data = CDNS_WDT_ZMR_WDEN_MASK | CDNS_WDT_ZMR_RSTLEN_16 |
    CDNS_WDT_ZMR_ZKEY_VAL;
// Reset on timeout if specified in device tree.
    if (wdt.rst) {
    data |= CDNS_WDT_ZMR_RSTEN_MASK;
    data &= ~CDNS_WDT_ZMR_IRQEN_MASK;
    } else {
    data &= ~CDNS_WDT_ZMR_RSTEN_MASK;
    data |= CDNS_WDT_ZMR_IRQEN_MASK;
    }
    cdns_wdt_writereg(wdt, CDNS_WDT_ZMR_OFFSET, data);
    cdns_wdt_writereg(wdt, CDNS_WDT_RESTART_OFFSET,
    CDNS_WDT_RESTART_KEY);
    spin_unlock(&wdt.io_lock);
    return 0;
    }
//
// cdns_wdt_settimeout - Set a new timeout value for the watchdog device.
//
// @wdd: watchdog device
// @new_time: new timeout value that needs to be set
// Return: 0 on success
//
// Update the watchdog_device timeout with new value which is used when
// cdns_wdt_start is called.
//
    static int cdns_wdt_settimeout(struct watchdog_device *wdd,
    unsigned int new_time)
    {
    wdd.timeout = new_time;
    return cdns_wdt_start(wdd);
    }
//
// cdns_wdt_irq_handler - Notifies of watchdog timeout.
//
// @irq: interrupt number
// @dev_id: pointer to a platform device structure
// Return: IRQ_HANDLED
//
// The handler is invoked when the watchdog times out and a
// reset on timeout has not been enabled.
//
#[no_mangle]
unsafe extern "C" fn cdns_wdt_irq_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t cdns_wdt_irq_handler(int irq, void *dev_id)
    {
    struct platform_device *pdev = dev_id;
    dev_info(&pdev.dev,
    "Watchdog timed out. Internal reset not enabled\n");
    return IRQ_HANDLED;
    }
//
// Info structure used to indicate the features supported by the device
// to the upper layers. This is defined in watchdog.h header file.
//
    static const struct watchdog_info cdns_wdt_info = {
    .identity	= "cdns_wdt watchdog",
    .options	= WDIOF_SETTIMEOUT | WDIOF_KEEPALIVEPING |
    WDIOF_MAGICCLOSE,
    };
// Watchdog Core Ops
    static const struct watchdog_ops cdns_wdt_ops = {
    .owner = THIS_MODULE,
    .start = cdns_wdt_start,
    .stop = cdns_wdt_stop,
    .ping = cdns_wdt_reload,
    .set_timeout = cdns_wdt_settimeout,
    };
// Platform Operations
//
// cdns_wdt_probe - Probe call for the device.
//
// @pdev: handle to the platform device structure.
// Return: 0 on success, negative error otherwise.
//
// It does all the memory allocation and registration for the device.
//
#[no_mangle]
unsafe extern "C" fn cdns_wdt_probe(pdev: *mut platform_device) -> c_int {
    static int cdns_wdt_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    int ret, irq;
    unsigned long clock_f;
    struct cdns_wdt *wdt;
    struct watchdog_device *cdns_wdt_device;
    wdt = devm_kzalloc(dev, sizeof(*wdt), GFP_KERNEL);
    if (!wdt)
    return -ENOMEM;
    cdns_wdt_device = &wdt.cdns_wdt_device;
    cdns_wdt_device.info = &cdns_wdt_info;
    cdns_wdt_device.ops = &cdns_wdt_ops;
    cdns_wdt_device.timeout = CDNS_WDT_DEFAULT_TIMEOUT;
    cdns_wdt_device.min_timeout = CDNS_WDT_MIN_TIMEOUT;
    cdns_wdt_device.max_timeout = CDNS_WDT_MAX_TIMEOUT;
    wdt.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(wdt.regs))
    return PTR_ERR(wdt.regs);
// Register the interrupt
    wdt.rst = of_property_read_bool(dev.of_node, "reset-on-timeout");
    irq = platform_get_irq(pdev, 0);
    if (!wdt.rst && irq >= 0) {
    ret = devm_request_irq(dev, irq, cdns_wdt_irq_handler, 0,
    pdev.name, pdev);
    if (ret)
    return ret;
    }
// Initialize the members of cdns_wdt structure
    cdns_wdt_device.parent = dev;
    watchdog_init_timeout(cdns_wdt_device, wdt_timeout, dev);
    watchdog_set_nowayout(cdns_wdt_device, nowayout);
    watchdog_stop_on_reboot(cdns_wdt_device);
    watchdog_set_drvdata(cdns_wdt_device, wdt);
    wdt.clk = devm_clk_get_enabled(dev, core::ptr::null_mut());
    if (IS_ERR(wdt.clk))
    return dev_err_probe(dev, PTR_ERR(wdt.clk),
    "input clock not found\n");
    clock_f = clk_get_rate(wdt.clk);
    if (clock_f <= CDNS_WDT_CLK_75MHZ) {
    wdt.prescaler = CDNS_WDT_PRESCALE_512;
    wdt.ctrl_clksel = CDNS_WDT_PRESCALE_SELECT_512;
    } else {
    wdt.prescaler = CDNS_WDT_PRESCALE_4096;
    wdt.ctrl_clksel = CDNS_WDT_PRESCALE_SELECT_4096;
    }
    spin_lock_init(&wdt.io_lock);
    watchdog_stop_on_reboot(cdns_wdt_device);
    watchdog_stop_on_unregister(cdns_wdt_device);
    ret = devm_watchdog_register_device(dev, cdns_wdt_device);
    if (ret)
    return ret;
    platform_set_drvdata(pdev, wdt);
    dev_info(dev, "Xilinx Watchdog Timer with timeout %ds%s\n",
    cdns_wdt_device.timeout, nowayout ? ", nowayout" : "");
    return 0;
    }
//
// cdns_wdt_suspend - Stop the device.
//
// @dev: handle to the device structure.
// Return: 0 always.
//
#[no_mangle]
unsafe extern "C" fn cdns_wdt_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused cdns_wdt_suspend(struct device *dev)
    {
    struct cdns_wdt *wdt = dev_get_drvdata(dev);
    if (watchdog_active(&wdt.cdns_wdt_device)) {
    cdns_wdt_stop(&wdt.cdns_wdt_device);
    clk_disable_unprepare(wdt.clk);
    }
    return 0;
    }
//
// cdns_wdt_resume - Resume the device.
//
// @dev: handle to the device structure.
// Return: 0 on success, errno otherwise.
//
#[no_mangle]
unsafe extern "C" fn cdns_wdt_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused cdns_wdt_resume(struct device *dev)
    {
    int ret;
    struct cdns_wdt *wdt = dev_get_drvdata(dev);
    if (watchdog_active(&wdt.cdns_wdt_device)) {
    ret = clk_prepare_enable(wdt.clk);
    if (ret) {
    dev_err(dev, "unable to enable clock\n");
    return ret;
    }
    cdns_wdt_start(&wdt.cdns_wdt_device);
    }
    return 0;
    }
    static SIMPLE_DEV_PM_OPS(cdns_wdt_pm_ops, cdns_wdt_suspend, cdns_wdt_resume);
    static const struct of_device_id cdns_wdt_of_match[] = {
    { .compatible = "cdns,wdt-r1p2", },
    { /* end of table */ }
    };
    MODULE_DEVICE_TABLE(of, cdns_wdt_of_match);
// Driver Structure
    static struct platform_driver cdns_wdt_driver = {
    .probe		= cdns_wdt_probe,
    .driver		= {
    .name	= "cdns-wdt",
    .of_match_table = cdns_wdt_of_match,
    .pm	= &cdns_wdt_pm_ops,
    },
    };
    module_platform_driver(cdns_wdt_driver);
    MODULE_AUTHOR("Xilinx, Inc.");
    MODULE_DESCRIPTION("Watchdog driver for Cadence WDT");
    MODULE_LICENSE("GPL");
