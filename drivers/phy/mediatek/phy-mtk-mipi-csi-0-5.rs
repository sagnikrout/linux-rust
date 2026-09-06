//! Automatically rewritten from C to Rust
//! Source: drivers/phy/mediatek/phy-mtk-mipi-csi-0-5.c
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
// MediaTek MIPI CSI v0.5 driver
//
// Copyright (c) 2023, MediaTek Inc.
// Copyright (c) 2023, BayLibre Inc.
//

pub const CSIXB_OFFSET: c_uint = 0x1000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_mipi_cdphy_port {
    pub dev: *mut device,
    pub base: *mut void __iomem,
    pub phy: *mut phy,
    pub type: u32,
    pub mode: u32,
    pub num_lanes: u32,
}

    enum PHY_TYPE {
    DPHY = 0,
    CPHY,
    CDPHY,
    };
#[no_mangle]
unsafe extern "C" fn mtk_phy_csi_cdphy_ana_eq_tune(base: *mut void __iomem) {
    static void mtk_phy_csi_cdphy_ana_eq_tune(void __iomem *base)
    {
    mtk_phy_update_field(base + MIPI_RX_ANA18_CSIXA, RG_CSI0A_L0_T0AB_EQ_IS, 1);
    mtk_phy_update_field(base + MIPI_RX_ANA18_CSIXA, RG_CSI0A_L0_T0AB_EQ_BW, 1);
    mtk_phy_update_field(base + MIPI_RX_ANA1C_CSIXA, RG_CSI0A_L1_T1AB_EQ_IS, 1);
    mtk_phy_update_field(base + MIPI_RX_ANA1C_CSIXA, RG_CSI0A_L1_T1AB_EQ_BW, 1);
    mtk_phy_update_field(base + MIPI_RX_ANA20_CSI0A, RG_CSI0A_L2_T1BC_EQ_IS, 1);
    mtk_phy_update_field(base + MIPI_RX_ANA20_CSI0A, RG_CSI0A_L2_T1BC_EQ_BW, 1);
    mtk_phy_update_field(base + CSIXB_OFFSET + MIPI_RX_ANA18_CSIXA, RG_CSI0A_L0_T0AB_EQ_IS, 1);
    mtk_phy_update_field(base + CSIXB_OFFSET + MIPI_RX_ANA18_CSIXA, RG_CSI0A_L0_T0AB_EQ_BW, 1);
    mtk_phy_update_field(base + CSIXB_OFFSET + MIPI_RX_ANA1C_CSIXA, RG_CSI0A_L1_T1AB_EQ_IS, 1);
    mtk_phy_update_field(base + CSIXB_OFFSET + MIPI_RX_ANA1C_CSIXA, RG_CSI0A_L1_T1AB_EQ_BW, 1);
    mtk_phy_update_field(base + CSIXB_OFFSET + MIPI_RX_ANA20_CSI0A, RG_CSI0A_L2_T1BC_EQ_IS, 1);
    mtk_phy_update_field(base + CSIXB_OFFSET + MIPI_RX_ANA20_CSI0A, RG_CSI0A_L2_T1BC_EQ_BW, 1);
    }
#[no_mangle]
unsafe extern "C" fn mtk_phy_csi_dphy_ana_eq_tune(base: *mut void __iomem) {
    static void mtk_phy_csi_dphy_ana_eq_tune(void __iomem *base)
    {
    mtk_phy_update_field(base + MIPI_RX_ANA18_CSIXA, RG_CSI1A_L0_EQ_IS, 1);
    mtk_phy_update_field(base + MIPI_RX_ANA18_CSIXA, RG_CSI1A_L0_EQ_BW, 1);
    mtk_phy_update_field(base + MIPI_RX_ANA18_CSIXA, RG_CSI1A_L1_EQ_IS, 1);
    mtk_phy_update_field(base + MIPI_RX_ANA18_CSIXA, RG_CSI1A_L1_EQ_BW, 1);
    mtk_phy_update_field(base + MIPI_RX_ANA1C_CSIXA, RG_CSI1A_L2_EQ_IS, 1);
    mtk_phy_update_field(base + MIPI_RX_ANA1C_CSIXA, RG_CSI1A_L2_EQ_BW, 1);
    mtk_phy_update_field(base + CSIXB_OFFSET + MIPI_RX_ANA18_CSIXA, RG_CSI1A_L0_EQ_IS, 1);
    mtk_phy_update_field(base + CSIXB_OFFSET + MIPI_RX_ANA18_CSIXA, RG_CSI1A_L0_EQ_BW, 1);
    mtk_phy_update_field(base + CSIXB_OFFSET + MIPI_RX_ANA18_CSIXA, RG_CSI1A_L1_EQ_IS, 1);
    mtk_phy_update_field(base + CSIXB_OFFSET + MIPI_RX_ANA18_CSIXA, RG_CSI1A_L1_EQ_BW, 1);
    mtk_phy_update_field(base + CSIXB_OFFSET + MIPI_RX_ANA1C_CSIXA, RG_CSI1A_L2_EQ_IS, 1);
    mtk_phy_update_field(base + CSIXB_OFFSET + MIPI_RX_ANA1C_CSIXA, RG_CSI1A_L2_EQ_BW, 1);
    }
#[no_mangle]
unsafe extern "C" fn mtk_mipi_phy_power_on(phy: *mut phy) -> c_int {
    static int mtk_mipi_phy_power_on(struct phy *phy)
    {
    struct mtk_mipi_cdphy_port *port = phy_get_drvdata(phy);
    void __iomem *base = port.base;
//
// The driver currently supports DPHY and CD-PHY phys,
// but the only mode supported is DPHY,
// so CD-PHY capable phys must be configured in DPHY mode
//
    if (port.type == CDPHY) {
    mtk_phy_update_field(base + MIPI_RX_ANA00_CSIXA, RG_CSI0A_CPHY_EN, 0);
    mtk_phy_update_field(base + CSIXB_OFFSET + MIPI_RX_ANA00_CSIXA,
    RG_CSI0A_CPHY_EN, 0);
    }
//
// Lane configuration:
//
// Only 4 data + 1 clock is supported for now with the following mapping:
//
// CSIXA_LNR0 --> D2
// CSIXA_LNR1 --> D0
// CSIXA_LNR2 --> C
// CSIXB_LNR0 --> D1
// CSIXB_LNR1 --> D3
//
    mtk_phy_update_field(base + MIPI_RX_ANA00_CSIXA, RG_CSIXA_DPHY_L0_CKMODE_EN, 0);
    mtk_phy_update_field(base + MIPI_RX_ANA00_CSIXA, RG_CSIXA_DPHY_L0_CKSEL, 1);
    mtk_phy_update_field(base + MIPI_RX_ANA00_CSIXA, RG_CSIXA_DPHY_L1_CKMODE_EN, 0);
    mtk_phy_update_field(base + MIPI_RX_ANA00_CSIXA, RG_CSIXA_DPHY_L1_CKSEL, 1);
    mtk_phy_update_field(base + MIPI_RX_ANA00_CSIXA, RG_CSIXA_DPHY_L2_CKMODE_EN, 1);
    mtk_phy_update_field(base + MIPI_RX_ANA00_CSIXA, RG_CSIXA_DPHY_L2_CKSEL, 1);
    mtk_phy_update_field(base + CSIXB_OFFSET + MIPI_RX_ANA00_CSIXA,
    RG_CSIXA_DPHY_L0_CKMODE_EN, 0);
    mtk_phy_update_field(base + CSIXB_OFFSET + MIPI_RX_ANA00_CSIXA, RG_CSIXA_DPHY_L0_CKSEL, 1);
    mtk_phy_update_field(base + CSIXB_OFFSET + MIPI_RX_ANA00_CSIXA,
    RG_CSIXA_DPHY_L1_CKMODE_EN, 0);
    mtk_phy_update_field(base + CSIXB_OFFSET + MIPI_RX_ANA00_CSIXA, RG_CSIXA_DPHY_L1_CKSEL, 1);
    mtk_phy_update_field(base + CSIXB_OFFSET + MIPI_RX_ANA00_CSIXA,
    RG_CSIXA_DPHY_L2_CKMODE_EN, 0);
    mtk_phy_update_field(base + CSIXB_OFFSET + MIPI_RX_ANA00_CSIXA, RG_CSIXA_DPHY_L2_CKSEL, 1);
// Byte clock invert
    mtk_phy_update_field(base + MIPI_RX_ANAA8_CSIXA, RG_CSIXA_CDPHY_L0_T0_BYTECK_INVERT, 1);
    mtk_phy_update_field(base + MIPI_RX_ANAA8_CSIXA, RG_CSIXA_DPHY_L1_BYTECK_INVERT, 1);
    mtk_phy_update_field(base + MIPI_RX_ANAA8_CSIXA, RG_CSIXA_CDPHY_L2_T1_BYTECK_INVERT, 1);
    mtk_phy_update_field(base + CSIXB_OFFSET + MIPI_RX_ANAA8_CSIXA,
    RG_CSIXA_CDPHY_L0_T0_BYTECK_INVERT, 1);
    mtk_phy_update_field(base + CSIXB_OFFSET + MIPI_RX_ANAA8_CSIXA,
    RG_CSIXA_DPHY_L1_BYTECK_INVERT, 1);
    mtk_phy_update_field(base + CSIXB_OFFSET + MIPI_RX_ANAA8_CSIXA,
    RG_CSIXA_CDPHY_L2_T1_BYTECK_INVERT, 1);
// Start ANA EQ tuning
    if (port.type == CDPHY)
    mtk_phy_csi_cdphy_ana_eq_tune(base);
    else
    mtk_phy_csi_dphy_ana_eq_tune(base);
// End ANA EQ tuning
    mtk_phy_set_bits(base + MIPI_RX_ANA40_CSIXA, 0x90);
    mtk_phy_update_field(base + MIPI_RX_ANA24_CSIXA, RG_CSIXA_RESERVE, 0x40);
    mtk_phy_update_field(base + CSIXB_OFFSET + MIPI_RX_ANA24_CSIXA, RG_CSIXA_RESERVE, 0x40);
    mtk_phy_update_field(base + MIPI_RX_WRAPPER80_CSIXA, CSR_CSI_RST_MODE, 0);
    mtk_phy_update_field(base + CSIXB_OFFSET + MIPI_RX_WRAPPER80_CSIXA, CSR_CSI_RST_MODE, 0);
// ANA power on
    mtk_phy_update_field(base + MIPI_RX_ANA00_CSIXA, RG_CSIXA_BG_CORE_EN, 1);
    mtk_phy_update_field(base + CSIXB_OFFSET + MIPI_RX_ANA00_CSIXA, RG_CSIXA_BG_CORE_EN, 1);
    usleep_range(20, 40);
    mtk_phy_update_field(base + MIPI_RX_ANA00_CSIXA, RG_CSIXA_BG_LPF_EN, 1);
    mtk_phy_update_field(base + CSIXB_OFFSET + MIPI_RX_ANA00_CSIXA, RG_CSIXA_BG_LPF_EN, 1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mtk_mipi_phy_power_off(phy: *mut phy) -> c_int {
    static int mtk_mipi_phy_power_off(struct phy *phy)
    {
    struct mtk_mipi_cdphy_port *port = phy_get_drvdata(phy);
    void __iomem *base = port.base;
// Disable MIPI BG.
    mtk_phy_update_field(base + MIPI_RX_ANA00_CSIXA, RG_CSIXA_BG_CORE_EN, 0);
    mtk_phy_update_field(base + MIPI_RX_ANA00_CSIXA, RG_CSIXA_BG_LPF_EN, 0);
    mtk_phy_update_field(base + CSIXB_OFFSET + MIPI_RX_ANA00_CSIXA, RG_CSIXA_BG_CORE_EN, 0);
    mtk_phy_update_field(base + CSIXB_OFFSET + MIPI_RX_ANA00_CSIXA, RG_CSIXA_BG_LPF_EN, 0);
    return 0;
    }
    static struct phy *mtk_mipi_cdphy_xlate(struct device *dev,
    const struct of_phandle_args *args)
    {
    struct mtk_mipi_cdphy_port *priv = dev_get_drvdata(dev);
//
// If PHY is CD-PHY then we need to get the operating mode
// For now only D-PHY mode is supported
//
    if (priv.type == CDPHY) {
    if (args.args_count != 1) {
    dev_err(dev, "invalid number of arguments\n");
    return ERR_PTR(-EINVAL);
    }
    switch (args.args[0]) {
    case PHY_TYPE_DPHY:
    priv.mode = DPHY;
    if (priv.num_lanes != 4) {
    dev_err(dev, "Only 4D1C mode is supported for now!\n");
    return ERR_PTR(-EINVAL);
    }
    break;
    default:
    dev_err(dev, "Unsupported PHY type: %i\n", args.args[0]);
    return ERR_PTR(-EINVAL);
    }
    } else {
    if (args.args_count) {
    dev_err(dev, "invalid number of arguments\n");
    return ERR_PTR(-EINVAL);
    }
    priv.mode = DPHY;
    }
    return priv.phy;
    }
    static const struct phy_ops mtk_cdphy_ops = {
    .power_on	= mtk_mipi_phy_power_on,
    .power_off	= mtk_mipi_phy_power_off,
    .owner		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn mtk_mipi_cdphy_probe(pdev: *mut platform_device) -> c_int {
    static int mtk_mipi_cdphy_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct phy_provider *phy_provider;
    struct mtk_mipi_cdphy_port *port;
    struct phy *phy;
    int ret;
    u32 phy_type;
    port = devm_kzalloc(dev, sizeof(*port), GFP_KERNEL);
    if (!port)
    return -ENOMEM;
    dev_set_drvdata(dev, port);
    port.dev = dev;
    port.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(port.base))
    return PTR_ERR(port.base);
    ret = of_property_read_u32(dev.of_node, "num-lanes", &port.num_lanes);
    if (ret) {
    dev_err(dev, "Failed to read num-lanes property: %i\n", ret);
    return ret;
    }
//
// phy-type is optional, if not present, PHY is considered to be CD-PHY
//
    if (device_property_present(dev, "phy-type")) {
    ret = of_property_read_u32(dev.of_node, "phy-type", &phy_type);
    if (ret) {
    dev_err(dev, "Failed to read phy-type property: %i\n", ret);
    return ret;
    }
    switch (phy_type) {
    case PHY_TYPE_DPHY:
    port.type = DPHY;
    break;
    default:
    dev_err(dev, "Unsupported PHY type: %i\n", phy_type);
    return -EINVAL;
    }
    } else {
    port.type = CDPHY;
    }
    phy = devm_phy_create(dev, core::ptr::null_mut(), &mtk_cdphy_ops);
    if (IS_ERR(phy)) {
    dev_err(dev, "Failed to create PHY: %ld\n", PTR_ERR(phy));
    return PTR_ERR(phy);
    }
    port.phy = phy;
    phy_set_drvdata(phy, port);
    phy_provider = devm_of_phy_provider_register(dev, mtk_mipi_cdphy_xlate);
    if (IS_ERR(phy_provider)) {
    dev_err(dev, "Failed to register PHY provider: %ld\n",
    PTR_ERR(phy_provider));
    return PTR_ERR(phy_provider);
    }
    return 0;
    }
    static const struct of_device_id mtk_mipi_cdphy_of_match[] = {
    { .compatible = "mediatek,mt8365-csi-rx" },
    { /* sentinel */},
    };
    MODULE_DEVICE_TABLE(of, mtk_mipi_cdphy_of_match);
    static struct platform_driver mipi_cdphy_pdrv = {
    .probe = mtk_mipi_cdphy_probe,
    .driver	= {
    .name	= "mtk-mipi-csi-0-5",
    .of_match_table = mtk_mipi_cdphy_of_match,
    },
    };
    module_platform_driver(mipi_cdphy_pdrv);
    MODULE_DESCRIPTION("MediaTek MIPI CSI CD-PHY v0.5 Driver");
    MODULE_AUTHOR("Louis Kuo <louis.kuo@mediatek.com>");
    MODULE_LICENSE("GPL");
