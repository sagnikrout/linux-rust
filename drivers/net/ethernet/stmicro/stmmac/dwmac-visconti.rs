//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/stmicro/stmmac/dwmac-visconti.c
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
// Toshiba Visconti Ethernet Support
//
// (C) Copyright 2020 TOSHIBA CORPORATION
// (C) Copyright 2020 Toshiba Electronic Devices & Storage Corporation
//

pub const REG_ETHER_CONTROL: c_uint = 0x52D4;

pub const REG_ETHER_CLOCK_SEL: c_uint = 0x52D0;

pub const ETHER_CLK_SEL_DIV_SEL_20: c_int = 0;

pub const ETHER_CLK_SEL_FREQ_SEL_2P5M: c_int = 0;
pub const ETHER_CLK_SEL_TX_CLK_EXT_SEL_IN: c_int = 0;

pub const ETHER_CLK_SEL_RX_CLK_EXT_SEL_IN: c_int = 0;

pub const ETHER_CLK_SEL_TX_CLK_O_TX_I: c_int = 0;

pub const ETHER_CLK_SEL_RMII_CLK_SEL_IN: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct visconti_eth {
    pub reg: *mut void __iomem,
    pub phy_ref_clk: *mut clk,
    pub dev: *mut device,
}

    static int visconti_eth_set_clk_tx_rate(void *bsp_priv, struct clk *clk_tx_i,
    phy_interface_t interface, int speed)
    {
    struct visconti_eth *dwmac = bsp_priv;
    unsigned long clk_sel, val;
    if (phy_interface_mode_is_rgmii(interface)) {
    switch (speed) {
    case SPEED_1000:
    clk_sel = ETHER_CLK_SEL_FREQ_SEL_125M;
    break;
    case SPEED_100:
    clk_sel = ETHER_CLK_SEL_FREQ_SEL_25M;
    break;
    case SPEED_10:
    clk_sel = ETHER_CLK_SEL_FREQ_SEL_2P5M;
    break;
    default:
    return -EINVAL;
    }
// Stop internal clock
    val = readl(dwmac.reg + REG_ETHER_CLOCK_SEL);
    val &= ~(ETHER_CLK_SEL_RMII_CLK_EN |
    ETHER_CLK_SEL_RX_TX_CLK_EN);
    val |= ETHER_CLK_SEL_TX_O_E_N_IN;
    writel(val, dwmac.reg + REG_ETHER_CLOCK_SEL);
// Set Clock-Mux, Start clock, Set TX_O direction
    val = clk_sel | ETHER_CLK_SEL_RX_CLK_EXT_SEL_RXC;
    writel(val, dwmac.reg + REG_ETHER_CLOCK_SEL);
    val |= ETHER_CLK_SEL_RX_TX_CLK_EN;
    writel(val, dwmac.reg + REG_ETHER_CLOCK_SEL);
    val &= ~ETHER_CLK_SEL_TX_O_E_N_IN;
    writel(val, dwmac.reg + REG_ETHER_CLOCK_SEL);
    } else if (interface == PHY_INTERFACE_MODE_RMII) {
    switch (speed) {
    case SPEED_100:
    clk_sel = ETHER_CLK_SEL_DIV_SEL_2;
    break;
    case SPEED_10:
    clk_sel = ETHER_CLK_SEL_DIV_SEL_20;
    break;
    default:
    return -EINVAL;
    }
// Stop internal clock
    val = readl(dwmac.reg + REG_ETHER_CLOCK_SEL);
    val &= ~(ETHER_CLK_SEL_RMII_CLK_EN |
    ETHER_CLK_SEL_RX_TX_CLK_EN);
    val |= ETHER_CLK_SEL_TX_O_E_N_IN;
    writel(val, dwmac.reg + REG_ETHER_CLOCK_SEL);
// Set Clock-Mux, Start clock, Set TX_O direction
    val = clk_sel | ETHER_CLK_SEL_RX_CLK_EXT_SEL_DIV |
    ETHER_CLK_SEL_TX_CLK_EXT_SEL_DIV |
    ETHER_CLK_SEL_TX_O_E_N_IN |
    ETHER_CLK_SEL_RMII_CLK_SEL_RX_C;
    writel(val, dwmac.reg + REG_ETHER_CLOCK_SEL);
    val |= ETHER_CLK_SEL_RMII_CLK_RST;
    writel(val, dwmac.reg + REG_ETHER_CLOCK_SEL);
    val |= ETHER_CLK_SEL_RMII_CLK_EN | ETHER_CLK_SEL_RX_TX_CLK_EN;
    writel(val, dwmac.reg + REG_ETHER_CLOCK_SEL);
    } else {
// Stop internal clock
    val = readl(dwmac.reg + REG_ETHER_CLOCK_SEL);
    val &= ~(ETHER_CLK_SEL_RMII_CLK_EN |
    ETHER_CLK_SEL_RX_TX_CLK_EN);
    val |= ETHER_CLK_SEL_TX_O_E_N_IN;
    writel(val, dwmac.reg + REG_ETHER_CLOCK_SEL);
// Set Clock-Mux, Start clock, Set TX_O direction
    val = ETHER_CLK_SEL_RX_CLK_EXT_SEL_RXC |
    ETHER_CLK_SEL_TX_CLK_EXT_SEL_TXC |
    ETHER_CLK_SEL_TX_O_E_N_IN;
    writel(val, dwmac.reg + REG_ETHER_CLOCK_SEL);
    val |= ETHER_CLK_SEL_RX_TX_CLK_EN;
    writel(val, dwmac.reg + REG_ETHER_CLOCK_SEL);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn visconti_eth_init_hw(pdev: *mut platform_device, plat_dat: *mut plat_stmmacenet_data) -> c_int {
    static int visconti_eth_init_hw(struct platform_device *pdev, struct plat_stmmacenet_data *plat_dat)
    {
    struct visconti_eth *dwmac = plat_dat.bsp_priv;
    unsigned int clk_sel_val;
    int phy_intf_sel;
    phy_intf_sel = stmmac_get_phy_intf_sel(plat_dat.phy_interface);
    if (phy_intf_sel != PHY_INTF_SEL_GMII_MII &&
    phy_intf_sel != PHY_INTF_SEL_RGMII &&
    phy_intf_sel != PHY_INTF_SEL_RMII) {
    dev_err(&pdev.dev, "Unsupported phy-mode (%d)\n", plat_dat.phy_interface);
    return -EOPNOTSUPP;
    }
    writel(phy_intf_sel, dwmac.reg + REG_ETHER_CONTROL);
// Enable TX/RX clock
    clk_sel_val = ETHER_CLK_SEL_FREQ_SEL_125M;
    writel(clk_sel_val, dwmac.reg + REG_ETHER_CLOCK_SEL);
    writel((clk_sel_val | ETHER_CLK_SEL_RMII_CLK_EN | ETHER_CLK_SEL_RX_TX_CLK_EN),
    dwmac.reg + REG_ETHER_CLOCK_SEL);
// release internal-reset
    phy_intf_sel |= ETHER_ETH_CONTROL_RESET;
    writel(phy_intf_sel, dwmac.reg + REG_ETHER_CONTROL);
    return 0;
    }
    static int visconti_eth_clock_probe(struct platform_device *pdev,
    struct plat_stmmacenet_data *plat_dat)
    {
    struct visconti_eth *dwmac = plat_dat.bsp_priv;
    int err;
    dwmac.phy_ref_clk = devm_clk_get(&pdev.dev, "phy_ref_clk");
    if (IS_ERR(dwmac.phy_ref_clk))
    return dev_err_probe(&pdev.dev, PTR_ERR(dwmac.phy_ref_clk),
    "phy_ref_clk clock not found.\n");
    err = clk_prepare_enable(dwmac.phy_ref_clk);
    if (err < 0) {
    dev_err(&pdev.dev, "failed to enable phy_ref clock: %d\n", err);
    return err;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn visconti_eth_clock_remove(pdev: *mut platform_device) {
    static void visconti_eth_clock_remove(struct platform_device *pdev)
    {
    struct visconti_eth *dwmac = get_stmmac_bsp_priv(&pdev.dev);
    struct net_device *ndev = platform_get_drvdata(pdev);
    struct stmmac_priv *priv = netdev_priv(ndev);
    clk_disable_unprepare(dwmac.phy_ref_clk);
    clk_disable_unprepare(priv.plat.stmmac_clk);
    }
#[no_mangle]
unsafe extern "C" fn visconti_eth_dwmac_probe(pdev: *mut platform_device) -> c_int {
    static int visconti_eth_dwmac_probe(struct platform_device *pdev)
    {
    struct plat_stmmacenet_data *plat_dat;
    struct stmmac_resources stmmac_res;
    struct visconti_eth *dwmac;
    int ret;
    ret = stmmac_get_platform_resources(pdev, &stmmac_res);
    if (ret)
    return ret;
    plat_dat = devm_stmmac_probe_config_dt(pdev, stmmac_res.mac);
    if (IS_ERR(plat_dat))
    return PTR_ERR(plat_dat);
    dwmac = devm_kzalloc(&pdev.dev, sizeof(*dwmac), GFP_KERNEL);
    if (!dwmac)
    return -ENOMEM;
    dwmac.reg = stmmac_res.addr;
    dwmac.dev = &pdev.dev;
    plat_dat.bsp_priv = dwmac;
    plat_dat.set_clk_tx_rate = visconti_eth_set_clk_tx_rate;
    ret = visconti_eth_clock_probe(pdev, plat_dat);
    if (ret)
    return ret;
    visconti_eth_init_hw(pdev, plat_dat);
    plat_dat.dma_cfg.aal = 1;
    ret = stmmac_dvr_probe(&pdev.dev, plat_dat, &stmmac_res);
    if (ret)
    goto remove;
    return ret;
    remove:
    visconti_eth_clock_remove(pdev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn visconti_eth_dwmac_remove(pdev: *mut platform_device) {
    static void visconti_eth_dwmac_remove(struct platform_device *pdev)
    {
    stmmac_pltfr_remove(pdev);
    visconti_eth_clock_remove(pdev);
    }
    static const struct of_device_id visconti_eth_dwmac_match[] = {
    { .compatible = "toshiba,visconti-dwmac" },
    { }
    };
    MODULE_DEVICE_TABLE(of, visconti_eth_dwmac_match);
    static struct platform_driver visconti_eth_dwmac_driver = {
    .probe  = visconti_eth_dwmac_probe,
    .remove = visconti_eth_dwmac_remove,
    .driver = {
    .name           = "visconti-eth-dwmac",
    .of_match_table = visconti_eth_dwmac_match,
    },
    };
    module_platform_driver(visconti_eth_dwmac_driver);
    MODULE_AUTHOR("Toshiba");
    MODULE_DESCRIPTION("Toshiba Visconti Ethernet DWMAC glue driver");
    MODULE_AUTHOR("Nobuhiro Iwamatsu <nobuhiro1.iwamatsu@toshiba.co.jp");
    MODULE_LICENSE("GPL v2");
