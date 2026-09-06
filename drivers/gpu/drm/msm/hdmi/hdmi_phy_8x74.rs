//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/msm/hdmi/hdmi_phy_8x74.c
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

    static void hdmi_phy_8x74_powerup(struct hdmi_phy *phy,
    unsigned long pixclock)
    {
    hdmi_phy_write(phy, REG_HDMI_8x74_ANA_CFG0,   0x1b);
    hdmi_phy_write(phy, REG_HDMI_8x74_ANA_CFG1,   0xf2);
    hdmi_phy_write(phy, REG_HDMI_8x74_BIST_CFG0,  0x0);
    hdmi_phy_write(phy, REG_HDMI_8x74_BIST_PATN0, 0x0);
    hdmi_phy_write(phy, REG_HDMI_8x74_BIST_PATN1, 0x0);
    hdmi_phy_write(phy, REG_HDMI_8x74_BIST_PATN2, 0x0);
    hdmi_phy_write(phy, REG_HDMI_8x74_BIST_PATN3, 0x0);
    hdmi_phy_write(phy, REG_HDMI_8x74_PD_CTRL1,   0x20);
    }
#[no_mangle]
unsafe extern "C" fn hdmi_phy_8x74_powerdown(phy: *mut hdmi_phy) {
    static void hdmi_phy_8x74_powerdown(struct hdmi_phy *phy)
    {
    hdmi_phy_write(phy, REG_HDMI_8x74_PD_CTRL0, 0x7f);
    }
    static const char * const hdmi_phy_8x74_reg_names[] = {
    "core-vdda",
    "vddio",
    };
    static const char * const hdmi_phy_8x74_clk_names[] = {
    "iface", "alt_iface"
    };
    const struct hdmi_phy_cfg msm_hdmi_phy_8x74_cfg = {
    .type = MSM_HDMI_PHY_8x74,
    .powerup = hdmi_phy_8x74_powerup,
    .powerdown = hdmi_phy_8x74_powerdown,
    .reg_names = hdmi_phy_8x74_reg_names,
    .num_regs = ARRAY_SIZE(hdmi_phy_8x74_reg_names),
    .clk_names = hdmi_phy_8x74_clk_names,
    .num_clks = ARRAY_SIZE(hdmi_phy_8x74_clk_names),
    };
