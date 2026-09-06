//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/phy/mscc/mscc_mac.h
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
// Driver for Microsemi VSC85xx PHYs
//
// Copyright (c) 2020 Microsemi Corporation
//
pub const MSCC_MAC_CFG_ENA_CFG: c_uint = 0x00;
pub const MSCC_MAC_CFG_MODE_CFG: c_uint = 0x01;
pub const MSCC_MAC_CFG_MAXLEN_CFG: c_uint = 0x02;
pub const MSCC_MAC_CFG_NUM_TAGS_CFG: c_uint = 0x03;
pub const MSCC_MAC_CFG_TAGS_CFG: c_uint = 0x04;
pub const MSCC_MAC_CFG_ADV_CHK_CFG: c_uint = 0x07;
pub const MSCC_MAC_CFG_LFS_CFG: c_uint = 0x08;
pub const MSCC_MAC_CFG_LB_CFG: c_uint = 0x09;
pub const MSCC_MAC_CFG_PKTINF_CFG: c_uint = 0x0a;
pub const MSCC_MAC_PAUSE_CFG_TX_FRAME_CTRL: c_uint = 0x0b;
pub const MSCC_MAC_PAUSE_CFG_TX_FRAME_CTRL_2: c_uint = 0x0c;
pub const MSCC_MAC_PAUSE_CFG_RX_FRAME_CTRL: c_uint = 0x0d;
pub const MSCC_MAC_PAUSE_CFG_STATE: c_uint = 0x0e;
pub const MSCC_MAC_PAUSE_CFG_MAC_ADDRESS_LSB: c_uint = 0x0f;
pub const MSCC_MAC_PAUSE_CFG_MAC_ADDRESS_MSB: c_uint = 0x10;
pub const MSCC_MAC_STATUS_RX_LANE_STICKY_0: c_uint = 0x11;
pub const MSCC_MAC_STATUS_RX_LANE_STICKY_1: c_uint = 0x12;
pub const MSCC_MAC_STATUS_TX_MONITOR_STICKY: c_uint = 0x13;
pub const MSCC_MAC_STATUS_TX_MONITOR_STICKY_MASK: c_uint = 0x14;
pub const MSCC_MAC_STATUS_STICKY: c_uint = 0x15;
pub const MSCC_MAC_STATUS_STICKY_MASK: c_uint = 0x16;
pub const MSCC_MAC_STATS_32BIT_RX_HIH_CKSM_ERR_CNT: c_uint = 0x17;
pub const MSCC_MAC_STATS_32BIT_RX_XGMII_PROT_ERR_CNT: c_uint = 0x18;
pub const MSCC_MAC_STATS_32BIT_RX_SYMBOL_ERR_CNT: c_uint = 0x19;
pub const MSCC_MAC_STATS_32BIT_RX_PAUSE_CNT: c_uint = 0x1a;
pub const MSCC_MAC_STATS_32BIT_RX_UNSUP_OPCODE_CNT: c_uint = 0x1b;
pub const MSCC_MAC_STATS_32BIT_RX_UC_CNT: c_uint = 0x1c;
pub const MSCC_MAC_STATS_32BIT_RX_MC_CNT: c_uint = 0x1d;
pub const MSCC_MAC_STATS_32BIT_RX_BC_CNT: c_uint = 0x1e;
pub const MSCC_MAC_STATS_32BIT_RX_CRC_ERR_CNT: c_uint = 0x1f;
pub const MSCC_MAC_STATS_32BIT_RX_UNDERSIZE_CNT: c_uint = 0x20;
pub const MSCC_MAC_STATS_32BIT_RX_FRAGMENTS_CNT: c_uint = 0x21;
pub const MSCC_MAC_STATS_32BIT_RX_IN_RANGE_LEN_ERR_CNT: c_uint = 0x22;
pub const MSCC_MAC_STATS_32BIT_RX_OUT_OF_RANGE_LEN_ERR_CNT: c_uint = 0x23;
pub const MSCC_MAC_STATS_32BIT_RX_OVERSIZE_CNT: c_uint = 0x24;
pub const MSCC_MAC_STATS_32BIT_RX_JABBERS_CNT: c_uint = 0x25;
pub const MSCC_MAC_STATS_32BIT_RX_SIZE64_CNT: c_uint = 0x26;
pub const MSCC_MAC_STATS_32BIT_RX_SIZE65TO127_CNT: c_uint = 0x27;
pub const MSCC_MAC_STATS_32BIT_RX_SIZE128TO255_CNT: c_uint = 0x28;
pub const MSCC_MAC_STATS_32BIT_RX_SIZE256TO511_CNT: c_uint = 0x29;
pub const MSCC_MAC_STATS_32BIT_RX_SIZE512TO1023_CNT: c_uint = 0x2a;
pub const MSCC_MAC_STATS_32BIT_RX_SIZE1024TO1518_CNT: c_uint = 0x2b;
pub const MSCC_MAC_STATS_32BIT_RX_SIZE1519TOMAX_CNT: c_uint = 0x2c;
pub const MSCC_MAC_STATS_32BIT_RX_IPG_SHRINK_CNT: c_uint = 0x2d;
pub const MSCC_MAC_STATS_32BIT_TX_PAUSE_CNT: c_uint = 0x2e;
pub const MSCC_MAC_STATS_32BIT_TX_UC_CNT: c_uint = 0x2f;
pub const MSCC_MAC_STATS_32BIT_TX_MC_CNT: c_uint = 0x30;
pub const MSCC_MAC_STATS_32BIT_TX_BC_CNT: c_uint = 0x31;
pub const MSCC_MAC_STATS_32BIT_TX_SIZE64_CNT: c_uint = 0x32;
pub const MSCC_MAC_STATS_32BIT_TX_SIZE65TO127_CNT: c_uint = 0x33;
pub const MSCC_MAC_STATS_32BIT_TX_SIZE128TO255_CNT: c_uint = 0x34;
pub const MSCC_MAC_STATS_32BIT_TX_SIZE256TO511_CNT: c_uint = 0x35;
pub const MSCC_MAC_STATS_32BIT_TX_SIZE512TO1023_CNT: c_uint = 0x36;
pub const MSCC_MAC_STATS_32BIT_TX_SIZE1024TO1518_CNT: c_uint = 0x37;
pub const MSCC_MAC_STATS_32BIT_TX_SIZE1519TOMAX_CNT: c_uint = 0x38;
pub const MSCC_MAC_STATS_40BIT_RX_BAD_BYTES_CNT: c_uint = 0x39;
pub const MSCC_MAC_STATS_40BIT_RX_BAD_BYTES_MSB_CNT: c_uint = 0x3a;
pub const MSCC_MAC_STATS_40BIT_RX_OK_BYTES_CNT: c_uint = 0x3b;
pub const MSCC_MAC_STATS_40BIT_RX_OK_BYTES_MSB_CNT: c_uint = 0x3c;
pub const MSCC_MAC_STATS_40BIT_RX_IN_BYTES_CNT: c_uint = 0x3d;
pub const MSCC_MAC_STATS_40BIT_RX_IN_BYTES_MSB_CNT: c_uint = 0x3e;
pub const MSCC_MAC_STATS_40BIT_TX_OK_BYTES_CNT: c_uint = 0x3f;
pub const MSCC_MAC_STATS_40BIT_TX_OK_BYTES_MSB_CNT: c_uint = 0x40;
pub const MSCC_MAC_STATS_40BIT_TX_OUT_BYTES_CNT: c_uint = 0x41;
pub const MSCC_MAC_STATS_40BIT_TX_OUT_BYTES_MSB_CNT: c_uint = 0x42;

pub const MSCC_MAC_CFG_TAGS_CFG_RSZ: c_uint = 0x4;

pub const MSCC_PROC_IP_1588_TOP_CFG_STAT_MODE_CTL: c_uint = 0x2;

