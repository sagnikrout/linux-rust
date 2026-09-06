//! Automatically rewritten from C to Rust
//! Source: drivers/phy/rockchip/phy-rockchip-pcie.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Rockchip PCIe PHY driver
//
// Copyright (C) 2016 Shawn Lin <shawn.lin@rock-chips.com>
// Copyright (C) 2016 ROCKCHIP, Inc.
//

pub const PHY_MAX_LANE_NUM: c_int = 4;

pub const PHY_CFG_WR_ENABLE: c_int = 1;
pub const PHY_CFG_WR_DISABLE: c_int = 0;

pub const PHY_CFG_PLL_LOCK: c_uint = 0x10;
pub const PHY_CFG_CLK_TEST: c_uint = 0x10;
pub const PHY_CFG_CLK_SCC: c_uint = 0x12;

pub const PHY_LANE_A_STATUS: c_uint = 0x30;
pub const PHY_LANE_B_STATUS: c_uint = 0x31;
pub const PHY_LANE_C_STATUS: c_uint = 0x32;
pub const PHY_LANE_D_STATUS: c_uint = 0x33;
pub const PHY_LANE_RX_DET_SHIFT: c_int = 11;
pub const PHY_LANE_RX_DET_TH: c_uint = 0x1;
pub const PHY_LANE_IDLE_OFF: c_uint = 0x1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rockchip_pcie_data {
    pub pcie_conf: c_uint,
    pub pcie_status: c_uint,
    pub pcie_laneoff: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rockchip_pcie_phy {
    pub phy_data: *const rockchip_pcie_data,
    pub reg_base: *mut regmap,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_pcie_instance {
    pub phy: *mut phy,
    pub index: u32,
    pub phys: [}; PHY_MAX_LANE_NUM],
    pub pcie_mutex: mutex,
    pub phy_rst: *mut reset_control,
    pub clk_pciephy_ref: *mut clk,
    pub pwr_cnt: c_int,
    pub init_cnt: c_int,
}

    static struct rockchip_pcie_phy *to_pcie_phy(struct phy_pcie_instance *inst)
    {
    return container_of(inst, struct rockchip_pcie_phy,
    phys[inst.index]);
    }
    static struct phy *rockchip_pcie_phy_of_xlate(struct device *dev,
    const struct of_phandle_args *args)
    {
    struct rockchip_pcie_phy *rk_phy = dev_get_drvdata(dev);
    if (args.args_count == 0)
    return rk_phy.phys[0].phy;
    if (WARN_ON(args.args[0] >= PHY_MAX_LANE_NUM))
    return ERR_PTR(-ENODEV);
    return rk_phy.phys[args.args[0]].phy;
    }
    static inline void phy_wr_cfg(struct rockchip_pcie_phy *rk_phy,
    u32 addr, u32 data)
    {
    regmap_write(rk_phy.reg_base, rk_phy.phy_data.pcie_conf,
    FIELD_PREP_WM16(PHY_CFG_DATA_MASK, data) |
    FIELD_PREP_WM16(PHY_CFG_ADDR_MASK, addr));
    udelay(1);
    regmap_write(rk_phy.reg_base, rk_phy.phy_data.pcie_conf,
    FIELD_PREP_WM16(PHY_CFG_WR_MASK, PHY_CFG_WR_ENABLE));
    udelay(1);
    regmap_write(rk_phy.reg_base, rk_phy.phy_data.pcie_conf,
    FIELD_PREP_WM16(PHY_CFG_WR_MASK, PHY_CFG_WR_DISABLE));
    }
#[no_mangle]
unsafe extern "C" fn rockchip_pcie_phy_power_off(phy: *mut phy) -> c_int {
    static int rockchip_pcie_phy_power_off(struct phy *phy)
    {
    struct phy_pcie_instance *inst = phy_get_drvdata(phy);
    struct rockchip_pcie_phy *rk_phy = to_pcie_phy(inst);
    let mut err: c_int = 0;
    guard(mutex)(&rk_phy.pcie_mutex);
    regmap_write(rk_phy.reg_base, rk_phy.phy_data.pcie_laneoff,
    FIELD_PREP_WM16(PHY_LANE_IDLE_MASK,
    PHY_LANE_IDLE_OFF) << inst.index);
    if (--rk_phy.pwr_cnt) {
    return 0;
    }
    err = reset_control_assert(rk_phy.phy_rst);
    if (err) {
    dev_err(&phy.dev, "assert phy_rst err %d\n", err);
    rk_phy.pwr_cnt++;
    regmap_write(rk_phy.reg_base, rk_phy.phy_data.pcie_laneoff,
    FIELD_PREP_WM16(PHY_LANE_IDLE_MASK,
    !PHY_LANE_IDLE_OFF) << inst.index);
    return err;
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn rockchip_pcie_phy_power_on(phy: *mut phy) -> c_int {
    static int rockchip_pcie_phy_power_on(struct phy *phy)
    {
    struct phy_pcie_instance *inst = phy_get_drvdata(phy);
    struct rockchip_pcie_phy *rk_phy = to_pcie_phy(inst);
    let mut err: c_int = 0;
    u32 status;
    guard(mutex)(&rk_phy.pcie_mutex);
    regmap_write(rk_phy.reg_base, rk_phy.phy_data.pcie_laneoff,
    FIELD_PREP_WM16(PHY_LANE_IDLE_MASK,
    !PHY_LANE_IDLE_OFF) << inst.index);
    if (rk_phy.pwr_cnt++) {
    return 0;
    }
    err = reset_control_deassert(rk_phy.phy_rst);
    if (err) {
    dev_err(&phy.dev, "deassert phy_rst err %d\n", err);
    rk_phy.pwr_cnt--;
    return err;
    }
    regmap_write(rk_phy.reg_base, rk_phy.phy_data.pcie_conf,
    FIELD_PREP_WM16(PHY_CFG_ADDR_MASK, PHY_CFG_PLL_LOCK));
//
// No documented timeout value for phy operation below,
// so we make it large enough here. And we use loop-break
// method which should not be harmful.
//
    err = regmap_read_poll_timeout(rk_phy.reg_base,
    rk_phy.phy_data.pcie_status,
    status,
    status & PHY_PLL_LOCKED,
    200, 100000);
    if (err) {
    dev_err(&phy.dev, "pll lock timeout!\n");
    goto err_pll_lock;
    }
    phy_wr_cfg(rk_phy, PHY_CFG_CLK_TEST, PHY_CFG_SEPE_RATE);
    phy_wr_cfg(rk_phy, PHY_CFG_CLK_SCC, PHY_CFG_PLL_100M);
    err = regmap_read_poll_timeout(rk_phy.reg_base,
    rk_phy.phy_data.pcie_status,
    status,
    !(status & PHY_PLL_OUTPUT),
    200, 100000);
    if (err) {
    dev_err(&phy.dev, "pll output enable timeout!\n");
    goto err_pll_lock;
    }
    regmap_write(rk_phy.reg_base, rk_phy.phy_data.pcie_conf,
    FIELD_PREP_WM16(PHY_CFG_ADDR_MASK, PHY_CFG_PLL_LOCK));
    err = regmap_read_poll_timeout(rk_phy.reg_base,
    rk_phy.phy_data.pcie_status,
    status,
    status & PHY_PLL_LOCKED,
    200, 100000);
    if (err) {
    dev_err(&phy.dev, "pll relock timeout!\n");
    goto err_pll_lock;
    }
    return err;
    err_pll_lock:
    reset_control_assert(rk_phy.phy_rst);
    rk_phy.pwr_cnt--;
    return err;
    }
#[no_mangle]
unsafe extern "C" fn rockchip_pcie_phy_init(phy: *mut phy) -> c_int {
    static int rockchip_pcie_phy_init(struct phy *phy)
    {
    struct phy_pcie_instance *inst = phy_get_drvdata(phy);
    struct rockchip_pcie_phy *rk_phy = to_pcie_phy(inst);
    let mut err: c_int = 0;
    guard(mutex)(&rk_phy.pcie_mutex);
    if (rk_phy.init_cnt++) {
    return 0;
    }
    err = reset_control_assert(rk_phy.phy_rst);
    if (err) {
    dev_err(&phy.dev, "assert phy_rst err %d\n", err);
    rk_phy.init_cnt--;
    return err;
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn rockchip_pcie_phy_exit(phy: *mut phy) -> c_int {
    static int rockchip_pcie_phy_exit(struct phy *phy)
    {
    struct phy_pcie_instance *inst = phy_get_drvdata(phy);
    struct rockchip_pcie_phy *rk_phy = to_pcie_phy(inst);
    guard(mutex)(&rk_phy.pcie_mutex);
    if (--rk_phy.init_cnt)
    goto err_init_cnt;
    err_init_cnt:
    return 0;
    }
    static const struct phy_ops ops = {
    .init		= rockchip_pcie_phy_init,
    .exit		= rockchip_pcie_phy_exit,
    .power_on	= rockchip_pcie_phy_power_on,
    .power_off	= rockchip_pcie_phy_power_off,
    .owner		= THIS_MODULE,
    };
    static const struct rockchip_pcie_data rk3399_pcie_data = {
    .pcie_conf = 0xe220,
    .pcie_status = 0xe2a4,
    .pcie_laneoff = 0xe214,
    };
    static const struct of_device_id rockchip_pcie_phy_dt_ids[] = {
    {
    .compatible = "rockchip,rk3399-pcie-phy",
    .data = &rk3399_pcie_data,
    },
    {}
    };
    MODULE_DEVICE_TABLE(of, rockchip_pcie_phy_dt_ids);
#[no_mangle]
unsafe extern "C" fn rockchip_pcie_phy_probe(pdev: *mut platform_device) -> c_int {
    static int rockchip_pcie_phy_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct rockchip_pcie_phy *rk_phy;
    struct phy_provider *phy_provider;
    struct regmap *grf;
    int i;
    u32 phy_num;
    grf = syscon_node_to_regmap(dev.parent.of_node);
    if (IS_ERR(grf)) {
    dev_err(dev, "Cannot find GRF syscon\n");
    return PTR_ERR(grf);
    }
    rk_phy = devm_kzalloc(dev, sizeof(*rk_phy), GFP_KERNEL);
    if (!rk_phy)
    return -ENOMEM;
    rk_phy.phy_data = device_get_match_data(&pdev.dev);
    if (!rk_phy.phy_data)
    return -EINVAL;
    rk_phy.reg_base = grf;
    mutex_init(&rk_phy.pcie_mutex);
    rk_phy.phy_rst = devm_reset_control_get(dev, "phy");
    if (IS_ERR(rk_phy.phy_rst))
    return dev_err_probe(&pdev.dev, PTR_ERR(rk_phy.phy_rst),
    "missing phy property for reset controller\n");
    rk_phy.clk_pciephy_ref = devm_clk_get_enabled(dev, "refclk");
    if (IS_ERR(rk_phy.clk_pciephy_ref))
    return dev_err_probe(&pdev.dev, PTR_ERR(rk_phy.clk_pciephy_ref),
    "failed to get phyclk\n");
// parse #phy-cells to see if it's legacy PHY model
    if (of_property_read_u32(dev.of_node, "#phy-cells", &phy_num))
    return -ENOENT;
    phy_num = (phy_num == 0) ? 1 : PHY_MAX_LANE_NUM;
    dev_dbg(dev, "phy number is %d\n", phy_num);
    for (i = 0; i < phy_num; i++) {
    rk_phy.phys[i].phy = devm_phy_create(dev, dev.of_node, &ops);
    if (IS_ERR(rk_phy.phys[i].phy)) {
    dev_err(dev, "failed to create PHY%d\n", i);
    return PTR_ERR(rk_phy.phys[i].phy);
    }
    rk_phy.phys[i].index = i;
    phy_set_drvdata(rk_phy.phys[i].phy, &rk_phy.phys[i]);
    }
    platform_set_drvdata(pdev, rk_phy);
    phy_provider = devm_of_phy_provider_register(dev,
    rockchip_pcie_phy_of_xlate);
    return PTR_ERR_OR_ZERO(phy_provider);
    }
    static struct platform_driver rockchip_pcie_driver = {
    .probe		= rockchip_pcie_phy_probe,
    .driver		= {
    .name	= "rockchip-pcie-phy",
    .of_match_table = rockchip_pcie_phy_dt_ids,
    },
    };
    module_platform_driver(rockchip_pcie_driver);
    MODULE_AUTHOR("Shawn Lin <shawn.lin@rock-chips.com>");
    MODULE_DESCRIPTION("Rockchip PCIe PHY driver");
    MODULE_LICENSE("GPL v2");
