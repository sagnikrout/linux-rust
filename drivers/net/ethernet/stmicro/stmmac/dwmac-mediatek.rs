//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/stmicro/stmmac/dwmac-mediatek.c
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
// Copyright (c) 2018 MediaTek Inc.
//

// Peri Configuration register for mt2712
pub const PERI_ETH_PHY_INTF_SEL: c_uint = 0x418;

pub const PERI_ETH_DLY: c_uint = 0x428;

pub const PERI_ETH_DLY_FINE: c_uint = 0x800;

// Peri Configuration register for mt8195
pub const MT8195_PERI_ETH_CTRL0: c_uint = 0xFD0;

pub const MT8195_PERI_ETH_CTRL1: c_uint = 0xFD4;

pub const MT8195_PERI_ETH_CTRL2: c_uint = 0xFD8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mac_delay_struct {
    pub tx_delay: u32,
    pub rx_delay: u32,
    pub tx_inv: bool,
    pub rx_inv: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mediatek_dwmac_plat_data {
    pub variant: *const mediatek_dwmac_variant,
    pub mac_delay: mac_delay_struct,
    pub rmii_internal_clk: *mut clk,
    pub clks: *mut clk_bulk_data,
    pub peri_regmap: *mut regmap,
    pub np: *mut device_node,
    pub dev: *mut device,
    pub phy_mode: phy_interface_t,
    pub rmii_clk_from_mac: bool,
    pub rmii_rxc: bool,
    pub mac_wol: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mediatek_dwmac_variant {
    int (*dwmac_set_phy_interface)(struct mediatek_dwmac_plat_data *plat,
    pub phy_intf_sel): u8,
    pub plat): *mut *mut int (dwmac_set_delay)(struct mediatek_dwmac_plat_data,
// clock ids to be requested
    pub clk_list: *const *const c_char,
    pub num_clks: c_int,
    pub rx_delay_max: u32,
    pub tx_delay_max: u32,
    pub dma_bit_mask: u8,
}

// list of clocks required for mac
    static const char * const mt2712_dwmac_clk_l[] = {
    "axi", "apb", "mac_main", "ptp_ref"
    };
    static const char * const mt8195_dwmac_clk_l[] = {
    "axi", "apb", "mac_cg", "mac_main", "ptp_ref"
    };
    static int mt2712_set_interface(struct mediatek_dwmac_plat_data *plat,
    u8 phy_intf_sel)
    {
    let mut intf_val: u32 = phy_intf_sel;
    if (phy_intf_sel == PHY_INTF_SEL_RMII) {
    if (plat.rmii_clk_from_mac)
    intf_val |= RMII_CLK_SRC_INTERNAL;
    if (plat.rmii_rxc)
    intf_val |= RMII_CLK_SRC_RXC;
    }
    regmap_write(plat.peri_regmap, PERI_ETH_PHY_INTF_SEL, intf_val);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mt2712_delay_ps2stage(plat: *mut mediatek_dwmac_plat_data) {
    static void mt2712_delay_ps2stage(struct mediatek_dwmac_plat_data *plat)
    {
    struct mac_delay_struct *mac_delay = &plat.mac_delay;
    switch (plat.phy_mode) {
    case PHY_INTERFACE_MODE_MII:
    case PHY_INTERFACE_MODE_RMII:
// 550ps per stage for MII/RMII
    mac_delay.tx_delay /= 550;
    mac_delay.rx_delay /= 550;
    break;
    case PHY_INTERFACE_MODE_RGMII:
    case PHY_INTERFACE_MODE_RGMII_TXID:
    case PHY_INTERFACE_MODE_RGMII_RXID:
    case PHY_INTERFACE_MODE_RGMII_ID:
// 170ps per stage for RGMII
    mac_delay.tx_delay /= 170;
    mac_delay.rx_delay /= 170;
    break;
    default:
    dev_err(plat.dev, "phy interface not supported\n");
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn mt2712_delay_stage2ps(plat: *mut mediatek_dwmac_plat_data) {
    static void mt2712_delay_stage2ps(struct mediatek_dwmac_plat_data *plat)
    {
    struct mac_delay_struct *mac_delay = &plat.mac_delay;
    switch (plat.phy_mode) {
    case PHY_INTERFACE_MODE_MII:
    case PHY_INTERFACE_MODE_RMII:
// 550ps per stage for MII/RMII
    mac_delay.tx_delay *= 550;
    mac_delay.rx_delay *= 550;
    break;
    case PHY_INTERFACE_MODE_RGMII:
    case PHY_INTERFACE_MODE_RGMII_TXID:
    case PHY_INTERFACE_MODE_RGMII_RXID:
    case PHY_INTERFACE_MODE_RGMII_ID:
// 170ps per stage for RGMII
    mac_delay.tx_delay *= 170;
    mac_delay.rx_delay *= 170;
    break;
    default:
    dev_err(plat.dev, "phy interface not supported\n");
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn mt2712_set_delay(plat: *mut mediatek_dwmac_plat_data) -> c_int {
    static int mt2712_set_delay(struct mediatek_dwmac_plat_data *plat)
    {
    struct mac_delay_struct *mac_delay = &plat.mac_delay;
    let mut delay_val: u32 = 0, fine_val = 0;
    mt2712_delay_ps2stage(plat);
    switch (plat.phy_mode) {
    case PHY_INTERFACE_MODE_MII:
    delay_val |= FIELD_PREP(ETH_DLY_TXC_ENABLE, !!mac_delay.tx_delay);
    delay_val |= FIELD_PREP(ETH_DLY_TXC_STAGES, mac_delay.tx_delay);
    delay_val |= FIELD_PREP(ETH_DLY_TXC_INV, mac_delay.tx_inv);
    delay_val |= FIELD_PREP(ETH_DLY_RXC_ENABLE, !!mac_delay.rx_delay);
    delay_val |= FIELD_PREP(ETH_DLY_RXC_STAGES, mac_delay.rx_delay);
    delay_val |= FIELD_PREP(ETH_DLY_RXC_INV, mac_delay.rx_inv);
    break;
    case PHY_INTERFACE_MODE_RMII:
    if (plat.rmii_clk_from_mac) {
// case 1: mac provides the rmii reference clock,
// and the clock output to TXC pin.
// The egress timing can be adjusted by GTXC delay macro circuit.
// The ingress timing can be adjusted by TXC delay macro circuit.
//
    delay_val |= FIELD_PREP(ETH_DLY_TXC_ENABLE, !!mac_delay.rx_delay);
    delay_val |= FIELD_PREP(ETH_DLY_TXC_STAGES, mac_delay.rx_delay);
    delay_val |= FIELD_PREP(ETH_DLY_TXC_INV, mac_delay.rx_inv);
    delay_val |= FIELD_PREP(ETH_DLY_GTXC_ENABLE, !!mac_delay.tx_delay);
    delay_val |= FIELD_PREP(ETH_DLY_GTXC_STAGES, mac_delay.tx_delay);
    delay_val |= FIELD_PREP(ETH_DLY_GTXC_INV, mac_delay.tx_inv);
    } else {
// case 2: the rmii reference clock is from external phy,
// and the property "rmii_rxc" indicates which pin(TXC/RXC)
// the reference clk is connected to. The reference clock is a
// received signal, so rx_delay/rx_inv are used to indicate
// the reference clock timing adjustment
//
    if (plat.rmii_rxc) {
// the rmii reference clock from outside is connected
// to RXC pin, the reference clock will be adjusted
// by RXC delay macro circuit.
//
    delay_val |= FIELD_PREP(ETH_DLY_RXC_ENABLE, !!mac_delay.rx_delay);
    delay_val |= FIELD_PREP(ETH_DLY_RXC_STAGES, mac_delay.rx_delay);
    delay_val |= FIELD_PREP(ETH_DLY_RXC_INV, mac_delay.rx_inv);
    } else {
// the rmii reference clock from outside is connected
// to TXC pin, the reference clock will be adjusted
// by TXC delay macro circuit.
//
    delay_val |= FIELD_PREP(ETH_DLY_TXC_ENABLE, !!mac_delay.rx_delay);
    delay_val |= FIELD_PREP(ETH_DLY_TXC_STAGES, mac_delay.rx_delay);
    delay_val |= FIELD_PREP(ETH_DLY_TXC_INV, mac_delay.rx_inv);
    }
// tx_inv will inverse the tx clock inside mac relateive to
// reference clock from external phy,
// and this bit is located in the same register with fine-tune
//
    if (mac_delay.tx_inv)
    fine_val = ETH_RMII_DLY_TX_INV;
    }
    break;
    case PHY_INTERFACE_MODE_RGMII:
    case PHY_INTERFACE_MODE_RGMII_TXID:
    case PHY_INTERFACE_MODE_RGMII_RXID:
    case PHY_INTERFACE_MODE_RGMII_ID:
    fine_val = ETH_FINE_DLY_GTXC | ETH_FINE_DLY_RXC;
    delay_val |= FIELD_PREP(ETH_DLY_GTXC_ENABLE, !!mac_delay.tx_delay);
    delay_val |= FIELD_PREP(ETH_DLY_GTXC_STAGES, mac_delay.tx_delay);
    delay_val |= FIELD_PREP(ETH_DLY_GTXC_INV, mac_delay.tx_inv);
    delay_val |= FIELD_PREP(ETH_DLY_RXC_ENABLE, !!mac_delay.rx_delay);
    delay_val |= FIELD_PREP(ETH_DLY_RXC_STAGES, mac_delay.rx_delay);
    delay_val |= FIELD_PREP(ETH_DLY_RXC_INV, mac_delay.rx_inv);
    break;
    default:
    dev_err(plat.dev, "phy interface not supported\n");
    return -EINVAL;
    }
    regmap_write(plat.peri_regmap, PERI_ETH_DLY, delay_val);
    regmap_write(plat.peri_regmap, PERI_ETH_DLY_FINE, fine_val);
    mt2712_delay_stage2ps(plat);
    return 0;
    }
    static const struct mediatek_dwmac_variant mt2712_gmac_variant = {
    .dwmac_set_phy_interface = mt2712_set_interface,
    .dwmac_set_delay = mt2712_set_delay,
    .clk_list = mt2712_dwmac_clk_l,
    .num_clks = ARRAY_SIZE(mt2712_dwmac_clk_l),
    .rx_delay_max = 17600,
    .tx_delay_max = 17600,
    .dma_bit_mask = 33,
    };
    static int mt8195_set_interface(struct mediatek_dwmac_plat_data *plat,
    u8 phy_intf_sel)
    {
    let mut intf_val: u32 = FIELD_PREP(MT8195_ETH_INTF_SEL, phy_intf_sel);
    if (phy_intf_sel == PHY_INTF_SEL_RMII) {
    if (plat.rmii_clk_from_mac)
    intf_val |= MT8195_RMII_CLK_SRC_INTERNAL;
    if (plat.rmii_rxc)
    intf_val |= MT8195_RMII_CLK_SRC_RXC;
    }
// MT8195 only support external PHY
    intf_val |= MT8195_EXT_PHY_MODE;
    regmap_write(plat.peri_regmap, MT8195_PERI_ETH_CTRL0, intf_val);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mt8195_delay_ps2stage(plat: *mut mediatek_dwmac_plat_data) {
    static void mt8195_delay_ps2stage(struct mediatek_dwmac_plat_data *plat)
    {
    struct mac_delay_struct *mac_delay = &plat.mac_delay;
// 290ps per stage
    mac_delay.tx_delay /= 290;
    mac_delay.rx_delay /= 290;
    }
#[no_mangle]
unsafe extern "C" fn mt8195_delay_stage2ps(plat: *mut mediatek_dwmac_plat_data) {
    static void mt8195_delay_stage2ps(struct mediatek_dwmac_plat_data *plat)
    {
    struct mac_delay_struct *mac_delay = &plat.mac_delay;
// 290ps per stage
    mac_delay.tx_delay *= 290;
    mac_delay.rx_delay *= 290;
    }
#[no_mangle]
unsafe extern "C" fn mt8195_set_delay(plat: *mut mediatek_dwmac_plat_data) -> c_int {
    static int mt8195_set_delay(struct mediatek_dwmac_plat_data *plat)
    {
    struct mac_delay_struct *mac_delay = &plat.mac_delay;
    let mut gtxc_delay_val: u32 = 0, delay_val = 0, rmii_delay_val = 0;
    mt8195_delay_ps2stage(plat);
    switch (plat.phy_mode) {
    case PHY_INTERFACE_MODE_MII:
    delay_val |= FIELD_PREP(MT8195_DLY_TXC_ENABLE, !!mac_delay.tx_delay);
    delay_val |= FIELD_PREP(MT8195_DLY_TXC_STAGES, mac_delay.tx_delay);
    delay_val |= FIELD_PREP(MT8195_DLY_TXC_INV, mac_delay.tx_inv);
    delay_val |= FIELD_PREP(MT8195_DLY_RXC_ENABLE, !!mac_delay.rx_delay);
    delay_val |= FIELD_PREP(MT8195_DLY_RXC_STAGES, mac_delay.rx_delay);
    delay_val |= FIELD_PREP(MT8195_DLY_RXC_INV, mac_delay.rx_inv);
    break;
    case PHY_INTERFACE_MODE_RMII:
    if (plat.rmii_clk_from_mac) {
// case 1: mac provides the rmii reference clock,
// and the clock output to TXC pin.
// The egress timing can be adjusted by RMII_TXC delay macro circuit.
// The ingress timing can be adjusted by RMII_RXC delay macro circuit.
//
    rmii_delay_val |= FIELD_PREP(MT8195_DLY_RMII_TXC_ENABLE,
    !!mac_delay.tx_delay);
    rmii_delay_val |= FIELD_PREP(MT8195_DLY_RMII_TXC_STAGES,
    mac_delay.tx_delay);
    rmii_delay_val |= FIELD_PREP(MT8195_DLY_RMII_TXC_INV,
    mac_delay.tx_inv);
    rmii_delay_val |= FIELD_PREP(MT8195_DLY_RMII_RXC_ENABLE,
    !!mac_delay.rx_delay);
    rmii_delay_val |= FIELD_PREP(MT8195_DLY_RMII_RXC_STAGES,
    mac_delay.rx_delay);
    rmii_delay_val |= FIELD_PREP(MT8195_DLY_RMII_RXC_INV,
    mac_delay.rx_inv);
    } else {
// case 2: the rmii reference clock is from external phy,
// and the property "rmii_rxc" indicates which pin(TXC/RXC)
// the reference clk is connected to. The reference clock is a
// received signal, so rx_delay/rx_inv are used to indicate
// the reference clock timing adjustment
//
    if (plat.rmii_rxc) {
// the rmii reference clock from outside is connected
// to RXC pin, the reference clock will be adjusted
// by RXC delay macro circuit.
//
    delay_val |= FIELD_PREP(MT8195_DLY_RXC_ENABLE,
    !!mac_delay.rx_delay);
    delay_val |= FIELD_PREP(MT8195_DLY_RXC_STAGES,
    mac_delay.rx_delay);
    delay_val |= FIELD_PREP(MT8195_DLY_RXC_INV,
    mac_delay.rx_inv);
    } else {
// the rmii reference clock from outside is connected
// to TXC pin, the reference clock will be adjusted
// by TXC delay macro circuit.
//
    delay_val |= FIELD_PREP(MT8195_DLY_TXC_ENABLE,
    !!mac_delay.rx_delay);
    delay_val |= FIELD_PREP(MT8195_DLY_TXC_STAGES,
    mac_delay.rx_delay);
    delay_val |= FIELD_PREP(MT8195_DLY_TXC_INV,
    mac_delay.rx_inv);
    }
    }
    break;
    case PHY_INTERFACE_MODE_RGMII:
    case PHY_INTERFACE_MODE_RGMII_TXID:
    case PHY_INTERFACE_MODE_RGMII_RXID:
    case PHY_INTERFACE_MODE_RGMII_ID:
    gtxc_delay_val |= FIELD_PREP(MT8195_DLY_GTXC_ENABLE, !!mac_delay.tx_delay);
    gtxc_delay_val |= FIELD_PREP(MT8195_DLY_GTXC_STAGES, mac_delay.tx_delay);
    gtxc_delay_val |= FIELD_PREP(MT8195_DLY_GTXC_INV, mac_delay.tx_inv);
    delay_val |= FIELD_PREP(MT8195_DLY_RXC_ENABLE, !!mac_delay.rx_delay);
    delay_val |= FIELD_PREP(MT8195_DLY_RXC_STAGES, mac_delay.rx_delay);
    delay_val |= FIELD_PREP(MT8195_DLY_RXC_INV, mac_delay.rx_inv);
    break;
    default:
    dev_err(plat.dev, "phy interface not supported\n");
    return -EINVAL;
    }
    regmap_update_bits(plat.peri_regmap,
    MT8195_PERI_ETH_CTRL0,
    MT8195_RGMII_TXC_PHASE_CTRL |
    MT8195_DLY_GTXC_INV |
    MT8195_DLY_GTXC_ENABLE |
    MT8195_DLY_GTXC_STAGES,
    gtxc_delay_val);
    regmap_write(plat.peri_regmap, MT8195_PERI_ETH_CTRL1, delay_val);
    regmap_write(plat.peri_regmap, MT8195_PERI_ETH_CTRL2, rmii_delay_val);
    mt8195_delay_stage2ps(plat);
    return 0;
    }
    static const struct mediatek_dwmac_variant mt8195_gmac_variant = {
    .dwmac_set_phy_interface = mt8195_set_interface,
    .dwmac_set_delay = mt8195_set_delay,
    .clk_list = mt8195_dwmac_clk_l,
    .num_clks = ARRAY_SIZE(mt8195_dwmac_clk_l),
    .rx_delay_max = 9280,
    .tx_delay_max = 9280,
    .dma_bit_mask = 35,
    };
#[no_mangle]
unsafe extern "C" fn mediatek_dwmac_config_dt(plat: *mut mediatek_dwmac_plat_data) -> c_int {
    static int mediatek_dwmac_config_dt(struct mediatek_dwmac_plat_data *plat)
    {
    struct mac_delay_struct *mac_delay = &plat.mac_delay;
    u32 tx_delay_ps, rx_delay_ps;
    plat.peri_regmap = syscon_regmap_lookup_by_phandle(plat.np, "mediatek,pericfg");
    if (IS_ERR(plat.peri_regmap)) {
    dev_err(plat.dev, "Failed to get pericfg syscon\n");
    return PTR_ERR(plat.peri_regmap);
    }
    if (!of_property_read_u32(plat.np, "mediatek,tx-delay-ps", &tx_delay_ps)) {
    if (tx_delay_ps < plat.variant.tx_delay_max) {
    mac_delay.tx_delay = tx_delay_ps;
    } else {
    dev_err(plat.dev, "Invalid TX clock delay: %dps\n", tx_delay_ps);
    return -EINVAL;
    }
    }
    if (!of_property_read_u32(plat.np, "mediatek,rx-delay-ps", &rx_delay_ps)) {
    if (rx_delay_ps < plat.variant.rx_delay_max) {
    mac_delay.rx_delay = rx_delay_ps;
    } else {
    dev_err(plat.dev, "Invalid RX clock delay: %dps\n", rx_delay_ps);
    return -EINVAL;
    }
    }
    mac_delay.tx_inv = of_property_read_bool(plat.np, "mediatek,txc-inverse");
    mac_delay.rx_inv = of_property_read_bool(plat.np, "mediatek,rxc-inverse");
    plat.rmii_rxc = of_property_read_bool(plat.np, "mediatek,rmii-rxc");
    plat.rmii_clk_from_mac = of_property_read_bool(plat.np, "mediatek,rmii-clk-from-mac");
    plat.mac_wol = of_property_read_bool(plat.np, "mediatek,mac-wol");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mediatek_dwmac_clk_init(plat: *mut mediatek_dwmac_plat_data) -> c_int {
    static int mediatek_dwmac_clk_init(struct mediatek_dwmac_plat_data *plat)
    {
    const struct mediatek_dwmac_variant *variant = plat.variant;
    int i, ret;
    plat.clks = devm_kcalloc(plat.dev, variant.num_clks, sizeof(*plat.clks), GFP_KERNEL);
    if (!plat.clks)
    return -ENOMEM;
    for (i = 0; i < variant.num_clks; i++)
    plat.clks[i].id = variant.clk_list[i];
    ret = devm_clk_bulk_get(plat.dev, variant.num_clks, plat.clks);
    if (ret)
    return ret;
// The clock labeled as "rmii_internal" is needed only in RMII(when
// MAC provides the reference clock), and useless for RGMII/MII or
// RMII(when PHY provides the reference clock).
// So, "rmii_internal" clock is got and configured only when
// reference clock of RMII is from MAC.
//
    if (plat.rmii_clk_from_mac) {
    plat.rmii_internal_clk = devm_clk_get(plat.dev, "rmii_internal");
    if (IS_ERR(plat.rmii_internal_clk))
    ret = PTR_ERR(plat.rmii_internal_clk);
    } else {
    plat.rmii_internal_clk = core::ptr::null_mut();
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mediatek_dwmac_init(dev: *mut device, priv: *mut c_void) -> c_int {
    static int mediatek_dwmac_init(struct device *dev, void *priv)
    {
    struct mediatek_dwmac_plat_data *plat = priv;
    const struct mediatek_dwmac_variant *variant = plat.variant;
    int phy_intf_sel, ret;
    if (variant.dwmac_set_phy_interface) {
    phy_intf_sel = stmmac_get_phy_intf_sel(plat.phy_mode);
    if (phy_intf_sel != PHY_INTF_SEL_GMII_MII &&
    phy_intf_sel != PHY_INTF_SEL_RGMII &&
    phy_intf_sel != PHY_INTF_SEL_RMII) {
    dev_err(plat.dev, "phy interface not supported\n");
    return phy_intf_sel < 0 ? phy_intf_sel : -EINVAL;
    }
    ret = variant.dwmac_set_phy_interface(plat, phy_intf_sel);
    if (ret) {
    dev_err(dev, "failed to set phy interface, err = %d\n", ret);
    return ret;
    }
    }
    if (variant.dwmac_set_delay) {
    ret = variant.dwmac_set_delay(plat);
    if (ret) {
    dev_err(dev, "failed to set delay value, err = %d\n", ret);
    return ret;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mediatek_dwmac_clks_config(priv: *mut c_void, enabled: bool) -> c_int {
    static int mediatek_dwmac_clks_config(void *priv, bool enabled)
    {
    struct mediatek_dwmac_plat_data *plat = priv;
    const struct mediatek_dwmac_variant *variant = plat.variant;
    let mut ret: c_int = 0;
    if (enabled) {
    ret = clk_bulk_prepare_enable(variant.num_clks, plat.clks);
    if (ret) {
    dev_err(plat.dev, "failed to enable clks, err = %d\n", ret);
    return ret;
    }
    ret = clk_prepare_enable(plat.rmii_internal_clk);
    if (ret) {
    dev_err(plat.dev, "failed to enable rmii internal clk, err = %d\n", ret);
    return ret;
    }
    } else {
    clk_disable_unprepare(plat.rmii_internal_clk);
    clk_bulk_disable_unprepare(variant.num_clks, plat.clks);
    }
    return ret;
    }
    static int mediatek_dwmac_common_data(struct platform_device *pdev,
    struct plat_stmmacenet_data *plat,
    struct mediatek_dwmac_plat_data *priv_plat)
    {
    int i;
    priv_plat.phy_mode = plat.phy_interface;
    if (priv_plat.mac_wol)
    plat.flags &= ~STMMAC_FLAG_USE_PHY_WOL;
    else
    plat.flags |= STMMAC_FLAG_USE_PHY_WOL;
    plat.riwt_off = true;
    plat.maxmtu = ETH_DATA_LEN;
    plat.host_dma_width = priv_plat.variant.dma_bit_mask;
    plat.bsp_priv = priv_plat;
    plat.resume = mediatek_dwmac_init;
    plat.clks_config = mediatek_dwmac_clks_config;
    plat.safety_feat_cfg = devm_kzalloc(&pdev.dev,
    sizeof(*plat.safety_feat_cfg),
    GFP_KERNEL);
    if (!plat.safety_feat_cfg)
    return -ENOMEM;
    plat.safety_feat_cfg.tsoee = 1;
    plat.safety_feat_cfg.mrxpee = 0;
    plat.safety_feat_cfg.mestee = 1;
    plat.safety_feat_cfg.mrxee = 1;
    plat.safety_feat_cfg.mtxee = 1;
    plat.safety_feat_cfg.epsi = 0;
    plat.safety_feat_cfg.edpp = 1;
    plat.safety_feat_cfg.prtyen = 1;
    plat.safety_feat_cfg.tmouten = 1;
    for (i = 0; i < plat.tx_queues_to_use; i++) {
// Default TX Q0 to use TSO and rest TXQ for TBS
    if (i > 0)
    plat.tx_queues_cfg[i].tbs_en = 1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mediatek_dwmac_probe(pdev: *mut platform_device) -> c_int {
    static int mediatek_dwmac_probe(struct platform_device *pdev)
    {
    struct mediatek_dwmac_plat_data *priv_plat;
    struct plat_stmmacenet_data *plat_dat;
    struct stmmac_resources stmmac_res;
    int ret;
    priv_plat = devm_kzalloc(&pdev.dev, sizeof(*priv_plat), GFP_KERNEL);
    if (!priv_plat)
    return -ENOMEM;
    priv_plat.variant = of_device_get_match_data(&pdev.dev);
    if (!priv_plat.variant) {
    dev_err(&pdev.dev, "Missing dwmac-mediatek variant\n");
    return -EINVAL;
    }
    priv_plat.dev = &pdev.dev;
    priv_plat.np = pdev.dev.of_node;
    ret = mediatek_dwmac_config_dt(priv_plat);
    if (ret)
    return ret;
    ret = mediatek_dwmac_clk_init(priv_plat);
    if (ret)
    return ret;
    ret = stmmac_get_platform_resources(pdev, &stmmac_res);
    if (ret)
    return ret;
    plat_dat = devm_stmmac_probe_config_dt(pdev, stmmac_res.mac);
    if (IS_ERR(plat_dat))
    return PTR_ERR(plat_dat);
    mediatek_dwmac_common_data(pdev, plat_dat, priv_plat);
    mediatek_dwmac_init(&pdev.dev, priv_plat);
    ret = mediatek_dwmac_clks_config(priv_plat, true);
    if (ret)
    return ret;
    ret = stmmac_dvr_probe(&pdev.dev, plat_dat, &stmmac_res);
    if (ret)
    goto err_drv_probe;
    return 0;
    err_drv_probe:
    mediatek_dwmac_clks_config(priv_plat, false);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mediatek_dwmac_remove(pdev: *mut platform_device) {
    static void mediatek_dwmac_remove(struct platform_device *pdev)
    {
    struct mediatek_dwmac_plat_data *priv_plat = get_stmmac_bsp_priv(&pdev.dev);
    stmmac_pltfr_remove(pdev);
    mediatek_dwmac_clks_config(priv_plat, false);
    }
    static const struct of_device_id mediatek_dwmac_match[] = {
    { .compatible = "mediatek,mt2712-gmac",
    .data = &mt2712_gmac_variant },
    { .compatible = "mediatek,mt8195-gmac",
    .data = &mt8195_gmac_variant },
    { }
    };
    MODULE_DEVICE_TABLE(of, mediatek_dwmac_match);
    static struct platform_driver mediatek_dwmac_driver = {
    .probe  = mediatek_dwmac_probe,
    .remove = mediatek_dwmac_remove,
    .driver = {
    .name           = "dwmac-mediatek",
    .pm		= &stmmac_pltfr_pm_ops,
    .of_match_table = mediatek_dwmac_match,
    },
    };
    module_platform_driver(mediatek_dwmac_driver);
    MODULE_AUTHOR("Biao Huang <biao.huang@mediatek.com>");
    MODULE_DESCRIPTION("MediaTek DWMAC specific glue layer");
    MODULE_LICENSE("GPL v2");
