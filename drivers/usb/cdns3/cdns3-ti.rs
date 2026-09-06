//! Automatically rewritten from C to Rust
//! Source: drivers/usb/cdns3/cdns3-ti.c
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
// cdns3-ti.c - TI specific Glue layer for Cadence USB Controller
//
// Copyright (C) 2019 Texas Instruments Incorporated - https://www.ti.com
//

// USB Wrapper register offsets
pub const USBSS_PID: c_uint = 0x0;
pub const USBSS_W1: c_uint = 0x4;
pub const USBSS_STATIC_CONFIG: c_uint = 0x8;
pub const USBSS_PHY_TEST: c_uint = 0xc;
pub const USBSS_DEBUG_CTRL: c_uint = 0x10;
pub const USBSS_DEBUG_INFO: c_uint = 0x14;
pub const USBSS_DEBUG_LINK_STATE: c_uint = 0x18;
pub const USBSS_DEVICE_CTRL: c_uint = 0x1c;
// Wrapper 1 register bits

pub const USBSS_W1_MODESTRAP_SHIFT: c_int = 17;

// Static config register bits

pub const USBSS1_STATIC_PLL_REF_SEL_SHIFT: c_int = 5;

pub const USBSS1_STATIC_LOOPBACK_MODE_SHIFT: c_int = 3;

pub const USBSS1_STATIC_VBUS_SEL_SHIFT: c_int = 1;

// Modestrap modes
    enum modestrap_mode { USBSS_MODESTRAP_MODE_NONE,
    USBSS_MODESTRAP_MODE_HOST,
    USBSS_MODESTRAP_MODE_PERIPHERAL};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns_ti {
    pub dev: *mut device,
    pub usbss: *mut void __iomem,
    pub usb2_only:1: unsigned,
    pub vbus_divider:1: unsigned,
    pub usb2_refclk: *mut clk,
    pub lpm_clk: *mut clk,
    pub usb2_refclk_rate_code: c_int,
}

    static const int cdns_ti_rate_table[] = {	/* in KHZ */
    9600,
    10000,
    12000,
    19200,
    20000,
    24000,
    25000,
    26000,
    38400,
    40000,
    58000,
    50000,
    52000,
    };
#[no_mangle]
pub unsafe extern "C" fn cdns_ti_readl(data: *mut cdns_ti, offset: u32) -> u32 {
    static inline u32 cdns_ti_readl(struct cdns_ti *data, u32 offset)
    {
    return readl(data.usbss + offset);
    }
#[no_mangle]
pub unsafe extern "C" fn cdns_ti_writel(data: *mut cdns_ti, offset: u32, value: u32) {
    static inline void cdns_ti_writel(struct cdns_ti *data, u32 offset, u32 value)
    {
    writel(value, data.usbss + offset);
    }
    static struct cdns3_platform_data cdns_ti_pdata = {
    .quirks = CDNS3_DRD_SUSPEND_RESIDENCY_ENABLE,   /* Errata i2409 */
    };
    static const struct of_dev_auxdata cdns_ti_auxdata[] = {
    {
    .compatible = "cdns,usb3",
    .platform_data = &cdns_ti_pdata,
    },
    {},
    };
#[no_mangle]
unsafe extern "C" fn cdns_ti_reset_and_init_hw(data: *mut cdns_ti) {
    static void cdns_ti_reset_and_init_hw(struct cdns_ti *data)
    {
    u32 reg;
// assert RESET
    reg = cdns_ti_readl(data, USBSS_W1);
    reg &= ~USBSS_W1_PWRUP_RST;
    cdns_ti_writel(data, USBSS_W1, reg);
// set static config
    reg = cdns_ti_readl(data, USBSS_STATIC_CONFIG);
    reg &= ~USBSS1_STATIC_PLL_REF_SEL_MASK;
    reg |= data.usb2_refclk_rate_code << USBSS1_STATIC_PLL_REF_SEL_SHIFT;
    reg &= ~USBSS1_STATIC_VBUS_SEL_MASK;
    if (data.vbus_divider)
    reg |= 1 << USBSS1_STATIC_VBUS_SEL_SHIFT;
    cdns_ti_writel(data, USBSS_STATIC_CONFIG, reg);
    reg = cdns_ti_readl(data, USBSS_STATIC_CONFIG);
// set USB2_ONLY mode if requested
    reg = cdns_ti_readl(data, USBSS_W1);
    if (data.usb2_only)
    reg |= USBSS_W1_USB2_ONLY;
// set default modestrap
    reg |= USBSS_W1_MODESTRAP_SEL;
    reg &= ~USBSS_W1_MODESTRAP_MASK;
    reg |= USBSS_MODESTRAP_MODE_NONE << USBSS_W1_MODESTRAP_SHIFT;
    cdns_ti_writel(data, USBSS_W1, reg);
// de-assert RESET
    reg |= USBSS_W1_PWRUP_RST;
    cdns_ti_writel(data, USBSS_W1, reg);
    }
#[no_mangle]
unsafe extern "C" fn cdns_ti_probe(pdev: *mut platform_device) -> c_int {
    static int cdns_ti_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *node = pdev.dev.of_node;
    struct cdns_ti *data;
    unsigned long rate;
    int error, i;
    data = devm_kzalloc(dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    platform_set_drvdata(pdev, data);
    data.dev = dev;
    data.usbss = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(data.usbss)) {
    dev_err(dev, "can't map IOMEM resource\n");
    return PTR_ERR(data.usbss);
    }
    data.usb2_refclk = devm_clk_get(dev, "ref");
    if (IS_ERR(data.usb2_refclk)) {
    dev_err(dev, "can't get usb2_refclk\n");
    return PTR_ERR(data.usb2_refclk);
    }
    data.lpm_clk = devm_clk_get(dev, "lpm");
    if (IS_ERR(data.lpm_clk)) {
    dev_err(dev, "can't get lpm_clk\n");
    return PTR_ERR(data.lpm_clk);
    }
    rate = clk_get_rate(data.usb2_refclk);
    rate /= 1000;	/* To KHz */
    for (i = 0; i < ARRAY_SIZE(cdns_ti_rate_table); i++) {
    if (cdns_ti_rate_table[i] == rate)
    break;
    }
    if (i == ARRAY_SIZE(cdns_ti_rate_table)) {
    dev_err(dev, "unsupported usb2_refclk rate: %lu KHz\n", rate);
    return -EINVAL;
    }
    data.usb2_refclk_rate_code = i;
    data.vbus_divider = device_property_read_bool(dev, "ti,vbus-divider");
    data.usb2_only = device_property_read_bool(dev, "ti,usb2-only");
//
// The call below to pm_runtime_get_sync() MIGHT reset hardware, if it
// detects it as uninitialised. We want to enforce a reset at probe,
// and so do it manually here. This means the first runtime_resume()
// will be a no-op.
//
    cdns_ti_reset_and_init_hw(data);
    pm_runtime_enable(dev);
    error = pm_runtime_get_sync(dev);
    if (error < 0) {
    dev_err(dev, "pm_runtime_get_sync failed: %d\n", error);
    goto err;
    }
    error = of_platform_populate(node, core::ptr::null_mut(), cdns_ti_auxdata, dev);
    if (error) {
    dev_err(dev, "failed to create children: %d\n", error);
    goto err;
    }
    return 0;
    err:
    pm_runtime_put_sync(data.dev);
    pm_runtime_disable(data.dev);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn cdns_ti_remove_core(dev: *mut device, c: *mut c_void) -> c_int {
    static int cdns_ti_remove_core(struct device *dev, void *c)
    {
    struct platform_device *pdev = to_platform_device(dev);
    platform_device_unregister(pdev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cdns_ti_remove(pdev: *mut platform_device) {
    static void cdns_ti_remove(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    device_for_each_child(dev, core::ptr::null_mut(), cdns_ti_remove_core);
    pm_runtime_put_sync(dev);
    pm_runtime_disable(dev);
    platform_set_drvdata(pdev, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn cdns_ti_runtime_resume(dev: *mut device) -> c_int {
    static int cdns_ti_runtime_resume(struct device *dev)
    {
    let mut mask: u32 = USBSS_W1_PWRUP_RST | USBSS_W1_MODESTRAP_SEL;
    struct cdns_ti *data = dev_get_drvdata(dev);
    u32 w1;
    w1 = cdns_ti_readl(data, USBSS_W1);
    if ((w1 & mask) != mask)
    cdns_ti_reset_and_init_hw(data);
    return 0;
    }
    static const struct dev_pm_ops cdns_ti_pm_ops = {
    RUNTIME_PM_OPS(core::ptr::null_mut(), cdns_ti_runtime_resume, core::ptr::null_mut())
    SYSTEM_SLEEP_PM_OPS(pm_runtime_force_suspend, pm_runtime_force_resume)
    };
    static const struct of_device_id cdns_ti_of_match[] = {
    { .compatible = "ti,j721e-usb", },
    { .compatible = "ti,am64-usb", },
    {},
    };
    MODULE_DEVICE_TABLE(of, cdns_ti_of_match);
    static struct platform_driver cdns_ti_driver = {
    .probe		= cdns_ti_probe,
    .remove		= cdns_ti_remove,
    .driver		= {
    .name	= "cdns3-ti",
    .of_match_table	= cdns_ti_of_match,
    .pm     = pm_ptr(&cdns_ti_pm_ops),
    },
    };
    module_platform_driver(cdns_ti_driver);
    MODULE_ALIAS("platform:cdns3-ti");
    MODULE_AUTHOR("Roger Quadros <rogerq@ti.com>");
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("Cadence USB3 TI Glue Layer");
