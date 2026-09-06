//! Automatically rewritten from C to Rust
//! Source: drivers/net/phy/meson-gxl.c
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
// Amlogic Meson GXL Internal PHY Driver
//
// Copyright (C) 2015 Amlogic, Inc. All rights reserved.
// Copyright (C) 2016 BayLibre, SAS. All rights reserved.
// Author: Neil Armstrong <narmstrong@baylibre.com>
//

pub const TSTCNTL: c_int = 20;

pub const TSTREAD1: c_int = 21;
pub const TSTWRITE: c_int = 23;
pub const BANK_ANALOG_DSP: c_int = 0;
pub const BANK_WOL: c_int = 1;
pub const BANK_BIST: c_int = 3;
// WOL Registers
pub const LPI_STATUS: c_uint = 0xc;

// BIST Registers
pub const FR_PLL_CONTROL: c_uint = 0x1b;
pub const FR_PLL_DIV0: c_uint = 0x1c;
pub const FR_PLL_DIV1: c_uint = 0x1d;
#[no_mangle]
unsafe extern "C" fn meson_gxl_open_banks(phydev: *mut phy_device) -> c_int {
    static int meson_gxl_open_banks(struct phy_device *phydev)
    {
    int ret;
// Enable Analog and DSP register Bank access by
// toggling TSTCNTL_TEST_MODE bit in the TSTCNTL register
//
    ret = phy_write(phydev, TSTCNTL, 0);
    if (ret)
    return ret;
    ret = phy_write(phydev, TSTCNTL, TSTCNTL_TEST_MODE);
    if (ret)
    return ret;
    ret = phy_write(phydev, TSTCNTL, 0);
    if (ret)
    return ret;
    return phy_write(phydev, TSTCNTL, TSTCNTL_TEST_MODE);
    }
#[no_mangle]
unsafe extern "C" fn meson_gxl_close_banks(phydev: *mut phy_device) {
    static void meson_gxl_close_banks(struct phy_device *phydev)
    {
    phy_write(phydev, TSTCNTL, 0);
    }
    static int meson_gxl_read_reg(struct phy_device *phydev,
    unsigned int bank, unsigned int reg)
    {
    int ret;
    ret = meson_gxl_open_banks(phydev);
    if (ret)
    goto out;
    ret = phy_write(phydev, TSTCNTL, TSTCNTL_READ |
    FIELD_PREP(TSTCNTL_REG_BANK_SEL, bank) |
    TSTCNTL_TEST_MODE |
    FIELD_PREP(TSTCNTL_READ_ADDRESS, reg));
    if (ret)
    goto out;
    ret = phy_read(phydev, TSTREAD1);
    out:
// Close the bank access on our way out
    meson_gxl_close_banks(phydev);
    return ret;
    }
    static int meson_gxl_write_reg(struct phy_device *phydev,
    unsigned int bank, unsigned int reg,
    uint16_t value)
    {
    int ret;
    ret = meson_gxl_open_banks(phydev);
    if (ret)
    goto out;
    ret = phy_write(phydev, TSTWRITE, value);
    if (ret)
    goto out;
    ret = phy_write(phydev, TSTCNTL, TSTCNTL_WRITE |
    FIELD_PREP(TSTCNTL_REG_BANK_SEL, bank) |
    TSTCNTL_TEST_MODE |
    FIELD_PREP(TSTCNTL_WRITE_ADDRESS, reg));
    out:
// Close the bank access on our way out
    meson_gxl_close_banks(phydev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn meson_gxl_config_init(phydev: *mut phy_device) -> c_int {
    static int meson_gxl_config_init(struct phy_device *phydev)
    {
    int ret;
// Enable fractional PLL
    ret = meson_gxl_write_reg(phydev, BANK_BIST, FR_PLL_CONTROL, 0x5);
    if (ret)
    return ret;
// Program fraction FR_PLL_DIV1
    ret = meson_gxl_write_reg(phydev, BANK_BIST, FR_PLL_DIV1, 0x029a);
    if (ret)
    return ret;
// Program fraction FR_PLL_DIV1
    ret = meson_gxl_write_reg(phydev, BANK_BIST, FR_PLL_DIV0, 0xaaaa);
    if (ret)
    return ret;
    return 0;
    }
// This function is provided to cope with the possible failures of this phy
// during aneg process. When aneg fails, the PHY reports that aneg is done
// but the value found in MII_LPA is wrong:
// - Early failures: MII_LPA is just 0x0001. if MII_EXPANSION reports that
// the link partner (LP) supports aneg but the LP never acked our base
// code word, it is likely that we never sent it to begin with.
// - Late failures: MII_LPA is filled with a value which seems to make sense
// but it actually is not what the LP is advertising. It seems that we
// can detect this using a magic bit in the WOL bank (reg 12 - bit 12).
// If this particular bit is not set when aneg is reported being done,
// it means MII_LPA is likely to be wrong.
//
// In both case, forcing a restart of the aneg process solve the problem.
// When this failure happens, the first retry is usually successful but,
// in some cases, it may take up to 6 retries to get a decent result
//
#[no_mangle]
unsafe extern "C" fn meson_gxl_read_status(phydev: *mut phy_device) -> c_int {
    static int meson_gxl_read_status(struct phy_device *phydev)
    {
    int ret, wol, lpa, exp;
    if (phydev.autoneg == AUTONEG_ENABLE) {
    ret = genphy_aneg_done(phydev);
    if (ret < 0)
    return ret;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !ret) -> else {
    else if (!ret)
    goto read_status_continue;
// Aneg is done, let's check everything is fine
    wol = meson_gxl_read_reg(phydev, BANK_WOL, LPI_STATUS);
    if (wol < 0)
    return wol;
    lpa = phy_read(phydev, MII_LPA);
    if (lpa < 0)
    return lpa;
    exp = phy_read(phydev, MII_EXPANSION);
    if (exp < 0)
    return exp;
    if (!(wol & LPI_STATUS_RSV12) ||
    ((exp & EXPANSION_NWAY) && !(lpa & LPA_LPACK))) {
// Looks like aneg failed after all
    phydev_dbg(phydev, "LPA corruption - aneg restart\n");
    return genphy_restart_aneg(phydev);
    }
    }
    read_status_continue:
    return genphy_read_status(phydev);
    }
    static struct phy_driver meson_gxl_phy[] = {
    {
    PHY_ID_MATCH_EXACT(0x01814400),
    .name		= "Meson GXL Internal PHY",
// PHY_BASIC_FEATURES
    .flags		= PHY_IS_INTERNAL,
    .soft_reset     = genphy_soft_reset,
    .config_init	= meson_gxl_config_init,
    .read_status	= meson_gxl_read_status,
    .config_intr	= smsc_phy_config_intr,
    .handle_interrupt = smsc_phy_handle_interrupt,
    .suspend        = genphy_suspend,
    .resume         = genphy_resume,
    .read_mmd	= genphy_read_mmd_unsupported,
    .write_mmd	= genphy_write_mmd_unsupported,
    }, {
    PHY_ID_MATCH_EXACT(0x01803301),
    .name		= "Meson G12A Internal PHY",
// PHY_BASIC_FEATURES
    .flags		= PHY_IS_INTERNAL,
    .probe		= smsc_phy_probe,
    .config_init	= smsc_phy_config_init,
    .soft_reset     = genphy_soft_reset,
    .read_status	= lan87xx_read_status,
    .config_intr	= smsc_phy_config_intr,
    .handle_interrupt = smsc_phy_handle_interrupt,
    .get_tunable	= smsc_phy_get_tunable,
    .set_tunable	= smsc_phy_set_tunable,
    .suspend        = genphy_suspend,
    .resume         = genphy_resume,
    .read_mmd	= genphy_read_mmd_unsupported,
    .write_mmd	= genphy_write_mmd_unsupported,
    },
    };
    static const struct mdio_device_id __maybe_unused meson_gxl_tbl[] = {
    { PHY_ID_MATCH_VENDOR(0x01814400) },
    { PHY_ID_MATCH_VENDOR(0x01803301) },
    { }
    };
    module_phy_driver(meson_gxl_phy);
    MODULE_DEVICE_TABLE(mdio, meson_gxl_tbl);
    MODULE_DESCRIPTION("Amlogic Meson GXL Internal PHY driver");
    MODULE_AUTHOR("Baoqi wang");
    MODULE_AUTHOR("Neil Armstrong <narmstrong@baylibre.com>");
    MODULE_AUTHOR("Jerome Brunet <jbrunet@baylibre.com>");
    MODULE_LICENSE("GPL");
