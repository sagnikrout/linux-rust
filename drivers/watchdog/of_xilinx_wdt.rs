//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/of_xilinx_wdt.c
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
// Watchdog Device Driver for Xilinx axi/xps_timebase_wdt
//
// (C) Copyright 2013 - 2014 Xilinx, Inc.
// (C) Copyright 2011 (Alejandro Cabrera <aldaya@gmail.com>)
//

// Register offsets for the Wdt device
pub const XWT_TWCSR0_OFFSET: c_uint = 0x0 /* Control/Status Register0 */;
pub const XWT_TWCSR1_OFFSET: c_uint = 0x4 /* Control/Status Register1 */;
pub const XWT_TBR_OFFSET: c_uint = 0x8 /* Timebase Register Offset */;
// Control/Status Register Masks

// Control/Status Register 0/1 bits

// SelfTest constants
pub const XWT_MAX_SELFTEST_LOOP_COUNT: c_uint = 0x00010000;
pub const XWT_TIMER_FAILED: c_uint = 0xFFFFFFFF;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xwdt_device {
    pub base: *mut void __iomem,
    pub wdt_interval: u32,
    pub /: *mut *mut spinlock_t spinlock; / spinlock for register handling,
    pub xilinx_wdt_wdd: watchdog_device,
    pub clk: *mut clk,
}

#[no_mangle]
unsafe extern "C" fn xilinx_wdt_start(wdd: *mut watchdog_device) -> c_int {
    static int xilinx_wdt_start(struct watchdog_device *wdd)
    {
    int ret;
    u32 control_status_reg;
    struct xwdt_device *xdev = watchdog_get_drvdata(wdd);
    ret = clk_enable(xdev.clk);
    if (ret) {
    dev_err(wdd.parent, "Failed to enable clock\n");
    return ret;
    }
    spin_lock(&xdev.spinlock);
// Clean previous status and enable the watchdog timer
    control_status_reg = ioread32(xdev.base + XWT_TWCSR0_OFFSET);
    control_status_reg |= (XWT_CSR0_WRS_MASK | XWT_CSR0_WDS_MASK);
    iowrite32((control_status_reg | XWT_CSR0_EWDT1_MASK),
    xdev.base + XWT_TWCSR0_OFFSET);
    iowrite32(XWT_CSRX_EWDT2_MASK, xdev.base + XWT_TWCSR1_OFFSET);
    spin_unlock(&xdev.spinlock);
    dev_dbg(wdd.parent, "Watchdog Started!\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xilinx_wdt_stop(wdd: *mut watchdog_device) -> c_int {
    static int xilinx_wdt_stop(struct watchdog_device *wdd)
    {
    u32 control_status_reg;
    struct xwdt_device *xdev = watchdog_get_drvdata(wdd);
    spin_lock(&xdev.spinlock);
    control_status_reg = ioread32(xdev.base + XWT_TWCSR0_OFFSET);
    iowrite32((control_status_reg & ~XWT_CSR0_EWDT1_MASK),
    xdev.base + XWT_TWCSR0_OFFSET);
    iowrite32(0, xdev.base + XWT_TWCSR1_OFFSET);
    spin_unlock(&xdev.spinlock);
    clk_disable(xdev.clk);
    dev_dbg(wdd.parent, "Watchdog Stopped!\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xilinx_wdt_keepalive(wdd: *mut watchdog_device) -> c_int {
    static int xilinx_wdt_keepalive(struct watchdog_device *wdd)
    {
    u32 control_status_reg;
    struct xwdt_device *xdev = watchdog_get_drvdata(wdd);
    spin_lock(&xdev.spinlock);
    control_status_reg = ioread32(xdev.base + XWT_TWCSR0_OFFSET);
    control_status_reg |= (XWT_CSR0_WRS_MASK | XWT_CSR0_WDS_MASK);
    iowrite32(control_status_reg, xdev.base + XWT_TWCSR0_OFFSET);
    spin_unlock(&xdev.spinlock);
    return 0;
    }
    static const struct watchdog_info xilinx_wdt_ident = {
    .options =  WDIOF_MAGICCLOSE |
    WDIOF_KEEPALIVEPING,
    .firmware_version =	1,
    .identity =	WATCHDOG_NAME,
    };
    static const struct watchdog_ops xilinx_wdt_ops = {
    .owner = THIS_MODULE,
    .start = xilinx_wdt_start,
    .stop = xilinx_wdt_stop,
    .ping = xilinx_wdt_keepalive,
    };
#[no_mangle]
unsafe extern "C" fn xwdt_selftest(xdev: *mut xwdt_device) -> u32 {
    static u32 xwdt_selftest(struct xwdt_device *xdev)
    {
    int i;
    u32 timer_value1;
    u32 timer_value2;
    spin_lock(&xdev.spinlock);
    timer_value1 = ioread32(xdev.base + XWT_TBR_OFFSET);
    timer_value2 = ioread32(xdev.base + XWT_TBR_OFFSET);
    for (i = 0;
    ((i <= XWT_MAX_SELFTEST_LOOP_COUNT) &&
    (timer_value2 == timer_value1)); i++) {
    timer_value2 = ioread32(xdev.base + XWT_TBR_OFFSET);
    }
    spin_unlock(&xdev.spinlock);
    if (timer_value2 != timer_value1)
    return ~XWT_TIMER_FAILED;
    else
    return XWT_TIMER_FAILED;
    }
#[no_mangle]
unsafe extern "C" fn xwdt_probe(pdev: *mut platform_device) -> c_int {
    static int xwdt_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    int rc;
    let mut pfreq: u32 = 0, enable_once = 0;
    struct xwdt_device *xdev;
    struct watchdog_device *xilinx_wdt_wdd;
    xdev = devm_kzalloc(dev, sizeof(*xdev), GFP_KERNEL);
    if (!xdev)
    return -ENOMEM;
    xilinx_wdt_wdd = &xdev.xilinx_wdt_wdd;
    xilinx_wdt_wdd.info = &xilinx_wdt_ident;
    xilinx_wdt_wdd.ops = &xilinx_wdt_ops;
    xilinx_wdt_wdd.parent = dev;
    xdev.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(xdev.base))
    return PTR_ERR(xdev.base);
    rc = of_property_read_u32(dev.of_node, "xlnx,wdt-interval",
    &xdev.wdt_interval);
    if (rc)
    dev_warn(dev, "Parameter \"xlnx,wdt-interval\" not found\n");
    rc = of_property_read_u32(dev.of_node, "xlnx,wdt-enable-once",
    &enable_once);
    if (rc)
    dev_warn(dev,
    "Parameter \"xlnx,wdt-enable-once\" not found\n");
    watchdog_set_nowayout(xilinx_wdt_wdd, enable_once);
    xdev.clk = devm_clk_get_prepared(dev, core::ptr::null_mut());
    if (IS_ERR(xdev.clk)) {
    if (PTR_ERR(xdev.clk) != -ENOENT)
    return PTR_ERR(xdev.clk);
//
// Clock framework support is optional, continue on
// anyways if we don't find a matching clock.
//
    xdev.clk = core::ptr::null_mut();
    rc = of_property_read_u32(dev.of_node, "clock-frequency",
    &pfreq);
    if (rc)
    dev_warn(dev,
    "The watchdog clock freq cannot be obtained\n");
    } else {
    pfreq = clk_get_rate(xdev.clk);
    }
//
// Twice of the 2^wdt_interval / freq  because the first wdt overflow is
// ignored (interrupt), reset is only generated at second wdt overflow
//
    if (pfreq && xdev.wdt_interval)
    xilinx_wdt_wdd.timeout = 2 * ((1 << xdev.wdt_interval) /
    pfreq);
    spin_lock_init(&xdev.spinlock);
    watchdog_set_drvdata(xilinx_wdt_wdd, xdev);
    rc = clk_enable(xdev.clk);
    if (rc) {
    dev_err(dev, "unable to enable clock\n");
    return rc;
    }
    rc = xwdt_selftest(xdev);
    if (rc == XWT_TIMER_FAILED) {
    dev_err(dev, "SelfTest routine error\n");
    clk_disable(xdev.clk);
    return rc;
    }
    clk_disable(xdev.clk);
    rc = devm_watchdog_register_device(dev, xilinx_wdt_wdd);
    if (rc)
    return rc;
    dev_info(dev, "Xilinx Watchdog Timer with timeout %ds\n",
    xilinx_wdt_wdd.timeout);
    platform_set_drvdata(pdev, xdev);
    return 0;
    }
//
// xwdt_suspend - Suspend the device.
//
// @dev: handle to the device structure.
// Return: 0 always.
//
#[no_mangle]
unsafe extern "C" fn xwdt_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused xwdt_suspend(struct device *dev)
    {
    struct xwdt_device *xdev = dev_get_drvdata(dev);
    if (watchdog_active(&xdev.xilinx_wdt_wdd))
    xilinx_wdt_stop(&xdev.xilinx_wdt_wdd);
    return 0;
    }
//
// xwdt_resume - Resume the device.
//
// @dev: handle to the device structure.
// Return: 0 on success, errno otherwise.
//
#[no_mangle]
unsafe extern "C" fn xwdt_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused xwdt_resume(struct device *dev)
    {
    struct xwdt_device *xdev = dev_get_drvdata(dev);
    let mut ret: c_int = 0;
    if (watchdog_active(&xdev.xilinx_wdt_wdd))
    ret = xilinx_wdt_start(&xdev.xilinx_wdt_wdd);
    return ret;
    }
    static SIMPLE_DEV_PM_OPS(xwdt_pm_ops, xwdt_suspend, xwdt_resume);
// Match table for of_platform binding
    static const struct of_device_id xwdt_of_match[] = {
    { .compatible = "xlnx,xps-timebase-wdt-1.00.a", },
    { .compatible = "xlnx,xps-timebase-wdt-1.01.a", },
    {},
    };
    MODULE_DEVICE_TABLE(of, xwdt_of_match);
    static struct platform_driver xwdt_driver = {
    .probe       = xwdt_probe,
    .driver = {
    .name  = WATCHDOG_NAME,
    .of_match_table = xwdt_of_match,
    .pm = &xwdt_pm_ops,
    },
    };
    module_platform_driver(xwdt_driver);
    MODULE_AUTHOR("Alejandro Cabrera <aldaya@gmail.com>");
    MODULE_DESCRIPTION("Xilinx Watchdog driver");
    MODULE_LICENSE("GPL");
