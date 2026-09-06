//! Automatically rewritten from C to Rust
//! Source: drivers/phy/freescale/phy-fsl-imx8qm-lvds-phy.c
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
// Copyright 2017-2020,2022 NXP
//

pub const REG_SET: c_uint = 0x4;
pub const REG_CLR: c_uint = 0x8;
pub const PHY_CTRL: c_uint = 0x0;

// Power On Reset(POR) value

// PHY initialization value and mask

pub const PHY_STATUS: c_uint = 0x10;

pub const PHY_NUM: c_int = 2;

pub const PLL_LOCK_SLEEP: c_int = 10;
pub const PLL_LOCK_TIMEOUT: c_int = 1000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mixel_lvds_phy {
    pub phy: *mut phy,
    pub cfg: phy_configure_opts_lvds,
    pub id: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mixel_lvds_phy_priv {
    pub regmap: *mut regmap,
    pub /: *mut *mut mutex lock; / protect remap access and cfg of our own,
    pub phy_ref_clk: *mut clk,
    pub phys: [*mut mixel_lvds_phy; PHY_NUM],
}

#[no_mangle]
unsafe extern "C" fn mixel_lvds_phy_init(phy: *mut phy) -> c_int {
    static int mixel_lvds_phy_init(struct phy *phy)
    {
    struct mixel_lvds_phy_priv *priv = dev_get_drvdata(phy.dev.parent);
    mutex_lock(&priv.lock);
    regmap_update_bits(priv.regmap,
    PHY_CTRL, CTRL_INIT_MASK, CTRL_INIT_VAL);
    mutex_unlock(&priv.lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mixel_lvds_phy_power_on(phy: *mut phy) -> c_int {
    static int mixel_lvds_phy_power_on(struct phy *phy)
    {
    struct mixel_lvds_phy_priv *priv = dev_get_drvdata(phy.dev.parent);
    struct mixel_lvds_phy *lvds_phy = phy_get_drvdata(phy);
    struct mixel_lvds_phy *companion = priv.phys[lvds_phy.id ^ 1];
    struct phy_configure_opts_lvds *cfg = &lvds_phy.cfg;
    let mut val: u32 = 0;
    u32 locked;
    int ret;
// The master PHY would power on the slave PHY.
    if (cfg.is_slave)
    return 0;
    ret = clk_prepare_enable(priv.phy_ref_clk);
    if (ret < 0) {
    dev_err(&phy.dev,
    "failed to enable PHY reference clock: %d\n", ret);
    return ret;
    }
    mutex_lock(&priv.lock);
    if (cfg.bits_per_lane_and_dclk_cycle == 7) {
    if (cfg.differential_clk_rate < 44000000)
    val |= M(0x2);
#[no_mangle]
pub unsafe extern "C" fn if(90000000: cfg->differential_clk_rate <) -> else {
    else if (cfg.differential_clk_rate < 90000000)
    val |= M(0x1);
    else
    val |= M(0x0);
    } else {
    val = NB;
    if (cfg.differential_clk_rate < 32000000)
    val |= M(0x2);
#[no_mangle]
pub unsafe extern "C" fn if(63000000: cfg->differential_clk_rate <) -> else {
    else if (cfg.differential_clk_rate < 63000000)
    val |= M(0x1);
    else
    val |= M(0x0);
    }
    regmap_update_bits(priv.regmap, PHY_CTRL, M_MASK | NB, val);
//
// Enable two channels synchronously,
// if the companion PHY is a slave PHY.
//
    if (companion.cfg.is_slave)
    val = CH_EN(0) | CH_EN(1);
    else
    val = CH_EN(lvds_phy.id);
    regmap_write(priv.regmap, PHY_CTRL + REG_SET, val);
    ret = regmap_read_poll_timeout(priv.regmap, PHY_STATUS, locked,
    locked, PLL_LOCK_SLEEP,
    PLL_LOCK_TIMEOUT);
    if (ret < 0) {
    dev_err(&phy.dev, "failed to get PHY lock: %d\n", ret);
    clk_disable_unprepare(priv.phy_ref_clk);
    }
    mutex_unlock(&priv.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mixel_lvds_phy_power_off(phy: *mut phy) -> c_int {
    static int mixel_lvds_phy_power_off(struct phy *phy)
    {
    struct mixel_lvds_phy_priv *priv = dev_get_drvdata(phy.dev.parent);
    struct mixel_lvds_phy *lvds_phy = phy_get_drvdata(phy);
    struct mixel_lvds_phy *companion = priv.phys[lvds_phy.id ^ 1];
    struct phy_configure_opts_lvds *cfg = &lvds_phy.cfg;
// The master PHY would power off the slave PHY.
    if (cfg.is_slave)
    return 0;
    mutex_lock(&priv.lock);
    if (companion.cfg.is_slave)
    regmap_write(priv.regmap, PHY_CTRL + REG_CLR,
    CH_EN(0) | CH_EN(1));
    else
    regmap_write(priv.regmap, PHY_CTRL + REG_CLR,
    CH_EN(lvds_phy.id));
    mutex_unlock(&priv.lock);
    clk_disable_unprepare(priv.phy_ref_clk);
    return 0;
    }
    static int mixel_lvds_phy_configure(struct phy *phy,
    union phy_configure_opts *opts)
    {
    struct mixel_lvds_phy_priv *priv = dev_get_drvdata(phy.dev.parent);
    struct phy_configure_opts_lvds *cfg = &opts.lvds;
    int ret;
    ret = clk_set_rate(priv.phy_ref_clk, cfg.differential_clk_rate);
    if (ret)
    dev_err(&phy.dev, "failed to set PHY reference clock rate(%lu): %d\n",
    cfg.differential_clk_rate, ret);
    return ret;
    }
// Assume the master PHY's configuration set is cached first.
#[no_mangle]
unsafe extern "C" fn mixel_lvds_phy_check_slave(slave_phy: *mut phy) -> c_int {
    static int mixel_lvds_phy_check_slave(struct phy *slave_phy)
    {
    struct device *dev = &slave_phy.dev;
    struct mixel_lvds_phy_priv *priv = dev_get_drvdata(dev.parent);
    struct mixel_lvds_phy *slv = phy_get_drvdata(slave_phy);
    struct mixel_lvds_phy *mst = priv.phys[slv.id ^ 1];
    struct phy_configure_opts_lvds *mst_cfg = &mst.cfg;
    struct phy_configure_opts_lvds *slv_cfg = &slv.cfg;
    if (mst_cfg.bits_per_lane_and_dclk_cycle !=
    slv_cfg.bits_per_lane_and_dclk_cycle) {
    dev_err(dev, "number bits mismatch(mst: %u vs slv: %u)\n",
    mst_cfg.bits_per_lane_and_dclk_cycle,
    slv_cfg.bits_per_lane_and_dclk_cycle);
    return -EINVAL;
    }
    if (mst_cfg.differential_clk_rate !=
    slv_cfg.differential_clk_rate) {
    dev_err(dev, "dclk rate mismatch(mst: %lu vs slv: %lu)\n",
    mst_cfg.differential_clk_rate,
    slv_cfg.differential_clk_rate);
    return -EINVAL;
    }
    if (mst_cfg.lanes != slv_cfg.lanes) {
    dev_err(dev, "lanes mismatch(mst: %u vs slv: %u)\n",
    mst_cfg.lanes, slv_cfg.lanes);
    return -EINVAL;
    }
    if (mst_cfg.is_slave == slv_cfg.is_slave) {
    dev_err(dev, "master PHY is not found\n");
    return -EINVAL;
    }
    return 0;
    }
    static int mixel_lvds_phy_validate(struct phy *phy, enum phy_mode mode,
    int submode, union phy_configure_opts *opts)
    {
    struct mixel_lvds_phy_priv *priv = dev_get_drvdata(phy.dev.parent);
    struct mixel_lvds_phy *lvds_phy = phy_get_drvdata(phy);
    struct phy_configure_opts_lvds *cfg = &opts.lvds;
    let mut ret: c_int = 0;
    if (mode != PHY_MODE_LVDS) {
    dev_err(&phy.dev, "invalid PHY mode(%d)\n", mode);
    return -EINVAL;
    }
    if (cfg.bits_per_lane_and_dclk_cycle != 7 &&
    cfg.bits_per_lane_and_dclk_cycle != 10) {
    dev_err(&phy.dev, "invalid bits per data lane(%u)\n",
    cfg.bits_per_lane_and_dclk_cycle);
    return -EINVAL;
    }
    if (cfg.lanes != 4 && cfg.lanes != 3) {
    dev_err(&phy.dev, "invalid data lanes(%u)\n", cfg.lanes);
    return -EINVAL;
    }
    if (cfg.differential_clk_rate < MIN_CLKIN_FREQ ||
    cfg.differential_clk_rate > MAX_CLKIN_FREQ) {
    dev_err(&phy.dev, "invalid differential clock rate(%lu)\n",
    cfg.differential_clk_rate);
    return -EINVAL;
    }
    mutex_lock(&priv.lock);
// cache configuration set of our own for check
    memcpy(&lvds_phy.cfg, cfg, sizeof(*cfg));
    if (cfg.is_slave) {
    ret = mixel_lvds_phy_check_slave(phy);
    if (ret)
    dev_err(&phy.dev, "failed to check slave PHY: %d\n", ret);
    }
    mutex_unlock(&priv.lock);
    return ret;
    }
    static const struct phy_ops mixel_lvds_phy_ops = {
    .init = mixel_lvds_phy_init,
    .power_on = mixel_lvds_phy_power_on,
    .power_off = mixel_lvds_phy_power_off,
    .configure = mixel_lvds_phy_configure,
    .validate = mixel_lvds_phy_validate,
    .owner = THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn mixel_lvds_phy_reset(dev: *mut device) -> c_int {
    static int mixel_lvds_phy_reset(struct device *dev)
    {
    struct mixel_lvds_phy_priv *priv = dev_get_drvdata(dev);
    int ret;
    ret = pm_runtime_resume_and_get(dev);
    if (ret < 0) {
    dev_err(dev, "failed to get PM runtime: %d\n", ret);
    return ret;
    }
    regmap_write(priv.regmap, PHY_CTRL, CTRL_RESET_VAL);
    pm_runtime_put_sync(dev);
    return 0;
    }
    static struct phy *mixel_lvds_phy_xlate(struct device *dev,
    const struct of_phandle_args *args)
    {
    struct mixel_lvds_phy_priv *priv = dev_get_drvdata(dev);
    unsigned int phy_id;
    if (args.args_count != 1) {
    dev_err(dev,
    "invalid argument number(%d) for 'phys' property\n",
    args.args_count);
    return ERR_PTR(-EINVAL);
    }
    phy_id = args.args[0];
    if (phy_id >= PHY_NUM) {
    dev_err(dev, "invalid PHY index(%d)\n", phy_id);
    return ERR_PTR(-ENODEV);
    }
    return priv.phys[phy_id].phy;
    }
#[no_mangle]
unsafe extern "C" fn mixel_lvds_phy_probe(pdev: *mut platform_device) -> c_int {
    static int mixel_lvds_phy_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct phy_provider *phy_provider;
    struct mixel_lvds_phy_priv *priv;
    struct mixel_lvds_phy *lvds_phy;
    struct phy *phy;
    int i;
    int ret;
    if (!dev.of_node)
    return -ENODEV;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.regmap = syscon_node_to_regmap(dev.of_node.parent);
    if (IS_ERR(priv.regmap))
    return dev_err_probe(dev, PTR_ERR(priv.regmap),
    "failed to get regmap\n");
    priv.phy_ref_clk = devm_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(priv.phy_ref_clk))
    return dev_err_probe(dev, PTR_ERR(priv.phy_ref_clk),
    "failed to get PHY reference clock\n");
    mutex_init(&priv.lock);
    dev_set_drvdata(dev, priv);
    ret = devm_pm_runtime_enable(dev);
    if (ret)
    return ret;
    ret = mixel_lvds_phy_reset(dev);
    if (ret) {
    dev_err(dev, "failed to do POR reset: %d\n", ret);
    return ret;
    }
    for (i = 0; i < PHY_NUM; i++) {
    lvds_phy = devm_kzalloc(dev, sizeof(*lvds_phy), GFP_KERNEL);
    if (!lvds_phy)
    return -ENOMEM;
    phy = devm_phy_create(dev, core::ptr::null_mut(), &mixel_lvds_phy_ops);
    if (IS_ERR(phy)) {
    ret = PTR_ERR(phy);
    dev_err(dev, "failed to create PHY for channel%d: %d\n",
    i, ret);
    return ret;
    }
    lvds_phy.phy = phy;
    lvds_phy.id = i;
    priv.phys[i] = lvds_phy;
    phy_set_drvdata(phy, lvds_phy);
    }
    phy_provider = devm_of_phy_provider_register(dev, mixel_lvds_phy_xlate);
    if (IS_ERR(phy_provider)) {
    ret = PTR_ERR(phy_provider);
    dev_err(dev, "failed to register PHY provider: %d\n", ret);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mixel_lvds_phy_runtime_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused mixel_lvds_phy_runtime_suspend(struct device *dev)
    {
    struct mixel_lvds_phy_priv *priv = dev_get_drvdata(dev);
// power down
    mutex_lock(&priv.lock);
    regmap_write(priv.regmap, PHY_CTRL + REG_SET, PD);
    mutex_unlock(&priv.lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mixel_lvds_phy_runtime_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused mixel_lvds_phy_runtime_resume(struct device *dev)
    {
    struct mixel_lvds_phy_priv *priv = dev_get_drvdata(dev);
// power up + control initialization
    mutex_lock(&priv.lock);
    regmap_update_bits(priv.regmap, PHY_CTRL,
    CTRL_INIT_MASK | PD, CTRL_INIT_VAL);
    mutex_unlock(&priv.lock);
    return 0;
    }
    static const struct dev_pm_ops mixel_lvds_phy_pm_ops = {
    SET_RUNTIME_PM_OPS(mixel_lvds_phy_runtime_suspend,
    mixel_lvds_phy_runtime_resume, core::ptr::null_mut())
    };
    static const struct of_device_id mixel_lvds_phy_of_match[] = {
    { .compatible = "fsl,imx8qm-lvds-phy" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, mixel_lvds_phy_of_match);
    static struct platform_driver mixel_lvds_phy_driver = {
    .probe = mixel_lvds_phy_probe,
    .driver = {
    .pm = &mixel_lvds_phy_pm_ops,
    .name = "mixel-lvds-phy",
    .of_match_table = mixel_lvds_phy_of_match,
    }
    };
    module_platform_driver(mixel_lvds_phy_driver);
    MODULE_DESCRIPTION("Mixel LVDS PHY driver");
    MODULE_AUTHOR("Liu Ying <victor.liu@nxp.com>");
    MODULE_LICENSE("GPL");
