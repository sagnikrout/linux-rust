//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/can/dev/peak_canfd.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// CAN driver for PEAK System micro-CAN based adapters
//
// Copyright (C) 2003-2025 PEAK System-Technik GmbH
// Author: Stéphane Grosjean <s.grosjean@peak-system.fr>
//
// uCAN commands opcodes list (low-order 10 bits)
pub const PUCAN_CMD_NOP: c_uint = 0x000;
pub const PUCAN_CMD_RESET_MODE: c_uint = 0x001;
pub const PUCAN_CMD_NORMAL_MODE: c_uint = 0x002;
pub const PUCAN_CMD_LISTEN_ONLY_MODE: c_uint = 0x003;
pub const PUCAN_CMD_TIMING_SLOW: c_uint = 0x004;
pub const PUCAN_CMD_TIMING_FAST: c_uint = 0x005;
pub const PUCAN_CMD_SET_STD_FILTER: c_uint = 0x006;
pub const PUCAN_CMD_RESERVED2: c_uint = 0x007;
pub const PUCAN_CMD_FILTER_STD: c_uint = 0x008;
pub const PUCAN_CMD_TX_ABORT: c_uint = 0x009;
pub const PUCAN_CMD_WR_ERR_CNT: c_uint = 0x00a;
pub const PUCAN_CMD_SET_EN_OPTION: c_uint = 0x00b;
pub const PUCAN_CMD_CLR_DIS_OPTION: c_uint = 0x00c;
pub const PUCAN_CMD_RX_BARRIER: c_uint = 0x010;
pub const PUCAN_CMD_END_OF_COLLECTION: c_uint = 0x3ff;
// uCAN received messages list
pub const PUCAN_MSG_CAN_RX: c_uint = 0x0001;
pub const PUCAN_MSG_ERROR: c_uint = 0x0002;
pub const PUCAN_MSG_STATUS: c_uint = 0x0003;
pub const PUCAN_MSG_BUSLOAD: c_uint = 0x0004;
pub const PUCAN_MSG_CACHE_CRITICAL: c_uint = 0x0102;
// uCAN transmitted messages
pub const PUCAN_MSG_CAN_TX: c_uint = 0x1000;
// uCAN command common header
// return the opcode from the opcode_channel field of a command
pub const PUCAN_TSLOW_BRP_BITS: c_int = 10;
pub const PUCAN_TSLOW_TSGEG1_BITS: c_int = 8;
pub const PUCAN_TSLOW_TSGEG2_BITS: c_int = 7;
pub const PUCAN_TSLOW_SJW_BITS: c_int = 7;

// uCAN TIMING_SLOW command fields

pub const PUCAN_TFAST_BRP_BITS: c_int = 10;
pub const PUCAN_TFAST_TSGEG1_BITS: c_int = 5;
pub const PUCAN_TFAST_TSGEG2_BITS: c_int = 4;
pub const PUCAN_TFAST_SJW_BITS: c_int = 4;

// uCAN TIMING_FAST command fields

// uCAN FILTER_STD command fields
pub const PUCAN_FLTSTD_ROW_IDX_BITS: c_int = 6;

// uCAN SET_STD_FILTER command fields
// uCAN TX_ABORT commands fields
pub const PUCAN_TX_ABORT_FLUSH: c_uint = 0x0001;
// uCAN WR_ERR_CNT command fields
pub const PUCAN_WRERRCNT_TE: c_uint = 0x4000	/* Tx error cntr write Enable */;
pub const PUCAN_WRERRCNT_RE: c_uint = 0x8000	/* Rx error cntr write Enable */;
// uCAN SET_EN/CLR_DIS _OPTION command fields
pub const PUCAN_OPTION_ERROR: c_uint = 0x0001;
pub const PUCAN_OPTION_BUSLOAD: c_uint = 0x0002;
pub const PUCAN_OPTION_CANDFDISO: c_uint = 0x0004;
// uCAN received messages global format
// uCAN flags for CAN/CANFD messages
pub const PUCAN_MSG_SELF_RECEIVE: c_uint = 0x80;
pub const PUCAN_MSG_ERROR_STATE_IND: c_uint = 0x40	/* error state indicator */;
pub const PUCAN_MSG_BITRATE_SWITCH: c_uint = 0x20	/* bitrate switch */;
pub const PUCAN_MSG_EXT_DATA_LEN: c_uint = 0x10	/* extended data length */;
pub const PUCAN_MSG_SINGLE_SHOT: c_uint = 0x08;
pub const PUCAN_MSG_LOOPED_BACK: c_uint = 0x04;
pub const PUCAN_MSG_EXT_ID: c_uint = 0x02;
pub const PUCAN_MSG_RTR: c_uint = 0x01;
// uCAN error types
pub const PUCAN_ERMSG_BIT_ERROR: c_int = 0;
pub const PUCAN_ERMSG_FORM_ERROR: c_int = 1;
pub const PUCAN_ERMSG_STUFF_ERROR: c_int = 2;
pub const PUCAN_ERMSG_OTHER_ERROR: c_int = 3;
pub const PUCAN_ERMSG_ERR_CNT_DEC: c_int = 4;
pub const PUCAN_RX_BARRIER: c_uint = 0x10;
pub const PUCAN_BUS_PASSIVE: c_uint = 0x20;
pub const PUCAN_BUS_WARNING: c_uint = 0x40;
pub const PUCAN_BUS_BUSOFF: c_uint = 0x80;
// uCAN transmitted message format

// build the cmd opcode_channel field with respect to the correct endianness
extern "C" {
    pub fn cpu_to_le16(0x3ff): ((index) << 12) | ((opcode) &) -> return;
}
// return the channel number part from any received message channel_dlc field
// return the dlc value from any received message channel_dlc field
