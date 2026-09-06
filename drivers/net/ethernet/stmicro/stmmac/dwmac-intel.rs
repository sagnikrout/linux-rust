//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/stmicro/stmmac/dwmac-intel.h
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
// Copyright (c) 2020, Intel Corporation
// DWMAC Intel header file
//
pub const POLL_DELAY_US: c_int = 8;
// SERDES Register
pub const SERDES_GCR: c_uint = 0x0	/* Global Conguration */;
pub const SERDES_GSR0: c_uint = 0x5	/* Global Status Reg0 */;
pub const SERDES_GCR0: c_uint = 0xb	/* Global Configuration Reg0 */;
// SERDES defines

pub const SERDES_PWR_ST_SHIFT: c_int = 4;
pub const SERDES_PWR_ST_P0: c_uint = 0x0;
pub const SERDES_PWR_ST_P3: c_uint = 0x3;
pub const SERDES_LINK_MODE_2G5: c_uint = 0x3;
pub const SERSED_LINK_MODE_1G: c_uint = 0x2;
pub const SERDES_PCLK_37p5MHZ: c_uint = 0x0;
pub const SERDES_PCLK_70MHZ: c_uint = 0x1;
pub const SERDES_RATE_PCIE_GEN1: c_uint = 0x0;
pub const SERDES_RATE_PCIE_GEN2: c_uint = 0x1;
pub const SERDES_RATE_PCIE_SHIFT: c_int = 8;
pub const SERDES_PCLK_SHIFT: c_int = 12;
pub const INTEL_MGBE_ADHOC_ADDR: c_uint = 0x15;
pub const INTEL_MGBE_XPCS_ADDR: c_uint = 0x16;
// Cross-timestamping defines
pub const ART_CPUID_LEAF: c_uint = 0x15;
pub const EHL_PSE_ART_MHZ: c_int = 19200000;
// Selection for PTP Clock Freq belongs to PSE & PCH GbE

// Modphy Register index
pub const R_PCH_FIA_15_PCR_LOS1_REG_BASE: c_int = 8;
pub const R_PCH_FIA_15_PCR_LOS2_REG_BASE: c_int = 9;
pub const R_PCH_FIA_15_PCR_LOS3_REG_BASE: c_int = 10;
pub const R_PCH_FIA_15_PCR_LOS4_REG_BASE: c_int = 11;
pub const R_PCH_FIA_15_PCR_LOS5_REG_BASE: c_int = 12;

pub const PID_MODPHY1_B_MODPHY_PCR_LCPLL_DWORD0: c_int = 13;
pub const PID_MODPHY1_N_MODPHY_PCR_LCPLL_DWORD2: c_int = 14;
pub const PID_MODPHY1_N_MODPHY_PCR_LCPLL_DWORD7: c_int = 15;
pub const PID_MODPHY1_N_MODPHY_PCR_LPPLL_DWORD10: c_int = 16;
pub const PID_MODPHY1_N_MODPHY_PCR_CMN_ANA_DWORD30: c_int = 17;
pub const PID_MODPHY3_B_MODPHY_PCR_LCPLL_DWORD0: c_int = 18;
pub const PID_MODPHY3_N_MODPHY_PCR_LCPLL_DWORD2: c_int = 19;
pub const PID_MODPHY3_N_MODPHY_PCR_LCPLL_DWORD7: c_int = 20;
pub const PID_MODPHY3_N_MODPHY_PCR_LPPLL_DWORD10: c_int = 21;
pub const PID_MODPHY3_N_MODPHY_PCR_CMN_ANA_DWORD30: c_int = 22;
pub const B_MODPHY_PCR_LCPLL_DWORD0_1G: c_uint = 0x46AAAA41;
pub const N_MODPHY_PCR_LCPLL_DWORD2_1G: c_uint = 0x00000139;
pub const N_MODPHY_PCR_LCPLL_DWORD7_1G: c_uint = 0x002A0003;
pub const N_MODPHY_PCR_LPPLL_DWORD10_1G: c_uint = 0x00170008;
pub const N_MODPHY_PCR_CMN_ANA_DWORD30_1G: c_uint = 0x0000D4AC;
pub const B_MODPHY_PCR_LCPLL_DWORD0_2P5G: c_uint = 0x58555551;
pub const N_MODPHY_PCR_LCPLL_DWORD2_2P5G: c_uint = 0x0000012D;
pub const N_MODPHY_PCR_LCPLL_DWORD7_2P5G: c_uint = 0x001F0003;
pub const N_MODPHY_PCR_LPPLL_DWORD10_2P5G: c_uint = 0x00170008;
pub const N_MODPHY_PCR_CMN_ANA_DWORD30_2P5G: c_uint = 0x8200ACAC;
