//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/msm/hdmi/hdmi_phy_8960.c
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
// Copyright (C) 2013 Red Hat
// Author: Rob Clark <robdclark@gmail.com>
//

    static void hdmi_phy_8960_powerup(struct hdmi_phy *phy,
    unsigned long pixclock)
    {
    DBG("pixclock: %lu", pixclock);
    hdmi_phy_write(phy, REG_HDMI_8960_PHY_REG2, 0x00);
    hdmi_phy_write(phy, REG_HDMI_8960_PHY_REG0, 0x1b);
    hdmi_phy_write(phy, REG_HDMI_8960_PHY_REG1, 0xf2);
    hdmi_phy_write(phy, REG_HDMI_8960_PHY_REG4, 0x00);
    hdmi_phy_write(phy, REG_HDMI_8960_PHY_REG5, 0x00);
    hdmi_phy_write(phy, REG_HDMI_8960_PHY_REG6, 0x00);
    hdmi_phy_write(phy, REG_HDMI_8960_PHY_REG7, 0x00);
    hdmi_phy_write(phy, REG_HDMI_8960_PHY_REG8, 0x00);
    hdmi_phy_write(phy, REG_HDMI_8960_PHY_REG9, 0x00);
    hdmi_phy_write(phy, REG_HDMI_8960_PHY_REG10, 0x00);
    hdmi_phy_write(phy, REG_HDMI_8960_PHY_REG11, 0x00);
    hdmi_phy_write(phy, REG_HDMI_8960_PHY_REG3, 0x20);
    }
#[no_mangle]
unsafe extern "C" fn hdmi_phy_8960_powerdown(phy: *mut hdmi_phy) {
    static void hdmi_phy_8960_powerdown(struct hdmi_phy *phy)
    {
    DBG("");
    hdmi_phy_write(phy, REG_HDMI_8960_PHY_REG2, 0x7f);
    }
    static const char * const hdmi_phy_8960_reg_names[] = {
    "core-vdda",
    };
    static const char * const hdmi_phy_8960_clk_names[] = {
    "slave_iface",
    };
    const struct hdmi_phy_cfg msm_hdmi_phy_8960_cfg = {
    .type = MSM_HDMI_PHY_8960,
    .powerup = hdmi_phy_8960_powerup,
    .powerdown = hdmi_phy_8960_powerdown,
    .reg_names = hdmi_phy_8960_reg_names,
    .num_regs = ARRAY_SIZE(hdmi_phy_8960_reg_names),
    .clk_names = hdmi_phy_8960_clk_names,
    .num_clks = ARRAY_SIZE(hdmi_phy_8960_clk_names),
    };
