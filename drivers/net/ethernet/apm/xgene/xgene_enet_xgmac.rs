//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/apm/xgene/xgene_enet_xgmac.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
// Applied Micro X-Gene SoC Ethernet Driver
//
// Copyright (c) 2014, Applied Micro Circuits Corporation
// Authors: Iyappan Subramanian <isubramanian@apm.com>
// Keyur Chudgar <kchudgar@apm.com>
//
pub const X2_BLOCK_ETH_MAC_CSR_OFFSET: c_uint = 0x3000;
pub const BLOCK_AXG_MAC_OFFSET: c_uint = 0x0800;
pub const BLOCK_AXG_STATS_OFFSET: c_uint = 0x0800;
pub const BLOCK_AXG_MAC_CSR_OFFSET: c_uint = 0x2000;
pub const BLOCK_PCS_OFFSET: c_uint = 0x3800;
pub const XGENET_CONFIG_REG_ADDR: c_uint = 0x20;
pub const XGENET_SRST_ADDR: c_uint = 0x00;
pub const XGENET_CLKEN_ADDR: c_uint = 0x08;

pub const AXGMAC_CONFIG_0: c_uint = 0x0000;
pub const AXGMAC_CONFIG_1: c_uint = 0x0004;

pub const HSTMACADR_LSW_ADDR: c_uint = 0x0010;
pub const HSTMACADR_MSW_ADDR: c_uint = 0x0014;
pub const HSTMAXFRAME_LENGTH_ADDR: c_uint = 0x0020;
pub const XG_MCX_RX_DV_GATE_REG_0_ADDR: c_uint = 0x0004;
pub const XG_MCX_ECM_CFG_0_ADDR: c_uint = 0x0074;
pub const XG_MCX_MULTI_DPF0_ADDR: c_uint = 0x007c;
pub const XG_MCX_MULTI_DPF1_ADDR: c_uint = 0x0080;
pub const XG_DEF_PAUSE_THRES: c_uint = 0x390;
pub const XG_DEF_PAUSE_OFF_THRES: c_uint = 0x2c0;
pub const XG_RSIF_CONFIG_REG_ADDR: c_uint = 0x00a0;
pub const XG_RSIF_CLE_BUFF_THRESH: c_uint = 0x3;

pub const XG_RSIF_CONFIG1_REG_ADDR: c_uint = 0x00b8;
pub const XG_RSIF_PLC_CLE_BUFF_THRESH: c_uint = 0x1;

pub const XG_MCX_ECM_CONFIG0_REG_0_ADDR: c_uint = 0x0070;
pub const XG_MCX_ICM_ECM_DROP_COUNT_REG0_ADDR: c_uint = 0x0124;
pub const XCLE_BYPASS_REG0_ADDR: c_uint = 0x0160;
pub const XCLE_BYPASS_REG1_ADDR: c_uint = 0x0164;
pub const XG_CFG_BYPASS_ADDR: c_uint = 0x0204;
pub const XG_CFG_LINK_AGGR_RESUME_0_ADDR: c_uint = 0x0214;
pub const XG_LINK_STATUS_ADDR: c_uint = 0x0228;
pub const XG_TSIF_MSS_REG0_ADDR: c_uint = 0x02a4;
pub const XG_DEBUG_REG_ADDR: c_uint = 0x0400;
pub const XG_ENET_SPARE_CFG_REG_ADDR: c_uint = 0x040c;
pub const XG_ENET_SPARE_CFG_REG_1_ADDR: c_uint = 0x0410;
pub const XGENET_RX_DV_GATE_REG_0_ADDR: c_uint = 0x0804;
pub const XGENET_ECM_CONFIG0_REG_0: c_uint = 0x0870;
pub const XGENET_ICM_ECM_DROP_COUNT_REG0: c_uint = 0x0924;
pub const XGENET_CSR_ECM_CFG_0_ADDR: c_uint = 0x0880;
pub const XGENET_CSR_MULTI_DPF0_ADDR: c_uint = 0x0888;
pub const XGENET_CSR_MULTI_DPF1_ADDR: c_uint = 0x088c;
pub const XG_RXBUF_PAUSE_THRESH: c_uint = 0x0020;
pub const XG_MCX_ICM_CONFIG0_REG_0_ADDR: c_uint = 0x00e0;
pub const XG_MCX_ICM_CONFIG2_REG_0_ADDR: c_uint = 0x00e8;
pub const PCS_CONTROL_1: c_uint = 0x0000;

