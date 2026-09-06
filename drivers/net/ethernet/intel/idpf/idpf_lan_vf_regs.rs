//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/idpf/idpf_lan_vf_regs.h
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
// Reset
pub const VFGEN_RSTAT: c_uint = 0x00008800;
pub const VFGEN_RSTAT_VFR_STATE_S: c_int = 0;

// Control(VF Mailbox) Queue
pub const VF_BASE: c_uint = 0x00006000;

pub const VF_ATQLEN_ATQLEN_S: c_int = 0;

pub const VF_ATQLEN_ATQVFE_S: c_int = 28;

pub const VF_ATQLEN_ATQOVFL_S: c_int = 29;

pub const VF_ATQLEN_ATQCRIT_S: c_int = 30;

pub const VF_ATQLEN_ATQENABLE_S: c_int = 31;

pub const VF_ATQH_ATQH_S: c_int = 0;

pub const VF_ARQLEN_ARQLEN_S: c_int = 0;

pub const VF_ARQLEN_ARQVFE_S: c_int = 28;

pub const VF_ARQLEN_ARQOVFL_S: c_int = 29;

pub const VF_ARQLEN_ARQCRIT_S: c_int = 30;

pub const VF_ARQLEN_ARQENABLE_S: c_int = 31;

pub const VF_ARQH_ARQH_S: c_int = 0;

// Transmit queues
pub const VF_QTX_TAIL_BASE: c_uint = 0x00000000;

pub const VF_QTX_TAIL_EXT_BASE: c_uint = 0x00040000;

// Receive queues
pub const VF_QRX_TAIL_BASE: c_uint = 0x00002000;

pub const VF_QRX_TAIL_EXT_BASE: c_uint = 0x00050000;

pub const VF_QRXB_TAIL_BASE: c_uint = 0x00060000;

// Interrupts
pub const VF_INT_DYN_CTL0: c_uint = 0x00005C00;
pub const VF_INT_DYN_CTL0_INTENA_S: c_int = 0;

pub const VF_INT_DYN_CTL0_ITR_INDX_S: c_int = 3;

pub const VF_INT_DYN_CTLN_INTENA_S: c_int = 0;

pub const VF_INT_DYN_CTLN_CLEARPBA_S: c_int = 1;

pub const VF_INT_DYN_CTLN_SWINT_TRIG_S: c_int = 2;

pub const VF_INT_DYN_CTLN_ITR_INDX_S: c_int = 3;

pub const VF_INT_DYN_CTLN_INTERVAL_S: c_int = 5;

pub const VF_INT_DYN_CTLN_SW_ITR_INDX_ENA_S: c_int = 24;

pub const VF_INT_DYN_CTLN_SW_ITR_INDX_S: c_int = 25;

pub const VF_INT_DYN_CTLN_WB_ON_ITR_S: c_int = 30;

pub const VF_INT_DYN_CTLN_INTENA_MSK_S: c_int = 31;

// _ITR is ITR index, _INT is interrupt index, _itrn_indx_spacing is spacing
// b/w itrn registers of the same vector
//

// For VF with 16 vector support, itrn_reg_spacing is 0x4, itrn_indx_spacing
// is 0x40 and base register offset is 0x00002800
//

// For VF with 64 vector support, itrn_reg_spacing is 0x4, itrn_indx_spacing
// is 0x100 and base register offset is 0x00002C00
//

// For VF with 2k vector support, itrn_reg_spacing is 0x4, itrn_indx_spacing
// is 0x2000 and base register offset is 0x00072000
//

pub const VF_INT_ITRN_MAX_INDEX: c_int = 2;
pub const VF_INT_ITRN_INTERVAL_S: c_int = 0;

pub const VF_INT_PBA_CLEAR: c_uint = 0x00008900;
pub const VF_INT_ICR0_ENA1: c_uint = 0x00005000;
pub const VF_INT_ICR0_ENA1_ADMINQ_S: c_int = 30;

pub const VF_INT_ICR0_ENA1_RSVD_S: c_int = 31;
pub const VF_INT_ICR01: c_uint = 0x00004800;

pub const VF_QF_HENA_MAX_INDX: c_int = 1;

pub const VF_QF_HKEY_MAX_INDX: c_int = 12;

pub const VF_QF_HLUT_MAX_INDX: c_int = 15;
