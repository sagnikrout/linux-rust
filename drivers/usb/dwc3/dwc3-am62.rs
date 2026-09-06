//! Automatically rewritten from C to Rust
//! Source: drivers/usb/dwc3/dwc3-am62.c
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
// dwc3-am62.c - TI specific Glue layer for AM62 DWC3 USB Controller
//
// Copyright (C) 2022 Texas Instruments Incorporated - https://www.ti.com
//

// USB WRAPPER register offsets
pub const USBSS_PID: c_uint = 0x0;
pub const USBSS_OVERCURRENT_CTRL: c_uint = 0x4;
pub const USBSS_PHY_CONFIG: c_uint = 0x8;
pub const USBSS_PHY_TEST: c_uint = 0xc;
pub const USBSS_CORE_STAT: c_uint = 0x14;
pub const USBSS_HOST_VBUS_CTRL: c_uint = 0x18;
pub const USBSS_MODE_CONTROL: c_uint = 0x1c;
pub const USBSS_WAKEUP_CONFIG: c_uint = 0x30;
pub const USBSS_WAKEUP_STAT: c_uint = 0x34;
pub const USBSS_OVERRIDE_CONFIG: c_uint = 0x38;
pub const USBSS_IRQ_MISC_STATUS_RAW: c_uint = 0x430;
pub const USBSS_IRQ_MISC_STATUS: c_uint = 0x434;
pub const USBSS_IRQ_MISC_ENABLE_SET: c_uint = 0x438;
pub const USBSS_IRQ_MISC_ENABLE_CLR: c_uint = 0x43c;
pub const USBSS_IRQ_MISC_EOI: c_uint = 0x440;
pub const USBSS_INTR_TEST: c_uint = 0x490;
pub const USBSS_VBUS_FILTER: c_uint = 0x614;
pub const USBSS_VBUS_STAT: c_uint = 0x618;
pub const USBSS_DEBUG_CFG: c_uint = 0x708;
pub const USBSS_DEBUG_DATA: c_uint = 0x70c;
pub const USBSS_HOST_HUB_CTRL: c_uint = 0x714;
// PHY CONFIG register bits

pub const USBSS_PHY_VBUS_SEL_SHIFT: c_int = 1;

// CORE STAT register bits

pub const USBSS_CORE_OPERATIONAL_MODE_SHIFT: c_int = 12;
// MODE CONTROL register bits

// WAKEUP CONFIG register bits

    USBSS_WAKEUP_CFG_SESSVALID_EN | \
    USBSS_WAKEUP_CFG_LINESTATE_EN | \
    USBSS_WAKEUP_CFG_OVERCURRENT_EN)
pub const USBSS_WAKEUP_CFG_NONE: c_int = 0;
// WAKEUP STAT register bits

// IRQ_MISC_STATUS_RAW register bits

// IRQ_MISC_STATUS register bits

// IRQ_MISC_ENABLE_SET register bits

// IRQ_MISC_ENABLE_CLR register bits

// IRQ_MISC_EOI register bits

// VBUS_STAT register bits

// USB_PHY_CTRL register bits in CTRL_MMR

// USB PHY2 register offsets
pub const USB_PHY_PLL_REG12: c_uint = 0x130;

pub const DWC3_AM62_AUTOSUSPEND_DELAY: c_int = 100;
pub const USBSS_DEBUG_CFG_OFF: c_uint = 0x0;
pub const USBSS_DEBUG_CFG_DISABLED: c_uint = 0x7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwc3_am62 {
    pub dev: *mut device,
    pub usbss: *mut void __iomem,
    pub usb2_refclk: *mut clk,
    pub rate_code: c_int,
    pub syscon: *mut regmap,
    pub offset: c_uint,
    pub vbus_divider: c_uint,
    pub wakeup_stat: u32,
    pub phy_regs: *mut void __iomem,
}

    static const int dwc3_ti_rate_table[] = {	/* in KHZ */
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
pub unsafe extern "C" fn dwc3_ti_readl(am62: *mut dwc3_am62, offset: u32) -> u32 {
    static inline u32 dwc3_ti_readl(struct dwc3_am62 *am62, u32 offset)
    {
    return readl((am62.usbss) + offset);
    }
#[no_mangle]
pub unsafe extern "C" fn dwc3_ti_writel(am62: *mut dwc3_am62, offset: u32, value: u32) {
    static inline void dwc3_ti_writel(struct dwc3_am62 *am62, u32 offset, u32 value)
    {
    writel(value, (am62.usbss) + offset);
    }
#[no_mangle]
unsafe extern "C" fn phy_syscon_pll_refclk(am62: *mut dwc3_am62) -> c_int {
    static int phy_syscon_pll_refclk(struct dwc3_am62 *am62)
    {
    struct device *dev = am62.dev;
    struct device_node *node = dev.of_node;
    struct regmap *syscon;
    int ret;
    syscon = syscon_regmap_lookup_by_phandle_args(node, "ti,syscon-phy-pll-refclk",
    1, &am62.offset);
    if (IS_ERR(syscon)) {
    dev_err(dev, "unable to get ti,syscon-phy-pll-refclk regmap\n");
    return PTR_ERR(syscon);
    }
    am62.syscon = syscon;
// Core voltage. PHY_CORE_VOLTAGE bit Recommended to be 0 always
    ret = regmap_update_bits(am62.syscon, am62.offset, PHY_CORE_VOLTAGE_MASK, 0);
    if (ret) {
    dev_err(dev, "failed to set phy core voltage\n");
    return ret;
    }
    ret = regmap_update_bits(am62.syscon, am62.offset, PHY_PLL_REFCLK_MASK, am62.rate_code);
    if (ret) {
    dev_err(dev, "failed to set phy pll reference clock rate\n");
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dwc3_ti_init(am62: *mut dwc3_am62) -> c_int {
    static int dwc3_ti_init(struct dwc3_am62 *am62)
    {
    int ret;
    u32 reg;
// Read the syscon property and set the rate code
    ret = phy_syscon_pll_refclk(am62);
    if (ret)
    return ret;
// Workaround Errata i2409
    if (am62.phy_regs) {
    reg = readl(am62.phy_regs + USB_PHY_PLL_REG12);
    reg |= USB_PHY_PLL_LDO_REF_EN | USB_PHY_PLL_LDO_REF_EN_EN;
    writel(reg, am62.phy_regs + USB_PHY_PLL_REG12);
    }
// VBUS divider select
    reg = dwc3_ti_readl(am62, USBSS_PHY_CONFIG);
    if (am62.vbus_divider)
    reg |= 1 << USBSS_PHY_VBUS_SEL_SHIFT;
    dwc3_ti_writel(am62, USBSS_PHY_CONFIG, reg);
    ret = clk_prepare_enable(am62.usb2_refclk);
    if (ret)
    return ret;
// Set mode valid bit to indicate role is valid
    reg = dwc3_ti_readl(am62, USBSS_MODE_CONTROL);
    reg |= USBSS_MODE_VALID;
    dwc3_ti_writel(am62, USBSS_MODE_CONTROL, reg);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dwc3_ti_probe(pdev: *mut platform_device) -> c_int {
    static int dwc3_ti_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *node = pdev.dev.of_node;
    struct dwc3_am62 *am62;
    unsigned long rate;
    int i, ret;
    am62 = devm_kzalloc(dev, sizeof(*am62), GFP_KERNEL);
    if (!am62)
    return -ENOMEM;
    am62.dev = dev;
    platform_set_drvdata(pdev, am62);
    am62.usbss = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(am62.usbss)) {
    dev_err(dev, "can't map IOMEM resource\n");
    return PTR_ERR(am62.usbss);
    }
    am62.usb2_refclk = devm_clk_get(dev, "ref");
    if (IS_ERR(am62.usb2_refclk)) {
    dev_err(dev, "can't get usb2_refclk\n");
    return PTR_ERR(am62.usb2_refclk);
    }
// Calculate the rate code
    rate = clk_get_rate(am62.usb2_refclk);
    rate /= 1000;	// To KHz
    for (i = 0; i < ARRAY_SIZE(dwc3_ti_rate_table); i++) {
    if (dwc3_ti_rate_table[i] == rate)
    break;
    }
    if (i == ARRAY_SIZE(dwc3_ti_rate_table)) {
    dev_err(dev, "unsupported usb2_refclk rate: %lu KHz\n", rate);
    return -EINVAL;
    }
    am62.rate_code = i;
    am62.phy_regs = devm_platform_ioremap_resource(pdev, 1);
    if (IS_ERR(am62.phy_regs)) {
    dev_err(dev, "can't map PHY IOMEM resource. Won't apply i2409 fix.\n");
    am62.phy_regs = core::ptr::null_mut();
    }
    am62.vbus_divider = device_property_read_bool(dev, "ti,vbus-divider");
    ret = dwc3_ti_init(am62);
    if (ret)
    return ret;
    pm_runtime_set_active(dev);
    pm_runtime_enable(dev);
//
// Don't ignore its dependencies with its children
//
    pm_suspend_ignore_children(dev, false);
    pm_runtime_get_noresume(dev);
    ret = of_platform_populate(node, core::ptr::null_mut(), core::ptr::null_mut(), dev);
    if (ret) {
    dev_err(dev, "failed to create dwc3 core: %d\n", ret);
    goto err_pm_disable;
    }
// Device has capability to wakeup system from sleep
    device_set_wakeup_capable(dev, true);
    ret = device_wakeup_enable(dev);
    if (ret)
    dev_err(dev, "couldn't enable device as a wakeup source: %d\n", ret);
// Setting up autosuspend
    pm_runtime_set_autosuspend_delay(dev, DWC3_AM62_AUTOSUSPEND_DELAY);
    pm_runtime_use_autosuspend(dev);
    pm_runtime_put_autosuspend(dev);
    return 0;
    err_pm_disable:
    clk_disable_unprepare(am62.usb2_refclk);
    pm_runtime_disable(dev);
    pm_runtime_set_suspended(dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn dwc3_ti_remove(pdev: *mut platform_device) {
    static void dwc3_ti_remove(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct dwc3_am62 *am62 = platform_get_drvdata(pdev);
    u32 reg;
    pm_runtime_get_sync(dev);
    device_init_wakeup(dev, false);
    of_platform_depopulate(dev);
// Clear mode valid bit
    reg = dwc3_ti_readl(am62, USBSS_MODE_CONTROL);
    reg &= ~USBSS_MODE_VALID;
    dwc3_ti_writel(am62, USBSS_MODE_CONTROL, reg);
    pm_runtime_put_sync(dev);
    pm_runtime_disable(dev);
    pm_runtime_dont_use_autosuspend(dev);
    pm_runtime_set_suspended(dev);
    }

#[no_mangle]
unsafe extern "C" fn dwc3_ti_suspend_common(dev: *mut device) -> c_int {
    static int dwc3_ti_suspend_common(struct device *dev)
    {
    struct dwc3_am62 *am62 = dev_get_drvdata(dev);
    u32 reg, current_prtcap_dir;
    if (device_may_wakeup(dev)) {
    reg = dwc3_ti_readl(am62, USBSS_CORE_STAT);
    current_prtcap_dir = (reg & USBSS_CORE_OPERATIONAL_MODE_MASK)
    >> USBSS_CORE_OPERATIONAL_MODE_SHIFT;
// Set wakeup config enable bits
    reg = dwc3_ti_readl(am62, USBSS_WAKEUP_CONFIG);
    if (current_prtcap_dir == DWC3_GCTL_PRTCAP_HOST) {
    reg = USBSS_WAKEUP_CFG_LINESTATE_EN | USBSS_WAKEUP_CFG_OVERCURRENT_EN;
    } else {
    reg = USBSS_WAKEUP_CFG_VBUSVALID_EN | USBSS_WAKEUP_CFG_SESSVALID_EN;
//
// Enable LINESTATE wake up only if connected to bus
// and in U2/L3 state else it causes spurious wake-up.
//
    }
    dwc3_ti_writel(am62, USBSS_WAKEUP_CONFIG, reg);
// clear wakeup status so we know what caused the wake up
    dwc3_ti_writel(am62, USBSS_WAKEUP_STAT, USBSS_WAKEUP_STAT_CLR);
    }
// just to track if module resets on suspend
    dwc3_ti_writel(am62, USBSS_DEBUG_CFG, USBSS_DEBUG_CFG_DISABLED);
    clk_disable_unprepare(am62.usb2_refclk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dwc3_ti_resume_common(dev: *mut device) -> c_int {
    static int dwc3_ti_resume_common(struct device *dev)
    {
    struct dwc3_am62 *am62 = dev_get_drvdata(dev);
    u32 reg;
    int ret;
    reg = dwc3_ti_readl(am62, USBSS_DEBUG_CFG);
    if (reg != USBSS_DEBUG_CFG_DISABLED) {
// lost power/context
    ret = dwc3_ti_init(am62);
    if (ret)
    return ret;
    } else {
    dwc3_ti_writel(am62, USBSS_DEBUG_CFG, USBSS_DEBUG_CFG_OFF);
    ret = clk_prepare_enable(am62.usb2_refclk);
    if (ret)
    return ret;
    }
    if (device_may_wakeup(dev)) {
// Clear wakeup config enable bits
    dwc3_ti_writel(am62, USBSS_WAKEUP_CONFIG, USBSS_WAKEUP_CFG_NONE);
    }
    reg = dwc3_ti_readl(am62, USBSS_WAKEUP_STAT);
    am62.wakeup_stat = reg;
    return 0;
    }
    static UNIVERSAL_DEV_PM_OPS(dwc3_ti_pm_ops, dwc3_ti_suspend_common,
    dwc3_ti_resume_common, core::ptr::null_mut());

    static const struct of_device_id dwc3_ti_of_match[] = {
    { .compatible = "ti,am62-usb"},
    {},
    };
    MODULE_DEVICE_TABLE(of, dwc3_ti_of_match);
    static struct platform_driver dwc3_ti_driver = {
    .probe		= dwc3_ti_probe,
    .remove		= dwc3_ti_remove,
    .driver		= {
    .name	= "dwc3-am62",
    .pm	= DEV_PM_OPS,
    .of_match_table = dwc3_ti_of_match,
    },
    };
    module_platform_driver(dwc3_ti_driver);
    MODULE_ALIAS("platform:dwc3-am62");
    MODULE_AUTHOR("Aswath Govindraju <a-govindraju@ti.com>");
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("DesignWare USB3 TI Glue Layer");
