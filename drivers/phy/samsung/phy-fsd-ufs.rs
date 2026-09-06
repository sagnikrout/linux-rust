//! Automatically rewritten from C to Rust
//! Source: drivers/phy/samsung/phy-fsd-ufs.c
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
// UFS PHY driver data for FSD SoC
//
// Copyright (C) 2022 Samsung Electronics Co., Ltd.
//

pub const FSD_EMBEDDED_COMBO_PHY_CTRL: c_uint = 0x724;
pub const FSD_EMBEDDED_COMBO_PHY_CTRL_MASK: c_uint = 0x1;

pub const FSD_EMBEDDED_COMBO_PHY_CDR_LOCK_STATUS: c_uint = 0x6e;
    static const struct samsung_ufs_phy_cfg fsd_pre_init_cfg[] = {
    PHY_COMN_REG_CFG(0x00f, 0xfa, PWR_MODE_ANY),
    PHY_COMN_REG_CFG(0x010, 0x82, PWR_MODE_ANY),
    PHY_COMN_REG_CFG(0x011, 0x1e, PWR_MODE_ANY),
    PHY_COMN_REG_CFG(0x017, 0x94, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG(0x035, 0x58, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG(0x036, 0x32, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG(0x037, 0x40, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG(0x03b, 0x83, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG(0x042, 0x88, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG(0x043, 0xa6, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG(0x048, 0x74, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG(0x04c, 0x5b, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG(0x04d, 0x83, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG(0x05c, 0x14, PWR_MODE_ANY),
    END_UFS_PHY_CFG
    };
// Calibration for HS mode series A/B
    static const struct samsung_ufs_phy_cfg fsd_pre_pwr_hs_cfg[] = {
    END_UFS_PHY_CFG
    };
// Calibration for HS mode series A/B atfer PMC
    static const struct samsung_ufs_phy_cfg fsd_post_pwr_hs_cfg[] = {
    END_UFS_PHY_CFG
    };
    static const struct samsung_ufs_phy_cfg *fsd_ufs_phy_cfgs[CFG_TAG_MAX] = {
    [CFG_PRE_INIT]		= fsd_pre_init_cfg,
    [CFG_PRE_PWR_HS]	= fsd_pre_pwr_hs_cfg,
    [CFG_POST_PWR_HS]	= fsd_post_pwr_hs_cfg,
    };
    static const char * const fsd_ufs_phy_clks[] = {
    "ref_clk",
    };
    const struct samsung_ufs_phy_drvdata fsd_ufs_phy = {
    .cfgs = fsd_ufs_phy_cfgs,
    .isol = {
    .offset = FSD_EMBEDDED_COMBO_PHY_CTRL,
    .mask = FSD_EMBEDDED_COMBO_PHY_CTRL_MASK,
    .en = FSD_EMBEDDED_COMBO_PHY_CTRL_EN,
    },
    .clk_list = fsd_ufs_phy_clks,
    .num_clks = ARRAY_SIZE(fsd_ufs_phy_clks),
    .cdr_lock_status_offset = FSD_EMBEDDED_COMBO_PHY_CDR_LOCK_STATUS,
    .wait_for_cdr = samsung_ufs_phy_wait_for_lock_acq,
    };
