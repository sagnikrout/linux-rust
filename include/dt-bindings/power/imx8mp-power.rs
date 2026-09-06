//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/power/imx8mp-power.h
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


// SPDX-License-Identifier: (GPL-2.0 OR MIT)
//
// Copyright (C) 2020 Pengutronix, Sascha Hauer <kernel@pengutronix.de>
//
pub const IMX8MP_POWER_DOMAIN_MIPI_PHY1: c_int = 0;
pub const IMX8MP_POWER_DOMAIN_PCIE_PHY: c_int = 1;
pub const IMX8MP_POWER_DOMAIN_USB1_PHY: c_int = 2;
pub const IMX8MP_POWER_DOMAIN_USB2_PHY: c_int = 3;
pub const IMX8MP_POWER_DOMAIN_MLMIX: c_int = 4;
pub const IMX8MP_POWER_DOMAIN_AUDIOMIX: c_int = 5;
pub const IMX8MP_POWER_DOMAIN_GPU2D: c_int = 6;
pub const IMX8MP_POWER_DOMAIN_GPUMIX: c_int = 7;
pub const IMX8MP_POWER_DOMAIN_VPUMIX: c_int = 8;
pub const IMX8MP_POWER_DOMAIN_GPU3D: c_int = 9;
pub const IMX8MP_POWER_DOMAIN_MEDIAMIX: c_int = 10;
pub const IMX8MP_POWER_DOMAIN_VPU_G1: c_int = 11;
pub const IMX8MP_POWER_DOMAIN_VPU_G2: c_int = 12;
pub const IMX8MP_POWER_DOMAIN_VPU_VC8000E: c_int = 13;
pub const IMX8MP_POWER_DOMAIN_HDMIMIX: c_int = 14;
pub const IMX8MP_POWER_DOMAIN_HDMI_PHY: c_int = 15;
pub const IMX8MP_POWER_DOMAIN_MIPI_PHY2: c_int = 16;
pub const IMX8MP_POWER_DOMAIN_HSIOMIX: c_int = 17;
pub const IMX8MP_POWER_DOMAIN_MEDIAMIX_ISPDWP: c_int = 18;
pub const IMX8MP_HSIOBLK_PD_USB: c_int = 0;
pub const IMX8MP_HSIOBLK_PD_USB_PHY1: c_int = 1;
pub const IMX8MP_HSIOBLK_PD_USB_PHY2: c_int = 2;
pub const IMX8MP_HSIOBLK_PD_PCIE: c_int = 3;
pub const IMX8MP_HSIOBLK_PD_PCIE_PHY: c_int = 4;
pub const IMX8MP_MEDIABLK_PD_MIPI_DSI_1: c_int = 0;
pub const IMX8MP_MEDIABLK_PD_MIPI_CSI2_1: c_int = 1;
pub const IMX8MP_MEDIABLK_PD_LCDIF_1: c_int = 2;
pub const IMX8MP_MEDIABLK_PD_ISI: c_int = 3;
pub const IMX8MP_MEDIABLK_PD_MIPI_CSI2_2: c_int = 4;
pub const IMX8MP_MEDIABLK_PD_LCDIF_2: c_int = 5;
pub const IMX8MP_MEDIABLK_PD_ISP: c_int = 6;
pub const IMX8MP_MEDIABLK_PD_DWE: c_int = 7;
pub const IMX8MP_MEDIABLK_PD_MIPI_DSI_2: c_int = 8;
pub const IMX8MP_HDMIBLK_PD_IRQSTEER: c_int = 0;
pub const IMX8MP_HDMIBLK_PD_LCDIF: c_int = 1;
pub const IMX8MP_HDMIBLK_PD_PAI: c_int = 2;
pub const IMX8MP_HDMIBLK_PD_PVI: c_int = 3;
pub const IMX8MP_HDMIBLK_PD_TRNG: c_int = 4;
pub const IMX8MP_HDMIBLK_PD_HDMI_TX: c_int = 5;
pub const IMX8MP_HDMIBLK_PD_HDMI_TX_PHY: c_int = 6;
pub const IMX8MP_HDMIBLK_PD_HDCP: c_int = 7;
pub const IMX8MP_HDMIBLK_PD_HRV: c_int = 8;
pub const IMX8MP_VPUBLK_PD_G1: c_int = 0;
pub const IMX8MP_VPUBLK_PD_G2: c_int = 1;
pub const IMX8MP_VPUBLK_PD_VC8000E: c_int = 2;
