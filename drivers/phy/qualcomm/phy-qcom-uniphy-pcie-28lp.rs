//! Automatically rewritten from C to Rust
//! Source: drivers/phy/qualcomm/phy-qcom-uniphy-pcie-28lp.c
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
// Copyright (c) 2025, The Linux Foundation. All rights reserved.
//

pub const RST_ASSERT_DELAY_MIN_US: c_int = 100;
pub const RST_ASSERT_DELAY_MAX_US: c_int = 150;
pub const PIPE_CLK_DELAY_MIN_US: c_int = 5000;
pub const PIPE_CLK_DELAY_MAX_US: c_int = 5100;
pub const CLK_EN_DELAY_MIN_US: c_int = 30;
pub const CLK_EN_DELAY_MAX_US: c_int = 50;
pub const CDR_CTRL_REG_1: c_uint = 0x80;
pub const CDR_CTRL_REG_2: c_uint = 0x84;
pub const CDR_CTRL_REG_3: c_uint = 0x88;
pub const CDR_CTRL_REG_4: c_uint = 0x8c;
pub const CDR_CTRL_REG_5: c_uint = 0x90;
pub const CDR_CTRL_REG_6: c_uint = 0x94;
pub const CDR_CTRL_REG_7: c_uint = 0x98;
pub const SSCG_CTRL_REG_1: c_uint = 0x9c;
pub const SSCG_CTRL_REG_2: c_uint = 0xa0;
pub const SSCG_CTRL_REG_3: c_uint = 0xa4;
pub const SSCG_CTRL_REG_4: c_uint = 0xa8;
pub const SSCG_CTRL_REG_5: c_uint = 0xac;
pub const SSCG_CTRL_REG_6: c_uint = 0xb0;
pub const PCS_INTERNAL_CONTROL_2: c_uint = 0x2d8;
pub const PHY_CFG_PLLCFG: c_uint = 0x220;
pub const PHY_CFG_EIOS_DTCT_REG: c_uint = 0x3e4;
pub const PHY_CFG_GEN3_ALIGN_HOLDOFF_TIME: c_uint = 0x3e8;
    enum qcom_uniphy_pcie_type {
    PHY_TYPE_PCIE = 1,
    PHY_TYPE_PCIE_GEN2,
    PHY_TYPE_PCIE_GEN3,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_uniphy_pcie_regs {
    pub offset: u32,
    pub val: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_uniphy_pcie_data {
    pub /: *mut *mut int lane_offset; / offset between the lane register bases,
    pub phy_type: u32,
    pub init_seq: *const qcom_uniphy_pcie_regs,
    pub init_seq_num: u32,
    pub pipe_clk_rate: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_uniphy_pcie {
    pub phy: phy,
    pub dev: *mut device,
    pub data: *const qcom_uniphy_pcie_data,
    pub clks: *mut clk_bulk_data,
    pub num_clks: c_int,
    pub resets: *mut reset_control,
    pub base: *mut void __iomem,
    pub lanes: c_int,
}

    static const struct qcom_uniphy_pcie_regs ipq5018_regs[] = {
    {
    .offset = SSCG_CTRL_REG_4,
    .val = 0x1cb9,
    }, {
    .offset = SSCG_CTRL_REG_5,
    .val = 0x023a,
    }, {
    .offset = SSCG_CTRL_REG_3,
    .val = 0xd360,
    }, {
    .offset = SSCG_CTRL_REG_1,
    .val = 0x1,
    }, {
    .offset = SSCG_CTRL_REG_2,
    .val = 0xeb,
    }, {
    .offset = CDR_CTRL_REG_4,
    .val = 0x3f9,
    }, {
    .offset = CDR_CTRL_REG_5,
    .val = 0x1c9,
    }, {
    .offset = CDR_CTRL_REG_2,
    .val = 0x419,
    }, {
    .offset = CDR_CTRL_REG_1,
    .val = 0x200,
    }, {
    .offset = PCS_INTERNAL_CONTROL_2,
    .val = 0xf101,
    },
    };
    static const struct qcom_uniphy_pcie_regs ipq5332_regs[] = {
    {
    .offset = PHY_CFG_PLLCFG,
    .val = 0x30,
    }, {
    .offset = PHY_CFG_EIOS_DTCT_REG,
    .val = 0x53ef,
    }, {
    .offset = PHY_CFG_GEN3_ALIGN_HOLDOFF_TIME,
    .val = 0xcf,
    },
    };
    static const struct qcom_uniphy_pcie_data ipq5018_data = {
    .lane_offset	= 0x800,
    .phy_type	= PHY_TYPE_PCIE_GEN2,
    .init_seq	= ipq5018_regs,
    .init_seq_num	= ARRAY_SIZE(ipq5018_regs),
    .pipe_clk_rate	= 125 * MEGA,
    };
    static const struct qcom_uniphy_pcie_data ipq5332_data = {
    .lane_offset	= 0x800,
    .phy_type	= PHY_TYPE_PCIE_GEN3,
    .init_seq	= ipq5332_regs,
    .init_seq_num	= ARRAY_SIZE(ipq5332_regs),
    .pipe_clk_rate	= 250 * MEGA,
    };
#[no_mangle]
unsafe extern "C" fn qcom_uniphy_pcie_init(phy: *mut qcom_uniphy_pcie) {
    static void qcom_uniphy_pcie_init(struct qcom_uniphy_pcie *phy)
    {
    const struct qcom_uniphy_pcie_data *data = phy.data;
    const struct qcom_uniphy_pcie_regs *init_seq;
    void __iomem *base = phy.base;
    int lane, i;
    for (lane = 0; lane < phy.lanes; lane++) {
    init_seq = data.init_seq;
    for (i = 0; i < data.init_seq_num; i++)
    writel(init_seq[i].val, base + init_seq[i].offset);
    base += data.lane_offset;
    }
    }
#[no_mangle]
unsafe extern "C" fn qcom_uniphy_pcie_power_off(x: *mut phy) -> c_int {
    static int qcom_uniphy_pcie_power_off(struct phy *x)
    {
    struct qcom_uniphy_pcie *phy = phy_get_drvdata(x);
    clk_bulk_disable_unprepare(phy.num_clks, phy.clks);
    return reset_control_assert(phy.resets);
    }
#[no_mangle]
unsafe extern "C" fn qcom_uniphy_pcie_power_on(x: *mut phy) -> c_int {
    static int qcom_uniphy_pcie_power_on(struct phy *x)
    {
    struct qcom_uniphy_pcie *phy = phy_get_drvdata(x);
    int ret;
    ret = reset_control_assert(phy.resets);
    if (ret) {
    dev_err(phy.dev, "reset assert failed (%d)\n", ret);
    return ret;
    }
    usleep_range(RST_ASSERT_DELAY_MIN_US, RST_ASSERT_DELAY_MAX_US);
    ret = reset_control_deassert(phy.resets);
    if (ret) {
    dev_err(phy.dev, "reset deassert failed (%d)\n", ret);
    return ret;
    }
    usleep_range(PIPE_CLK_DELAY_MIN_US, PIPE_CLK_DELAY_MAX_US);
    ret = clk_bulk_prepare_enable(phy.num_clks, phy.clks);
    if (ret) {
    dev_err(phy.dev, "clk prepare and enable failed %d\n", ret);
    return ret;
    }
    usleep_range(CLK_EN_DELAY_MIN_US, CLK_EN_DELAY_MAX_US);
    qcom_uniphy_pcie_init(phy);
    return 0;
    }
    static inline int qcom_uniphy_pcie_get_resources(struct platform_device *pdev,
    struct qcom_uniphy_pcie *phy)
    {
    struct resource *res;
    phy.base = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(phy.base))
    return PTR_ERR(phy.base);
    phy.num_clks = devm_clk_bulk_get_all(phy.dev, &phy.clks);
    if (phy.num_clks < 0)
    return phy.num_clks;
    phy.resets = devm_reset_control_array_get_exclusive(phy.dev);
    if (IS_ERR(phy.resets))
    return PTR_ERR(phy.resets);
    return 0;
    }
//
// Register a fixed rate pipe clock.
//
// The <s>_pipe_clksrc generated by PHY goes to the GCC that gate
// controls it. The <s>_pipe_clk coming out of the GCC is requested
// by the PHY driver for its operations.
// We register the <s>_pipe_clksrc here. The gcc driver takes care
// of assigning this <s>_pipe_clksrc as parent to <s>_pipe_clk.
// Below picture shows this relationship.
//
// +---------------+
// |   PHY block   |<<---------------------------------------+
// |               |                                         |
// |   +-------+   |                   +-----+               |
// I/P---^-->|  PLL  |---^--->pipe_clksrc--->| GCC |--->pipe_clk---+
// clk  |   +-------+   |                   +-----+
// +---------------+
//
#[no_mangle]
pub unsafe extern "C" fn phy_pipe_clk_register(phy: *mut qcom_uniphy_pcie, id: c_int) -> c_int {
    static inline int phy_pipe_clk_register(struct qcom_uniphy_pcie *phy, int id)
    {
    const struct qcom_uniphy_pcie_data *data = phy.data;
    struct clk_hw *hw;
    char name[64];
    snprintf(name, sizeof(name), "phy%d_pipe_clk_src", id);
    hw = devm_clk_hw_register_fixed_rate(phy.dev, name, core::ptr::null_mut(), 0,
    data.pipe_clk_rate);
    if (IS_ERR(hw))
    return dev_err_probe(phy.dev, PTR_ERR(hw),
    "Unable to register %s\n", name);
    return devm_of_clk_add_hw_provider(phy.dev, of_clk_hw_simple_get, hw);
    }
    static const struct of_device_id qcom_uniphy_pcie_id_table[] = {
    {
    .compatible = "qcom,ipq5018-uniphy-pcie-phy",
    .data = &ipq5018_data,
    }, {
    .compatible = "qcom,ipq5332-uniphy-pcie-phy",
    .data = &ipq5332_data,
    }, {
// Sentinel
    },
    };
    MODULE_DEVICE_TABLE(of, qcom_uniphy_pcie_id_table);
    static const struct phy_ops pcie_ops = {
    .power_on	= qcom_uniphy_pcie_power_on,
    .power_off	= qcom_uniphy_pcie_power_off,
    .owner          = THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn qcom_uniphy_pcie_probe(pdev: *mut platform_device) -> c_int {
    static int qcom_uniphy_pcie_probe(struct platform_device *pdev)
    {
    struct phy_provider *phy_provider;
    struct device *dev = &pdev.dev;
    struct qcom_uniphy_pcie *phy;
    struct phy *generic_phy;
    int ret;
    phy = devm_kzalloc(&pdev.dev, sizeof(*phy), GFP_KERNEL);
    if (!phy)
    return -ENOMEM;
    platform_set_drvdata(pdev, phy);
    phy.dev = &pdev.dev;
    phy.data = of_device_get_match_data(dev);
    if (!phy.data)
    return -EINVAL;
    ret = of_property_read_u32(dev_of_node(dev), "num-lanes", &phy.lanes);
    if (ret)
    return dev_err_probe(dev, ret, "Couldn't read num-lanes\n");
    ret = qcom_uniphy_pcie_get_resources(pdev, phy);
    if (ret < 0)
    return dev_err_probe(&pdev.dev, ret,
    "failed to get resources: %d\n", ret);
    generic_phy = devm_phy_create(phy.dev, core::ptr::null_mut(), &pcie_ops);
    if (IS_ERR(generic_phy))
    return PTR_ERR(generic_phy);
    phy_set_drvdata(generic_phy, phy);
    ret = phy_pipe_clk_register(phy, generic_phy.id);
    if (ret)
    dev_err(&pdev.dev, "failed to register phy pipe clk\n");
    phy_provider = devm_of_phy_provider_register(phy.dev,
    of_phy_simple_xlate);
    if (IS_ERR(phy_provider))
    return PTR_ERR(phy_provider);
    return 0;
    }
    static struct platform_driver qcom_uniphy_pcie_driver = {
    .probe		= qcom_uniphy_pcie_probe,
    .driver		= {
    .name	= "qcom-uniphy-pcie",
    .of_match_table = qcom_uniphy_pcie_id_table,
    },
    };
    module_platform_driver(qcom_uniphy_pcie_driver);
    MODULE_DESCRIPTION("PCIE QCOM UNIPHY driver");
    MODULE_LICENSE("GPL");
