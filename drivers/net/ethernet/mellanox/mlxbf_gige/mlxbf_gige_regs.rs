//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlxbf_gige/mlxbf_gige_regs.h
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


// SPDX-License-Identifier: GPL-2.0-only OR BSD-3-Clause
// Header file for Mellanox BlueField GigE register defines
//
// Copyright (C) 2020-2021 NVIDIA CORPORATION & AFFILIATES
//

pub const MLXBF_GIGE_VERSION: c_uint = 0x0000;
pub const MLXBF_GIGE_VERSION_BF2: c_uint = 0x0;
pub const MLXBF_GIGE_VERSION_BF3: c_uint = 0x1;
pub const MLXBF_GIGE_STATUS: c_uint = 0x0010;

pub const MLXBF_GIGE_INT_STATUS: c_uint = 0x0028;

pub const MLXBF_GIGE_INT_EN: c_uint = 0x0030;

pub const MLXBF_GIGE_INT_MASK: c_uint = 0x0038;

pub const MLXBF_GIGE_CONTROL: c_uint = 0x0040;

pub const MLXBF_GIGE_RX_WQ_BASE: c_uint = 0x0200;
pub const MLXBF_GIGE_RX_WQE_SIZE_LOG2: c_uint = 0x0208;
pub const MLXBF_GIGE_RX_WQE_SIZE_LOG2_RESET_VAL: c_int = 7;
pub const MLXBF_GIGE_RX_CQ_BASE: c_uint = 0x0210;
pub const MLXBF_GIGE_TX_WQ_BASE: c_uint = 0x0218;
pub const MLXBF_GIGE_TX_WQ_SIZE_LOG2: c_uint = 0x0220;
pub const MLXBF_GIGE_TX_WQ_SIZE_LOG2_RESET_VAL: c_int = 7;
pub const MLXBF_GIGE_TX_CI_UPDATE_ADDRESS: c_uint = 0x0228;
pub const MLXBF_GIGE_RX_WQE_PI: c_uint = 0x0230;
pub const MLXBF_GIGE_TX_PRODUCER_INDEX: c_uint = 0x0238;
pub const MLXBF_GIGE_RX_MAC_FILTER: c_uint = 0x0240;
pub const MLXBF_GIGE_RX_MAC_FILTER_STRIDE: c_uint = 0x0008;
pub const MLXBF_GIGE_RX_DIN_DROP_COUNTER: c_uint = 0x0260;
pub const MLXBF_GIGE_TX_CONSUMER_INDEX: c_uint = 0x0310;
pub const MLXBF_GIGE_TX_CONTROL: c_uint = 0x0318;

pub const MLXBF_GIGE_TX_STATUS: c_uint = 0x0388;

pub const MLXBF_GIGE_RX_MAC_FILTER_DMAC_RANGE_START: c_uint = 0x0520;
pub const MLXBF_GIGE_RX_MAC_FILTER_DMAC_RANGE_END: c_uint = 0x0528;
pub const MLXBF_GIGE_RX_MAC_FILTER_GENERAL: c_uint = 0x0530;

pub const MLXBF_GIGE_RX_MAC_FILTER_COUNT_DISC: c_uint = 0x0540;

pub const MLXBF_GIGE_RX_MAC_FILTER_COUNT_PASS: c_uint = 0x0548;

pub const MLXBF_GIGE_RX_PASS_COUNTER_ALL: c_uint = 0x0550;
pub const MLXBF_GIGE_RX_DISC_COUNTER_ALL: c_uint = 0x0560;
pub const MLXBF_GIGE_RX: c_uint = 0x0578;

pub const MLXBF_GIGE_RX_DMA: c_uint = 0x0580;

pub const MLXBF_GIGE_RX_CQE_PACKET_CI: c_uint = 0x05b0;
pub const MLXBF_GIGE_MAC_CFG: c_uint = 0x05e8;
// NOTE: MLXBF_GIGE_MAC_CFG is the last defined register offset,
// so use that plus size of single register to derive total size
//

pub const MLXBF_GIGE_PLU_TX_REG0: c_uint = 0x80;

pub const MLXBF_GIGE_PLU_RX_REG0: c_uint = 0x10;

pub const MLXBF_GIGE_1G_SGMII_MODE: c_uint = 0x0;
pub const MLXBF_GIGE_10M_SGMII_MODE: c_uint = 0x1;
pub const MLXBF_GIGE_100M_SGMII_MODE: c_uint = 0x2;
// ipg_size default value for 1G is fixed by HW to 11 + End = 12.
// So for 100M it is 12 * 10 - 1 = 119
// For 10M, it is 12 * 100 - 1 = 1199
//
pub const MLXBF_GIGE_1G_IPG_SIZE: c_int = 11;
pub const MLXBF_GIGE_100M_IPG_SIZE: c_int = 119;
pub const MLXBF_GIGE_10M_IPG_SIZE: c_int = 1199;
// Offsets into OOB LLU block for pause frame counters
pub const MLXBF_GIGE_BF2_TX_PAUSE_CNT_HI: c_uint = 0x33d8;
pub const MLXBF_GIGE_BF2_TX_PAUSE_CNT_LO: c_uint = 0x33dc;
pub const MLXBF_GIGE_BF2_RX_PAUSE_CNT_HI: c_uint = 0x3210;
pub const MLXBF_GIGE_BF2_RX_PAUSE_CNT_LO: c_uint = 0x3214;
pub const MLXBF_GIGE_BF3_TX_PAUSE_CNT_HI: c_uint = 0x3a88;
pub const MLXBF_GIGE_BF3_TX_PAUSE_CNT_LO: c_uint = 0x3a8c;
pub const MLXBF_GIGE_BF3_RX_PAUSE_CNT_HI: c_uint = 0x38c0;
pub const MLXBF_GIGE_BF3_RX_PAUSE_CNT_LO: c_uint = 0x38c4;

pub const MLXBF_GIGE_BF2_LLU_GENERAL_CONFIG: c_uint = 0x2110;
pub const MLXBF_GIGE_BF3_LLU_GENERAL_CONFIG: c_uint = 0x2030;

