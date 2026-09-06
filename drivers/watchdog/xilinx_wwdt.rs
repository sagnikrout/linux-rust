//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/xilinx_wwdt.c
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
// Window watchdog device driver for Xilinx Versal WWDT
//
// Copyright (C) 2022 - 2024, Advanced Micro Devices, Inc.
//

// Max timeout is calculated at 100MHz source clock
pub const XWWDT_DEFAULT_TIMEOUT: c_int = 42;
pub const XWWDT_MIN_TIMEOUT: c_int = 1;
// Register offsets for the WWDT device
pub const XWWDT_MWR_OFFSET: c_uint = 0x00;
pub const XWWDT_ESR_OFFSET: c_uint = 0x04;
pub const XWWDT_FCR_OFFSET: c_uint = 0x08;
pub const XWWDT_FWR_OFFSET: c_uint = 0x0c;
pub const XWWDT_SWR_OFFSET: c_uint = 0x10;
// Master Write Control Register Masks

// Enable and Status Register Masks

pub const XWWDT_CLOSE_WINDOW_PERCENT: c_int = 50;
// Maximum count value of each 32 bit window

// Maximum count value of closed and open window combined

    static int wwdt_timeout;
    static int closed_window_percent;
    module_param(wwdt_timeout, int, 0);
    MODULE_PARM_DESC(wwdt_timeout,
    "Watchdog time in seconds. (default="
    __MODULE_STRING(XWWDT_DEFAULT_TIMEOUT) ")");
    module_param(closed_window_percent, int, 0);
    MODULE_PARM_DESC(closed_window_percent,
    "Watchdog closed window percentage. (default="
    __MODULE_STRING(XWWDT_CLOSE_WINDOW_PERCENT) ")");
//
// struct xwwdt_device - Watchdog device structure
// @base: base io address of WDT device
// @spinlock: spinlock for IO register access
// @xilinx_wwdt_wdd: watchdog device structure
// @freq: source clock frequency of WWDT
// @close_percent: Closed window percent
// @closed_timeout: Closed window timeout in ticks
// @open_timeout: Open window timeout in ticks
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xwwdt_device {
    pub base: *mut void __iomem,
    pub /: *mut *mut spinlock_t spinlock; / spinlock for register handling,
    pub xilinx_wwdt_wdd: watchdog_device,
    pub freq: c_ulong,
    pub close_percent: u32,
    pub closed_timeout: u64,
    pub open_timeout: u64,
}

#[no_mangle]
unsafe extern "C" fn xilinx_wwdt_start(wdd: *mut watchdog_device) -> c_int {
    static int xilinx_wwdt_start(struct watchdog_device *wdd)
    {
    struct xwwdt_device *xdev = watchdog_get_drvdata(wdd);
    struct watchdog_device *xilinx_wwdt_wdd = &xdev.xilinx_wwdt_wdd;
    u32 control_status_reg;
    spin_lock(&xdev.spinlock);
    iowrite32(XWWDT_MWR_MASK, xdev.base + XWWDT_MWR_OFFSET);
    iowrite32(~(u32)XWWDT_ESR_WEN_MASK, xdev.base + XWWDT_ESR_OFFSET);
    iowrite32((u32)xdev.closed_timeout, xdev.base + XWWDT_FWR_OFFSET);
    iowrite32((u32)xdev.open_timeout, xdev.base + XWWDT_SWR_OFFSET);
// Enable the window watchdog timer
    control_status_reg = ioread32(xdev.base + XWWDT_ESR_OFFSET);
    control_status_reg |= XWWDT_ESR_WEN_MASK;
    iowrite32(control_status_reg, xdev.base + XWWDT_ESR_OFFSET);
    spin_unlock(&xdev.spinlock);
    dev_dbg(xilinx_wwdt_wdd.parent, "Watchdog Started!\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xilinx_wwdt_keepalive(wdd: *mut watchdog_device) -> c_int {
    static int xilinx_wwdt_keepalive(struct watchdog_device *wdd)
    {
    struct xwwdt_device *xdev = watchdog_get_drvdata(wdd);
    u32 control_status_reg;
    spin_lock(&xdev.spinlock);
// Enable write access control bit for the window watchdog
    iowrite32(XWWDT_MWR_MASK, xdev.base + XWWDT_MWR_OFFSET);
// Trigger restart kick to watchdog
    control_status_reg = ioread32(xdev.base + XWWDT_ESR_OFFSET);
    control_status_reg |= XWWDT_ESR_WSW_MASK;
    iowrite32(control_status_reg, xdev.base + XWWDT_ESR_OFFSET);
    spin_unlock(&xdev.spinlock);
    return 0;
    }
    static const struct watchdog_info xilinx_wwdt_ident = {
    .options = WDIOF_KEEPALIVEPING |
    WDIOF_SETTIMEOUT,
    .firmware_version = 1,
    .identity = "xlnx_window watchdog",
    };
    static const struct watchdog_ops xilinx_wwdt_ops = {
    .owner = THIS_MODULE,
    .start = xilinx_wwdt_start,
    .ping = xilinx_wwdt_keepalive,
    };
#[no_mangle]
unsafe extern "C" fn xwwdt_probe(pdev: *mut platform_device) -> c_int {
    static int xwwdt_probe(struct platform_device *pdev)
    {
    struct watchdog_device *xilinx_wwdt_wdd;
    struct device *dev = &pdev.dev;
    struct xwwdt_device *xdev;
    u64 max_per_window_ms;
    u64 min_per_window_ms;
    u64 timeout_count;
    struct clk *clk;
    u32 timeout_ms;
    u64 ms_count;
    int ret;
    xdev = devm_kzalloc(dev, sizeof(*xdev), GFP_KERNEL);
    if (!xdev)
    return -ENOMEM;
    xilinx_wwdt_wdd = &xdev.xilinx_wwdt_wdd;
    xilinx_wwdt_wdd.info = &xilinx_wwdt_ident;
    xilinx_wwdt_wdd.ops = &xilinx_wwdt_ops;
    xilinx_wwdt_wdd.parent = dev;
    xdev.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(xdev.base))
    return PTR_ERR(xdev.base);
    clk = devm_clk_get_enabled(dev, core::ptr::null_mut());
    if (IS_ERR(clk))
    return PTR_ERR(clk);
    xdev.freq = clk_get_rate(clk);
    if (xdev.freq < 1000000)
    return -EINVAL;
    xilinx_wwdt_wdd.min_timeout = XWWDT_MIN_TIMEOUT;
    xilinx_wwdt_wdd.timeout = XWWDT_DEFAULT_TIMEOUT;
    xilinx_wwdt_wdd.max_hw_heartbeat_ms =
    div64_u64(XWWDT_MAX_COUNT_WINDOW_COMBINED, xdev.freq) * 1000;
    if (closed_window_percent == 0 || closed_window_percent >= 100)
    xdev.close_percent = XWWDT_CLOSE_WINDOW_PERCENT;
    else
    xdev.close_percent = closed_window_percent;
    watchdog_init_timeout(xilinx_wwdt_wdd, wwdt_timeout, &pdev.dev);
// Calculate ticks for 1 milli-second
    ms_count = div_u64(xdev.freq, 1000);
    timeout_ms = xilinx_wwdt_wdd.timeout * 1000;
    timeout_count = timeout_ms * ms_count;
    if (timeout_ms > xilinx_wwdt_wdd.max_hw_heartbeat_ms) {
//
// To avoid ping restrictions until the minimum hardware heartbeat,
// we will solely rely on the open window and
// adjust the minimum hardware heartbeat to 0.
//
    xdev.closed_timeout = 0;
    xdev.open_timeout = XWWDT_MAX_COUNT_WINDOW;
    xilinx_wwdt_wdd.min_hw_heartbeat_ms = 0;
    xilinx_wwdt_wdd.max_hw_heartbeat_ms = xilinx_wwdt_wdd.max_hw_heartbeat_ms / 2;
    } else {
    xdev.closed_timeout  = div64_u64(timeout_count * xdev.close_percent, 100);
    xilinx_wwdt_wdd.min_hw_heartbeat_ms =
    div64_u64(timeout_ms * xdev.close_percent, 100);
    if (timeout_ms > xilinx_wwdt_wdd.max_hw_heartbeat_ms / 2) {
    max_per_window_ms = xilinx_wwdt_wdd.max_hw_heartbeat_ms / 2;
    min_per_window_ms = timeout_ms - max_per_window_ms;
    if (xilinx_wwdt_wdd.min_hw_heartbeat_ms > max_per_window_ms) {
    dev_info(xilinx_wwdt_wdd.parent,
    "Closed window cannot be set to %d%%. Using maximum supported value.\n",
    xdev.close_percent);
    xdev.closed_timeout = max_per_window_ms * ms_count;
    xilinx_wwdt_wdd.min_hw_heartbeat_ms = max_per_window_ms;
    } else if (xilinx_wwdt_wdd.min_hw_heartbeat_ms < min_per_window_ms) {
    dev_info(xilinx_wwdt_wdd.parent,
    "Closed window cannot be set to %d%%. Using minimum supported value.\n",
    xdev.close_percent);
    xdev.closed_timeout = min_per_window_ms * ms_count;
    xilinx_wwdt_wdd.min_hw_heartbeat_ms = min_per_window_ms;
    }
    }
    xdev.open_timeout = timeout_count - xdev.closed_timeout;
    }
    spin_lock_init(&xdev.spinlock);
    watchdog_set_drvdata(xilinx_wwdt_wdd, xdev);
    watchdog_set_nowayout(xilinx_wwdt_wdd, 1);
    ret = devm_watchdog_register_device(dev, xilinx_wwdt_wdd);
    if (ret)
    return ret;
    dev_info(dev, "Xilinx window watchdog Timer with timeout %ds\n",
    xilinx_wwdt_wdd.timeout);
    return 0;
    }
    static const struct of_device_id xwwdt_of_match[] = {
    { .compatible = "xlnx,versal-wwdt", },
    {},
    };
    MODULE_DEVICE_TABLE(of, xwwdt_of_match);
    static struct platform_driver xwwdt_driver = {
    .probe = xwwdt_probe,
    .driver = {
    .name = "Xilinx window watchdog",
    .of_match_table = xwwdt_of_match,
    },
    };
    module_platform_driver(xwwdt_driver);
    MODULE_AUTHOR("Neeli Srinivas <srinivas.neeli@amd.com>");
    MODULE_DESCRIPTION("Xilinx window watchdog driver");
    MODULE_LICENSE("GPL");
