//! Automatically rewritten from C to Rust
//! Source: drivers/phy/mediatek/phy-mtk-dp.c
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
// MediaTek DisplayPort PHY driver
//
// Copyright (c) 2022, BayLibre Inc.
// Copyright (c) 2022, MediaTek Inc.
//

pub const PHY_OFFSET: c_uint = 0x1000;

pub const BIT_RATE_RBR: c_int = 0;
pub const BIT_RATE_HBR: c_int = 1;
pub const BIT_RATE_HBR2: c_int = 2;
pub const BIT_RATE_HBR3: c_int = 3;

    XTP_LN_TX_LCTXC0_SW0_PRE1_DEFAULT | \
    XTP_LN_TX_LCTXC0_SW0_PRE2_DEFAULT | \
    XTP_LN_TX_LCTXC0_SW0_PRE3_DEFAULT)

    XTP_LN_TX_LCTXC0_SW1_PRE1_DEFAULT | \
    XTP_LN_TX_LCTXC0_SW1_PRE2_DEFAULT | \
    XTP_LN_TX_LCTXC0_SW2_PRE0_DEFAULT)

    XTP_LN_TX_LCTXC0_SW3_PRE0_DEFAULT)
pub const XTP_LN_TX_LCTXCP1_SW0_PRE0_DEFAULT: c_int = 0;

    XTP_LN_TX_LCTXCP1_SW0_PRE1_DEFAULT | \
    XTP_LN_TX_LCTXCP1_SW0_PRE2_DEFAULT | \
    XTP_LN_TX_LCTXCP1_SW0_PRE3_DEFAULT)
pub const XTP_LN_TX_LCTXCP1_SW1_PRE0_DEFAULT: c_int = 0;

pub const XTP_LN_TX_LCTXCP1_SW2_PRE0_DEFAULT: c_int = 0;

    XTP_LN_TX_LCTXCP1_SW1_PRE1_DEFAULT | \
    XTP_LN_TX_LCTXCP1_SW1_PRE2_DEFAULT | \
    XTP_LN_TX_LCTXCP1_SW2_PRE0_DEFAULT)

pub const XTP_LN_TX_LCTXCP1_SW3_PRE0_DEFAULT: c_int = 0;

    XTP_LN_TX_LCTXCP1_SW3_PRE0_DEFAULT)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_dp_phy {
    pub regs: *mut regmap,
}

#[no_mangle]
unsafe extern "C" fn mtk_dp_phy_init(phy: *mut phy) -> c_int {
    static int mtk_dp_phy_init(struct phy *phy)
    {
    struct mtk_dp_phy *dp_phy = phy_get_drvdata(phy);
    static const u32 driving_params[] = {
    DRIVING_PARAM_3_DEFAULT,
    DRIVING_PARAM_4_DEFAULT,
    DRIVING_PARAM_5_DEFAULT,
    DRIVING_PARAM_6_DEFAULT,
    DRIVING_PARAM_7_DEFAULT,
    DRIVING_PARAM_8_DEFAULT
    };
    regmap_bulk_write(dp_phy.regs, MTK_DP_LANE0_DRIVING_PARAM_3,
    driving_params, ARRAY_SIZE(driving_params));
    regmap_bulk_write(dp_phy.regs, MTK_DP_LANE1_DRIVING_PARAM_3,
    driving_params, ARRAY_SIZE(driving_params));
    regmap_bulk_write(dp_phy.regs, MTK_DP_LANE2_DRIVING_PARAM_3,
    driving_params, ARRAY_SIZE(driving_params));
    regmap_bulk_write(dp_phy.regs, MTK_DP_LANE3_DRIVING_PARAM_3,
    driving_params, ARRAY_SIZE(driving_params));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mtk_dp_phy_configure(phy: *mut phy, opts: *mut union phy_configure_opts) -> c_int {
    static int mtk_dp_phy_configure(struct phy *phy, union phy_configure_opts *opts)
    {
    struct mtk_dp_phy *dp_phy = phy_get_drvdata(phy);
    u32 val;
    if (opts.dp.set_rate) {
    switch (opts.dp.link_rate) {
    default:
    dev_err(&phy.dev,
    "Implementation error, unknown linkrate %x\n",
    opts.dp.link_rate);
    return -EINVAL;
    case 1620:
    val = BIT_RATE_RBR;
    break;
    case 2700:
    val = BIT_RATE_HBR;
    break;
    case 5400:
    val = BIT_RATE_HBR2;
    break;
    case 8100:
    val = BIT_RATE_HBR3;
    break;
    }
    regmap_write(dp_phy.regs, MTK_DP_PHY_DIG_BIT_RATE, val);
    }
    regmap_update_bits(dp_phy.regs, MTK_DP_PHY_DIG_PLL_CTL_1,
    TPLL_SSC_EN, opts.dp.ssc ? TPLL_SSC_EN : 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mtk_dp_phy_reset(phy: *mut phy) -> c_int {
    static int mtk_dp_phy_reset(struct phy *phy)
    {
    struct mtk_dp_phy *dp_phy = phy_get_drvdata(phy);
    regmap_update_bits(dp_phy.regs, MTK_DP_PHY_DIG_SW_RST,
    DP_GLB_SW_RST_PHYD, 0);
    usleep_range(50, 200);
    regmap_update_bits(dp_phy.regs, MTK_DP_PHY_DIG_SW_RST,
    DP_GLB_SW_RST_PHYD, 1);
    return 0;
    }
    static const struct phy_ops mtk_dp_phy_dev_ops = {
    .init = mtk_dp_phy_init,
    .configure = mtk_dp_phy_configure,
    .reset = mtk_dp_phy_reset,
    .owner = THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn mtk_dp_phy_probe(pdev: *mut platform_device) -> c_int {
    static int mtk_dp_phy_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct mtk_dp_phy *dp_phy;
    struct phy *phy;
    struct regmap *regs;
    regs = *(struct regmap **)dev.platform_data;
    if (!regs)
    return dev_err_probe(dev, -EINVAL,
    "No data passed, requires struct regmap**\n");
    dp_phy = devm_kzalloc(dev, sizeof(*dp_phy), GFP_KERNEL);
    if (!dp_phy)
    return -ENOMEM;
    dp_phy.regs = regs;
    phy = devm_phy_create(dev, core::ptr::null_mut(), &mtk_dp_phy_dev_ops);
    if (IS_ERR(phy))
    return dev_err_probe(dev, PTR_ERR(phy),
    "Failed to create DP PHY\n");
    phy_set_drvdata(phy, dp_phy);
    if (!dev.of_node)
    phy_create_lookup(phy, "dp", dev_name(dev));
    return 0;
    }
    static struct platform_driver mtk_dp_phy_driver = {
    .probe = mtk_dp_phy_probe,
    .driver = {
    .name = "mediatek-dp-phy",
    },
    };
    module_platform_driver(mtk_dp_phy_driver);
    MODULE_AUTHOR("Markus Schneider-Pargmann <msp@baylibre.com>");
    MODULE_DESCRIPTION("MediaTek DP PHY Driver");
    MODULE_LICENSE("GPL");
