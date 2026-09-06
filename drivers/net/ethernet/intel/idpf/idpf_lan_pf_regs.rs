//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/idpf/idpf_lan_pf_regs.h
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
// Copyright (C) 2023 Intel Corporation
// Receive queues
pub const PF_QRX_BASE: c_uint = 0x00000000;

pub const PF_QRX_BUFFQ_BASE: c_uint = 0x03000000;

// Transmit queues
pub const PF_QTX_BASE: c_uint = 0x05000000;

// Control(PF Mailbox) Queue
pub const PF_FW_BASE: c_uint = 0x08400000;

pub const PF_FW_ARQLEN_ARQLEN_S: c_int = 0;

pub const PF_FW_ARQLEN_ARQVFE_S: c_int = 28;

pub const PF_FW_ARQLEN_ARQOVFL_S: c_int = 29;

pub const PF_FW_ARQLEN_ARQCRIT_S: c_int = 30;

pub const PF_FW_ARQLEN_ARQENABLE_S: c_int = 31;

pub const PF_FW_ARQH_ARQH_S: c_int = 0;

pub const PF_FW_ATQLEN_ATQLEN_S: c_int = 0;

pub const PF_FW_ATQLEN_ATQVFE_S: c_int = 28;

pub const PF_FW_ATQLEN_ATQOVFL_S: c_int = 29;

pub const PF_FW_ATQLEN_ATQCRIT_S: c_int = 30;

pub const PF_FW_ATQLEN_ATQENABLE_S: c_int = 31;

pub const PF_FW_ATQH_ATQH_S: c_int = 0;

// Timesync registers

// Interrupts
pub const PF_GLINT_BASE: c_uint = 0x08900000;

pub const PF_GLINT_DYN_CTL_INTENA_S: c_int = 0;

pub const PF_GLINT_DYN_CTL_CLEARPBA_S: c_int = 1;

pub const PF_GLINT_DYN_CTL_SWINT_TRIG_S: c_int = 2;

pub const PF_GLINT_DYN_CTL_ITR_INDX_S: c_int = 3;

pub const PF_GLINT_DYN_CTL_INTERVAL_S: c_int = 5;

pub const PF_GLINT_DYN_CTL_SW_ITR_INDX_ENA_S: c_int = 24;

pub const PF_GLINT_DYN_CTL_SW_ITR_INDX_S: c_int = 25;

pub const PF_GLINT_DYN_CTL_WB_ON_ITR_S: c_int = 30;

pub const PF_GLINT_DYN_CTL_INTENA_MSK_S: c_int = 31;

// _ITR is ITR index, _INT is interrupt index, _itrn_indx_spacing is
// spacing b/w itrn registers of the same vector.
//

// For PF, itrn_indx_spacing is 4 and itrn_reg_spacing is 0x1000

pub const PF_GLINT_ITR_MAX_INDEX: c_int = 2;
pub const PF_GLINT_ITR_INTERVAL_S: c_int = 0;

// Generic registers
pub const PF_INT_DIR_OICR_ENA: c_uint = 0x08406000;
pub const PF_INT_DIR_OICR_ENA_S: c_int = 0;

pub const PF_INT_DIR_OICR: c_uint = 0x08406004;
pub const PF_INT_DIR_OICR_TSYN_EVNT: c_int = 0;

pub const PF_INT_DIR_OICR_CAUSE: c_uint = 0x08406008;
pub const PF_INT_DIR_OICR_CAUSE_CAUSE_S: c_int = 0;

pub const PF_INT_PBA_CLEAR: c_uint = 0x0840600C;
pub const PF_FUNC_RID: c_uint = 0x08406010;
pub const PF_FUNC_RID_FUNCTION_NUMBER_S: c_int = 0;

pub const PF_FUNC_RID_DEVICE_NUMBER_S: c_int = 3;

pub const PF_FUNC_RID_BUS_NUMBER_S: c_int = 8;

// Reset registers
pub const PFGEN_RTRIG: c_uint = 0x08407000;
pub const PFGEN_RTRIG_CORER_S: c_int = 0;

pub const PFGEN_RTRIG_LINKR_S: c_int = 1;

pub const PFGEN_RTRIG_IMCR_S: c_int = 2;

pub const PFGEN_RSTAT: c_uint = 0x08407008 /* PFR Status */;
pub const PFGEN_RSTAT_PFR_STATE_S: c_int = 0;

pub const PFGEN_CTRL: c_uint = 0x0840700C;

