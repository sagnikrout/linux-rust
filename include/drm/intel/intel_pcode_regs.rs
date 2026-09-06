//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/intel/intel_pcode_regs.h
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
// Copyright © 2026 Intel Corporation

pub const GEN6_PCODE_ERROR_MASK: c_uint = 0xFF;
pub const GEN6_PCODE_SUCCESS: c_uint = 0x0;
pub const GEN6_PCODE_ILLEGAL_CMD: c_uint = 0x1;
pub const GEN6_PCODE_MIN_FREQ_TABLE_GT_RATIO_OUT_OF_RANGE: c_uint = 0x2;
pub const GEN6_PCODE_TIMEOUT: c_uint = 0x3;
pub const GEN6_PCODE_UNIMPLEMENTED_CMD: c_uint = 0xFF;
pub const GEN7_PCODE_TIMEOUT: c_uint = 0x2;
pub const GEN7_PCODE_ILLEGAL_DATA: c_uint = 0x3;
pub const GEN11_PCODE_ILLEGAL_SUBCOMMAND: c_uint = 0x4;
pub const GEN11_PCODE_LOCKED: c_uint = 0x6;
pub const GEN11_PCODE_REJECTED: c_uint = 0x11;
pub const GEN7_PCODE_MIN_FREQ_TABLE_GT_RATIO_OUT_OF_RANGE: c_uint = 0x10;
pub const GEN6_PCODE_WRITE_RC6VIDS: c_uint = 0x4;
pub const GEN6_PCODE_READ_RC6VIDS: c_uint = 0x5;

pub const BDW_PCODE_DISPLAY_FREQ_CHANGE_REQ: c_uint = 0x18;
pub const GEN9_PCODE_READ_MEM_LATENCY: c_uint = 0x6;

pub const SKL_PCODE_LOAD_HDCP_KEYS: c_uint = 0x5;
pub const SKL_PCODE_CDCLK_CONTROL: c_uint = 0x7;
pub const SKL_CDCLK_PREPARE_FOR_CHANGE: c_uint = 0x3;
pub const SKL_CDCLK_READY_FOR_CHANGE: c_uint = 0x1;
pub const GEN6_PCODE_WRITE_MIN_FREQ_TABLE: c_uint = 0x8;
pub const GEN6_PCODE_READ_MIN_FREQ_TABLE: c_uint = 0x9;
pub const GEN6_READ_OC_PARAMS: c_uint = 0xc;
pub const ICL_PCODE_MEM_SUBSYSYSTEM_INFO: c_uint = 0xd;

pub const DISPLAY_TO_PCODE_CDCLK_MAX: c_uint = 0x28D;

pub const ICL_PCODE_SAGV_DE_MEM_SS_CONFIG: c_uint = 0xe;

pub const GEN6_PCODE_READ_D_COMP: c_uint = 0x10;
pub const GEN6_PCODE_WRITE_D_COMP: c_uint = 0x11;
pub const ICL_PCODE_EXIT_TCCOLD: c_uint = 0x12;
pub const HSW_PCODE_DE_WRITE_FREQ_REQ: c_uint = 0x17;
pub const DISPLAY_IPS_CONTROL: c_uint = 0x19;
pub const TGL_PCODE_TCCOLD: c_uint = 0x26;

pub const TGL_PCODE_EXIT_TCCOLD_DATA_L_BLOCK_REQ: c_int = 0;

// See also IPS_CTL

pub const HSW_PCODE_DYNAMIC_DUTY_CYCLE_CONTROL: c_uint = 0x1A;
pub const GEN9_PCODE_SAGV_CONTROL: c_uint = 0x21;
pub const GEN9_SAGV_DISABLE: c_uint = 0x0;
pub const GEN9_SAGV_IS_DISABLED: c_uint = 0x1;
pub const GEN9_SAGV_ENABLE: c_uint = 0x3;
pub const DG1_PCODE_STATUS: c_uint = 0x7E;
pub const DG1_UNCORE_GET_INIT_STATUS: c_uint = 0x0;
pub const DG1_UNCORE_INIT_STATUS_COMPLETE: c_uint = 0x1;
pub const PCODE_POWER_SETUP: c_uint = 0x7C;
pub const POWER_SETUP_SUBCOMMAND_READ_I1: c_uint = 0x4;
pub const POWER_SETUP_SUBCOMMAND_WRITE_I1: c_uint = 0x5;

pub const POWER_SETUP_SUBCOMMAND_G8_ENABLE: c_uint = 0x6;
pub const GEN12_PCODE_READ_SAGV_BLOCK_TIME_US: c_uint = 0x23;
pub const XEHP_PCODE_FREQUENCY_CONFIG: c_uint = 0x6e	/* pvc */;
// XEHP_PCODE_FREQUENCY_CONFIG sub-commands (param1)
pub const PCODE_MBOX_FC_SC_READ_FUSED_P0: c_uint = 0x0;
pub const PCODE_MBOX_FC_SC_READ_FUSED_PN: c_uint = 0x1;
// PCODE_MBOX_DOMAIN_* - mailbox domain IDs
// XEHP_PCODE_FREQUENCY_CONFIG param2
pub const PCODE_MBOX_DOMAIN_NONE: c_uint = 0x0;
pub const PCODE_MBOX_DOMAIN_MEDIAFF: c_uint = 0x3;
