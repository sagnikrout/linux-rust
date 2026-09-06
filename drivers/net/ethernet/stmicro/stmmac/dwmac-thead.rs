//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/stmicro/stmmac/dwmac-thead.c
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
// T-HEAD DWMAC platform driver
//
// Copyright (C) 2021 Alibaba Group Holding Limited.
// Copyright (C) 2023 Jisheng Zhang <jszhang@kernel.org>
//

pub const GMAC_CLK_EN: c_uint = 0x00;

pub const GMAC_RXCLK_DELAY_CTRL: c_uint = 0x04;

pub const GMAC_TXCLK_DELAY_CTRL: c_uint = 0x08;

pub const GMAC_PLLCLK_DIV: c_uint = 0x0c;

pub const GMAC_GTXCLK_SEL: c_uint = 0x18;

pub const GMAC_INTF_CTRL: c_uint = 0x1c;

pub const GMAC_TXCLK_OEN: c_uint = 0x20;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct thead_dwmac {
    pub plat: *mut plat_stmmacenet_data,
    pub apb_base: *mut void __iomem,
    pub dev: *mut device,
}

#[no_mangle]
unsafe extern "C" fn thead_dwmac_set_phy_if(plat: *mut plat_stmmacenet_data) -> c_int {
    static int thead_dwmac_set_phy_if(struct plat_stmmacenet_data *plat)
    {
    struct thead_dwmac *dwmac = plat.bsp_priv;
    u32 phyif;
    switch (plat.phy_interface) {
    case PHY_INTERFACE_MODE_MII:
    phyif = GMAC_INTF_MII_GMII;
    break;
    case PHY_INTERFACE_MODE_RGMII:
    case PHY_INTERFACE_MODE_RGMII_ID:
    case PHY_INTERFACE_MODE_RGMII_TXID:
    case PHY_INTERFACE_MODE_RGMII_RXID:
    phyif = GMAC_INTF_RGMII;
    break;
    default:
    dev_err(dwmac.dev, "unsupported phy interface %s\n",
    phy_modes(plat.phy_interface));
    return -EINVAL;
    }
    writel(phyif, dwmac.apb_base + GMAC_INTF_CTRL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn thead_dwmac_set_txclk_dir(plat: *mut plat_stmmacenet_data) -> c_int {
    static int thead_dwmac_set_txclk_dir(struct plat_stmmacenet_data *plat)
    {
    struct thead_dwmac *dwmac = plat.bsp_priv;
    u32 txclk_dir;
    switch (plat.phy_interface) {
    case PHY_INTERFACE_MODE_MII:
    txclk_dir = TXCLK_DIR_INPUT;
    break;
    case PHY_INTERFACE_MODE_RGMII:
    case PHY_INTERFACE_MODE_RGMII_ID:
    case PHY_INTERFACE_MODE_RGMII_TXID:
    case PHY_INTERFACE_MODE_RGMII_RXID:
    txclk_dir = TXCLK_DIR_OUTPUT;
    break;
    default:
    dev_err(dwmac.dev, "unsupported phy interface %s\n",
    phy_modes(plat.phy_interface));
    return -EINVAL;
    }
    writel(txclk_dir, dwmac.apb_base + GMAC_TXCLK_OEN);
    return 0;
    }
    static int thead_set_clk_tx_rate(void *bsp_priv, struct clk *clk_tx_i,
    phy_interface_t interface, int speed)
    {
    struct thead_dwmac *dwmac = bsp_priv;
    struct plat_stmmacenet_data *plat;
    unsigned long rate;
    long tx_rate;
    u32 div, reg;
    plat = dwmac.plat;
    switch (plat.phy_interface) {
// For MII, rxc/txc is provided by phy
    case PHY_INTERFACE_MODE_MII:
    return 0;
    case PHY_INTERFACE_MODE_RGMII:
    case PHY_INTERFACE_MODE_RGMII_ID:
    case PHY_INTERFACE_MODE_RGMII_RXID:
    case PHY_INTERFACE_MODE_RGMII_TXID:
    rate = clk_get_rate(plat.stmmac_clk);
    writel(0, dwmac.apb_base + GMAC_PLLCLK_DIV);
    tx_rate = rgmii_clock(speed);
    if (tx_rate < 0) {
    dev_err(dwmac.dev, "invalid speed %d\n", speed);
    return tx_rate;
    }
    div = rate / tx_rate;
    if (rate != tx_rate * div) {
    dev_err(dwmac.dev, "invalid gmac rate %lu\n", rate);
    return -EINVAL;
    }
    reg = FIELD_PREP(GMAC_PLLCLK_DIV_EN, 1) |
    FIELD_PREP(GMAC_PLLCLK_DIV_NUM, div);
    writel(reg, dwmac.apb_base + GMAC_PLLCLK_DIV);
    return 0;
    default:
    dev_err(dwmac.dev, "unsupported phy interface %s\n",
    phy_modes(plat.phy_interface));
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn thead_dwmac_enable_clk(plat: *mut plat_stmmacenet_data) -> c_int {
    static int thead_dwmac_enable_clk(struct plat_stmmacenet_data *plat)
    {
    struct thead_dwmac *dwmac = plat.bsp_priv;
    u32 reg, div;
    switch (plat.phy_interface) {
    case PHY_INTERFACE_MODE_MII:
    reg = GMAC_RX_CLK_EN | GMAC_TX_CLK_EN;
    break;
    case PHY_INTERFACE_MODE_RGMII:
    case PHY_INTERFACE_MODE_RGMII_ID:
    case PHY_INTERFACE_MODE_RGMII_RXID:
    case PHY_INTERFACE_MODE_RGMII_TXID:
// use pll
    div = clk_get_rate(plat.stmmac_clk) / rgmii_clock(SPEED_1000);
    reg = FIELD_PREP(GMAC_PLLCLK_DIV_EN, 1) |
    FIELD_PREP(GMAC_PLLCLK_DIV_NUM, div);
    writel(0, dwmac.apb_base + GMAC_PLLCLK_DIV);
    writel(reg, dwmac.apb_base + GMAC_PLLCLK_DIV);
    writel(GMAC_GTXCLK_SEL_PLL, dwmac.apb_base + GMAC_GTXCLK_SEL);
    reg = GMAC_TX_CLK_EN | GMAC_TX_CLK_N_EN | GMAC_TX_CLK_OUT_EN |
    GMAC_RX_CLK_EN | GMAC_RX_CLK_N_EN;
    break;
    default:
    dev_err(dwmac.dev, "unsupported phy interface %s\n",
    phy_modes(plat.phy_interface));
    return -EINVAL;
    }
    writel(reg, dwmac.apb_base + GMAC_CLK_EN);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn thead_dwmac_init(dev: *mut device, priv: *mut c_void) -> c_int {
    static int thead_dwmac_init(struct device *dev, void *priv)
    {
    struct thead_dwmac *dwmac = priv;
    unsigned int reg;
    int ret;
    ret = thead_dwmac_set_phy_if(dwmac.plat);
    if (ret)
    return ret;
    ret = thead_dwmac_set_txclk_dir(dwmac.plat);
    if (ret)
    return ret;
    reg = readl(dwmac.apb_base + GMAC_RXCLK_DELAY_CTRL);
    reg &= ~(GMAC_RXCLK_DELAY);
    reg |= FIELD_PREP(GMAC_RXCLK_DELAY, 0);
    writel(reg, dwmac.apb_base + GMAC_RXCLK_DELAY_CTRL);
    reg = readl(dwmac.apb_base + GMAC_TXCLK_DELAY_CTRL);
    reg &= ~(GMAC_TXCLK_DELAY);
    reg |= FIELD_PREP(GMAC_TXCLK_DELAY, 0);
    writel(reg, dwmac.apb_base + GMAC_TXCLK_DELAY_CTRL);
    return thead_dwmac_enable_clk(dwmac.plat);
    }
#[no_mangle]
unsafe extern "C" fn thead_dwmac_probe(pdev: *mut platform_device) -> c_int {
    static int thead_dwmac_probe(struct platform_device *pdev)
    {
    struct stmmac_resources stmmac_res;
    struct plat_stmmacenet_data *plat;
    struct thead_dwmac *dwmac;
    struct clk *apb_clk;
    void __iomem *apb;
    int ret;
    ret = stmmac_get_platform_resources(pdev, &stmmac_res);
    if (ret)
    return dev_err_probe(&pdev.dev, ret,
    "failed to get resources\n");
    plat = devm_stmmac_probe_config_dt(pdev, stmmac_res.mac);
    if (IS_ERR(plat))
    return dev_err_probe(&pdev.dev, PTR_ERR(plat),
    "dt configuration failed\n");
//
// The APB clock is essential for accessing glue registers. However,
// old devicetrees don't describe it correctly. We continue to probe
// and emit a warning if it isn't present.
//
    apb_clk = devm_clk_get_enabled(&pdev.dev, "apb");
    if (PTR_ERR(apb_clk) == -ENOENT)
    dev_warn(&pdev.dev,
    "cannot get apb clock, link may break after speed changes\n");
#[no_mangle]
pub unsafe extern "C" fn if(_arg: IS_ERR(apb_clk)) -> else {
    else if (IS_ERR(apb_clk))
    return dev_err_probe(&pdev.dev, PTR_ERR(apb_clk),
    "failed to get apb clock\n");
    dwmac = devm_kzalloc(&pdev.dev, sizeof(*dwmac), GFP_KERNEL);
    if (!dwmac)
    return -ENOMEM;
    apb = devm_platform_ioremap_resource(pdev, 1);
    if (IS_ERR(apb))
    return dev_err_probe(&pdev.dev, PTR_ERR(apb),
    "failed to remap gmac apb registers\n");
    dwmac.dev = &pdev.dev;
    dwmac.plat = plat;
    dwmac.apb_base = apb;
    plat.bsp_priv = dwmac;
    plat.set_clk_tx_rate = thead_set_clk_tx_rate;
    plat.init = thead_dwmac_init;
    return devm_stmmac_pltfr_probe(pdev, plat, &stmmac_res);
    }
    static const struct of_device_id thead_dwmac_match[] = {
    { .compatible = "thead,th1520-gmac" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, thead_dwmac_match);
    static struct platform_driver thead_dwmac_driver = {
    .probe = thead_dwmac_probe,
    .driver = {
    .name = "thead-dwmac",
    .pm = &stmmac_pltfr_pm_ops,
    .of_match_table = thead_dwmac_match,
    },
    };
    module_platform_driver(thead_dwmac_driver);
    MODULE_AUTHOR("Jisheng Zhang <jszhang@kernel.org>");
    MODULE_AUTHOR("Drew Fustini <drew@pdp7.com>");
    MODULE_DESCRIPTION("T-HEAD DWMAC platform driver");
    MODULE_LICENSE("GPL");
