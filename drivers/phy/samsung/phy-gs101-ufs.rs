//! Automatically rewritten from C to Rust
//! Source: drivers/phy/samsung/phy-gs101-ufs.c
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
// UFS PHY driver data for Google Tensor gs101 SoC
//
// Copyright (C) 2024 Linaro Ltd
// Author: Peter Griffin <peter.griffin@linaro.org>
//

pub const TENSOR_GS101_PHY_CTRL: c_uint = 0x3ec8;
pub const TENSOR_GS101_PHY_CTRL_MASK: c_uint = 0x1;

pub const PHY_GS101_LANE_OFFSET: c_uint = 0x200;
pub const TRSV_REG338: c_uint = 0x338;

pub const TRSV_REG339: c_uint = 0x339;

pub const TRSV_REG222: c_uint = 0x222;

    ((lane) * PHY_GS101_LANE_OFFSET)))

    PHY_TRSV_REG_CFG_OFFSET(o, v, d, PHY_GS101_LANE_OFFSET)
// Calibration for phy initialization
    static const struct samsung_ufs_phy_cfg tensor_gs101_pre_init_cfg[] = {
    PHY_COMN_REG_CFG(0x43, 0x10,  PWR_MODE_ANY),
    PHY_COMN_REG_CFG(0x3C, 0x14,  PWR_MODE_ANY),
    PHY_COMN_REG_CFG(0x46, 0x48,  PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_GS101(0x200, 0x00, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_GS101(0x201, 0x06, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_GS101(0x202, 0x06, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_GS101(0x203, 0x0a, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_GS101(0x204, 0x00, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_GS101(0x205, 0x11, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_GS101(0x207, 0x0c, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_GS101(0x2E1, 0xc0, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_GS101(0x22D, 0xb8, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_GS101(0x234, 0x60, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_GS101(0x238, 0x13, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_GS101(0x239, 0x48, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_GS101(0x23A, 0x01, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_GS101(0x23B, 0x25, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_GS101(0x23C, 0x2a, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_GS101(0x23D, 0x01, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_GS101(0x23E, 0x13, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_GS101(0x23F, 0x13, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_GS101(0x240, 0x4a, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_GS101(0x243, 0x40, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_GS101(0x244, 0x02, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_GS101(0x25D, 0x00, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_GS101(0x25E, 0x3f, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_GS101(0x25F, 0xff, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_GS101(0x273, 0x33, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_GS101(0x274, 0x50, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_GS101(0x284, 0x02, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_GS101(0x285, 0x02, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_GS101(0x2A2, 0x04, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_GS101(0x25D, 0x01, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_GS101(0x2FA, 0x01, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_GS101(0x286, 0x03, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_GS101(0x287, 0x03, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_GS101(0x288, 0x03, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_GS101(0x289, 0x03, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_GS101(0x2B3, 0x04, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_GS101(0x2B6, 0x0b, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_GS101(0x2B7, 0x0b, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_GS101(0x2B8, 0x0b, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_GS101(0x2B9, 0x0b, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_GS101(0x2BA, 0x0b, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_GS101(0x2BB, 0x06, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_GS101(0x2BC, 0x06, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_GS101(0x2BD, 0x06, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_GS101(0x29E, 0x06, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_GS101(0x2E4, 0x1a, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_GS101(0x2ED, 0x25, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_GS101(0x269, 0x1a, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_GS101(0x2F4, 0x2f, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_GS101(0x34B, 0x01, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_GS101(0x34C, 0x23, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_GS101(0x34D, 0x23, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_GS101(0x34E, 0x45, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_GS101(0x34F, 0x00, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_GS101(0x350, 0x31, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_GS101(0x351, 0x00, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_GS101(0x352, 0x02, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_GS101(0x353, 0x00, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_GS101(0x354, 0x01, PWR_MODE_ANY),
    PHY_COMN_REG_CFG(0x43, 0x18, PWR_MODE_ANY),
    PHY_COMN_REG_CFG(0x43, 0x00, PWR_MODE_ANY),
    END_UFS_PHY_CFG,
    };
    static const struct samsung_ufs_phy_cfg tensor_gs101_pre_pwr_hs_config[] = {
    PHY_TRSV_REG_CFG_GS101(0x369, 0x11, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_GS101(0x246, 0x03, PWR_MODE_ANY),
    };
// Calibration for HS mode series A/B
    static const struct samsung_ufs_phy_cfg tensor_gs101_post_pwr_hs_config[] = {
    PHY_COMN_REG_CFG(0x8, 0x60, PWR_MODE_PWM_ANY),
    PHY_TRSV_REG_CFG_GS101(0x222, 0x08, PWR_MODE_PWM_ANY),
    PHY_TRSV_REG_CFG_GS101(0x246, 0x01, PWR_MODE_ANY),
    END_UFS_PHY_CFG,
    };
    static const struct samsung_ufs_phy_cfg tensor_gs101_post_h8_enter[] = {
    PHY_TRSV_REG_CFG_GS101(0x262, 0x08, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_GS101(0x265, 0x0A, PWR_MODE_ANY),
    PHY_COMN_REG_CFG(0x1, 0x8,  PWR_MODE_ANY),
    PHY_COMN_REG_CFG(0x0, 0x86,  PWR_MODE_ANY),
    PHY_COMN_REG_CFG(0x8, 0x60,  PWR_MODE_HS_ANY),
    PHY_TRSV_REG_CFG_GS101(0x222, 0x08, PWR_MODE_HS_ANY),
    PHY_TRSV_REG_CFG_GS101(0x246, 0x01, PWR_MODE_HS_ANY),
    END_UFS_PHY_CFG,
    };
    static const struct samsung_ufs_phy_cfg tensor_gs101_pre_h8_exit[] = {
    PHY_COMN_REG_CFG(0x0, 0xC6,  PWR_MODE_ANY),
    PHY_COMN_REG_CFG(0x1, 0x0C,  PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_GS101(0x262, 0x00, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_GS101(0x265, 0x00, PWR_MODE_ANY),
    PHY_COMN_REG_CFG(0x8, 0xE0,  PWR_MODE_HS_ANY),
    PHY_TRSV_REG_CFG_GS101(0x246, 0x03, PWR_MODE_HS_ANY),
    PHY_TRSV_REG_CFG_GS101(0x222, 0x18, PWR_MODE_HS_ANY),
    END_UFS_PHY_CFG,
    };
    static const struct samsung_ufs_phy_cfg *tensor_gs101_ufs_phy_cfgs[CFG_TAG_MAX] = {
    [CFG_PRE_INIT]		= tensor_gs101_pre_init_cfg,
    [CFG_PRE_PWR_HS]	= tensor_gs101_pre_pwr_hs_config,
    [CFG_POST_PWR_HS]	= tensor_gs101_post_pwr_hs_config,
    };
    static const struct samsung_ufs_phy_cfg *tensor_gs101_hibern8_cfgs[] = {
    [CFG_POST_HIBERN8_ENTER]	= tensor_gs101_post_h8_enter,
    [CFG_PRE_HIBERN8_EXIT]		= tensor_gs101_pre_h8_exit,
    };
    static const char * const tensor_gs101_ufs_phy_clks[] = {
    "ref_clk",
    };
#[no_mangle]
unsafe extern "C" fn gs101_phy_wait_for_calibration(phy: *mut phy, lane: u8) -> c_int {
    static int gs101_phy_wait_for_calibration(struct phy *phy, u8 lane)
    {
    struct samsung_ufs_phy *ufs_phy = get_samsung_ufs_phy(phy);
    let mut timeout_us: c_uint = 40000;
    let mut sleep_us: c_uint = 40;
    u32 val;
    u32 off;
    int err;
    off = PHY_PMA_TRSV_ADDR(TRSV_REG338, lane);
    err = readl_poll_timeout(ufs_phy.reg_pma + off,
    val, (val & LN0_MON_RX_CAL_DONE),
    sleep_us, timeout_us);
    if (err) {
    dev_err(ufs_phy.dev,
    "failed to get phy cal done %d\n", err);
    }
    return err;
    }
pub const DELAY_IN_US: c_int = 40;
pub const RETRY_CNT: c_int = 100;
#[no_mangle]
unsafe extern "C" fn gs101_phy_wait_for_cdr_lock(phy: *mut phy, lane: u8) -> c_int {
    static int gs101_phy_wait_for_cdr_lock(struct phy *phy, u8 lane)
    {
    struct samsung_ufs_phy *ufs_phy = get_samsung_ufs_phy(phy);
    u32 val;
    int i;
    for (i = 0; i < RETRY_CNT; i++) {
    udelay(DELAY_IN_US);
    val = readl(ufs_phy.reg_pma +
    PHY_PMA_TRSV_ADDR(TRSV_REG339, lane));
    if (val & LN0_MON_RX_CDR_FLD_CK_MODE_DONE)
    return 0;
    udelay(DELAY_IN_US);
// Override and enable clock data recovery
    writel(LN0_OVRD_RX_CDR_EN, ufs_phy.reg_pma +
    PHY_PMA_TRSV_ADDR(TRSV_REG222, lane));
    writel(LN0_OVRD_RX_CDR_EN | LN0_RX_CDR_EN,
    ufs_phy.reg_pma + PHY_PMA_TRSV_ADDR(TRSV_REG222, lane));
    }
    dev_err(ufs_phy.dev, "failed to get cdr lock\n");
    return -ETIMEDOUT;
    }
    const struct samsung_ufs_phy_drvdata tensor_gs101_ufs_phy = {
    .cfgs = tensor_gs101_ufs_phy_cfgs,
    .cfgs_hibern8 = tensor_gs101_hibern8_cfgs,
    .isol = {
    .offset = TENSOR_GS101_PHY_CTRL,
    .mask = TENSOR_GS101_PHY_CTRL_MASK,
    .en = TENSOR_GS101_PHY_CTRL_EN,
    },
    .clk_list = tensor_gs101_ufs_phy_clks,
    .num_clks = ARRAY_SIZE(tensor_gs101_ufs_phy_clks),
    .wait_for_cal = gs101_phy_wait_for_calibration,
    .wait_for_cdr = gs101_phy_wait_for_cdr_lock,
    };
