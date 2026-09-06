//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/qcom/camss/camss-csiphy-2ph-1-0.c
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
// camss-csiphy-2ph-1-0.c
//
// Qualcomm MSM Camera Subsystem - CSIPHY Module 2phase v1.0
//
// Copyright (c) 2011-2015, The Linux Foundation. All rights reserved.
// Copyright (C) 2016-2018 Linaro Ltd.
//

pub const CAMSS_CSI_PHY_LN_CLK: c_int = 1;
pub const CAMSS_CSI_PHY_GLBL_RESET: c_uint = 0x140;
pub const CAMSS_CSI_PHY_GLBL_PWR_CFG: c_uint = 0x144;
pub const CAMSS_CSI_PHY_GLBL_IRQ_CMD: c_uint = 0x164;
pub const CAMSS_CSI_PHY_HW_VERSION: c_uint = 0x188;

pub const CAMSS_CSI_PHY_GLBL_T_INIT_CFG0: c_uint = 0x1ec;
pub const CAMSS_CSI_PHY_T_WAKEUP_CFG0: c_uint = 0x1f4;
#[no_mangle]
unsafe extern "C" fn csiphy_get_lane_mask(lane_cfg: *mut csiphy_lanes_cfg) -> u8 {
    static u8 csiphy_get_lane_mask(struct csiphy_lanes_cfg *lane_cfg)
    {
    u8 lane_mask;
    int i;
    lane_mask = 1 << CAMSS_CSI_PHY_LN_CLK;
    for (i = 0; i < lane_cfg.num_data; i++)
    lane_mask |= 1 << lane_cfg.data[i].pos;
    return lane_mask;
    }
    static void csiphy_hw_version_read(struct csiphy_device *csiphy,
    struct device *dev)
    {
    u8 hw_version = readl_relaxed(csiphy.base +
    CAMSS_CSI_PHY_HW_VERSION);
    dev_dbg(dev, "CSIPHY HW Version = 0x%02x\n", hw_version);
    }
//
// csiphy_reset - Perform software reset on CSIPHY module
// @csiphy: CSIPHY device
//
#[no_mangle]
unsafe extern "C" fn csiphy_reset(csiphy: *mut csiphy_device) {
    static void csiphy_reset(struct csiphy_device *csiphy)
    {
    writel_relaxed(0x1, csiphy.base + CAMSS_CSI_PHY_GLBL_RESET);
    usleep_range(5000, 8000);
    writel_relaxed(0x0, csiphy.base + CAMSS_CSI_PHY_GLBL_RESET);
    }
//
// csiphy_settle_cnt_calc - Calculate settle count value
//
// Helper function to calculate settle count value. This is
// based on the CSI2 T_hs_settle parameter which in turn
// is calculated based on the CSI2 transmitter link frequency.
//
// Return settle count value or 0 if the CSI2 link frequency
// is not available
//
#[no_mangle]
unsafe extern "C" fn csiphy_settle_cnt_calc(link_freq: i64, timer_clk_rate: u32) -> u8 {
    static u8 csiphy_settle_cnt_calc(s64 link_freq, u32 timer_clk_rate)
    {
    u32 ui; /* ps */
    u32 timer_period; /* ps */
    u32 t_hs_prepare_max; /* ps */
    u32 t_hs_prepare_zero_min; /* ps */
    u32 t_hs_settle; /* ps */
    u8 settle_cnt;
    if (link_freq <= 0)
    return 0;
    ui = div_u64(1000000000000LL, link_freq);
    ui /= 2;
    t_hs_prepare_max = 85000 + 6 * ui;
    t_hs_prepare_zero_min = 145000 + 10 * ui;
    t_hs_settle = (t_hs_prepare_max + t_hs_prepare_zero_min) / 2;
    timer_period = div_u64(1000000000000LL, timer_clk_rate);
    settle_cnt = t_hs_settle / timer_period - 1;
    return settle_cnt;
    }
    static void csiphy_lanes_enable(struct csiphy_device *csiphy,
    struct csiphy_config *cfg,
    s64 link_freq, u8 lane_mask)
    {
    struct csiphy_lanes_cfg *c = &cfg.csi2.lane_cfg;
    u8 settle_cnt;
    u8 val, l = 0;
    let mut i: c_int = 0;
    settle_cnt = csiphy_settle_cnt_calc(link_freq, csiphy.timer_clk_rate);
    writel_relaxed(0x1, csiphy.base +
    CAMSS_CSI_PHY_GLBL_T_INIT_CFG0);
    writel_relaxed(0x1, csiphy.base +
    CAMSS_CSI_PHY_T_WAKEUP_CFG0);
    val = 0x1;
    val |= lane_mask << 1;
    writel_relaxed(val, csiphy.base + CAMSS_CSI_PHY_GLBL_PWR_CFG);
    val = cfg.combo_mode << 4;
    writel_relaxed(val, csiphy.base + CAMSS_CSI_PHY_GLBL_RESET);
    for (i = 0; i <= c.num_data; i++) {
    if (i == c.num_data)
    l = CAMSS_CSI_PHY_LN_CLK;
    else
    l = c.data[i].pos;
    writel_relaxed(0x10, csiphy.base +
    CAMSS_CSI_PHY_LNn_CFG2(l));
    writel_relaxed(settle_cnt, csiphy.base +
    CAMSS_CSI_PHY_LNn_CFG3(l));
    writel_relaxed(0x3f, csiphy.base +
    CAMSS_CSI_PHY_INTERRUPT_MASKn(l));
    writel_relaxed(0x3f, csiphy.base +
    CAMSS_CSI_PHY_INTERRUPT_CLEARn(l));
    }
    }
    static void csiphy_lanes_disable(struct csiphy_device *csiphy,
    struct csiphy_config *cfg)
    {
    struct csiphy_lanes_cfg *c = &cfg.csi2.lane_cfg;
    let mut l: u8 = 0;
    let mut i: c_int = 0;
    for (i = 0; i <= c.num_data; i++) {
    if (i == c.num_data)
    l = CAMSS_CSI_PHY_LN_CLK;
    else
    l = c.data[i].pos;
    writel_relaxed(0x0, csiphy.base +
    CAMSS_CSI_PHY_LNn_CFG2(l));
    }
    writel_relaxed(0x0, csiphy.base + CAMSS_CSI_PHY_GLBL_PWR_CFG);
    }
//
// csiphy_isr - CSIPHY module interrupt handler
// @irq: Interrupt line
// @dev: CSIPHY device
//
// Return IRQ_HANDLED on success
//
#[no_mangle]
unsafe extern "C" fn csiphy_isr(irq: c_int, dev: *mut c_void) -> irqreturn_t {
    static irqreturn_t csiphy_isr(int irq, void *dev)
    {
    struct csiphy_device *csiphy = dev;
    u8 i;
    for (i = 0; i < 8; i++) {
    u8 val = readl_relaxed(csiphy.base +
    CAMSS_CSI_PHY_INTERRUPT_STATUSn(i));
    writel_relaxed(val, csiphy.base +
    CAMSS_CSI_PHY_INTERRUPT_CLEARn(i));
    writel_relaxed(0x1, csiphy.base + CAMSS_CSI_PHY_GLBL_IRQ_CMD);
    writel_relaxed(0x0, csiphy.base + CAMSS_CSI_PHY_GLBL_IRQ_CMD);
    writel_relaxed(0x0, csiphy.base +
    CAMSS_CSI_PHY_INTERRUPT_CLEARn(i));
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn csiphy_init(csiphy: *mut csiphy_device) -> c_int {
    static int csiphy_init(struct csiphy_device *csiphy)
    {
    return 0;
    }
    const struct csiphy_hw_ops csiphy_ops_2ph_1_0 = {
    .get_lane_mask = csiphy_get_lane_mask,
    .hw_version_read = csiphy_hw_version_read,
    .reset = csiphy_reset,
    .lanes_enable = csiphy_lanes_enable,
    .lanes_disable = csiphy_lanes_disable,
    .isr = csiphy_isr,
    .init = csiphy_init,
    };
