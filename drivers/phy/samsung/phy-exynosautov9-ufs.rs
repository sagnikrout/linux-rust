//! Automatically rewritten from C to Rust
//! Source: drivers/phy/samsung/phy-exynosautov9-ufs.c
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
// UFS PHY driver data for Samsung EXYNOSAUTO v9 SoC
//
// Copyright (C) 2021 Samsung Electronics Co., Ltd.
//

pub const EXYNOSAUTOV9_EMBEDDED_COMBO_PHY_CTRL: c_uint = 0x728;
pub const EXYNOSAUTOV9_EMBEDDED_COMBO_PHY_CTRL_MASK: c_uint = 0x1;

pub const EXYNOSAUTOV9_EMBEDDED_COMBO_PHY_CDR_LOCK_STATUS: c_uint = 0x5e;

    PHY_TRSV_REG_CFG_OFFSET(o, v, d, 0x50)
// Calibration for phy initialization
    static const struct samsung_ufs_phy_cfg exynosautov9_pre_init_cfg[] = {
    PHY_COMN_REG_CFG(0x023, 0x80, PWR_MODE_ANY),
    PHY_COMN_REG_CFG(0x01d, 0x10, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_AUTOV9(0x044, 0xb5, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_AUTOV9(0x04d, 0x43, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_AUTOV9(0x05b, 0x20, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_AUTOV9(0x05e, 0xc0, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_AUTOV9(0x038, 0x12, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_AUTOV9(0x059, 0x58, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_AUTOV9(0x06c, 0x18, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_AUTOV9(0x06d, 0x02, PWR_MODE_ANY),
    PHY_COMN_REG_CFG(0x023, 0xc0, PWR_MODE_ANY),
    PHY_COMN_REG_CFG(0x023, 0x00, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_AUTOV9(0x042, 0x5d, PWR_MODE_ANY),
    PHY_TRSV_REG_CFG_AUTOV9(0x043, 0x80, PWR_MODE_ANY),
    END_UFS_PHY_CFG,
    };
// Calibration for HS mode series A/B
    static const struct samsung_ufs_phy_cfg exynosautov9_pre_pwr_hs_cfg[] = {
    PHY_TRSV_REG_CFG_AUTOV9(0x032, 0xbc, PWR_MODE_HS_ANY),
    PHY_TRSV_REG_CFG_AUTOV9(0x03c, 0x7f, PWR_MODE_HS_ANY),
    PHY_TRSV_REG_CFG_AUTOV9(0x048, 0xc0, PWR_MODE_HS_ANY),
    PHY_TRSV_REG_CFG_AUTOV9(0x04a, 0x00, PWR_MODE_HS_G3_SER_B),
    PHY_TRSV_REG_CFG_AUTOV9(0x04b, 0x10, PWR_MODE_HS_G1_SER_B |
    PWR_MODE_HS_G3_SER_B),
    PHY_TRSV_REG_CFG_AUTOV9(0x04d, 0x63, PWR_MODE_HS_G3_SER_B),
    END_UFS_PHY_CFG,
    };
    static const struct samsung_ufs_phy_cfg *exynosautov9_ufs_phy_cfgs[CFG_TAG_MAX] = {
    [CFG_PRE_INIT]		= exynosautov9_pre_init_cfg,
    [CFG_PRE_PWR_HS]	= exynosautov9_pre_pwr_hs_cfg,
    };
    static const char * const exynosautov9_ufs_phy_clks[] = {
    "ref_clk",
    };
    const struct samsung_ufs_phy_drvdata exynosautov9_ufs_phy = {
    .cfgs = exynosautov9_ufs_phy_cfgs,
    .isol = {
    .offset = EXYNOSAUTOV9_EMBEDDED_COMBO_PHY_CTRL,
    .mask = EXYNOSAUTOV9_EMBEDDED_COMBO_PHY_CTRL_MASK,
    .en = EXYNOSAUTOV9_EMBEDDED_COMBO_PHY_CTRL_EN,
    },
    .clk_list = exynosautov9_ufs_phy_clks,
    .num_clks = ARRAY_SIZE(exynosautov9_ufs_phy_clks),
    .cdr_lock_status_offset = EXYNOSAUTOV9_EMBEDDED_COMBO_PHY_CDR_LOCK_STATUS,
    .wait_for_cdr = samsung_ufs_phy_wait_for_lock_acq,
    };
