//! Automatically rewritten from C to Rust
//! Source: drivers/usb/cdns3/cdns3-imx.c
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
// cdns3-imx.c - NXP i.MX specific Glue layer for Cadence USB Controller
//
// Copyright (C) 2019 NXP
//

pub const USB3_CORE_CTRL1: c_uint = 0x00;
pub const USB3_CORE_CTRL2: c_uint = 0x04;
pub const USB3_INT_REG: c_uint = 0x08;
pub const USB3_CORE_STATUS: c_uint = 0x0c;
pub const XHCI_DEBUG_LINK_ST: c_uint = 0x10;
pub const XHCI_DEBUG_BUS: c_uint = 0x14;
pub const USB3_SSPHY_CTRL1: c_uint = 0x40;
pub const USB3_SSPHY_CTRL2: c_uint = 0x44;
pub const USB3_SSPHY_STATUS: c_uint = 0x4c;
pub const USB2_PHY_CTRL1: c_uint = 0x50;
pub const USB2_PHY_CTRL2: c_uint = 0x54;
pub const USB2_PHY_STATUS: c_uint = 0x5c;
// Register bits definition
// USB3_CORE_CTRL1

    RW_SW_RESET | PHY_SW_RESET | PHYAHB_SW_RESET)

// USB3_INT_REG

// USB3_CORE_STATUS

// USB3_SSPHY_STATUS

// OTG registers definition
pub const OTGSTS: c_uint = 0x4;
// OTGSTS

// xHCI registers definition
pub const XECP_PM_PMCSR: c_uint = 0x8018;
pub const XECP_AUX_CTRL_REG1: c_uint = 0x8120;
// Register bits definition
// XECP_AUX_CTRL_REG1

// XECP_PM_PMCSR

pub const PS_D0: c_int = 0;
pub const PS_D1: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns_imx {
    pub dev: *mut device,
    pub noncore: *mut void __iomem,
    pub clks: *mut clk_bulk_data,
    pub num_clks: c_int,
    pub cdns3_pdev: *mut platform_device,
}

#[no_mangle]
pub unsafe extern "C" fn cdns_imx_readl(data: *mut cdns_imx, offset: u32) -> u32 {
    static inline u32 cdns_imx_readl(struct cdns_imx *data, u32 offset)
    {
    return readl(data.noncore + offset);
    }
#[no_mangle]
pub unsafe extern "C" fn cdns_imx_writel(data: *mut cdns_imx, offset: u32, value: u32) {
    static inline void cdns_imx_writel(struct cdns_imx *data, u32 offset, u32 value)
    {
    writel(value, data.noncore + offset);
    }
    static const struct clk_bulk_data imx_cdns3_core_clks[] = {
    { .id = "lpm" },
    { .id = "bus" },
    { .id = "aclk" },
    { .id = "ipg" },
    { .id = "core" },
    };
#[no_mangle]
unsafe extern "C" fn cdns_imx_noncore_init(data: *mut cdns_imx) -> c_int {
    static int cdns_imx_noncore_init(struct cdns_imx *data)
    {
    u32 value;
    int ret;
    struct device *dev = data.dev;
    cdns_imx_writel(data, USB3_SSPHY_STATUS, CLK_VALID_MASK);
    udelay(1);
    ret = readl_poll_timeout(data.noncore + USB3_SSPHY_STATUS, value,
    (value & CLK_VALID_COMPARE_BITS) == CLK_VALID_COMPARE_BITS,
    10, 100000);
    if (ret) {
    dev_err(dev, "wait clkvld timeout\n");
    return ret;
    }
    value = cdns_imx_readl(data, USB3_CORE_CTRL1);
    value |= ALL_SW_RESET;
    cdns_imx_writel(data, USB3_CORE_CTRL1, value);
    udelay(1);
    value = cdns_imx_readl(data, USB3_CORE_CTRL1);
    value = (value & ~MODE_STRAP_MASK) | OTG_MODE | OC_DISABLE;
    cdns_imx_writel(data, USB3_CORE_CTRL1, value);
    value = cdns_imx_readl(data, USB3_INT_REG);
    value |= HOST_INT1_EN | DEV_INT_EN;
    cdns_imx_writel(data, USB3_INT_REG, value);
    value = cdns_imx_readl(data, USB3_CORE_CTRL1);
    value &= ~ALL_SW_RESET;
    cdns_imx_writel(data, USB3_CORE_CTRL1, value);
    return ret;
    }
    static int cdns_imx_platform_suspend(struct device *dev,
    bool suspend, bool wakeup);
    static struct cdns3_platform_data cdns_imx_pdata = {
    .platform_suspend = cdns_imx_platform_suspend,
    .quirks		  = CDNS3_DEFAULT_PM_RUNTIME_ALLOW,
    };
    static const struct of_dev_auxdata cdns_imx_auxdata[] = {
    {
    .compatible = "cdns,usb3",
    .platform_data = &cdns_imx_pdata,
    },
    {},
    };
#[no_mangle]
unsafe extern "C" fn cdns_imx_probe(pdev: *mut platform_device) -> c_int {
    static int cdns_imx_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *node = dev.of_node;
    struct cdns_imx *data;
    int ret;
    if (!node)
    return -ENODEV;
    data = devm_kzalloc(dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    platform_set_drvdata(pdev, data);
    data.dev = dev;
    data.noncore = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(data.noncore)) {
    dev_err(dev, "can't map IOMEM resource\n");
    return PTR_ERR(data.noncore);
    }
    data.num_clks = ARRAY_SIZE(imx_cdns3_core_clks);
    data.clks = devm_kmemdup(dev, imx_cdns3_core_clks,
    sizeof(imx_cdns3_core_clks), GFP_KERNEL);
    if (!data.clks)
    return -ENOMEM;
    ret = devm_clk_bulk_get(dev, data.num_clks, data.clks);
    if (ret)
    return ret;
    ret = clk_bulk_prepare_enable(data.num_clks, data.clks);
    if (ret)
    return ret;
    ret = cdns_imx_noncore_init(data);
    if (ret)
    goto err;
    ret = of_platform_populate(node, core::ptr::null_mut(), cdns_imx_auxdata, dev);
    if (ret) {
    dev_err(dev, "failed to create children: %d\n", ret);
    goto err;
    }
    device_set_wakeup_capable(dev, true);
    pm_runtime_set_active(dev);
    pm_runtime_enable(dev);
    return ret;
    err:
    clk_bulk_disable_unprepare(data.num_clks, data.clks);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cdns_imx_remove(pdev: *mut platform_device) {
    static void cdns_imx_remove(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct cdns_imx *data = dev_get_drvdata(dev);
    pm_runtime_get_sync(dev);
    of_platform_depopulate(dev);
    clk_bulk_disable_unprepare(data.num_clks, data.clks);
    pm_runtime_disable(dev);
    pm_runtime_put_noidle(dev);
    platform_set_drvdata(pdev, core::ptr::null_mut());
    }

#[no_mangle]
unsafe extern "C" fn cdns3_set_wakeup(data: *mut cdns_imx, enable: bool) {
    static void cdns3_set_wakeup(struct cdns_imx *data, bool enable)
    {
    u32 value;
    value = cdns_imx_readl(data, USB3_INT_REG);
    if (enable)
    value |= OTG_WAKEUP_EN | DEVU3_WAEKUP_EN;
    else
    value &= ~(OTG_WAKEUP_EN | DEVU3_WAEKUP_EN);
    cdns_imx_writel(data, USB3_INT_REG, value);
    }
    static int cdns_imx_platform_suspend(struct device *dev,
    bool suspend, bool wakeup)
    {
    struct cdns *cdns = dev_get_drvdata(dev);
    struct device *parent = dev.parent;
    struct cdns_imx *data = dev_get_drvdata(parent);
    void __iomem *otg_regs = (void __iomem *)(cdns.otg_regs);
    void __iomem *xhci_regs = cdns.xhci_regs;
    u32 value;
    let mut ret: c_int = 0;
    if (cdns.role != USB_ROLE_HOST)
    return 0;
    if (suspend) {
// SW request low power when all usb ports allow to it ???
    value = readl(xhci_regs + XECP_PM_PMCSR);
    value &= ~PS_MASK;
    value |= PS_D1;
    writel(value, xhci_regs + XECP_PM_PMCSR);
// mdctrl_clk_sel
    value = cdns_imx_readl(data, USB3_CORE_CTRL1);
    value |= MDCTRL_CLK_SEL;
    cdns_imx_writel(data, USB3_CORE_CTRL1, value);
// wait for mdctrl_clk_status
    value = cdns_imx_readl(data, USB3_CORE_STATUS);
    ret = readl_poll_timeout(data.noncore + USB3_CORE_STATUS, value,
    (value & MDCTRL_CLK_STATUS) == MDCTRL_CLK_STATUS,
    10, 100000);
    if (ret)
    dev_warn(parent, "wait mdctrl_clk_status timeout\n");
// wait lpm_clk_req to be 0
    value = cdns_imx_readl(data, USB3_INT_REG);
    ret = readl_poll_timeout(data.noncore + USB3_INT_REG, value,
    (value & LPM_CLK_REQ) != LPM_CLK_REQ,
    10, 100000);
    if (ret)
    dev_warn(parent, "wait lpm_clk_req timeout\n");
// wait phy_refclk_req to be 0
    value = cdns_imx_readl(data, USB3_SSPHY_STATUS);
    ret = readl_poll_timeout(data.noncore + USB3_SSPHY_STATUS, value,
    (value & PHY_REFCLK_REQ) != PHY_REFCLK_REQ,
    10, 100000);
    if (ret)
    dev_warn(parent, "wait phy_refclk_req timeout\n");
    cdns3_set_wakeup(data, wakeup);
    } else {
    cdns3_set_wakeup(data, false);
// SW request D0
    value = readl(xhci_regs + XECP_PM_PMCSR);
    value &= ~PS_MASK;
    value |= PS_D0;
    writel(value, xhci_regs + XECP_PM_PMCSR);
// clr CFG_RXDET_P3_EN
    value = readl(xhci_regs + XECP_AUX_CTRL_REG1);
    value &= ~CFG_RXDET_P3_EN;
    writel(value, xhci_regs + XECP_AUX_CTRL_REG1);
// clear mdctrl_clk_sel
    value = cdns_imx_readl(data, USB3_CORE_CTRL1);
    value &= ~MDCTRL_CLK_SEL;
    cdns_imx_writel(data, USB3_CORE_CTRL1, value);
// wait CLK_125_REQ to be 1
    value = cdns_imx_readl(data, USB3_INT_REG);
    ret = readl_poll_timeout(data.noncore + USB3_INT_REG, value,
    (value & CLK_125_REQ) == CLK_125_REQ,
    10, 100000);
    if (ret)
    dev_warn(parent, "wait CLK_125_REQ timeout\n");
// wait for mdctrl_clk_status is cleared
    value = cdns_imx_readl(data, USB3_CORE_STATUS);
    ret = readl_poll_timeout(data.noncore + USB3_CORE_STATUS, value,
    (value & MDCTRL_CLK_STATUS) != MDCTRL_CLK_STATUS,
    10, 100000);
    if (ret)
    dev_warn(parent, "wait mdctrl_clk_status cleared timeout\n");
// Wait until OTG_NRDY is 0
    value = readl(otg_regs + OTGSTS);
    ret = readl_poll_timeout(otg_regs + OTGSTS, value,
    (value & OTG_NRDY) != OTG_NRDY,
    10, 100000);
    if (ret)
    dev_warn(parent, "wait OTG ready timeout\n");
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cdns_imx_resume(dev: *mut device) -> c_int {
    static int cdns_imx_resume(struct device *dev)
    {
    struct cdns_imx *data = dev_get_drvdata(dev);
    return clk_bulk_prepare_enable(data.num_clks, data.clks);
    }
#[no_mangle]
unsafe extern "C" fn cdns_imx_suspend(dev: *mut device) -> c_int {
    static int cdns_imx_suspend(struct device *dev)
    {
    struct cdns_imx *data = dev_get_drvdata(dev);
    clk_bulk_disable_unprepare(data.num_clks, data.clks);
    return 0;
    }
// Indicate if the controller was power lost before
#[no_mangle]
pub unsafe extern "C" fn cdns_imx_is_power_lost(data: *mut cdns_imx) -> bool {
    static inline bool cdns_imx_is_power_lost(struct cdns_imx *data)
    {
    u32 value;
    value = cdns_imx_readl(data, USB3_CORE_CTRL1);
    if ((value & SW_RESET_MASK) == ALL_SW_RESET)
    return true;
    else
    return false;
    }
#[no_mangle]
unsafe extern "C" fn cdns_imx_system_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused cdns_imx_system_suspend(struct device *dev)
    {
    pm_runtime_put_sync(dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cdns_imx_system_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused cdns_imx_system_resume(struct device *dev)
    {
    struct cdns_imx *data = dev_get_drvdata(dev);
    int ret;
    ret = pm_runtime_resume_and_get(dev);
    if (ret < 0) {
    dev_err(dev, "Could not get runtime PM.\n");
    return ret;
    }
    if (cdns_imx_is_power_lost(data)) {
    dev_dbg(dev, "resume from power lost\n");
    ret = cdns_imx_noncore_init(data);
    if (ret)
    cdns_imx_suspend(dev);
    }
    return ret;
    }

    static int cdns_imx_platform_suspend(struct device *dev,
    bool suspend, bool wakeup)
    {
    return 0;
    }

    static const struct dev_pm_ops cdns_imx_pm_ops = {
    SET_RUNTIME_PM_OPS(cdns_imx_suspend, cdns_imx_resume, core::ptr::null_mut())
    SET_SYSTEM_SLEEP_PM_OPS(cdns_imx_system_suspend, cdns_imx_system_resume)
    };
    static const struct of_device_id cdns_imx_of_match[] = {
    { .compatible = "fsl,imx8qm-usb3", },
    {},
    };
    MODULE_DEVICE_TABLE(of, cdns_imx_of_match);
    static struct platform_driver cdns_imx_driver = {
    .probe		= cdns_imx_probe,
    .remove		= cdns_imx_remove,
    .driver		= {
    .name	= "cdns3-imx",
    .of_match_table	= cdns_imx_of_match,
    .pm	= &cdns_imx_pm_ops,
    },
    };
    module_platform_driver(cdns_imx_driver);
    MODULE_ALIAS("platform:cdns3-imx");
    MODULE_AUTHOR("Peter Chen <peter.chen@nxp.com>");
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("Cadence USB3 i.MX Glue Layer");
