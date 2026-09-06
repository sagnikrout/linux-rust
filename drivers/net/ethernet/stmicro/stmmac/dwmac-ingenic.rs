//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/stmicro/stmmac/dwmac-ingenic.c
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
// dwmac-ingenic.c - Ingenic SoCs DWMAC specific glue layer
//
// Copyright (c) 2021 周琰杰 (Zhou Yanjie) <zhouyanjie@wanyeetech.com>
//

pub const MACPHYC_TXCLK_SEL_OUTPUT: c_uint = 0x1;
pub const MACPHYC_TXCLK_SEL_INPUT: c_uint = 0x0;

pub const MACPHYC_MODE_SEL_RMII: c_uint = 0x0;

pub const MACPHYC_TX_SEL_ORIGIN: c_uint = 0x0;
pub const MACPHYC_TX_SEL_DELAY: c_uint = 0x1;

pub const MACPHYC_RX_SEL_ORIGIN: c_uint = 0x0;
pub const MACPHYC_RX_SEL_DELAY: c_uint = 0x1;

pub const MACPHYC_TX_DELAY_PS_MAX: c_int = 2496;
pub const MACPHYC_TX_DELAY_PS_MIN: c_int = 20;
pub const MACPHYC_RX_DELAY_PS_MAX: c_int = 2496;
pub const MACPHYC_RX_DELAY_PS_MIN: c_int = 20;
    enum ingenic_mac_version {
    ID_JZ4775,
    ID_X1000,
    ID_X1600,
    ID_X1830,
    ID_X2000,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ingenic_mac {
    pub soc_info: *const ingenic_soc_info,
    pub plat_dat: *mut plat_stmmacenet_data,
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub rx_delay: c_int,
    pub tx_delay: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ingenic_soc_info {
    pub version: enum ingenic_mac_version,
    pub mask: u32,
    pub phy_intf_sel): *mut *mut *mut int (set_mode)(struct ingenic_mac mac, u8,
    pub valid_phy_intf_sel: u8,
}

#[no_mangle]
unsafe extern "C" fn jz4775_mac_set_mode(mac: *mut ingenic_mac, phy_intf_sel: u8) -> c_int {
    static int jz4775_mac_set_mode(struct ingenic_mac *mac, u8 phy_intf_sel)
    {
    unsigned int val;
    val = FIELD_PREP(MACPHYC_PHY_INFT_MASK, phy_intf_sel) |
    FIELD_PREP(MACPHYC_TXCLK_SEL_MASK, MACPHYC_TXCLK_SEL_INPUT);
// Update MAC PHY control register
    return regmap_update_bits(mac.regmap, 0, mac.soc_info.mask, val);
    }
#[no_mangle]
unsafe extern "C" fn x1000_mac_set_mode(mac: *mut ingenic_mac, phy_intf_sel: u8) -> c_int {
    static int x1000_mac_set_mode(struct ingenic_mac *mac, u8 phy_intf_sel)
    {
// Update MAC PHY control register
    return regmap_update_bits(mac.regmap, 0, mac.soc_info.mask, 0);
    }
#[no_mangle]
unsafe extern "C" fn x1600_mac_set_mode(mac: *mut ingenic_mac, phy_intf_sel: u8) -> c_int {
    static int x1600_mac_set_mode(struct ingenic_mac *mac, u8 phy_intf_sel)
    {
    unsigned int val;
    val = FIELD_PREP(MACPHYC_PHY_INFT_MASK, phy_intf_sel);
// Update MAC PHY control register
    return regmap_update_bits(mac.regmap, 0, mac.soc_info.mask, val);
    }
#[no_mangle]
unsafe extern "C" fn x1830_mac_set_mode(mac: *mut ingenic_mac, phy_intf_sel: u8) -> c_int {
    static int x1830_mac_set_mode(struct ingenic_mac *mac, u8 phy_intf_sel)
    {
    unsigned int val;
    val = FIELD_PREP(MACPHYC_MODE_SEL_MASK, MACPHYC_MODE_SEL_RMII) |
    FIELD_PREP(MACPHYC_PHY_INFT_MASK, phy_intf_sel);
// Update MAC PHY control register
    return regmap_update_bits(mac.regmap, 0, mac.soc_info.mask, val);
    }
#[no_mangle]
unsafe extern "C" fn x2000_mac_set_mode(mac: *mut ingenic_mac, phy_intf_sel: u8) -> c_int {
    static int x2000_mac_set_mode(struct ingenic_mac *mac, u8 phy_intf_sel)
    {
    unsigned int val;
    val = FIELD_PREP(MACPHYC_PHY_INFT_MASK, phy_intf_sel);
    if (phy_intf_sel == PHY_INTF_SEL_RMII) {
    val |= FIELD_PREP(MACPHYC_TX_SEL_MASK, MACPHYC_TX_SEL_ORIGIN) |
    FIELD_PREP(MACPHYC_RX_SEL_MASK, MACPHYC_RX_SEL_ORIGIN);
    } else if (phy_intf_sel == PHY_INTF_SEL_RGMII) {
    if (mac.tx_delay == 0)
    val |= FIELD_PREP(MACPHYC_TX_SEL_MASK, MACPHYC_TX_SEL_ORIGIN);
    else
    val |= FIELD_PREP(MACPHYC_TX_SEL_MASK, MACPHYC_TX_SEL_DELAY) |
    FIELD_PREP(MACPHYC_TX_DELAY_MASK, (mac.tx_delay + 9750) / 19500 - 1);
    if (mac.rx_delay == 0)
    val |= FIELD_PREP(MACPHYC_RX_SEL_MASK, MACPHYC_RX_SEL_ORIGIN);
    else
    val |= FIELD_PREP(MACPHYC_RX_SEL_MASK, MACPHYC_RX_SEL_DELAY) |
    FIELD_PREP(MACPHYC_RX_DELAY_MASK, (mac.rx_delay + 9750) / 19500 - 1);
    }
// Update MAC PHY control register
    return regmap_update_bits(mac.regmap, 0, mac.soc_info.mask, val);
    }
#[no_mangle]
unsafe extern "C" fn ingenic_set_phy_intf_sel(bsp_priv: *mut c_void, phy_intf_sel: u8) -> c_int {
    static int ingenic_set_phy_intf_sel(void *bsp_priv, u8 phy_intf_sel)
    {
    struct ingenic_mac *mac = bsp_priv;
    if (!mac.soc_info.set_mode)
    return 0;
    if (phy_intf_sel >= BITS_PER_BYTE ||
    ~mac.soc_info.valid_phy_intf_sel & BIT(phy_intf_sel))
    return -EINVAL;
    dev_dbg(mac.dev, "MAC PHY control register: interface %s\n",
    phy_modes(mac.plat_dat.phy_interface));
    return mac.soc_info.set_mode(mac, phy_intf_sel);
    }
#[no_mangle]
unsafe extern "C" fn ingenic_mac_probe(pdev: *mut platform_device) -> c_int {
    static int ingenic_mac_probe(struct platform_device *pdev)
    {
    struct plat_stmmacenet_data *plat_dat;
    struct stmmac_resources stmmac_res;
    struct ingenic_mac *mac;
    const struct ingenic_soc_info *data;
    u32 tx_delay_ps, rx_delay_ps;
    int ret;
    ret = stmmac_get_platform_resources(pdev, &stmmac_res);
    if (ret)
    return ret;
    plat_dat = devm_stmmac_probe_config_dt(pdev, stmmac_res.mac);
    if (IS_ERR(plat_dat))
    return PTR_ERR(plat_dat);
    mac = devm_kzalloc(&pdev.dev, sizeof(*mac), GFP_KERNEL);
    if (!mac)
    return -ENOMEM;
    data = of_device_get_match_data(&pdev.dev);
    if (!data) {
    dev_err(&pdev.dev, "No of match data provided\n");
    return -EINVAL;
    }
// Get MAC PHY control register
    mac.regmap = syscon_regmap_lookup_by_phandle(pdev.dev.of_node, "mode-reg");
    if (IS_ERR(mac.regmap)) {
    dev_err(&pdev.dev, "%s: Failed to get syscon regmap\n", __func__);
    return PTR_ERR(mac.regmap);
    }
    if (!of_property_read_u32(pdev.dev.of_node, "tx-clk-delay-ps", &tx_delay_ps)) {
    if (tx_delay_ps >= MACPHYC_TX_DELAY_PS_MIN &&
    tx_delay_ps <= MACPHYC_TX_DELAY_PS_MAX) {
    mac.tx_delay = tx_delay_ps * 1000;
    } else {
    dev_err(&pdev.dev, "Invalid TX clock delay: %dps\n", tx_delay_ps);
    return -EINVAL;
    }
    }
    if (!of_property_read_u32(pdev.dev.of_node, "rx-clk-delay-ps", &rx_delay_ps)) {
    if (rx_delay_ps >= MACPHYC_RX_DELAY_PS_MIN &&
    rx_delay_ps <= MACPHYC_RX_DELAY_PS_MAX) {
    mac.rx_delay = rx_delay_ps * 1000;
    } else {
    dev_err(&pdev.dev, "Invalid RX clock delay: %dps\n", rx_delay_ps);
    return -EINVAL;
    }
    }
    mac.soc_info = data;
    mac.dev = &pdev.dev;
    mac.plat_dat = plat_dat;
    plat_dat.bsp_priv = mac;
    plat_dat.set_phy_intf_sel = ingenic_set_phy_intf_sel;
    return devm_stmmac_pltfr_probe(pdev, plat_dat, &stmmac_res);
    }
    static struct ingenic_soc_info jz4775_soc_info = {
    .version = ID_JZ4775,
    .mask = MACPHYC_TXCLK_SEL_MASK | MACPHYC_SOFT_RST_MASK | MACPHYC_PHY_INFT_MASK,
    .set_mode = jz4775_mac_set_mode,
    .valid_phy_intf_sel = BIT(PHY_INTF_SEL_GMII_MII) |
    BIT(PHY_INTF_SEL_RGMII) |
    BIT(PHY_INTF_SEL_RMII),
    };
    static struct ingenic_soc_info x1000_soc_info = {
    .version = ID_X1000,
    .mask = MACPHYC_SOFT_RST_MASK,
    .set_mode = x1000_mac_set_mode,
    .valid_phy_intf_sel = BIT(PHY_INTF_SEL_RMII),
    };
    static struct ingenic_soc_info x1600_soc_info = {
    .version = ID_X1600,
    .mask = MACPHYC_SOFT_RST_MASK | MACPHYC_PHY_INFT_MASK,
    .set_mode = x1600_mac_set_mode,
    .valid_phy_intf_sel = BIT(PHY_INTF_SEL_RMII),
    };
    static struct ingenic_soc_info x1830_soc_info = {
    .version = ID_X1830,
    .mask = MACPHYC_MODE_SEL_MASK | MACPHYC_SOFT_RST_MASK | MACPHYC_PHY_INFT_MASK,
    .set_mode = x1830_mac_set_mode,
    .valid_phy_intf_sel = BIT(PHY_INTF_SEL_RMII),
    };
    static struct ingenic_soc_info x2000_soc_info = {
    .version = ID_X2000,
    .mask = MACPHYC_TX_SEL_MASK | MACPHYC_TX_DELAY_MASK | MACPHYC_RX_SEL_MASK |
    MACPHYC_RX_DELAY_MASK | MACPHYC_SOFT_RST_MASK | MACPHYC_PHY_INFT_MASK,
    .set_mode = x2000_mac_set_mode,
    .valid_phy_intf_sel = BIT(PHY_INTF_SEL_RGMII) |
    BIT(PHY_INTF_SEL_RMII),
    };
    static const struct of_device_id ingenic_mac_of_matches[] = {
    { .compatible = "ingenic,jz4775-mac", .data = &jz4775_soc_info },
    { .compatible = "ingenic,x1000-mac", .data = &x1000_soc_info },
    { .compatible = "ingenic,x1600-mac", .data = &x1600_soc_info },
    { .compatible = "ingenic,x1830-mac", .data = &x1830_soc_info },
    { .compatible = "ingenic,x2000-mac", .data = &x2000_soc_info },
    { }
    };
    MODULE_DEVICE_TABLE(of, ingenic_mac_of_matches);
    static struct platform_driver ingenic_mac_driver = {
    .probe		= ingenic_mac_probe,
    .driver		= {
    .name	= "ingenic-mac",
    .pm		= &stmmac_pltfr_pm_ops,
    .of_match_table = ingenic_mac_of_matches,
    },
    };
    module_platform_driver(ingenic_mac_driver);
    MODULE_AUTHOR("周琰杰 (Zhou Yanjie) <zhouyanjie@wanyeetech.com>");
    MODULE_DESCRIPTION("Ingenic SoCs DWMAC specific glue layer");
    MODULE_LICENSE("GPL v2");
