//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_mg_phy_regs.h
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

pub const MG_TX_LINK_PARAMS_TX1LN0_PORT1: c_uint = 0x16812C;
pub const MG_TX_LINK_PARAMS_TX1LN1_PORT1: c_uint = 0x16852C;
pub const MG_TX_LINK_PARAMS_TX1LN0_PORT2: c_uint = 0x16912C;
pub const MG_TX_LINK_PARAMS_TX1LN1_PORT2: c_uint = 0x16952C;

pub const MG_TX_LINK_PARAMS_TX2LN0_PORT1: c_uint = 0x1680AC;
pub const MG_TX_LINK_PARAMS_TX2LN1_PORT1: c_uint = 0x1684AC;
pub const MG_TX_LINK_PARAMS_TX2LN0_PORT2: c_uint = 0x1690AC;
pub const MG_TX_LINK_PARAMS_TX2LN1_PORT2: c_uint = 0x1694AC;

pub const MG_TX_PISO_READLOAD_TX1LN0_PORT1: c_uint = 0x16814C;
pub const MG_TX_PISO_READLOAD_TX1LN1_PORT1: c_uint = 0x16854C;
pub const MG_TX_PISO_READLOAD_TX1LN0_PORT2: c_uint = 0x16914C;
pub const MG_TX_PISO_READLOAD_TX1LN1_PORT2: c_uint = 0x16954C;

pub const MG_TX_PISO_READLOAD_TX2LN0_PORT1: c_uint = 0x1680CC;
pub const MG_TX_PISO_READLOAD_TX2LN1_PORT1: c_uint = 0x1684CC;
pub const MG_TX_PISO_READLOAD_TX2LN0_PORT2: c_uint = 0x1690CC;
pub const MG_TX_PISO_READLOAD_TX2LN1_PORT2: c_uint = 0x1694CC;

pub const MG_TX_SWINGCTRL_TX1LN0_PORT1: c_uint = 0x168148;
pub const MG_TX_SWINGCTRL_TX1LN1_PORT1: c_uint = 0x168548;
pub const MG_TX_SWINGCTRL_TX1LN0_PORT2: c_uint = 0x169148;
pub const MG_TX_SWINGCTRL_TX1LN1_PORT2: c_uint = 0x169548;

pub const MG_TX_SWINGCTRL_TX2LN0_PORT1: c_uint = 0x1680C8;
pub const MG_TX_SWINGCTRL_TX2LN1_PORT1: c_uint = 0x1684C8;
pub const MG_TX_SWINGCTRL_TX2LN0_PORT2: c_uint = 0x1690C8;
pub const MG_TX_SWINGCTRL_TX2LN1_PORT2: c_uint = 0x1694C8;

pub const MG_TX_DRVCTRL_TX1LN0_TXPORT1: c_uint = 0x168144;
pub const MG_TX_DRVCTRL_TX1LN1_TXPORT1: c_uint = 0x168544;
pub const MG_TX_DRVCTRL_TX1LN0_TXPORT2: c_uint = 0x169144;
pub const MG_TX_DRVCTRL_TX1LN1_TXPORT2: c_uint = 0x169544;
pub const MG_TX_DRVCTRL_TX1LN0_TXPORT3: c_uint = 0x16A144;
pub const MG_TX_DRVCTRL_TX1LN1_TXPORT3: c_uint = 0x16A544;
pub const MG_TX_DRVCTRL_TX1LN0_TXPORT4: c_uint = 0x16B144;
pub const MG_TX_DRVCTRL_TX1LN1_TXPORT4: c_uint = 0x16B544;

pub const MG_TX_DRVCTRL_TX2LN0_PORT1: c_uint = 0x1680C4;
pub const MG_TX_DRVCTRL_TX2LN1_PORT1: c_uint = 0x1684C4;
pub const MG_TX_DRVCTRL_TX2LN0_PORT2: c_uint = 0x1690C4;
pub const MG_TX_DRVCTRL_TX2LN1_PORT2: c_uint = 0x1694C4;

pub const MG_CLKHUB_LN0_PORT1: c_uint = 0x16839C;
pub const MG_CLKHUB_LN1_PORT1: c_uint = 0x16879C;
pub const MG_CLKHUB_LN0_PORT2: c_uint = 0x16939C;
pub const MG_CLKHUB_LN1_PORT2: c_uint = 0x16979C;

pub const MG_TX_DCC_TX1LN0_PORT1: c_uint = 0x168110;
pub const MG_TX_DCC_TX1LN1_PORT1: c_uint = 0x168510;
pub const MG_TX_DCC_TX1LN0_PORT2: c_uint = 0x169110;
pub const MG_TX_DCC_TX1LN1_PORT2: c_uint = 0x169510;

pub const MG_TX_DCC_TX2LN0_PORT1: c_uint = 0x168090;
pub const MG_TX_DCC_TX2LN1_PORT1: c_uint = 0x168490;
pub const MG_TX_DCC_TX2LN0_PORT2: c_uint = 0x169090;
pub const MG_TX_DCC_TX2LN1_PORT2: c_uint = 0x169490;

pub const MG_DP_MODE_LN0_ACU_PORT1: c_uint = 0x1683A0;
pub const MG_DP_MODE_LN1_ACU_PORT1: c_uint = 0x1687A0;
pub const MG_DP_MODE_LN0_ACU_PORT2: c_uint = 0x1693A0;
pub const MG_DP_MODE_LN1_ACU_PORT2: c_uint = 0x1697A0;

pub const FIA1_BASE: c_uint = 0x163000;
pub const FIA2_BASE: c_uint = 0x16E000;
pub const FIA3_BASE: c_uint = 0x16F000;

// ICL PHY DFLEX registers

pub const _MG_REFCLKIN_CTL_PORT1: c_uint = 0x16892C;
pub const _MG_REFCLKIN_CTL_PORT2: c_uint = 0x16992C;

pub const _MG_CLKTOP2_CORECLKCTL1_PORT1: c_uint = 0x1688D8;
pub const _MG_CLKTOP2_CORECLKCTL1_PORT2: c_uint = 0x1698D8;

pub const _MG_CLKTOP2_HSCLKCTL_PORT1: c_uint = 0x1688D4;
pub const _MG_CLKTOP2_HSCLKCTL_PORT2: c_uint = 0x1698D4;

pub const MG_CLKTOP2_HSCLKCTL_DSDIV_RATIO_SHIFT: c_int = 8;

pub const _MG_PLL_DIV0_PORT1: c_uint = 0x168A00;
pub const _MG_PLL_DIV0_PORT2: c_uint = 0x169A00;

pub const MG_PLL_DIV0_FBDIV_FRAC_SHIFT: c_int = 8;

pub const _MG_PLL_DIV1_PORT1: c_uint = 0x168A04;
pub const _MG_PLL_DIV1_PORT2: c_uint = 0x169A04;

pub const _MG_PLL_LF_PORT1: c_uint = 0x168A08;
pub const _MG_PLL_LF_PORT2: c_uint = 0x169A08;

pub const _MG_PLL_FRAC_LOCK_PORT1: c_uint = 0x168A0C;
pub const _MG_PLL_FRAC_LOCK_PORT2: c_uint = 0x169A0C;

pub const _MG_PLL_SSC_PORT1: c_uint = 0x168A10;
pub const _MG_PLL_SSC_PORT2: c_uint = 0x169A10;

pub const _MG_PLL_BIAS_PORT1: c_uint = 0x168A14;
pub const _MG_PLL_BIAS_PORT2: c_uint = 0x169A14;

pub const _MG_PLL_TDC_COLDST_BIAS_PORT1: c_uint = 0x168A18;
pub const _MG_PLL_TDC_COLDST_BIAS_PORT2: c_uint = 0x169A18;

