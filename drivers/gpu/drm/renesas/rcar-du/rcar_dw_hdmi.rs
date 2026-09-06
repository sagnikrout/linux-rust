//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/renesas/rcar-du/rcar_dw_hdmi.c
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
// R-Car Gen3 HDMI PHY
//
// Copyright (C) 2016 Renesas Electronics Corporation
//
// Contact: Laurent Pinchart (laurent.pinchart@ideasonboard.com)
//

pub const RCAR_HDMI_PHY_OPMODE_PLLCFG: c_uint = 0x06	/* Mode of operation and PLL dividers */;
pub const RCAR_HDMI_PHY_PLLCURRGMPCTRL: c_uint = 0x10	/* PLL current and Gmp (conductance) */;
pub const RCAR_HDMI_PHY_PLLDIVCTRL: c_uint = 0x11	/* PLL dividers */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcar_hdmi_phy_params {
    pub mpixelclock: c_ulong,
    pub /: *mut *mut u16 opmode_div; / Mode of operation and PLL dividers,
    pub /: *mut *mut u16 curr_gmp; / PLL current and Gmp (conductance),
    pub /: *mut *mut u16 div; / PLL dividers,
}

    static const struct rcar_hdmi_phy_params rcar_hdmi_phy_params[] = {
    { 35500000,  0x0003, 0x0344, 0x0328 },
    { 44900000,  0x0003, 0x0285, 0x0128 },
    { 71000000,  0x0002, 0x1184, 0x0314 },
    { 90000000,  0x0002, 0x1144, 0x0114 },
    { 140250000, 0x0001, 0x20c4, 0x030a },
    { 182750000, 0x0001, 0x2084, 0x010a },
    { 281250000, 0x0000, 0x0084, 0x0305 },
    { 297000000, 0x0000, 0x0084, 0x0105 },
    { ~0UL,      0x0000, 0x0000, 0x0000 },
    };
    static enum drm_mode_status
    rcar_hdmi_mode_valid(struct dw_hdmi *hdmi, void *data,
    const struct drm_display_info *info,
    const struct drm_display_mode *mode)
    {
//
// The maximum supported clock frequency is 297 MHz, as shown in the PHY
// parameters table.
//
    if (mode.clock > 297000)
    return MODE_CLOCK_HIGH;
    return MODE_OK;
    }
    static int rcar_hdmi_phy_configure(struct dw_hdmi *hdmi, void *data,
    unsigned long mpixelclock)
    {
    const struct rcar_hdmi_phy_params *params = rcar_hdmi_phy_params;
    for (; params.mpixelclock != ~0UL; ++params) {
    if (mpixelclock <= params.mpixelclock)
    break;
    }
    if (params.mpixelclock == ~0UL)
    return -EINVAL;
    dw_hdmi_phy_i2c_write(hdmi, params.opmode_div,
    RCAR_HDMI_PHY_OPMODE_PLLCFG);
    dw_hdmi_phy_i2c_write(hdmi, params.curr_gmp,
    RCAR_HDMI_PHY_PLLCURRGMPCTRL);
    dw_hdmi_phy_i2c_write(hdmi, params.div, RCAR_HDMI_PHY_PLLDIVCTRL);
    return 0;
    }
    static const struct dw_hdmi_plat_data rcar_dw_hdmi_plat_data = {
    .output_port = 1,
    .mode_valid = rcar_hdmi_mode_valid,
    .configure_phy	= rcar_hdmi_phy_configure,
    };
#[no_mangle]
unsafe extern "C" fn rcar_dw_hdmi_probe(pdev: *mut platform_device) -> c_int {
    static int rcar_dw_hdmi_probe(struct platform_device *pdev)
    {
    struct dw_hdmi *hdmi;
    hdmi = dw_hdmi_probe(pdev, &rcar_dw_hdmi_plat_data);
    if (IS_ERR(hdmi))
    return PTR_ERR(hdmi);
    platform_set_drvdata(pdev, hdmi);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rcar_dw_hdmi_remove(pdev: *mut platform_device) {
    static void rcar_dw_hdmi_remove(struct platform_device *pdev)
    {
    struct dw_hdmi *hdmi = platform_get_drvdata(pdev);
    dw_hdmi_remove(hdmi);
    }
    static const struct of_device_id rcar_dw_hdmi_of_table[] = {
    { .compatible = "renesas,rcar-gen3-hdmi" },
    { /* Sentinel */ },
    };
    MODULE_DEVICE_TABLE(of, rcar_dw_hdmi_of_table);
    static struct platform_driver rcar_dw_hdmi_platform_driver = {
    .probe		= rcar_dw_hdmi_probe,
    .remove		= rcar_dw_hdmi_remove,
    .driver		= {
    .name	= "rcar-dw-hdmi",
    .of_match_table = rcar_dw_hdmi_of_table,
    },
    };
    module_platform_driver(rcar_dw_hdmi_platform_driver);
    MODULE_AUTHOR("Laurent Pinchart <laurent.pinchart@ideasonboard.com>");
    MODULE_DESCRIPTION("Renesas R-Car Gen3 HDMI Encoder Driver");
    MODULE_LICENSE("GPL");
