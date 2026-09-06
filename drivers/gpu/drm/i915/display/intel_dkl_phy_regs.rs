//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_dkl_phy_regs.h
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2022 Intel Corporation
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_dkl_phy_reg {
    pub reg:24: u32,
    pub bank_idx:4: u32,
}

pub const _DKL_PHY1_BASE: c_uint = 0x168000;
pub const _DKL_PHY2_BASE: c_uint = 0x169000;
pub const _DKL_PHY3_BASE: c_uint = 0x16A000;
pub const _DKL_PHY4_BASE: c_uint = 0x16B000;
pub const _DKL_PHY5_BASE: c_uint = 0x16C000;
pub const _DKL_PHY6_BASE: c_uint = 0x16D000;

// DEKEL PHY MMIO Address = Phy base + (internal address & ~index_mask)

pub const _DKL_BANK_SHIFT: c_int = 12;

pub const _DKL_PCS_DW5_LN0: c_uint = 0x0014;
pub const _DKL_PCS_DW5_LN1: c_uint = 0x1014;

pub const _DKL_PLL_DIV0: c_uint = 0x2200;

pub const _DKL_PLL_DIV1: c_uint = 0x2204;

pub const _DKL_PLL_SSC: c_uint = 0x2210;

pub const _DKL_PLL_BIAS: c_uint = 0x2214;

pub const _DKL_PLL_TDC_COLDST_BIAS: c_uint = 0x2218;

pub const _DKL_REFCLKIN_CTL: c_uint = 0x212C;

// Bits are the same as MG_REFCLKIN_CTL
pub const _DKL_CLKTOP2_HSCLKCTL: c_uint = 0x20D4;

// Bits are the same as MG_CLKTOP2_HSCLKCTL
pub const _DKL_CLKTOP2_CORECLKCTL1: c_uint = 0x20D8;

// Bits are the same as MG_CLKTOP2_CORECLKCTL1
pub const _DKL_TX_DPCNTL0_LN0: c_uint = 0x02C0;
pub const _DKL_TX_DPCNTL0_LN1: c_uint = 0x12C0;

pub const _DKL_TX_DPCNTL1_LN0: c_uint = 0x02C4;
pub const _DKL_TX_DPCNTL1_LN1: c_uint = 0x12C4;

// Bits are the same as DKL_TX_DPCNTRL0
pub const _DKL_TX_DPCNTL2_LN0: c_uint = 0x02C8;
pub const _DKL_TX_DPCNTL2_LN1: c_uint = 0x12C8;

pub const _DKL_TX_FW_CALIB_LN0: c_uint = 0x02F8;
pub const _DKL_TX_FW_CALIB_LN1: c_uint = 0x12F8;

pub const _DKL_TX_PMD_LANE_SUS_LN0: c_uint = 0x0D00;
pub const _DKL_TX_PMD_LANE_SUS_LN1: c_uint = 0x1D00;

pub const _DKL_TX_DW17_LN0: c_uint = 0x0DC4;
pub const _DKL_TX_DW17_LN1: c_uint = 0x1DC4;

pub const _DKL_TX_DW18_LN0: c_uint = 0x0DC8;
pub const _DKL_TX_DW18_LN1: c_uint = 0x1DC8;

pub const _DKL_DP_MODE_LN0: c_uint = 0x00A0;
pub const _DKL_DP_MODE_LN1: c_uint = 0x10A0;

pub const _DKL_CMN_UC_DW27: c_uint = 0x236C;

//
// Each Dekel PHY is addressed through a 4KB aperture. Each PHY has more than
// 4KB of register space, so a separate index is programmed in HIP_INDEX_REG0
// or HIP_INDEX_REG1, based on the port number, to set the upper 2 address
// bits that point the 4KB window into the full PHY register space.
//
pub const _HIP_INDEX_REG0: c_uint = 0x1010A0;
pub const _HIP_INDEX_REG1: c_uint = 0x1010A4;

