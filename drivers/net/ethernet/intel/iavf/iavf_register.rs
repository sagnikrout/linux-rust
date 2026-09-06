//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/iavf/iavf_register.h
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
// Copyright(c) 2013 - 2018 Intel Corporation.
pub const IAVF_VF_ARQBAH1: c_uint = 0x00006000 /* Reset: EMPR */;
pub const IAVF_VF_ARQBAL1: c_uint = 0x00006C00 /* Reset: EMPR */;
pub const IAVF_VF_ARQH1: c_uint = 0x00007400 /* Reset: EMPR */;
pub const IAVF_VF_ARQH1_ARQH_SHIFT: c_int = 0;

pub const IAVF_VF_ARQLEN1: c_uint = 0x00008000 /* Reset: EMPR */;
pub const IAVF_VF_ARQLEN1_ARQVFE_SHIFT: c_int = 28;

pub const IAVF_VF_ARQLEN1_ARQOVFL_SHIFT: c_int = 29;

pub const IAVF_VF_ARQLEN1_ARQCRIT_SHIFT: c_int = 30;

pub const IAVF_VF_ARQLEN1_ARQENABLE_SHIFT: c_int = 31;

pub const IAVF_VF_ARQT1: c_uint = 0x00007000 /* Reset: EMPR */;
pub const IAVF_VF_ATQBAH1: c_uint = 0x00007800 /* Reset: EMPR */;
pub const IAVF_VF_ATQBAL1: c_uint = 0x00007C00 /* Reset: EMPR */;
pub const IAVF_VF_ATQH1: c_uint = 0x00006400 /* Reset: EMPR */;
pub const IAVF_VF_ATQLEN1: c_uint = 0x00006800 /* Reset: EMPR */;
pub const IAVF_VF_ATQLEN1_ATQVFE_SHIFT: c_int = 28;

pub const IAVF_VF_ATQLEN1_ATQOVFL_SHIFT: c_int = 29;

pub const IAVF_VF_ATQLEN1_ATQCRIT_SHIFT: c_int = 30;

pub const IAVF_VF_ATQLEN1_ATQENABLE_SHIFT: c_int = 31;

pub const IAVF_VF_ATQT1: c_uint = 0x00008400 /* Reset: EMPR */;
pub const IAVF_VFGEN_RSTAT: c_uint = 0x00008800 /* Reset: VFR */;
pub const IAVF_VFGEN_RSTAT_VFR_STATE_SHIFT: c_int = 0;

pub const IAVF_VFINT_DYN_CTL01: c_uint = 0x00005C00 /* Reset: VFR */;
pub const IAVF_VFINT_DYN_CTL01_INTENA_SHIFT: c_int = 0;

pub const IAVF_VFINT_DYN_CTL01_ITR_INDX_SHIFT: c_int = 3;

pub const IAVF_VFINT_DYN_CTLN1_INTENA_SHIFT: c_int = 0;

pub const IAVF_VFINT_DYN_CTLN1_SWINT_TRIG_SHIFT: c_int = 2;

pub const IAVF_VFINT_DYN_CTLN1_ITR_INDX_SHIFT: c_int = 3;

pub const IAVF_VFINT_DYN_CTLN1_INTERVAL_SHIFT: c_int = 5;
pub const IAVF_VFINT_DYN_CTLN1_SW_ITR_INDX_ENA_SHIFT: c_int = 24;

pub const IAVF_VFINT_ICR0_ENA1: c_uint = 0x00005000 /* Reset: CORER */;
pub const IAVF_VFINT_ICR0_ENA1_ADMINQ_SHIFT: c_int = 30;

pub const IAVF_VFINT_ICR0_ENA1_RSVD_SHIFT: c_int = 31;
pub const IAVF_VFINT_ICR01: c_uint = 0x00004800 /* Reset: CORER */;

pub const IAVF_VFQF_HKEY_MAX_INDEX: c_int = 12;

pub const IAVF_VFQF_HLUT_MAX_INDEX: c_int = 15;
pub const IAVF_VFINT_DYN_CTLN1_WB_ON_ITR_SHIFT: c_int = 30;

