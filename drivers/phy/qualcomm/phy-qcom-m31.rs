//! Automatically rewritten from C to Rust
//! Source: drivers/phy/qualcomm/phy-qcom-m31.c
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
// Copyright (c) 2014-2023, The Linux Foundation. All rights reserved.
//

pub const USB2PHY_PORT_UTMI_CTRL1: c_uint = 0x40;
pub const USB2PHY_PORT_UTMI_CTRL2: c_uint = 0x44;

pub const HS_PHY_CTRL_REG: c_uint = 0x10;

pub const USB_PHY_UTMI_CTRL0: c_uint = 0x3c;
pub const USB_PHY_UTMI_CTRL5: c_uint = 0x50;

pub const USB_PHY_HS_PHY_CTRL_COMMON0: c_uint = 0x54;

pub const USB_PHY_HS_PHY_CTRL2: c_uint = 0x64;

pub const USB_PHY_CFG0: c_uint = 0x94;

pub const USB_PHY_REFCLK_CTRL: c_uint = 0xa0;

pub const USB2PHY_PORT_POWERDOWN: c_uint = 0xa4;

pub const POWER_DOWN: c_int = 0;
pub const USB_PHY_FSEL_SEL: c_uint = 0xb8;

pub const USB2PHY_USB_PHY_M31_XCFGI_1: c_uint = 0xbc;

pub const USB2PHY_USB_PHY_M31_XCFGI_4: c_uint = 0xc8;

pub const USB2PHY_USB_PHY_M31_XCFGI_5: c_uint = 0xcc;

pub const USB2PHY_USB_PHY_M31_XCFGI_9: c_uint = 0xdc;

pub const USB2PHY_USB_PHY_M31_XCFGI_11: c_uint = 0xe4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct m31_phy_regs {
    pub off: u32,
    pub val: u32,
    pub delay: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct m31_priv_data {
    pub ulpi_mode: bool,
    pub regs: *const m31_phy_regs,
    pub nregs: c_uint,
}

    static const struct m31_phy_regs m31_ipq5018_regs[] = {
    {
    .off = USB_PHY_CFG0,
    .val = UTMI_PHY_OVERRIDE_EN
    },
    {
    .off = USB_PHY_UTMI_CTRL5,
    .val = POR_EN,
    .delay = 15
    },
    {
    .off = USB_PHY_FSEL_SEL,
    .val = FREQ_SEL
    },
    {
    .off = USB_PHY_HS_PHY_CTRL_COMMON0,
    .val = COMMONONN | FSEL | RETENABLEN
    },
    {
    .off = USB_PHY_REFCLK_CTRL,
    .val = CLKCORE
    },
    {
    .off = USB_PHY_UTMI_CTRL5,
    .val = POR_EN
    },
    {
    .off = USB_PHY_HS_PHY_CTRL2,
    .val = USB2_SUSPEND_N_SEL | USB2_SUSPEND_N | USB2_UTMI_CLK_EN
    },
    {
    .off = USB_PHY_UTMI_CTRL5,
    .val = 0x0
    },
    {
    .off = USB_PHY_HS_PHY_CTRL2,
    .val = USB2_SUSPEND_N | USB2_UTMI_CLK_EN
    },
    {
    .off = USB_PHY_CFG0,
    .val = 0x0
    },
    };
    static struct m31_phy_regs m31_ipq5332_regs[] = {
    {
    USB_PHY_CFG0,
    UTMI_PHY_OVERRIDE_EN,
    0
    },
    {
    USB_PHY_UTMI_CTRL5,
    POR_EN,
    15
    },
    {
    USB_PHY_FSEL_SEL,
    FREQ_SEL,
    0
    },
    {
    USB_PHY_HS_PHY_CTRL_COMMON0,
    COMMONONN | FREQ_24MHZ | RETENABLEN,
    0
    },
    {
    USB_PHY_UTMI_CTRL5,
    POR_EN,
    0
    },
    {
    USB_PHY_HS_PHY_CTRL2,
    USB2_SUSPEND_N_SEL | USB2_SUSPEND_N | USB2_UTMI_CLK_EN,
    0
    },
    {
    USB2PHY_USB_PHY_M31_XCFGI_11,
    XCFG_COARSE_TUNE_NUM  | XCFG_FINE_TUNE_NUM,
    0
    },
    {
    USB2PHY_USB_PHY_M31_XCFGI_4,
    HSTX_SLEW_RATE_400PS | PLL_CHARGING_PUMP_CURRENT_35UA | ODT_VALUE_38_02_OHM,
    0
    },
    {
    USB2PHY_USB_PHY_M31_XCFGI_1,
    USB2_0_TX_ENABLE,
    0
    },
    {
    USB2PHY_USB_PHY_M31_XCFGI_5,
    HSTX_PRE_EMPHASIS_LEVEL_0_55MA,
    4
    },
    {
    USB2PHY_USB_PHY_M31_XCFGI_9,
    HSTX_CURRENT_17_1MA_385MV,
    },
    {
    USB_PHY_UTMI_CTRL5,
    0x0,
    0
    },
    {
    USB_PHY_HS_PHY_CTRL2,
    USB2_SUSPEND_N | USB2_UTMI_CLK_EN,
    0
    },
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct m31usb_phy {
    pub phy: *mut phy,
    pub base: *mut void __iomem,
    pub regs: *const m31_phy_regs,
    pub nregs: c_int,
    pub vreg: *mut regulator,
    pub clk: *mut clk,
    pub reset: *mut reset_control,
    pub ulpi_mode: bool,
}

#[no_mangle]
unsafe extern "C" fn m31usb_phy_init(phy: *mut phy) -> c_int {
    static int m31usb_phy_init(struct phy *phy)
    {
    struct m31usb_phy *qphy = phy_get_drvdata(phy);
    const struct m31_phy_regs *regs = qphy.regs;
    int i, ret;
    ret = regulator_enable(qphy.vreg);
    if (ret) {
    dev_err(&phy.dev, "failed to enable regulator, %d\n", ret);
    return ret;
    }
    ret = clk_prepare_enable(qphy.clk);
    if (ret) {
    regulator_disable(qphy.vreg);
    dev_err(&phy.dev, "failed to enable cfg ahb clock, %d\n", ret);
    return ret;
    }
// Perform phy reset
    reset_control_assert(qphy.reset);
    udelay(5);
    reset_control_deassert(qphy.reset);
// configure for ULPI mode if requested
    if (qphy.ulpi_mode)
    writel(0x0, qphy.base + USB2PHY_PORT_UTMI_CTRL2);
// Enable the PHY
    writel(POWER_UP, qphy.base + USB2PHY_PORT_POWERDOWN);
// Turn on phy ref clock
    for (i = 0; i < qphy.nregs; i++) {
    writel(regs[i].val, qphy.base + regs[i].off);
    if (regs[i].delay)
    udelay(regs[i].delay);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn m31usb_phy_shutdown(phy: *mut phy) -> c_int {
    static int m31usb_phy_shutdown(struct phy *phy)
    {
    struct m31usb_phy *qphy = phy_get_drvdata(phy);
// Disable the PHY
    writel_relaxed(POWER_DOWN, qphy.base + USB2PHY_PORT_POWERDOWN);
    clk_disable_unprepare(qphy.clk);
    regulator_disable(qphy.vreg);
    return 0;
    }
    static const struct phy_ops m31usb_phy_gen_ops = {
    .power_on	= m31usb_phy_init,
    .power_off	= m31usb_phy_shutdown,
    .owner		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn m31usb_phy_probe(pdev: *mut platform_device) -> c_int {
    static int m31usb_phy_probe(struct platform_device *pdev)
    {
    struct phy_provider *phy_provider;
    const struct m31_priv_data *data;
    struct device *dev = &pdev.dev;
    struct m31usb_phy *qphy;
    qphy = devm_kzalloc(dev, sizeof(*qphy), GFP_KERNEL);
    if (!qphy)
    return -ENOMEM;
    qphy.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(qphy.base))
    return PTR_ERR(qphy.base);
    qphy.reset = devm_reset_control_get_exclusive_by_index(dev, 0);
    if (IS_ERR(qphy.reset))
    return PTR_ERR(qphy.reset);
    qphy.clk = devm_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(qphy.clk))
    return dev_err_probe(dev, PTR_ERR(qphy.clk),
    "failed to get clk\n");
    data = of_device_get_match_data(dev);
    qphy.regs		= data.regs;
    qphy.nregs		= data.nregs;
    qphy.ulpi_mode		= data.ulpi_mode;
    qphy.phy = devm_phy_create(dev, core::ptr::null_mut(), &m31usb_phy_gen_ops);
    if (IS_ERR(qphy.phy))
    return dev_err_probe(dev, PTR_ERR(qphy.phy),
    "failed to create phy\n");
    qphy.vreg = devm_regulator_get(dev, "vdd");
    if (IS_ERR(qphy.vreg))
    return dev_err_probe(dev, PTR_ERR(qphy.vreg),
    "failed to get vreg\n");
    phy_set_drvdata(qphy.phy, qphy);
    phy_provider = devm_of_phy_provider_register(dev, of_phy_simple_xlate);
    return PTR_ERR_OR_ZERO(phy_provider);
    }
    static const struct m31_priv_data m31_ipq5018_data = {
    .ulpi_mode = false,
    .regs = m31_ipq5018_regs,
    .nregs = ARRAY_SIZE(m31_ipq5018_regs),
    };
    static const struct m31_priv_data m31_ipq5332_data = {
    .ulpi_mode = false,
    .regs = m31_ipq5332_regs,
    .nregs = ARRAY_SIZE(m31_ipq5332_regs),
    };
    static const struct of_device_id m31usb_phy_id_table[] = {
    { .compatible = "qcom,ipq5018-usb-hsphy", .data = &m31_ipq5018_data },
    { .compatible = "qcom,ipq5332-usb-hsphy", .data = &m31_ipq5332_data },
    { },
    };
    MODULE_DEVICE_TABLE(of, m31usb_phy_id_table);
    static struct platform_driver m31usb_phy_driver = {
    .probe = m31usb_phy_probe,
    .driver = {
    .name = "qcom-m31usb-phy",
    .of_match_table = m31usb_phy_id_table,
    },
    };
    module_platform_driver(m31usb_phy_driver);
    MODULE_DESCRIPTION("USB2 Qualcomm M31 HSPHY driver");
    MODULE_LICENSE("GPL");
