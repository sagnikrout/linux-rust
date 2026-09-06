//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/nvidia,tegra264.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Copyright (c) 2022-2025, NVIDIA CORPORATION. All rights reserved.
pub const TEGRA264_RESET_APE_TKE: c_int = 1;
pub const TEGRA264_RESET_CEC: c_int = 2;
pub const TEGRA264_RESET_ADSP_ALL: c_int = 3;
pub const TEGRA264_RESET_RCE_ALL: c_int = 4;
pub const TEGRA264_RESET_UFSHC: c_int = 5;
pub const TEGRA264_RESET_UFSHC_AXI_M: c_int = 6;
pub const TEGRA264_RESET_UFSHC_LP_SEQ: c_int = 7;
pub const TEGRA264_RESET_DPAUX: c_int = 8;
pub const TEGRA264_RESET_EQOS_PCS: c_int = 9;
pub const TEGRA264_RESET_HWPM: c_int = 10;
pub const TEGRA264_RESET_I2C1: c_int = 11;
pub const TEGRA264_RESET_I2C2: c_int = 12;
pub const TEGRA264_RESET_I2C3: c_int = 13;
pub const TEGRA264_RESET_I2C4: c_int = 14;
pub const TEGRA264_RESET_I2C6: c_int = 15;
pub const TEGRA264_RESET_I2C7: c_int = 16;
pub const TEGRA264_RESET_I2C8: c_int = 17;
pub const TEGRA264_RESET_I2C9: c_int = 18;
pub const TEGRA264_RESET_ISP: c_int = 19;
pub const TEGRA264_RESET_LA: c_int = 20;
pub const TEGRA264_RESET_NVCSI: c_int = 21;
pub const TEGRA264_RESET_EQOS_MAC: c_int = 22;
pub const TEGRA264_RESET_PWM10: c_int = 23;
pub const TEGRA264_RESET_PWM2: c_int = 24;
pub const TEGRA264_RESET_PWM3: c_int = 25;
pub const TEGRA264_RESET_PWM4: c_int = 26;
pub const TEGRA264_RESET_PWM5: c_int = 27;
pub const TEGRA264_RESET_PWM9: c_int = 28;
pub const TEGRA264_RESET_QSPI0: c_int = 29;
pub const TEGRA264_RESET_HDA: c_int = 30;
pub const TEGRA264_RESET_HDACODEC: c_int = 31;
pub const TEGRA264_RESET_I2C0: c_int = 32;
pub const TEGRA264_RESET_I2C10: c_int = 33;
pub const TEGRA264_RESET_SDMMC1: c_int = 34;
pub const TEGRA264_RESET_MIPI_CAL: c_int = 35;
pub const TEGRA264_RESET_SPI1: c_int = 36;
pub const TEGRA264_RESET_SPI2: c_int = 37;
pub const TEGRA264_RESET_SPI3: c_int = 38;
pub const TEGRA264_RESET_SPI4: c_int = 39;
pub const TEGRA264_RESET_SPI5: c_int = 40;
pub const TEGRA264_RESET_SPI7: c_int = 41;
pub const TEGRA264_RESET_SPI8: c_int = 42;
pub const TEGRA264_RESET_SPI9: c_int = 43;
pub const TEGRA264_RESET_TACH0: c_int = 44;
pub const TEGRA264_RESET_TSEC: c_int = 45;
pub const TEGRA264_RESET_VI: c_int = 46;
pub const TEGRA264_RESET_VI1: c_int = 47;
pub const TEGRA264_RESET_PVA0_ALL: c_int = 48;
pub const TEGRA264_RESET_VIC: c_int = 49;
pub const TEGRA264_RESET_MPHY_CLK_CTL: c_int = 50;
pub const TEGRA264_RESET_MPHY_L0_RX: c_int = 51;
pub const TEGRA264_RESET_MPHY_L0_TX: c_int = 52;
pub const TEGRA264_RESET_MPHY_L1_RX: c_int = 53;
pub const TEGRA264_RESET_MPHY_L1_TX: c_int = 54;
pub const TEGRA264_RESET_ISP1: c_int = 55;
pub const TEGRA264_RESET_I2C11: c_int = 56;
pub const TEGRA264_RESET_I2C12: c_int = 57;
pub const TEGRA264_RESET_I2C14: c_int = 58;
pub const TEGRA264_RESET_I2C15: c_int = 59;
pub const TEGRA264_RESET_I2C16: c_int = 60;
pub const TEGRA264_RESET_EQOS_MACSEC: c_int = 61;
pub const TEGRA264_RESET_MGBE0_PCS: c_int = 62;
pub const TEGRA264_RESET_MGBE0_MAC: c_int = 63;
pub const TEGRA264_RESET_MGBE0_MACSEC: c_int = 64;
pub const TEGRA264_RESET_MGBE1_PCS: c_int = 65;
pub const TEGRA264_RESET_MGBE1_MAC: c_int = 66;
pub const TEGRA264_RESET_MGBE1_MACSEC: c_int = 67;
pub const TEGRA264_RESET_MGBE2_PCS: c_int = 68;
pub const TEGRA264_RESET_MGBE2_MAC: c_int = 69;
pub const TEGRA264_RESET_MGBE2_MACSEC: c_int = 70;
pub const TEGRA264_RESET_MGBE3_PCS: c_int = 71;
pub const TEGRA264_RESET_MGBE3_MAC: c_int = 72;
pub const TEGRA264_RESET_MGBE3_MACSEC: c_int = 73;
pub const TEGRA264_RESET_ADSP_CORE0: c_int = 74;
pub const TEGRA264_RESET_ADSP_CORE1: c_int = 75;
pub const TEGRA264_RESET_APE: c_int = 76;
pub const TEGRA264_RESET_XUSB1_PADCTL: c_int = 77;
pub const TEGRA264_RESET_AON_CPU_ALL: c_int = 78;
pub const TEGRA264_RESET_AON_HSP: c_int = 79;
pub const TEGRA264_RESET_UART4: c_int = 80;
pub const TEGRA264_RESET_UART5: c_int = 81;
pub const TEGRA264_RESET_UART9: c_int = 82;
pub const TEGRA264_RESET_UART10: c_int = 83;
pub const TEGRA264_RESET_UART8: c_int = 84;
