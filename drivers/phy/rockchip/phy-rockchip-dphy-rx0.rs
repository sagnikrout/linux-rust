//! Automatically rewritten from C to Rust
//! Source: drivers/phy/rockchip/phy-rockchip-dphy-rx0.c
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


// SPDX-License-Identifier: (GPL-2.0+ OR MIT)
//
// Rockchip MIPI Synopsys DPHY RX0 driver
//
// Copyright (C) 2019 Collabora, Ltd.
//
// Based on:
//
// drivers/media/platform/rockchip/isp1/mipi_dphy_sy.c
// in https://chromium.googlesource.com/chromiumos/third_party/kernel,
// chromeos-4.4 branch.
//
// Copyright (C) 2017 Fuzhou Rockchip Electronics Co., Ltd.
// Jacob Chen <jacob2.chen@rock-chips.com>
// Shunqian Zheng <zhengsq@rock-chips.com>
//

pub const RK3399_GRF_SOC_CON9: c_uint = 0x6224;
pub const RK3399_GRF_SOC_CON21: c_uint = 0x6254;
pub const RK3399_GRF_SOC_CON22: c_uint = 0x6258;
pub const RK3399_GRF_SOC_CON23: c_uint = 0x625c;
pub const RK3399_GRF_SOC_CON24: c_uint = 0x6260;
pub const RK3399_GRF_SOC_CON25: c_uint = 0x6264;
pub const RK3399_GRF_SOC_STATUS1: c_uint = 0xe2a4;
pub const CLOCK_LANE_HS_RX_CONTROL: c_uint = 0x34;
pub const LANE0_HS_RX_CONTROL: c_uint = 0x44;
pub const LANE1_HS_RX_CONTROL: c_uint = 0x54;
pub const LANE2_HS_RX_CONTROL: c_uint = 0x84;
pub const LANE3_HS_RX_CONTROL: c_uint = 0x94;
pub const LANES_THS_SETTLE_CONTROL: c_uint = 0x75;
pub const THS_SETTLE_COUNTER_THRESHOLD: c_uint = 0x04;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hsfreq_range {
    pub range_h: u16,
    pub cfg_bit: u8,
}

    static const struct hsfreq_range rk3399_mipidphy_hsfreq_ranges[] = {
    {   89, 0x00 }, {   99, 0x10 }, {  109, 0x20 }, {  129, 0x01 },
    {  139, 0x11 }, {  149, 0x21 }, {  169, 0x02 }, {  179, 0x12 },
    {  199, 0x22 }, {  219, 0x03 }, {  239, 0x13 }, {  249, 0x23 },
    {  269, 0x04 }, {  299, 0x14 }, {  329, 0x05 }, {  359, 0x15 },
    {  399, 0x25 }, {  449, 0x06 }, {  499, 0x16 }, {  549, 0x07 },
    {  599, 0x17 }, {  649, 0x08 }, {  699, 0x18 }, {  749, 0x09 },
    {  799, 0x19 }, {  849, 0x29 }, {  899, 0x39 }, {  949, 0x0a },
    {  999, 0x1a }, { 1049, 0x2a }, { 1099, 0x3a }, { 1149, 0x0b },
    { 1199, 0x1b }, { 1249, 0x2b }, { 1299, 0x3b }, { 1349, 0x0c },
    { 1399, 0x1c }, { 1449, 0x2c }, { 1500, 0x3c }
    };
    static const char * const rk3399_mipidphy_clks[] = {
    "dphy-ref",
    "dphy-cfg",
    "grf",
    };
    enum dphy_reg_id {
    GRF_DPHY_RX0_TURNDISABLE = 0,
    GRF_DPHY_RX0_FORCERXMODE,
    GRF_DPHY_RX0_FORCETXSTOPMODE,
    GRF_DPHY_RX0_ENABLE,
    GRF_DPHY_RX0_TESTCLR,
    GRF_DPHY_RX0_TESTCLK,
    GRF_DPHY_RX0_TESTEN,
    GRF_DPHY_RX0_TESTDIN,
    GRF_DPHY_RX0_TURNREQUEST,
    GRF_DPHY_RX0_TESTDOUT,
    GRF_DPHY_TX0_TURNDISABLE,
    GRF_DPHY_TX0_FORCERXMODE,
    GRF_DPHY_TX0_FORCETXSTOPMODE,
    GRF_DPHY_TX0_TURNREQUEST,
    GRF_DPHY_TX1RX1_TURNDISABLE,
    GRF_DPHY_TX1RX1_FORCERXMODE,
    GRF_DPHY_TX1RX1_FORCETXSTOPMODE,
    GRF_DPHY_TX1RX1_ENABLE,
    GRF_DPHY_TX1RX1_MASTERSLAVEZ,
    GRF_DPHY_TX1RX1_BASEDIR,
    GRF_DPHY_TX1RX1_ENABLECLK,
    GRF_DPHY_TX1RX1_TURNREQUEST,
    GRF_DPHY_RX1_SRC_SEL,
// rk3288 only
    GRF_CON_DISABLE_ISP,
    GRF_CON_ISP_DPHY_SEL,
    GRF_DSI_CSI_TESTBUS_SEL,
    GRF_DVP_V18SEL,
// below is for rk3399 only
    GRF_DPHY_RX0_CLK_INV_SEL,
    GRF_DPHY_RX1_CLK_INV_SEL,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dphy_reg {
    pub offset: u16,
    pub mask: u8,
    pub shift: u8,
}

    { .offset = _offset, .mask = BIT(_width) - 1, .shift = _shift, }
    static const struct dphy_reg rk3399_grf_dphy_regs[] = {
    [GRF_DPHY_RX0_TURNREQUEST] = PHY_REG(RK3399_GRF_SOC_CON9, 4, 0),
    [GRF_DPHY_RX0_CLK_INV_SEL] = PHY_REG(RK3399_GRF_SOC_CON9, 1, 10),
    [GRF_DPHY_RX1_CLK_INV_SEL] = PHY_REG(RK3399_GRF_SOC_CON9, 1, 11),
    [GRF_DPHY_RX0_ENABLE] = PHY_REG(RK3399_GRF_SOC_CON21, 4, 0),
    [GRF_DPHY_RX0_FORCERXMODE] = PHY_REG(RK3399_GRF_SOC_CON21, 4, 4),
    [GRF_DPHY_RX0_FORCETXSTOPMODE] = PHY_REG(RK3399_GRF_SOC_CON21, 4, 8),
    [GRF_DPHY_RX0_TURNDISABLE] = PHY_REG(RK3399_GRF_SOC_CON21, 4, 12),
    [GRF_DPHY_TX0_FORCERXMODE] = PHY_REG(RK3399_GRF_SOC_CON22, 4, 0),
    [GRF_DPHY_TX0_FORCETXSTOPMODE] = PHY_REG(RK3399_GRF_SOC_CON22, 4, 4),
    [GRF_DPHY_TX0_TURNDISABLE] = PHY_REG(RK3399_GRF_SOC_CON22, 4, 8),
    [GRF_DPHY_TX0_TURNREQUEST] = PHY_REG(RK3399_GRF_SOC_CON22, 4, 12),
    [GRF_DPHY_TX1RX1_ENABLE] = PHY_REG(RK3399_GRF_SOC_CON23, 4, 0),
    [GRF_DPHY_TX1RX1_FORCERXMODE] = PHY_REG(RK3399_GRF_SOC_CON23, 4, 4),
    [GRF_DPHY_TX1RX1_FORCETXSTOPMODE] = PHY_REG(RK3399_GRF_SOC_CON23, 4, 8),
    [GRF_DPHY_TX1RX1_TURNDISABLE] = PHY_REG(RK3399_GRF_SOC_CON23, 4, 12),
    [GRF_DPHY_TX1RX1_TURNREQUEST] = PHY_REG(RK3399_GRF_SOC_CON24, 4, 0),
    [GRF_DPHY_RX1_SRC_SEL] = PHY_REG(RK3399_GRF_SOC_CON24, 1, 4),
    [GRF_DPHY_TX1RX1_BASEDIR] = PHY_REG(RK3399_GRF_SOC_CON24, 1, 5),
    [GRF_DPHY_TX1RX1_ENABLECLK] = PHY_REG(RK3399_GRF_SOC_CON24, 1, 6),
    [GRF_DPHY_TX1RX1_MASTERSLAVEZ] = PHY_REG(RK3399_GRF_SOC_CON24, 1, 7),
    [GRF_DPHY_RX0_TESTDIN] = PHY_REG(RK3399_GRF_SOC_CON25, 8, 0),
    [GRF_DPHY_RX0_TESTEN] = PHY_REG(RK3399_GRF_SOC_CON25, 1, 8),
    [GRF_DPHY_RX0_TESTCLK] = PHY_REG(RK3399_GRF_SOC_CON25, 1, 9),
    [GRF_DPHY_RX0_TESTCLR] = PHY_REG(RK3399_GRF_SOC_CON25, 1, 10),
    [GRF_DPHY_RX0_TESTDOUT] = PHY_REG(RK3399_GRF_SOC_STATUS1, 8, 0),
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rk_dphy_drv_data {
    pub clks: *const *const c_char,
    pub num_clks: c_uint,
    pub hsfreq_ranges: *const hsfreq_range,
    pub num_hsfreq_ranges: c_uint,
    pub regs: *const dphy_reg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rk_dphy {
    pub dev: *mut device,
    pub grf: *mut regmap,
    pub clks: *mut clk_bulk_data,
    pub drv_data: *const rk_dphy_drv_data,
    pub config: phy_configure_opts_mipi_dphy,
    pub hsfreq: u8,
}

    static inline void rk_dphy_write_grf(struct rk_dphy *priv,
    unsigned int index, u8 value)
    {
    const struct dphy_reg *reg = &priv.drv_data.regs[index];
// Update high word
    unsigned int val = (value << reg.shift) |
    (reg.mask << (reg.shift + 16));
    if (WARN_ON(!reg.offset))
    return;
    regmap_write(priv.grf, reg.offset, val);
    }
#[no_mangle]
unsafe extern "C" fn rk_dphy_write(priv: *mut rk_dphy, test_code: u8, test_data: u8) {
    static void rk_dphy_write(struct rk_dphy *priv, u8 test_code, u8 test_data)
    {
    rk_dphy_write_grf(priv, GRF_DPHY_RX0_TESTDIN, test_code);
    rk_dphy_write_grf(priv, GRF_DPHY_RX0_TESTEN, 1);
//
// With the falling edge on TESTCLK, the TESTDIN[7:0] signal content
// is latched internally as the current test code. Test data is
// programmed internally by rising edge on TESTCLK.
// This code assumes that TESTCLK is already 1.
//
    rk_dphy_write_grf(priv, GRF_DPHY_RX0_TESTCLK, 0);
    rk_dphy_write_grf(priv, GRF_DPHY_RX0_TESTEN, 0);
    rk_dphy_write_grf(priv, GRF_DPHY_RX0_TESTDIN, test_data);
    rk_dphy_write_grf(priv, GRF_DPHY_RX0_TESTCLK, 1);
    }
#[no_mangle]
unsafe extern "C" fn rk_dphy_enable(priv: *mut rk_dphy) {
    static void rk_dphy_enable(struct rk_dphy *priv)
    {
    rk_dphy_write_grf(priv, GRF_DPHY_RX0_FORCERXMODE, 0);
    rk_dphy_write_grf(priv, GRF_DPHY_RX0_FORCETXSTOPMODE, 0);
// Disable lane turn around, which is ignored in receive mode
    rk_dphy_write_grf(priv, GRF_DPHY_RX0_TURNREQUEST, 0);
    rk_dphy_write_grf(priv, GRF_DPHY_RX0_TURNDISABLE, 0xf);
    rk_dphy_write_grf(priv, GRF_DPHY_RX0_ENABLE,
    GENMASK(priv.config.lanes - 1, 0));
// dphy start
    rk_dphy_write_grf(priv, GRF_DPHY_RX0_TESTCLK, 1);
    rk_dphy_write_grf(priv, GRF_DPHY_RX0_TESTCLR, 1);
    usleep_range(100, 150);
    rk_dphy_write_grf(priv, GRF_DPHY_RX0_TESTCLR, 0);
    usleep_range(100, 150);
// set clock lane
// HS hsfreq_range & lane 0  settle bypass
    rk_dphy_write(priv, CLOCK_LANE_HS_RX_CONTROL, 0);
// HS RX Control of lane0
    rk_dphy_write(priv, LANE0_HS_RX_CONTROL, priv.hsfreq << 1);
// HS RX Control of lane1
    rk_dphy_write(priv, LANE1_HS_RX_CONTROL, priv.hsfreq << 1);
// HS RX Control of lane2
    rk_dphy_write(priv, LANE2_HS_RX_CONTROL, priv.hsfreq << 1);
// HS RX Control of lane3
    rk_dphy_write(priv, LANE3_HS_RX_CONTROL, priv.hsfreq << 1);
// HS RX Data Lanes Settle State Time Control
    rk_dphy_write(priv, LANES_THS_SETTLE_CONTROL,
    THS_SETTLE_COUNTER_THRESHOLD);
// Normal operation
    rk_dphy_write(priv, 0x0, 0);
    }
#[no_mangle]
unsafe extern "C" fn rk_dphy_configure(phy: *mut phy, opts: *mut union phy_configure_opts) -> c_int {
    static int rk_dphy_configure(struct phy *phy, union phy_configure_opts *opts)
    {
    struct rk_dphy *priv = phy_get_drvdata(phy);
    const struct rk_dphy_drv_data *drv_data = priv.drv_data;
    struct phy_configure_opts_mipi_dphy *config = &opts.mipi_dphy;
    let mut hsfreq: c_uint = 0;
    unsigned int i;
    u64 data_rate_mbps;
    int ret;
// pass with phy_mipi_dphy_get_default_config (with pixel rate?)
    ret = phy_mipi_dphy_config_validate(config);
    if (ret)
    return ret;
    data_rate_mbps = div_u64(config.hs_clk_rate, 1000 * 1000);
    dev_dbg(priv.dev, "lanes %d - data_rate_mbps %llu\n",
    config.lanes, data_rate_mbps);
    for (i = 0; i < drv_data.num_hsfreq_ranges; i++) {
    if (drv_data.hsfreq_ranges[i].range_h >= data_rate_mbps) {
    hsfreq = drv_data.hsfreq_ranges[i].cfg_bit;
    break;
    }
    }
    if (!hsfreq)
    return -EINVAL;
    priv.hsfreq = hsfreq;
    priv.config = *config;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rk_dphy_power_on(phy: *mut phy) -> c_int {
    static int rk_dphy_power_on(struct phy *phy)
    {
    struct rk_dphy *priv = phy_get_drvdata(phy);
    int ret;
    ret = clk_bulk_enable(priv.drv_data.num_clks, priv.clks);
    if (ret)
    return ret;
    rk_dphy_enable(priv);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rk_dphy_power_off(phy: *mut phy) -> c_int {
    static int rk_dphy_power_off(struct phy *phy)
    {
    struct rk_dphy *priv = phy_get_drvdata(phy);
    rk_dphy_write_grf(priv, GRF_DPHY_RX0_ENABLE, 0);
    clk_bulk_disable(priv.drv_data.num_clks, priv.clks);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rk_dphy_init(phy: *mut phy) -> c_int {
    static int rk_dphy_init(struct phy *phy)
    {
    struct rk_dphy *priv = phy_get_drvdata(phy);
    return clk_bulk_prepare(priv.drv_data.num_clks, priv.clks);
    }
#[no_mangle]
unsafe extern "C" fn rk_dphy_exit(phy: *mut phy) -> c_int {
    static int rk_dphy_exit(struct phy *phy)
    {
    struct rk_dphy *priv = phy_get_drvdata(phy);
    clk_bulk_unprepare(priv.drv_data.num_clks, priv.clks);
    return 0;
    }
    static const struct phy_ops rk_dphy_ops = {
    .power_on	= rk_dphy_power_on,
    .power_off	= rk_dphy_power_off,
    .init		= rk_dphy_init,
    .exit		= rk_dphy_exit,
    .configure	= rk_dphy_configure,
    .owner		= THIS_MODULE,
    };
    static const struct rk_dphy_drv_data rk3399_mipidphy_drv_data = {
    .clks = rk3399_mipidphy_clks,
    .num_clks = ARRAY_SIZE(rk3399_mipidphy_clks),
    .hsfreq_ranges = rk3399_mipidphy_hsfreq_ranges,
    .num_hsfreq_ranges = ARRAY_SIZE(rk3399_mipidphy_hsfreq_ranges),
    .regs = rk3399_grf_dphy_regs,
    };
    static const struct of_device_id rk_dphy_dt_ids[] = {
    {
    .compatible = "rockchip,rk3399-mipi-dphy-rx0",
    .data = &rk3399_mipidphy_drv_data,
    },
    {}
    };
    MODULE_DEVICE_TABLE(of, rk_dphy_dt_ids);
#[no_mangle]
unsafe extern "C" fn rk_dphy_probe(pdev: *mut platform_device) -> c_int {
    static int rk_dphy_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *np = dev.of_node;
    const struct rk_dphy_drv_data *drv_data;
    struct phy_provider *phy_provider;
    struct rk_dphy *priv;
    struct phy *phy;
    unsigned int i;
    int ret;
    if (!dev.parent || !dev.parent.of_node)
    return -ENODEV;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.dev = dev;
    priv.grf = syscon_node_to_regmap(dev.parent.of_node);
    if (IS_ERR(priv.grf)) {
    dev_err(dev, "Can't find GRF syscon\n");
    return -ENODEV;
    }
    drv_data = of_device_get_match_data(dev);
    priv.drv_data = drv_data;
    priv.clks = devm_kcalloc(&pdev.dev, drv_data.num_clks,
    sizeof(*priv.clks), GFP_KERNEL);
    if (!priv.clks)
    return -ENOMEM;
    for (i = 0; i < drv_data.num_clks; i++)
    priv.clks[i].id = drv_data.clks[i];
    ret = devm_clk_bulk_get(&pdev.dev, drv_data.num_clks, priv.clks);
    if (ret)
    return ret;
    phy = devm_phy_create(dev, np, &rk_dphy_ops);
    if (IS_ERR(phy)) {
    dev_err(dev, "failed to create phy\n");
    return PTR_ERR(phy);
    }
    phy_set_drvdata(phy, priv);
    phy_provider = devm_of_phy_provider_register(dev, of_phy_simple_xlate);
    return PTR_ERR_OR_ZERO(phy_provider);
    }
    static struct platform_driver rk_dphy_driver = {
    .probe = rk_dphy_probe,
    .driver = {
    .name	= "rockchip-mipi-dphy-rx0",
    .of_match_table = rk_dphy_dt_ids,
    },
    };
    module_platform_driver(rk_dphy_driver);
    MODULE_AUTHOR("Ezequiel Garcia <ezequiel@collabora.com>");
    MODULE_DESCRIPTION("Rockchip MIPI Synopsys DPHY RX0 driver");
    MODULE_LICENSE("Dual MIT/GPL");
