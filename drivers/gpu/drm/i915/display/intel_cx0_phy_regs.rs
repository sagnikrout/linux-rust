//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_cx0_phy_regs.h
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
// Copyright © 2023 Intel Corporation
//

// DDI Buffer Control
pub const _DDI_CLK_VALFREQ_A: c_uint = 0x64030;
pub const _DDI_CLK_VALFREQ_B: c_uint = 0x64130;

//
// Wrapper macro to convert from port number to the index used in some of the
// registers. For Display version 20 and above it converts the port number to a
// single range, starting with the TC offsets. When used together with
// _PICK_EVEN_2RANGES(idx, PORT_TC1, ...), this single range will be the second
// range. Example:
//
// PORT_TC1 -> PORT_TC1
// PORT_TC2 -> PORT_TC2
// PORT_TC3 -> PORT_TC3
// PORT_TC4 -> PORT_TC4
// PORT_A   -> PORT_TC4 + 1
// PORT_B   -> PORT_TC4 + 2
// ...
//

pub const _XELPDP_PORT_M2P_MSGBUS_CTL_LN0_A: c_uint = 0x64040;
pub const _XELPDP_PORT_M2P_MSGBUS_CTL_LN0_B: c_uint = 0x64140;
pub const _XELPDP_PORT_M2P_MSGBUS_CTL_LN0_USBC1: c_uint = 0x16F240;
pub const _XELPDP_PORT_M2P_MSGBUS_CTL_LN0_USBC2: c_uint = 0x16F440;

pub const XELPDP_PORT_P2M_COMMAND_READ_ACK: c_uint = 0x4;
pub const XELPDP_PORT_P2M_COMMAND_WRITE_ACK: c_uint = 0x5;

pub const XELPDP_MSGBUS_TIMEOUT_MS: c_int = 1;
pub const XELPDP_PCLK_PLL_ENABLE_TIMEOUT_US: c_int = 3200;
pub const XELPDP_PCLK_PLL_DISABLE_TIMEOUT_US: c_int = 20;
pub const XELPDP_PORT_BUF_SOC_READY_TIMEOUT_US: c_int = 100;
pub const XELPDP_PORT_RESET_START_TIMEOUT_US: c_int = 10;
pub const XELPDP_PORT_POWERDOWN_UPDATE_TIMEOUT_MS: c_int = 2;
pub const XELPDP_PORT_RESET_END_TIMEOUT_MS: c_int = 15;
pub const XELPDP_REFCLK_ENABLE_TIMEOUT_US: c_int = 10;
pub const _XELPDP_PORT_BUF_CTL1_LN0_A: c_uint = 0x64004;
pub const _XELPDP_PORT_BUF_CTL1_LN0_B: c_uint = 0x64104;
pub const _XELPDP_PORT_BUF_CTL1_LN0_USBC1: c_uint = 0x16F200;
pub const _XELPDP_PORT_BUF_CTL1_LN0_USBC2: c_uint = 0x16F400;

pub const XELPDP_P0_STATE_ACTIVE: c_uint = 0x0;
pub const XELPDP_P2_STATE_READY: c_uint = 0x2;
pub const XE3PLPD_P4_STATE_DISABLE: c_uint = 0x4;
pub const XELPDP_P2PG_STATE_DISABLE: c_uint = 0x9;
pub const XELPDP_P4PG_STATE_DISABLE: c_uint = 0xC;
pub const XELPDP_P2_STATE_RESET: c_uint = 0x2;
pub const _XELPDP_PORT_MSGBUS_TIMER_LN0_A: c_uint = 0x640d8;
pub const _XELPDP_PORT_MSGBUS_TIMER_LN0_B: c_uint = 0x641d8;
pub const _XELPDP_PORT_MSGBUS_TIMER_LN0_USBC1: c_uint = 0x16f258;
pub const _XELPDP_PORT_MSGBUS_TIMER_LN0_USBC2: c_uint = 0x16f458;

pub const _XELPDP_PORT_CLOCK_CTL_A: c_uint = 0x640E0;
pub const _XELPDP_PORT_CLOCK_CTL_B: c_uint = 0x641E0;
pub const _XELPDP_PORT_CLOCK_CTL_USBC1: c_uint = 0x16F260;
pub const _XELPDP_PORT_CLOCK_CTL_USBC2: c_uint = 0x16F460;

pub const XELPDP_DDI_CLOCK_SELECT_NONE: c_uint = 0x0;
pub const XELPDP_DDI_CLOCK_SELECT_MAXPCLK: c_uint = 0x8;
pub const XELPDP_DDI_CLOCK_SELECT_DIV18CLK: c_uint = 0x9;
pub const XELPDP_DDI_CLOCK_SELECT_TBT_162: c_uint = 0xc;
pub const XELPDP_DDI_CLOCK_SELECT_TBT_270: c_uint = 0xd;
pub const XELPDP_DDI_CLOCK_SELECT_TBT_540: c_uint = 0xe;
pub const XELPDP_DDI_CLOCK_SELECT_TBT_810: c_uint = 0xf;
pub const XELPDP_DDI_CLOCK_SELECT_TBT_312_5: c_uint = 0x18;
pub const XELPDP_DDI_CLOCK_SELECT_TBT_625: c_uint = 0x19;

// C10 Vendor Registers

pub const PHY_C10_VDR_CUSTOM_WIDTH: c_uint = 0xD02;

pub const PHY_C10_VDR_OVRD: c_uint = 0xD71;

pub const PHY_C10_VDR_PRE_OVRD_TX1: c_uint = 0xD80;

// PIPE SPEC Defined Registers

// C20 Registers
pub const PHY_C20_WR_ADDRESS_L: c_uint = 0xC02;
pub const PHY_C20_WR_ADDRESS_H: c_uint = 0xC03;
pub const PHY_C20_WR_DATA_L: c_uint = 0xC04;
pub const PHY_C20_WR_DATA_H: c_uint = 0xC05;
pub const PHY_C20_RD_ADDRESS_L: c_uint = 0xC06;
pub const PHY_C20_RD_ADDRESS_H: c_uint = 0xC07;
pub const PHY_C20_RD_DATA_L: c_uint = 0xC08;
pub const PHY_C20_RD_DATA_H: c_uint = 0xC09;
pub const PHY_C20_VDR_CUSTOM_SERDES_RATE: c_uint = 0xD00;

pub const PHY_C20_VDR_HDMI_RATE: c_uint = 0xD01;

pub const PHY_C20_VDR_CUSTOM_WIDTH: c_uint = 0xD02;

pub const _MTL_C20_A_TX_CNTX_CFG: c_uint = 0xCF2E;
pub const _MTL_C20_B_TX_CNTX_CFG: c_uint = 0xCF2A;
pub const _MTL_C20_A_CMN_CNTX_CFG: c_uint = 0xCDAA;
pub const _MTL_C20_B_CMN_CNTX_CFG: c_uint = 0xCDA5;
pub const _MTL_C20_A_MPLLA_CFG: c_uint = 0xCCF0;
pub const _MTL_C20_B_MPLLA_CFG: c_uint = 0xCCE5;
pub const _MTL_C20_A_MPLLB_CFG: c_uint = 0xCB5A;
pub const _MTL_C20_B_MPLLB_CFG: c_uint = 0xCB4E;
pub const _XE2HPD_C20_A_TX_CNTX_CFG: c_uint = 0xCF5E;
pub const _XE2HPD_C20_B_TX_CNTX_CFG: c_uint = 0xCF5A;
pub const _XE2HPD_C20_A_CMN_CNTX_CFG: c_uint = 0xCE8E;
pub const _XE2HPD_C20_B_CMN_CNTX_CFG: c_uint = 0xCE89;
pub const _XE2HPD_C20_A_MPLLA_CFG: c_uint = 0xCE58;
pub const _XE2HPD_C20_B_MPLLA_CFG: c_uint = 0xCE4D;
pub const _XE2HPD_C20_A_MPLLB_CFG: c_uint = 0xCCC2;
pub const _XE2HPD_C20_B_MPLLB_CFG: c_uint = 0xCCB6;

// C20 Phy VSwing Masks

// C20 HDMI computed pll definitions
pub const REFCLK_38_4_MHZ: c_int = 38400000;
pub const CLOCK_4999MHZ: c_int = 4999999999;
pub const CLOCK_9999MHZ: c_int = 9999999999;
pub const DATARATE_3000000000: c_int = 3000000000;
pub const DATARATE_3500000000: c_int = 3500000000;
pub const DATARATE_4000000000: c_int = 4000000000;
pub const MPLL_FRACN_DEN: c_uint = 0xFFFF;

pub const MPLLB_ANA_FREQ_VCO_0: c_int = 0;
pub const MPLLB_ANA_FREQ_VCO_1: c_int = 1;
pub const MPLLB_ANA_FREQ_VCO_2: c_int = 2;
pub const MPLLB_ANA_FREQ_VCO_3: c_int = 3;

pub const CAL_DAC_CODE_31: c_int = 31;

pub const CP_INT_GS_28: c_int = 28;

pub const CP_PROP_GS_30: c_int = 30;

pub const CP_INT_6: c_int = 6;

pub const CP_PROP_20: c_int = 20;

pub const V2I_2: c_int = 2;

pub const HDMI_DIV_1: c_int = 1;

