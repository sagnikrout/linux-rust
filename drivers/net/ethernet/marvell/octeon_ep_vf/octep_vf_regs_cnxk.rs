//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/marvell/octeon_ep_vf/octep_vf_regs_cnxk.h
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
// Marvell Octeon EP (EndPoint) VF Ethernet Driver
//
// Copyright (C) 2020 Marvell.
//
// ############################ RST #########################
pub const CNXK_VF_CONFIG_XPANSION_BAR: c_uint = 0x38;
pub const CNXK_VF_CONFIG_PCIE_CAP: c_uint = 0x70;
pub const CNXK_VF_CONFIG_PCIE_DEVCAP: c_uint = 0x74;
pub const CNXK_VF_CONFIG_PCIE_DEVCTL: c_uint = 0x78;
pub const CNXK_VF_CONFIG_PCIE_LINKCAP: c_uint = 0x7C;
pub const CNXK_VF_CONFIG_PCIE_LINKCTL: c_uint = 0x80;
pub const CNXK_VF_CONFIG_PCIE_SLOTCAP: c_uint = 0x84;
pub const CNXK_VF_CONFIG_PCIE_SLOTCTL: c_uint = 0x88;

// ###################### RING IN REGISTERS #########################
pub const CNXK_VF_SDP_R_IN_CONTROL_START: c_uint = 0x10000;
pub const CNXK_VF_SDP_R_IN_ENABLE_START: c_uint = 0x10010;
pub const CNXK_VF_SDP_R_IN_INSTR_BADDR_START: c_uint = 0x10020;
pub const CNXK_VF_SDP_R_IN_INSTR_RSIZE_START: c_uint = 0x10030;
pub const CNXK_VF_SDP_R_IN_INSTR_DBELL_START: c_uint = 0x10040;
pub const CNXK_VF_SDP_R_IN_CNTS_START: c_uint = 0x10050;
pub const CNXK_VF_SDP_R_IN_INT_LEVELS_START: c_uint = 0x10060;
pub const CNXK_VF_SDP_R_IN_PKT_CNT_START: c_uint = 0x10080;
pub const CNXK_VF_SDP_R_IN_BYTE_CNT_START: c_uint = 0x10090;
pub const CNXK_VF_SDP_R_ERR_TYPE_START: c_uint = 0x10400;

// ------------------ R_IN Masks ----------------
// Rings per Virtual Function

// Number of instructions to be read in one MAC read request.
// setting to Max value(4)
//

// ###################### RING OUT REGISTERS #########################
pub const CNXK_VF_SDP_R_OUT_CNTS_START: c_uint = 0x10100;
pub const CNXK_VF_SDP_R_OUT_INT_LEVELS_START: c_uint = 0x10110;
pub const CNXK_VF_SDP_R_OUT_SLIST_BADDR_START: c_uint = 0x10120;
pub const CNXK_VF_SDP_R_OUT_SLIST_RSIZE_START: c_uint = 0x10130;
pub const CNXK_VF_SDP_R_OUT_SLIST_DBELL_START: c_uint = 0x10140;
pub const CNXK_VF_SDP_R_OUT_CONTROL_START: c_uint = 0x10150;
pub const CNXK_VF_SDP_R_OUT_WMARK_START: c_uint = 0x10160;
pub const CNXK_VF_SDP_R_OUT_ENABLE_START: c_uint = 0x10170;
pub const CNXK_VF_SDP_R_OUT_PKT_CNT_START: c_uint = 0x10180;
pub const CNXK_VF_SDP_R_OUT_BYTE_CNT_START: c_uint = 0x10190;

// ------------------ R_OUT Masks ----------------

// ##################### Mail Box Registers ##########################
// SDP PF to VF Mailbox Data Register
pub const CNXK_VF_SDP_R_MBOX_PF_VF_DATA_START: c_uint = 0x10210;
// SDP Packet PF to VF Mailbox Interrupt Register
pub const CNXK_VF_SDP_R_MBOX_PF_VF_INT_START: c_uint = 0x10220;
// SDP VF to PF Mailbox Data Register
pub const CNXK_VF_SDP_R_MBOX_VF_PF_DATA_START: c_uint = 0x10230;

