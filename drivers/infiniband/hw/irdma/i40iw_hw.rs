//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/irdma/i40iw_hw.h
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


// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
// Copyright (c) 2015 - 2021 Intel Corporation
pub const I40E_VFPE_CQPTAIL1: c_uint = 0x0000A000 /* Reset: VFR */;
pub const I40E_VFPE_CQPDB1: c_uint = 0x0000BC00 /* Reset: VFR */;
pub const I40E_VFPE_CCQPSTATUS1: c_uint = 0x0000B800 /* Reset: VFR */;
pub const I40E_VFPE_CCQPHIGH1: c_uint = 0x00009800 /* Reset: VFR */;
pub const I40E_VFPE_CCQPLOW1: c_uint = 0x0000AC00 /* Reset: VFR */;
pub const I40E_VFPE_CQARM1: c_uint = 0x0000B400 /* Reset: VFR */;
pub const I40E_VFPE_CQACK1: c_uint = 0x0000B000 /* Reset: VFR */;
pub const I40E_VFPE_AEQALLOC1: c_uint = 0x0000A400 /* Reset: VFR */;
pub const I40E_VFPE_CQPERRCODES1: c_uint = 0x00009C00 /* Reset: VFR */;
pub const I40E_VFPE_WQEALLOC1: c_uint = 0x0000C000 /* Reset: VFR */;

pub const I40E_PFPE_CQPTAIL: c_uint = 0x00008080 /* Reset: PFR */;
pub const I40E_PFPE_CQPDB: c_uint = 0x00008000 /* Reset: PFR */;
pub const I40E_PFPE_CCQPSTATUS: c_uint = 0x00008100 /* Reset: PFR */;
pub const I40E_PFPE_CCQPHIGH: c_uint = 0x00008200 /* Reset: PFR */;
pub const I40E_PFPE_CCQPLOW: c_uint = 0x00008180 /* Reset: PFR */;
pub const I40E_PFPE_CQARM: c_uint = 0x00131080 /* Reset: PFR */;
pub const I40E_PFPE_CQACK: c_uint = 0x00131100 /* Reset: PFR */;
pub const I40E_PFPE_AEQALLOC: c_uint = 0x00131180 /* Reset: PFR */;
pub const I40E_PFPE_CQPERRCODES: c_uint = 0x00008880 /* Reset: PFR */;
pub const I40E_PFPE_WQEALLOC: c_uint = 0x00138C00 /* Reset: PFR */;
pub const I40E_GLPCI_LBARCTRL: c_uint = 0x000BE484 /* Reset: POR */;
pub const I40E_GLPE_CPUSTATUS0: c_uint = 0x0000D040 /* Reset: PE_CORER */;
pub const I40E_GLPE_CPUSTATUS1: c_uint = 0x0000D044 /* Reset: PE_CORER */;
pub const I40E_GLPE_CPUSTATUS2: c_uint = 0x0000D048 /* Reset: PE_CORER */;
pub const I40E_GLPE_CRITERR: c_uint = 0x000B4000 /* Reset: PE_CORER */;
pub const I40E_PFHMC_PDINV: c_uint = 0x000C0300 /* Reset: PFR */;

pub const I40E_PFINT_AEQCTL: c_uint = 0x00038700 /* Reset: CORER */;

pub const I40E_PFINT_LNKLSTN_MAX_INDEX: c_int = 511;

pub const I40E_PFINT_CEQCTL_MAX_INDEX: c_int = 511;
// shifts/masks for FLD_[LS/RS]_64 macros used in device table
pub const I40E_PFINT_CEQCTL_MSIX_INDX_S: c_int = 0;

pub const I40E_PFINT_CEQCTL_ITR_INDX_S: c_int = 11;

pub const I40E_PFINT_CEQCTL_MSIX0_INDX_S: c_int = 13;

pub const I40E_PFINT_CEQCTL_NEXTQ_INDX_S: c_int = 16;

pub const I40E_PFINT_CEQCTL_NEXTQ_TYPE_S: c_int = 27;

pub const I40E_PFINT_CEQCTL_CAUSE_ENA_S: c_int = 30;

pub const I40E_PFINT_CEQCTL_INTEVENT_S: c_int = 31;

pub const I40E_CQPSQ_STAG_PDID_S: c_int = 48;

pub const I40E_PFPE_CCQPSTATUS_CCQP_DONE_S: c_int = 0;

pub const I40E_PFPE_CCQPSTATUS_CCQP_ERR_S: c_int = 31;

pub const I40E_PFINT_DYN_CTLN_ITR_INDX_S: c_int = 3;

pub const I40E_PFINT_DYN_CTLN_INTENA_S: c_int = 0;

pub const I40E_CQPSQ_CQ_CEQID_S: c_int = 24;

pub const I40E_CQPSQ_CQ_CQID_S: c_int = 0;

pub const I40E_COMMIT_FPM_CQCNT_S: c_int = 0;

pub const I40E_CQPSQ_UPESD_HMCFNID_S: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i40iw_device_caps_const {
    I40IW_MAX_WQ_FRAGMENT_COUNT		= 3,
    I40IW_MAX_SGE_RD			= 1,
    I40IW_MAX_PUSH_PAGE_COUNT		= 0,
    I40IW_MAX_INLINE_DATA_SIZE		= 48,
    I40IW_MAX_IRD_SIZE			= 63,
    I40IW_MAX_ORD_SIZE			= 127,
    I40IW_MAX_WQ_ENTRIES			= 2048,
    I40IW_MAX_WQE_SIZE_RQ			= 128,
    I40IW_MAX_PDS				= 32768,
    I40IW_MAX_STATS_COUNT			= 16,
    I40IW_MAX_CQ_SIZE			= 1048575,
    I40IW_MAX_OUTBOUND_MSG_SIZE		= 2147483647,
    I40IW_MAX_INBOUND_MSG_SIZE		= 2147483647,
    I40IW_MIN_WQ_SIZE                       = 4 /* WQEs */,
}

pub const I40IW_QP_WQE_MIN_SIZE: c_int = 32;
pub const I40IW_QP_WQE_MAX_SIZE: c_int = 128;
pub const I40IW_MAX_RQ_WQE_SHIFT: c_int = 2;
pub const I40IW_MAX_QUANTA_PER_WR: c_int = 2;
pub const I40IW_QP_SW_MAX_SQ_QUANTA: c_int = 2048;
pub const I40IW_QP_SW_MAX_RQ_QUANTA: c_int = 16384;
pub const I40IW_QP_SW_MAX_WQ_QUANTA: c_int = 2048;

pub const I40IW_FIRST_VF_FPM_ID: c_int = 16;
pub const QUEUE_TYPE_CEQ: c_int = 2;
pub const NULL_QUEUE_INDEX: c_uint = 0x7FF;
extern "C" {
    pub fn i40iw_init_hw(dev: *mut irdma_sc_dev);
}
