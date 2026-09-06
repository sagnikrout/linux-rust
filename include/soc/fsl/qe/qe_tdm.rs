//! Automatically rewritten from C Header to Rust Module
//! Source: include/soc/fsl/qe/qe_tdm.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Internal header file for QE TDM mode routines.
//
// Copyright (C) 2016 Freescale Semiconductor, Inc. All rights reserved.
//
// Authors:	Zhao Qiang <qiang.zhao@nxp.com>
//

// SI RAM entries
pub const SIR_LAST: c_uint = 0x0001;
pub const SIR_BYTE: c_uint = 0x0002;

pub const SIR_SGS: c_uint = 0x0200;
pub const SIR_SWTR: c_uint = 0x4000;
pub const SIR_MCC: c_uint = 0x8000;
pub const SIR_IDLE: c_int = 0;
// SIxMR fields

pub const SIMR_SDM_NORMAL: c_uint = 0x0000;
pub const SIMR_SDM_INTERNAL_LOOPBACK: c_uint = 0x0800;
pub const SIMR_SDM_MASK: c_uint = 0x0c00;
pub const SIMR_CRT: c_uint = 0x0040;
pub const SIMR_SL: c_uint = 0x0020;
pub const SIMR_CE: c_uint = 0x0010;
pub const SIMR_FE: c_uint = 0x0008;
pub const SIMR_GM: c_uint = 0x0004;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tdm_ts_t {
    TDM_TX_TS,
    TDM_RX_TS
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tdm_framer_t {
    TDM_FRAMER_T1,
    TDM_FRAMER_E1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tdm_mode_t {
    TDM_INTERNAL_LOOPBACK,
    TDM_NORMAL
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct si_mode_info {
    pub simr_rfsd: u8,
    pub simr_tfsd: u8,
    pub simr_crt: u8,
    pub simr_sl: u8,
    pub simr_ce: u8,
    pub simr_fe: u8,
    pub simr_gm: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucc_tdm_info {
    pub uf_info: ucc_fast_info,
    pub si_info: si_mode_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucc_tdm {
    pub /: *mut *mut u16 tdm_port; / port for this tdm:TDMA,TDMB,
    pub siram_entry_id: u32,
    pub siram: *mut u16 __iomem,
    pub si_regs: *mut si1 __iomem,
    pub tdm_framer_type: tdm_framer_t,
    pub tdm_mode: tdm_mode_t,
    pub /: *mut *mut u8 num_of_ts; / the number of timeslots in this tdm frame,
    pub /: *mut *mut u32 tx_ts_mask; / tx time slot mask,
    pub /: *mut *mut u32 rx_ts_mask; / rx time slot mask,
}

extern "C" {
    pub fn ucc_tdm_init(utdm: *mut ucc_tdm, ut_info: *mut ucc_tdm_info);
}
