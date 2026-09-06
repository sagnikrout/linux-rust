//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/sta350.h
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
// Codec driver for ST STA350 2.1-channel high-efficiency digital audio system
//
// Copyright: 2011 Raumfeld GmbH
// Author: Sven Brandau <info@brandau.biz>
//
// based on code from:
// Raumfeld GmbH
// Johannes Stezenbach <js@sig21.net>
// Wolfson Microelectronics PLC.
// Mark Brown <broonie@opensource.wolfsonmicro.com>
//
// STA350 register addresses
pub const STA350_REGISTER_COUNT: c_uint = 0x4D;
pub const STA350_COEF_COUNT: c_int = 62;
pub const STA350_CONFA: c_uint = 0x00;
pub const STA350_CONFB: c_uint = 0x01;
pub const STA350_CONFC: c_uint = 0x02;
pub const STA350_CONFD: c_uint = 0x03;
pub const STA350_CONFE: c_uint = 0x04;
pub const STA350_CONFF: c_uint = 0x05;
pub const STA350_MMUTE: c_uint = 0x06;
pub const STA350_MVOL: c_uint = 0x07;
pub const STA350_C1VOL: c_uint = 0x08;
pub const STA350_C2VOL: c_uint = 0x09;
pub const STA350_C3VOL: c_uint = 0x0a;
pub const STA350_AUTO1: c_uint = 0x0b;
pub const STA350_AUTO2: c_uint = 0x0c;
pub const STA350_AUTO3: c_uint = 0x0d;
pub const STA350_C1CFG: c_uint = 0x0e;
pub const STA350_C2CFG: c_uint = 0x0f;
pub const STA350_C3CFG: c_uint = 0x10;
pub const STA350_TONE: c_uint = 0x11;
pub const STA350_L1AR: c_uint = 0x12;
pub const STA350_L1ATRT: c_uint = 0x13;
pub const STA350_L2AR: c_uint = 0x14;
pub const STA350_L2ATRT: c_uint = 0x15;
pub const STA350_CFADDR2: c_uint = 0x16;
pub const STA350_B1CF1: c_uint = 0x17;
pub const STA350_B1CF2: c_uint = 0x18;
pub const STA350_B1CF3: c_uint = 0x19;
pub const STA350_B2CF1: c_uint = 0x1a;
pub const STA350_B2CF2: c_uint = 0x1b;
pub const STA350_B2CF3: c_uint = 0x1c;
pub const STA350_A1CF1: c_uint = 0x1d;
pub const STA350_A1CF2: c_uint = 0x1e;
pub const STA350_A1CF3: c_uint = 0x1f;
pub const STA350_A2CF1: c_uint = 0x20;
pub const STA350_A2CF2: c_uint = 0x21;
pub const STA350_A2CF3: c_uint = 0x22;
pub const STA350_B0CF1: c_uint = 0x23;
pub const STA350_B0CF2: c_uint = 0x24;
pub const STA350_B0CF3: c_uint = 0x25;
pub const STA350_CFUD: c_uint = 0x26;
pub const STA350_MPCC1: c_uint = 0x27;
pub const STA350_MPCC2: c_uint = 0x28;
pub const STA350_DCC1: c_uint = 0x29;
pub const STA350_DCC2: c_uint = 0x2a;
pub const STA350_FDRC1: c_uint = 0x2b;
pub const STA350_FDRC2: c_uint = 0x2c;
pub const STA350_STATUS: c_uint = 0x2d;
// reserved: 0x2d - 0x30
pub const STA350_EQCFG: c_uint = 0x31;
pub const STA350_EATH1: c_uint = 0x32;
pub const STA350_ERTH1: c_uint = 0x33;
pub const STA350_EATH2: c_uint = 0x34;
pub const STA350_ERTH2: c_uint = 0x35;
pub const STA350_CONFX: c_uint = 0x36;
pub const STA350_SVCA: c_uint = 0x37;
pub const STA350_SVCB: c_uint = 0x38;
pub const STA350_RMS0A: c_uint = 0x39;
pub const STA350_RMS0B: c_uint = 0x3a;
pub const STA350_RMS0C: c_uint = 0x3b;
pub const STA350_RMS1A: c_uint = 0x3c;
pub const STA350_RMS1B: c_uint = 0x3d;
pub const STA350_RMS1C: c_uint = 0x3e;
pub const STA350_EVOLRES: c_uint = 0x3f;
// reserved: 0x40 - 0x47
pub const STA350_NSHAPE: c_uint = 0x48;
pub const STA350_CTXB4B1: c_uint = 0x49;
pub const STA350_CTXB7B5: c_uint = 0x4a;
pub const STA350_MISC1: c_uint = 0x4b;
pub const STA350_MISC2: c_uint = 0x4c;
// 0x00 CONFA
pub const STA350_CONFA_MCS_MASK: c_uint = 0x03;
pub const STA350_CONFA_MCS_SHIFT: c_int = 0;
pub const STA350_CONFA_IR_MASK: c_uint = 0x18;
pub const STA350_CONFA_IR_SHIFT: c_int = 3;

// 0x01 CONFB
pub const STA350_CONFB_SAI_MASK: c_uint = 0x0f;
pub const STA350_CONFB_SAI_SHIFT: c_int = 0;

// 0x02 CONFC
pub const STA350_CONFC_OM_MASK: c_uint = 0x03;
pub const STA350_CONFC_OM_SHIFT: c_int = 0;
pub const STA350_CONFC_CSZ_MASK: c_uint = 0x3c;
pub const STA350_CONFC_CSZ_SHIFT: c_int = 2;

// 0x03 CONFD
pub const STA350_CONFD_HPB_SHIFT: c_int = 0;
pub const STA350_CONFD_DEMP_SHIFT: c_int = 1;
pub const STA350_CONFD_DSPB_SHIFT: c_int = 2;
pub const STA350_CONFD_PSL_SHIFT: c_int = 3;
pub const STA350_CONFD_BQL_SHIFT: c_int = 4;
pub const STA350_CONFD_DRC_SHIFT: c_int = 5;
pub const STA350_CONFD_ZDE_SHIFT: c_int = 6;
pub const STA350_CONFD_SME_SHIFT: c_int = 7;
// 0x04 CONFE

pub const STA350_CONFE_MPCV_SHIFT: c_int = 0;

pub const STA350_CONFE_MPC_SHIFT: c_int = 1;

pub const STA350_CONFE_NSBW_SHIFT: c_int = 2;

pub const STA350_CONFE_AME_SHIFT: c_int = 3;

pub const STA350_CONFE_PWMS_SHIFT: c_int = 4;

pub const STA350_CONFE_DCCV_SHIFT: c_int = 5;

pub const STA350_CONFE_ZCE_SHIFT: c_int = 6;

pub const STA350_CONFE_SVE_SHIFT: c_int = 7;
// 0x05 CONFF
pub const STA350_CONFF_OCFG_MASK: c_uint = 0x03;
pub const STA350_CONFF_OCFG_SHIFT: c_int = 0;

// 0x06 MMUTE
pub const STA350_MMUTE_MMUTE: c_uint = 0x01;
pub const STA350_MMUTE_MMUTE_SHIFT: c_int = 0;
pub const STA350_MMUTE_C1M: c_uint = 0x02;
pub const STA350_MMUTE_C1M_SHIFT: c_int = 1;
pub const STA350_MMUTE_C2M: c_uint = 0x04;
pub const STA350_MMUTE_C2M_SHIFT: c_int = 2;
pub const STA350_MMUTE_C3M: c_uint = 0x08;
pub const STA350_MMUTE_C3M_SHIFT: c_int = 3;
pub const STA350_MMUTE_LOC_MASK: c_uint = 0xC0;
pub const STA350_MMUTE_LOC_SHIFT: c_int = 6;
// 0x0b AUTO1
pub const STA350_AUTO1_AMGC_MASK: c_uint = 0x30;
pub const STA350_AUTO1_AMGC_SHIFT: c_int = 4;
// 0x0c AUTO2
pub const STA350_AUTO2_AMAME: c_uint = 0x01;
pub const STA350_AUTO2_AMAM_MASK: c_uint = 0x0e;
pub const STA350_AUTO2_AMAM_SHIFT: c_int = 1;
pub const STA350_AUTO2_XO_MASK: c_uint = 0xf0;
pub const STA350_AUTO2_XO_SHIFT: c_int = 4;
// 0x0d AUTO3
pub const STA350_AUTO3_PEQ_MASK: c_uint = 0x1f;
pub const STA350_AUTO3_PEQ_SHIFT: c_int = 0;
// 0x0e 0x0f 0x10 CxCFG
pub const STA350_CxCFG_TCB_SHIFT: c_int = 0;
pub const STA350_CxCFG_EQBP_SHIFT: c_int = 1;
pub const STA350_CxCFG_VBP_SHIFT: c_int = 2;
pub const STA350_CxCFG_BO_SHIFT: c_int = 3;
pub const STA350_CxCFG_LS_SHIFT: c_int = 4;
pub const STA350_CxCFG_OM_MASK: c_uint = 0xc0;
pub const STA350_CxCFG_OM_SHIFT: c_int = 6;
// 0x11 TONE
pub const STA350_TONE_BTC_SHIFT: c_int = 0;
pub const STA350_TONE_TTC_SHIFT: c_int = 4;
// 0x12 0x13 0x14 0x15 limiter attack/release
pub const STA350_LxA_SHIFT: c_int = 0;
pub const STA350_LxR_SHIFT: c_int = 4;
// 0x26 CFUD
pub const STA350_CFUD_W1: c_uint = 0x01;
pub const STA350_CFUD_WA: c_uint = 0x02;
pub const STA350_CFUD_R1: c_uint = 0x04;
pub const STA350_CFUD_RA: c_uint = 0x08;
// biquad filter coefficient table offsets
pub const STA350_C1_BQ_BASE: c_int = 0;
pub const STA350_C2_BQ_BASE: c_int = 20;
pub const STA350_CH_BQ_NUM: c_int = 4;
pub const STA350_BQ_NUM_COEF: c_int = 5;
pub const STA350_XO_HP_BQ_BASE: c_int = 40;
pub const STA350_XO_LP_BQ_BASE: c_int = 45;
pub const STA350_C1_PRESCALE: c_int = 50;
pub const STA350_C2_PRESCALE: c_int = 51;
pub const STA350_C1_POSTSCALE: c_int = 52;
pub const STA350_C2_POSTSCALE: c_int = 53;
pub const STA350_C3_POSTSCALE: c_int = 54;
pub const STA350_TW_POSTSCALE: c_int = 55;
pub const STA350_C1_MIX1: c_int = 56;
pub const STA350_C1_MIX2: c_int = 57;
pub const STA350_C2_MIX1: c_int = 58;
pub const STA350_C2_MIX2: c_int = 59;
pub const STA350_C3_MIX1: c_int = 60;
pub const STA350_C3_MIX2: c_int = 61;
// miscellaneous register 1

// miscellaneous register 2
pub const STA350_MISC2_PNDLSL_MASK: c_uint = 0x1c;
pub const STA350_MISC2_PNDLSL_SHIFT: c_int = 2;
