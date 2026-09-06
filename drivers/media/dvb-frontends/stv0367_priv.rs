//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/stv0367_priv.h
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
// stv0367_priv.h
//
// Driver for ST STV0367 DVB-T & DVB-C demodulator IC.
//
// Copyright (C) ST Microelectronics.
// Copyright (C) 2010,2011 NetUP Inc.
// Copyright (C) 2010,2011 Igor M. Liplianin <liplianin@netup.ru>
//
// Common driver error constants

pub const NULL: c_int = 0;

// MACRO definitions

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stv0367_ter_signal_type {
    FE_TER_NOAGC = 0,
    FE_TER_AGCOK = 5,
    FE_TER_NOTPS = 6,
    FE_TER_TPSOK = 7,
    FE_TER_NOSYMBOL = 8,
    FE_TER_BAD_CPQ = 9,
    FE_TER_PRFOUNDOK = 10,
    FE_TER_NOPRFOUND = 11,
    FE_TER_LOCKOK = 12,
    FE_TER_NOLOCK = 13,
    FE_TER_SYMBOLOK = 15,
    FE_TER_CPAMPOK = 16,
    FE_TER_NOCPAMP = 17,
    FE_TER_SWNOK = 18
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stv0367_ts_mode {
    STV0367_OUTPUTMODE_DEFAULT,
    STV0367_SERIAL_PUNCT_CLOCK,
    STV0367_SERIAL_CONT_CLOCK,
    STV0367_PARALLEL_PUNCT_CLOCK,
    STV0367_DVBCI_CLOCK
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stv0367_clk_pol {
    STV0367_CLOCKPOLARITY_DEFAULT,
    STV0367_RISINGEDGE_CLOCK,
    STV0367_FALLINGEDGE_CLOCK
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stv0367_ter_bw {
    FE_TER_CHAN_BW_6M = 6,
    FE_TER_CHAN_BW_7M = 7,
    FE_TER_CHAN_BW_8M = 8
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum FE_TER_Rate_TPS {
    FE_TER_TPS_1_2 = 0,
    FE_TER_TPS_2_3 = 1,
    FE_TER_TPS_3_4 = 2,
    FE_TER_TPS_5_6 = 3,
    FE_TER_TPS_7_8 = 4
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stv0367_ter_mode {
    FE_TER_MODE_2K,
    FE_TER_MODE_8K,
    FE_TER_MODE_4K
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum FE_TER_Hierarchy_Alpha {
    FE_TER_HIER_ALPHA_NONE,	/* Regular modulation */
    FE_TER_HIER_ALPHA_1,	/* Hierarchical modulation a = 1*/
    FE_TER_HIER_ALPHA_2,	/* Hierarchical modulation a = 2*/
    FE_TER_HIER_ALPHA_4	/* Hierarchical modulation a = 4*/
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stv0367_ter_hierarchy {
    FE_TER_HIER_NONE,	/*Hierarchy None*/
    FE_TER_HIER_LOW_PRIO,	/*Hierarchy : Low Priority*/
    FE_TER_HIER_HIGH_PRIO,	/*Hierarchy : High Priority*/
    FE_TER_HIER_PRIO_ANY	/*Hierarchy  :Any*/
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fe_stv0367_ter_spec {
    FE_TER_INVERSION_NONE = 0,
    FE_TER_INVERSION = 1,
    FE_TER_INVERSION_AUTO = 2,
    FE_TER_INVERSION_UNK  = 4
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stv0367_ter_if_iq_mode {
    FE_TER_NORMAL_IF_TUNER = 0,
    FE_TER_LONGPATH_IF_TUNER = 1,
    FE_TER_IQ_TUNER = 2

}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum FE_TER_FECRate {
    FE_TER_FEC_NONE = 0x00,	/* no FEC rate specified */
    FE_TER_FEC_ALL = 0xFF,	 /* Logical OR of all FECs */
    FE_TER_FEC_1_2 = 1,
    FE_TER_FEC_2_3 = (1 << 1),
    FE_TER_FEC_3_4 = (1 << 2),
    FE_TER_FEC_4_5 = (1 << 3),
    FE_TER_FEC_5_6 = (1 << 4),
    FE_TER_FEC_6_7 = (1 << 5),
    FE_TER_FEC_7_8 = (1 << 6),
    FE_TER_FEC_8_9 = (1 << 7)
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum FE_TER_Rate {
    FE_TER_FE_1_2 = 0,
    FE_TER_FE_2_3 = 1,
    FE_TER_FE_3_4 = 2,
    FE_TER_FE_5_6 = 3,
    FE_TER_FE_6_7 = 4,
    FE_TER_FE_7_8 = 5
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stv0367_ter_force {
    FE_TER_FORCENONE = 0,
    FE_TER_FORCE_M_G = 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stv0367cab_mod {
    FE_CAB_MOD_QAM4,
    FE_CAB_MOD_QAM16,
    FE_CAB_MOD_QAM32,
    FE_CAB_MOD_QAM64,
    FE_CAB_MOD_QAM128,
    FE_CAB_MOD_QAM256,
    FE_CAB_MOD_QAM512,
    FE_CAB_MOD_QAM1024
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stv0367_cab_signal_info {
    pub locked: c_int,
    pub /: *mut *mut u32 frequency; / kHz,
    pub /: *mut *mut u32 symbol_rate; / Mbds,
    pub modulation: stv0367cab_mod,
    pub spect_inv: fe_spectral_inversion,
    pub /: *mut *mut s32 Power_dBmx10; / Power of the RF signal (dBm x 10),
    pub /: *mut *mut u32 CN_dBx10; / Carrier to noise ratio (dB x 10),
    pub /: *mut *mut u32 BER; / Bit error rate (x 10000000),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stv0367_cab_signal_type {
    FE_CAB_NOTUNER,
    FE_CAB_NOAGC,
    FE_CAB_NOSIGNAL,
    FE_CAB_NOTIMING,
    FE_CAB_TIMINGOK,
    FE_CAB_NOCARRIER,
    FE_CAB_CARRIEROK,
    FE_CAB_NOBLIND,
    FE_CAB_BLINDOK,
    FE_CAB_NODEMOD,
    FE_CAB_DEMODOK,
    FE_CAB_DATAOK
}
