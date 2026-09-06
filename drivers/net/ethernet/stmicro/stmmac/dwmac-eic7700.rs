//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/stmicro/stmmac/dwmac-eic7700.c
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
// Eswin DWC Ethernet linux driver
//
// Copyright 2025, Beijing ESWIN Computing Technology Co., Ltd.
//
// Authors:
// Zhi Li <lizhi2@eswincomputing.com>
// Shuang Liang <liangshuang@eswincomputing.com>
// Shangjuan Wei <weishangjuan@eswincomputing.com>
//

// eth_phy_ctrl_offset eth0:0x100

// eth_axi_lp_ctrl_offset eth0:0x108

//
// TX/RX Clock Delay Bit Masks:
// - TX Delay: bits [14:8] - TX_CLK delay (unit: 0.02ns per bit)
// - TX Invert : bit  [15]
// - RX Delay: bits [30:24] - RX_CLK delay (unit: 0.02ns per bit)
// - RX Invert : bit  [31]
//

pub const EIC7700_MAX_DELAY_STEPS: c_uint = 0x7F;
pub const EIC7700_DELAY_STEP_PS: c_int = 20;

    (EIC7700_MAX_DELAY_STEPS * EIC7700_DELAY_STEP_PS)
    static const char * const eic7700_clk_names[] = {
    "tx", "axi", "cfg",
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eic7700_dwmac_data {
    pub rgmii_rx_clk_invert: bool,
    pub has_internal_tx_delay: bool,
    pub tx_clk_inherent_skew_ps: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eic7700_qos_priv {
    pub dev: *mut device,
    pub plat_dat: *mut plat_stmmacenet_data,
    pub eic7700_hsp_regmap: *mut regmap,
    pub eth_axi_lp_ctrl_offset: u32,
    pub eth_phy_ctrl_offset: u32,
    pub eth_clk_offset: u32,
    pub eth_txd_offset: u32,
    pub eth_rxd_offset: u32,
    pub eth_clk_dly_param: u32,
    pub has_txd_offset: bool,
    pub has_rxd_offset: bool,
    pub eth_rx_clk_inv: bool,
}

#[no_mangle]
unsafe extern "C" fn eic7700_clks_config(priv: *mut c_void, enabled: bool) -> c_int {
    static int eic7700_clks_config(void *priv, bool enabled)
    {
    struct eic7700_qos_priv *dwc = (struct eic7700_qos_priv *)priv;
    struct plat_stmmacenet_data *plat = dwc.plat_dat;
    let mut ret: c_int = 0;
    if (enabled)
    ret = clk_bulk_prepare_enable(plat.num_clks, plat.clks);
    else
    clk_bulk_disable_unprepare(plat.num_clks, plat.clks);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn eic7700_dwmac_init(dev: *mut device, priv: *mut c_void) -> c_int {
    static int eic7700_dwmac_init(struct device *dev, void *priv)
    {
    struct eic7700_qos_priv *dwc = priv;
    int ret;
    ret = eic7700_clks_config(dwc, true);
    if (ret)
    return ret;
    ret = regmap_set_bits(dwc.eic7700_hsp_regmap,
    dwc.eth_phy_ctrl_offset,
    EIC7700_ETH_TX_CLK_SEL |
    EIC7700_ETH_PHY_INTF_SELI);
    if (ret) {
    eic7700_clks_config(dwc, false);
    return ret;
    }
    regmap_write(dwc.eic7700_hsp_regmap, dwc.eth_axi_lp_ctrl_offset,
    EIC7700_ETH_CSYSREQ_VAL);
    if (dwc.has_txd_offset)
    regmap_write(dwc.eic7700_hsp_regmap, dwc.eth_txd_offset, 0);
    if (dwc.has_rxd_offset)
    regmap_write(dwc.eic7700_hsp_regmap, dwc.eth_rxd_offset, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn eic7700_dwmac_exit(dev: *mut device, priv: *mut c_void) {
    static void eic7700_dwmac_exit(struct device *dev, void *priv)
    {
    struct eic7700_qos_priv *dwc = priv;
    eic7700_clks_config(dwc, false);
    }
#[no_mangle]
unsafe extern "C" fn eic7700_dwmac_suspend(dev: *mut device, priv: *mut c_void) -> c_int {
    static int eic7700_dwmac_suspend(struct device *dev, void *priv)
    {
    return pm_runtime_force_suspend(dev);
    }
#[no_mangle]
unsafe extern "C" fn eic7700_dwmac_resume(dev: *mut device, priv: *mut c_void) -> c_int {
    static int eic7700_dwmac_resume(struct device *dev, void *priv)
    {
    int ret;
    ret = pm_runtime_force_resume(dev);
    if (ret)
    dev_err(dev, "%s failed: %d\n", __func__, ret);
    return ret;
    }
//
// eth1 requires RX clock inversion at 1000Mbps due to silicon-inherent
// RX sampling skew at MAC input.
//
// The configuration is updated in fix_mac_speed() because the required
// sampling behavior depends on the negotiated link speed.
//
    static void eic7700_dwmac_fix_speed(void *priv, phy_interface_t interface,
    int speed, unsigned int mode)
    {
    struct eic7700_qos_priv *dwc = (struct eic7700_qos_priv *)priv;
    let mut dly_param: u32 = dwc.eth_clk_dly_param;
    switch (speed) {
    case SPEED_1000:
    if (dwc.eth_rx_clk_inv)
    dly_param |= EIC7700_ETH_RX_INV_DELAY;
    break;
    case SPEED_100:
    case SPEED_10:
    break;
    default:
    dev_warn(dwc.dev, "unsupported speed %u\n", speed);
    return;
    }
    regmap_write(dwc.eic7700_hsp_regmap, dwc.eth_clk_offset, dly_param);
    }
#[no_mangle]
unsafe extern "C" fn eic7700_dwmac_probe(pdev: *mut platform_device) -> c_int {
    static int eic7700_dwmac_probe(struct platform_device *pdev)
    {
    const struct eic7700_dwmac_data *data;
    struct plat_stmmacenet_data *plat_dat;
    struct stmmac_resources stmmac_res;
    struct eic7700_qos_priv *dwc_priv;
    u32 delay_ps, val;
    int i, ret;
    ret = stmmac_get_platform_resources(pdev, &stmmac_res);
    if (ret)
    return dev_err_probe(&pdev.dev, ret,
    "failed to get resources\n");
    plat_dat = devm_stmmac_probe_config_dt(pdev, stmmac_res.mac);
    if (IS_ERR(plat_dat))
    return dev_err_probe(&pdev.dev, PTR_ERR(plat_dat),
    "dt configuration failed\n");
    dwc_priv = devm_kzalloc(&pdev.dev, sizeof(*dwc_priv), GFP_KERNEL);
    if (!dwc_priv)
    return -ENOMEM;
    dwc_priv.dev = &pdev.dev;
    data = device_get_match_data(&pdev.dev);
    if (!data)
    return dev_err_probe(&pdev.dev,
    -EINVAL, "no match data found\n");
    dwc_priv.eth_rx_clk_inv = data.rgmii_rx_clk_invert;
//
// The MAC silicon unconditionally adds ~2 ns TX delay; prevent
// the PHY from also adding TX delay to avoid doubling it.
//
// DT specifies rgmii-id (TX from MAC silicon, RX from PHY);
// override to rgmii-rxid so the PHY only adds its RX delay.
//
    if (data.has_internal_tx_delay) {
    plat_dat.phy_interface =
    phy_fix_phy_mode_for_mac_delays(plat_dat.phy_interface,
    true, false);
    if (plat_dat.phy_interface == PHY_INTERFACE_MODE_NA)
    return dev_err_probe(&pdev.dev, -EINVAL,
    "phy interface mode is NA\n");
    }
// Read rx-internal-delay-ps and update rx_clk delay
    if (!of_property_read_u32(pdev.dev.of_node,
    "rx-internal-delay-ps", &delay_ps)) {
    if (delay_ps % EIC7700_DELAY_STEP_PS)
    return dev_err_probe(&pdev.dev, -EINVAL,
    "rx delay must be multiple of %dps\n",
    EIC7700_DELAY_STEP_PS);
    if (delay_ps > EIC7700_MAX_DELAY_PS)
    return dev_err_probe(&pdev.dev, -EINVAL,
    "rx delay out of range\n");
    val = delay_ps / EIC7700_DELAY_STEP_PS;
    dwc_priv.eth_clk_dly_param &= ~EIC7700_ETH_RX_ADJ_DELAY;
    dwc_priv.eth_clk_dly_param |=
    FIELD_PREP(EIC7700_ETH_RX_ADJ_DELAY, val);
    }
// Read tx-internal-delay-ps and update tx_clk delay.
//
// For eswin,eic7700-qos-eth-clk-inversion, the DT property describes
// the effective TX delay at the MAC output, including the inherent
// silicon delay. Subtract the fixed component to obtain the
// programmable delay value.
//
    if (!of_property_read_u32(pdev.dev.of_node,
    "tx-internal-delay-ps", &delay_ps)) {
    if (delay_ps % EIC7700_DELAY_STEP_PS)
    return dev_err_probe(&pdev.dev, -EINVAL,
    "tx delay must be multiple of %dps\n",
    EIC7700_DELAY_STEP_PS);
    if (delay_ps < data.tx_clk_inherent_skew_ps)
    return dev_err_probe(&pdev.dev, -EINVAL,
    "tx delay %ups below inherent skew %ups\n",
    delay_ps, data.tx_clk_inherent_skew_ps);
    delay_ps -= data.tx_clk_inherent_skew_ps;
    if (delay_ps > EIC7700_MAX_DELAY_PS)
    return dev_err_probe(&pdev.dev, -EINVAL,
    "tx delay out of programmable range\n");
    val = delay_ps / EIC7700_DELAY_STEP_PS;
    dwc_priv.eth_clk_dly_param &= ~EIC7700_ETH_TX_ADJ_DELAY;
    dwc_priv.eth_clk_dly_param |=
    FIELD_PREP(EIC7700_ETH_TX_ADJ_DELAY, val);
    }
    dwc_priv.eic7700_hsp_regmap =
    syscon_regmap_lookup_by_phandle(pdev.dev.of_node,
    "eswin,hsp-sp-csr");
    if (IS_ERR(dwc_priv.eic7700_hsp_regmap))
    return dev_err_probe(&pdev.dev,
    PTR_ERR(dwc_priv.eic7700_hsp_regmap),
    "Failed to get hsp-sp-csr regmap\n");
    ret = of_property_read_u32_index(pdev.dev.of_node,
    "eswin,hsp-sp-csr",
    1, &dwc_priv.eth_phy_ctrl_offset);
    if (ret)
    return dev_err_probe(&pdev.dev, ret,
    "can't get eth_phy_ctrl_offset\n");
    ret = of_property_read_u32_index(pdev.dev.of_node,
    "eswin,hsp-sp-csr",
    2, &dwc_priv.eth_axi_lp_ctrl_offset);
    if (ret)
    return dev_err_probe(&pdev.dev, ret,
    "can't get eth_axi_lp_ctrl_offset\n");
    ret = of_property_read_u32_index(pdev.dev.of_node,
    "eswin,hsp-sp-csr",
    3, &dwc_priv.eth_clk_offset);
    if (ret)
    return dev_err_probe(&pdev.dev, ret,
    "can't get eth_clk_offset\n");
    ret = of_property_read_u32_index(pdev.dev.of_node,
    "eswin,hsp-sp-csr",
    4, &dwc_priv.eth_txd_offset);
    if (!ret)
    dwc_priv.has_txd_offset = true;
    ret = of_property_read_u32_index(pdev.dev.of_node,
    "eswin,hsp-sp-csr",
    5, &dwc_priv.eth_rxd_offset);
    if (!ret)
    dwc_priv.has_rxd_offset = true;
    plat_dat.num_clks = ARRAY_SIZE(eic7700_clk_names);
    plat_dat.clks = devm_kcalloc(&pdev.dev,
    plat_dat.num_clks,
    sizeof(*plat_dat.clks),
    GFP_KERNEL);
    if (!plat_dat.clks)
    return -ENOMEM;
    for (i = 0; i < ARRAY_SIZE(eic7700_clk_names); i++)
    plat_dat.clks[i].id = eic7700_clk_names[i];
    ret = devm_clk_bulk_get_optional(&pdev.dev,
    plat_dat.num_clks,
    plat_dat.clks);
    if (ret)
    return dev_err_probe(&pdev.dev, ret,
    "Failed to get clocks\n");
    plat_dat.clk_tx_i = stmmac_pltfr_find_clk(plat_dat, "tx");
    plat_dat.set_clk_tx_rate = stmmac_set_clk_tx_rate;
    plat_dat.clks_config = eic7700_clks_config;
    plat_dat.bsp_priv = dwc_priv;
    dwc_priv.plat_dat = plat_dat;
    plat_dat.init = eic7700_dwmac_init;
    plat_dat.exit = eic7700_dwmac_exit;
    plat_dat.suspend = eic7700_dwmac_suspend;
    plat_dat.resume = eic7700_dwmac_resume;
    plat_dat.fix_mac_speed = eic7700_dwmac_fix_speed;
    return devm_stmmac_pltfr_probe(pdev, plat_dat, &stmmac_res);
    }
    static const struct eic7700_dwmac_data eic7700_dwmac_data = {
    .rgmii_rx_clk_invert = false,
    .has_internal_tx_delay = false,
    .tx_clk_inherent_skew_ps = 0,
    };
    static const struct eic7700_dwmac_data eic7700_dwmac_data_clk_inversion = {
    .rgmii_rx_clk_invert = true,
    .has_internal_tx_delay = true,
    .tx_clk_inherent_skew_ps = 2000,
    };
    static const struct of_device_id eic7700_dwmac_match[] = {
    {	.compatible = "eswin,eic7700-qos-eth",
    .data = &eic7700_dwmac_data,
    },
    {
    .compatible = "eswin,eic7700-qos-eth-clk-inversion",
    .data = &eic7700_dwmac_data_clk_inversion,
    },
    { }
    };
    MODULE_DEVICE_TABLE(of, eic7700_dwmac_match);
    static struct platform_driver eic7700_dwmac_driver = {
    .probe  = eic7700_dwmac_probe,
    .driver = {
    .name           = "eic7700-eth-dwmac",
    .pm             = &stmmac_pltfr_pm_ops,
    .of_match_table = eic7700_dwmac_match,
    },
    };
    module_platform_driver(eic7700_dwmac_driver);
    MODULE_AUTHOR("Zhi Li <lizhi2@eswincomputing.com>");
    MODULE_AUTHOR("Shuang Liang <liangshuang@eswincomputing.com>");
    MODULE_AUTHOR("Shangjuan Wei <weishangjuan@eswincomputing.com>");
    MODULE_DESCRIPTION("Eswin eic7700 qos ethernet driver");
    MODULE_LICENSE("GPL");
