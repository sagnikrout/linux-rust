//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/thunderbolt/sb_regs.h
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
//
// USB4 port sideband registers found on routers and retimers
//
// Copyright (C) 2020, Intel Corporation
// Authors: Mika Westerberg <mika.westerberg@linux.intel.com>
// Rajmohan Mani <rajmohan.mani@intel.com>
//
pub const USB4_SB_VENDOR_ID: c_uint = 0x00;
pub const USB4_SB_PRODUCT_ID: c_uint = 0x01;
pub const USB4_SB_FW_VERSION: c_uint = 0x02;
pub const USB4_SB_DEBUG_CONF: c_uint = 0x05;
pub const USB4_SB_DEBUG: c_uint = 0x06;
pub const USB4_SB_LRD_TUNING: c_uint = 0x07;
pub const USB4_SB_OPCODE: c_uint = 0x08;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum usb4_sb_opcode {
    USB4_SB_OPCODE_ERR = 0x20525245,			/* "ERR " */
    USB4_SB_OPCODE_ONS = 0x444d4321,			/* "!CMD" */
    USB4_SB_OPCODE_ROUTER_OFFLINE = 0x4e45534c,		/* "LSEN" */
    USB4_SB_OPCODE_ENUMERATE_RETIMERS = 0x4d554e45,		/* "ENUM" */
    USB4_SB_OPCODE_SET_INBOUND_SBTX = 0x5055534c,		/* "LSUP" */
    USB4_SB_OPCODE_UNSET_INBOUND_SBTX = 0x50555355,		/* "USUP" */
    USB4_SB_OPCODE_QUERY_LAST_RETIMER = 0x5453414c,		/* "LAST" */
    USB4_SB_OPCODE_QUERY_CABLE_RETIMER = 0x524c4243,	/* "CBLR" */
    USB4_SB_OPCODE_GET_NVM_SECTOR_SIZE = 0x53534e47,	/* "GNSS" */
    USB4_SB_OPCODE_NVM_SET_OFFSET = 0x53504f42,		/* "BOPS" */
    USB4_SB_OPCODE_NVM_BLOCK_WRITE = 0x574b4c42,		/* "BLKW" */
    USB4_SB_OPCODE_NVM_AUTH_WRITE = 0x48545541,		/* "AUTH" */
    USB4_SB_OPCODE_NVM_READ = 0x52524641,			/* "AFRR" */
    USB4_SB_OPCODE_READ_LANE_MARGINING_CAP = 0x50434452,	/* "RDCP" */
    USB4_SB_OPCODE_RUN_HW_LANE_MARGINING = 0x474d4852,	/* "RHMG" */
    USB4_SB_OPCODE_RUN_SW_LANE_MARGINING = 0x474d5352,	/* "RSMG" */
    USB4_SB_OPCODE_READ_SW_MARGIN_ERR = 0x57534452,		/* "RDSW" */
}

pub const USB4_SB_METADATA: c_uint = 0x09;

pub const USB4_SB_LINK_CONF: c_uint = 0x0c;
pub const USB4_SB_GEN23_TXFFE: c_uint = 0x0d;
pub const USB4_SB_GEN4_TXFFE: c_uint = 0x0e;
pub const USB4_SB_VERSION: c_uint = 0x0f;
pub const USB4_SB_DATA: c_uint = 0x12;
// USB4_SB_OPCODE_READ_LANE_MARGINING_CAP

pub const USB4_MARGIN_CAP_0_VOLTAGE_MIN: c_uint = 0x0;
pub const USB4_MARGIN_CAP_0_VOLTAGE_HL: c_uint = 0x1;
pub const USB4_MARGIN_CAP_0_VOLTAGE_BOTH: c_uint = 0x2;

pub const USB4_MARGIN_CAP_1_TIME_MIN: c_uint = 0x0;
pub const USB4_MARGIN_CAP_1_TIME_LR: c_uint = 0x1;
pub const USB4_MARGIN_CAP_1_TIME_BOTH: c_uint = 0x2;

pub const USB4_MARGIN_CAP_2_VOLTAGE_MIN: c_uint = 0x0;
pub const USB4_MARGIN_CAP_2_VOLTAGE_BOTH: c_uint = 0x1;

pub const USB4_MARGIN_CAP_2_TIME_MIN: c_uint = 0x0;
pub const USB4_MARGIN_CAP_2_TIME_BOTH: c_uint = 0x1;
// USB4_SB_OPCODE_RUN_HW_LANE_MARGINING

pub const USB4_MARGIN_HW_BER_SHIFT: c_int = 5;

// Applicable to all margin values

// Shifts for parsing the lane results
pub const USB4_MARGIN_HW_RES_LANE_SHIFT: c_int = 16;
pub const USB4_MARGIN_HW_RES_LL_SHIFT: c_int = 8;
// USB4_SB_OPCODE_RUN_SW_LANE_MARGINING

