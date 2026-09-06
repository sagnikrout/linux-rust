//! Automatically rewritten from C to Rust
//! Source: drivers/phy/qualcomm/phy-qcom-pcie2.c
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
// Copyright (c) 2014-2017, The Linux Foundation. All rights reserved.
// Copyright (c) 2019, Linaro Ltd.
//

pub const PCIE20_PARF_PHY_STTS: c_uint = 0x3c;
pub const PCIE2_PHY_RESET_CTRL: c_uint = 0x44;
pub const PCIE20_PARF_PHY_REFCLK_CTRL2: c_uint = 0xa0;
pub const PCIE20_PARF_PHY_REFCLK_CTRL3: c_uint = 0xa4;
pub const PCIE20_PARF_PCS_SWING_CTRL1: c_uint = 0x88;
pub const PCIE20_PARF_PCS_SWING_CTRL2: c_uint = 0x8c;
pub const PCIE20_PARF_PCS_DEEMPH1: c_uint = 0x74;
pub const PCIE20_PARF_PCS_DEEMPH2: c_uint = 0x78;
pub const PCIE20_PARF_PCS_DEEMPH3: c_uint = 0x7c;
pub const PCIE20_PARF_CONFIGBITS: c_uint = 0x84;
pub const PCIE20_PARF_PHY_CTRL3: c_uint = 0x94;
pub const PCIE20_PARF_PCS_CTRL: c_uint = 0x80;
pub const TX_AMP_VAL: c_int = 120;
pub const PHY_RX0_EQ_GEN1_VAL: c_int = 0;
pub const PHY_RX0_EQ_GEN2_VAL: c_int = 4;
pub const TX_DEEMPH_GEN1_VAL: c_int = 24;
pub const TX_DEEMPH_GEN2_3_5DB_VAL: c_int = 26;
pub const TX_DEEMPH_GEN2_6DB_VAL: c_int = 36;
pub const PHY_TX0_TERM_OFFST_VAL: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_phy {
    pub dev: *mut device,
    pub base: *mut void __iomem,
    pub vregs: [regulator_bulk_data; 2],
    pub phy_reset: *mut reset_control,
    pub pipe_reset: *mut reset_control,
    pub pipe_clk: *mut clk,
}

#[no_mangle]
unsafe extern "C" fn qcom_pcie2_phy_init(phy: *mut phy) -> c_int {
    static int qcom_pcie2_phy_init(struct phy *phy)
    {
    struct qcom_phy *qphy = phy_get_drvdata(phy);
    int ret;
    ret = reset_control_deassert(qphy.phy_reset);
    if (ret) {
    dev_err(qphy.dev, "cannot deassert pipe reset\n");
    return ret;
    }
    ret = regulator_bulk_enable(ARRAY_SIZE(qphy.vregs), qphy.vregs);
    if (ret)
    reset_control_assert(qphy.phy_reset);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie2_phy_power_on(phy: *mut phy) -> c_int {
    static int qcom_pcie2_phy_power_on(struct phy *phy)
    {
    struct qcom_phy *qphy = phy_get_drvdata(phy);
    int ret;
    u32 val;
// Program REF_CLK source
    val = readl(qphy.base + PCIE20_PARF_PHY_REFCLK_CTRL2);
    val &= ~BIT(1);
    writel(val, qphy.base + PCIE20_PARF_PHY_REFCLK_CTRL2);
    usleep_range(1000, 2000);
// Don't use PAD for refclock
    val = readl(qphy.base + PCIE20_PARF_PHY_REFCLK_CTRL2);
    val &= ~BIT(0);
    writel(val, qphy.base + PCIE20_PARF_PHY_REFCLK_CTRL2);
// Program SSP ENABLE
    val = readl(qphy.base + PCIE20_PARF_PHY_REFCLK_CTRL3);
    val |= BIT(0);
    writel(val, qphy.base + PCIE20_PARF_PHY_REFCLK_CTRL3);
    usleep_range(1000, 2000);
// Assert Phy SW Reset
    val = readl(qphy.base + PCIE2_PHY_RESET_CTRL);
    val |= BIT(0);
    writel(val, qphy.base + PCIE2_PHY_RESET_CTRL);
// Program Tx Amplitude
    val = readl(qphy.base + PCIE20_PARF_PCS_SWING_CTRL1);
    val &= ~0x7f;
    val |= TX_AMP_VAL;
    writel(val, qphy.base + PCIE20_PARF_PCS_SWING_CTRL1);
    val = readl(qphy.base + PCIE20_PARF_PCS_SWING_CTRL2);
    val &= ~0x7f;
    val |= TX_AMP_VAL;
    writel(val, qphy.base + PCIE20_PARF_PCS_SWING_CTRL2);
// Program De-Emphasis
    val = readl(qphy.base + PCIE20_PARF_PCS_DEEMPH1);
    val &= ~0x3f;
    val |= TX_DEEMPH_GEN2_6DB_VAL;
    writel(val, qphy.base + PCIE20_PARF_PCS_DEEMPH1);
    val = readl(qphy.base + PCIE20_PARF_PCS_DEEMPH2);
    val &= ~0x3f;
    val |= TX_DEEMPH_GEN2_3_5DB_VAL;
    writel(val, qphy.base + PCIE20_PARF_PCS_DEEMPH2);
    val = readl(qphy.base + PCIE20_PARF_PCS_DEEMPH3);
    val &= ~0x3f;
    val |= TX_DEEMPH_GEN1_VAL;
    writel(val, qphy.base + PCIE20_PARF_PCS_DEEMPH3);
// Program Rx_Eq
    val = readl(qphy.base + PCIE20_PARF_CONFIGBITS);
    val &= ~0x7;
    val |= PHY_RX0_EQ_GEN2_VAL;
    writel(val, qphy.base + PCIE20_PARF_CONFIGBITS);
// Program Tx0_term_offset
    val = readl(qphy.base + PCIE20_PARF_PHY_CTRL3);
    val &= ~0x1f;
    val |= PHY_TX0_TERM_OFFST_VAL;
    writel(val, qphy.base + PCIE20_PARF_PHY_CTRL3);
// disable Tx2Rx Loopback
    val = readl(qphy.base + PCIE20_PARF_PCS_CTRL);
    val &= ~BIT(1);
    writel(val, qphy.base + PCIE20_PARF_PCS_CTRL);
// De-assert Phy SW Reset
    val = readl(qphy.base + PCIE2_PHY_RESET_CTRL);
    val &= ~BIT(0);
    writel(val, qphy.base + PCIE2_PHY_RESET_CTRL);
    usleep_range(1000, 2000);
    ret = reset_control_deassert(qphy.pipe_reset);
    if (ret) {
    dev_err(qphy.dev, "cannot deassert pipe reset\n");
    goto out;
    }
    clk_set_rate(qphy.pipe_clk, 250000000);
    ret = clk_prepare_enable(qphy.pipe_clk);
    if (ret) {
    dev_err(qphy.dev, "failed to enable pipe clock\n");
    goto out;
    }
    ret = readl_poll_timeout(qphy.base + PCIE20_PARF_PHY_STTS, val,
    !(val & BIT(0)), 1000, 10);
    if (ret)
    dev_err(qphy.dev, "phy initialization failed\n");
    out:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie2_phy_power_off(phy: *mut phy) -> c_int {
    static int qcom_pcie2_phy_power_off(struct phy *phy)
    {
    struct qcom_phy *qphy = phy_get_drvdata(phy);
    u32 val;
    val = readl(qphy.base + PCIE2_PHY_RESET_CTRL);
    val |= BIT(0);
    writel(val, qphy.base + PCIE2_PHY_RESET_CTRL);
    clk_disable_unprepare(qphy.pipe_clk);
    reset_control_assert(qphy.pipe_reset);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie2_phy_exit(phy: *mut phy) -> c_int {
    static int qcom_pcie2_phy_exit(struct phy *phy)
    {
    struct qcom_phy *qphy = phy_get_drvdata(phy);
    regulator_bulk_disable(ARRAY_SIZE(qphy.vregs), qphy.vregs);
    reset_control_assert(qphy.phy_reset);
    return 0;
    }
    static const struct phy_ops qcom_pcie2_ops = {
    .init = qcom_pcie2_phy_init,
    .power_on = qcom_pcie2_phy_power_on,
    .power_off = qcom_pcie2_phy_power_off,
    .exit = qcom_pcie2_phy_exit,
    .owner = THIS_MODULE,
    };
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
unsafe extern "C" fn phy_pipe_clksrc_register(qphy: *mut qcom_phy) -> c_int {
    static int phy_pipe_clksrc_register(struct qcom_phy *qphy)
    {
    struct device_node *np = qphy.dev.of_node;
    struct clk_fixed_rate *fixed;
    let mut init: clk_init_data = { };
    int ret;
    ret = of_property_read_string(np, "clock-output-names", &init.name);
    if (ret) {
    dev_err(qphy.dev, "%s: No clock-output-names\n", np.name);
    return ret;
    }
    fixed = devm_kzalloc(qphy.dev, sizeof(*fixed), GFP_KERNEL);
    if (!fixed)
    return -ENOMEM;
    init.ops = &clk_fixed_rate_ops;
// controllers using QMP phys use 250MHz pipe clock interface
    fixed.fixed_rate = 250000000;
    fixed.hw.init = &init;
    ret = devm_clk_hw_register(qphy.dev, &fixed.hw);
    if (ret < 0)
    return ret;
    return devm_of_clk_add_hw_provider(qphy.dev, of_clk_hw_simple_get, &fixed.hw);
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie2_phy_probe(pdev: *mut platform_device) -> c_int {
    static int qcom_pcie2_phy_probe(struct platform_device *pdev)
    {
    struct phy_provider *phy_provider;
    struct qcom_phy *qphy;
    struct device *dev = &pdev.dev;
    struct phy *phy;
    int ret;
    qphy = devm_kzalloc(dev, sizeof(*qphy), GFP_KERNEL);
    if (!qphy)
    return -ENOMEM;
    qphy.dev = dev;
    qphy.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(qphy.base))
    return PTR_ERR(qphy.base);
    ret = phy_pipe_clksrc_register(qphy);
    if (ret) {
    dev_err(dev, "failed to register pipe_clk\n");
    return ret;
    }
    qphy.vregs[0].supply = "vdda-vp";
    qphy.vregs[1].supply = "vdda-vph";
    ret = devm_regulator_bulk_get(dev, ARRAY_SIZE(qphy.vregs), qphy.vregs);
    if (ret < 0)
    return ret;
    qphy.pipe_clk = devm_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(qphy.pipe_clk)) {
    dev_err(dev, "failed to acquire pipe clock\n");
    return PTR_ERR(qphy.pipe_clk);
    }
    qphy.phy_reset = devm_reset_control_get_exclusive(dev, "phy");
    if (IS_ERR(qphy.phy_reset)) {
    dev_err(dev, "failed to acquire phy reset\n");
    return PTR_ERR(qphy.phy_reset);
    }
    qphy.pipe_reset = devm_reset_control_get_exclusive(dev, "pipe");
    if (IS_ERR(qphy.pipe_reset)) {
    dev_err(dev, "failed to acquire pipe reset\n");
    return PTR_ERR(qphy.pipe_reset);
    }
    phy = devm_phy_create(dev, dev.of_node, &qcom_pcie2_ops);
    if (IS_ERR(phy)) {
    dev_err(dev, "failed to create phy\n");
    return PTR_ERR(phy);
    }
    phy_set_drvdata(phy, qphy);
    phy_provider = devm_of_phy_provider_register(dev, of_phy_simple_xlate);
    if (IS_ERR(phy_provider))
    dev_err(dev, "failed to register phy provider\n");
    return PTR_ERR_OR_ZERO(phy_provider);
    }
    static const struct of_device_id qcom_pcie2_phy_match_table[] = {
    { .compatible = "qcom,pcie2-phy" },
    {}
    };
    MODULE_DEVICE_TABLE(of, qcom_pcie2_phy_match_table);
    static struct platform_driver qcom_pcie2_phy_driver = {
    .probe = qcom_pcie2_phy_probe,
    .driver = {
    .name = "phy-qcom-pcie2",
    .of_match_table = qcom_pcie2_phy_match_table,
    },
    };
    module_platform_driver(qcom_pcie2_phy_driver);
    MODULE_DESCRIPTION("Qualcomm PCIe PHY driver");
    MODULE_LICENSE("GPL v2");
