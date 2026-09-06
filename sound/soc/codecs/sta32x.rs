//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/sta32x.h
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
// Codec driver for ST STA32x 2.1-channel high-efficiency digital audio system
//
// Copyright: 2011 Raumfeld GmbH
// Author: Johannes Stezenbach <js@sig21.net>
//
// based on code from:
// Wolfson Microelectronics PLC.
// Mark Brown <broonie@opensource.wolfsonmicro.com>
//
// STA326 register addresses
pub const STA32X_REGISTER_COUNT: c_uint = 0x2d;
pub const STA32X_COEF_COUNT: c_int = 62;
pub const STA32X_CONFA: c_uint = 0x00;
pub const STA32X_CONFB: c_uint = 0x01;
pub const STA32X_CONFC: c_uint = 0x02;
pub const STA32X_CONFD: c_uint = 0x03;
pub const STA32X_CONFE: c_uint = 0x04;
pub const STA32X_CONFF: c_uint = 0x05;
pub const STA32X_MMUTE: c_uint = 0x06;
pub const STA32X_MVOL: c_uint = 0x07;
pub const STA32X_C1VOL: c_uint = 0x08;
pub const STA32X_C2VOL: c_uint = 0x09;
pub const STA32X_C3VOL: c_uint = 0x0a;
pub const STA32X_AUTO1: c_uint = 0x0b;
pub const STA32X_AUTO2: c_uint = 0x0c;
pub const STA32X_AUTO3: c_uint = 0x0d;
pub const STA32X_C1CFG: c_uint = 0x0e;
pub const STA32X_C2CFG: c_uint = 0x0f;
pub const STA32X_C3CFG: c_uint = 0x10;
pub const STA32X_TONE: c_uint = 0x11;
pub const STA32X_L1AR: c_uint = 0x12;
pub const STA32X_L1ATRT: c_uint = 0x13;
pub const STA32X_L2AR: c_uint = 0x14;
pub const STA32X_L2ATRT: c_uint = 0x15;
pub const STA32X_CFADDR2: c_uint = 0x16;
pub const STA32X_B1CF1: c_uint = 0x17;
pub const STA32X_B1CF2: c_uint = 0x18;
pub const STA32X_B1CF3: c_uint = 0x19;
pub const STA32X_B2CF1: c_uint = 0x1a;
pub const STA32X_B2CF2: c_uint = 0x1b;
pub const STA32X_B2CF3: c_uint = 0x1c;
pub const STA32X_A1CF1: c_uint = 0x1d;
pub const STA32X_A1CF2: c_uint = 0x1e;
pub const STA32X_A1CF3: c_uint = 0x1f;
pub const STA32X_A2CF1: c_uint = 0x20;
pub const STA32X_A2CF2: c_uint = 0x21;
pub const STA32X_A2CF3: c_uint = 0x22;
pub const STA32X_B0CF1: c_uint = 0x23;
pub const STA32X_B0CF2: c_uint = 0x24;
pub const STA32X_B0CF3: c_uint = 0x25;
pub const STA32X_CFUD: c_uint = 0x26;
pub const STA32X_MPCC1: c_uint = 0x27;
pub const STA32X_MPCC2: c_uint = 0x28;
// Reserved 0x29
// Reserved 0x2a
pub const STA32X_Reserved: c_uint = 0x2a;
pub const STA32X_FDRC1: c_uint = 0x2b;
pub const STA32X_FDRC2: c_uint = 0x2c;
// Reserved 0x2d
// STA326 register field definitions
// 0x00 CONFA
pub const STA32X_CONFA_MCS_MASK: c_uint = 0x03;
pub const STA32X_CONFA_MCS_SHIFT: c_int = 0;
pub const STA32X_CONFA_IR_MASK: c_uint = 0x18;
pub const STA32X_CONFA_IR_SHIFT: c_int = 3;
pub const STA32X_CONFA_TWRB: c_uint = 0x20;
pub const STA32X_CONFA_TWAB: c_uint = 0x40;
pub const STA32X_CONFA_FDRB: c_uint = 0x80;
// 0x01 CONFB
pub const STA32X_CONFB_SAI_MASK: c_uint = 0x0f;
pub const STA32X_CONFB_SAI_SHIFT: c_int = 0;
pub const STA32X_CONFB_SAIFB: c_uint = 0x10;
pub const STA32X_CONFB_DSCKE: c_uint = 0x20;
pub const STA32X_CONFB_C1IM: c_uint = 0x40;
pub const STA32X_CONFB_C2IM: c_uint = 0x80;
// 0x02 CONFC
pub const STA32X_CONFC_OM_MASK: c_uint = 0x03;
pub const STA32X_CONFC_OM_SHIFT: c_int = 0;
pub const STA32X_CONFC_CSZ_MASK: c_uint = 0x7c;
pub const STA32X_CONFC_CSZ_SHIFT: c_int = 2;
// 0x03 CONFD
pub const STA32X_CONFD_HPB: c_uint = 0x01;
pub const STA32X_CONFD_HPB_SHIFT: c_int = 0;
pub const STA32X_CONFD_DEMP: c_uint = 0x02;
pub const STA32X_CONFD_DEMP_SHIFT: c_int = 1;
pub const STA32X_CONFD_DSPB: c_uint = 0x04;
pub const STA32X_CONFD_DSPB_SHIFT: c_int = 2;
pub const STA32X_CONFD_PSL: c_uint = 0x08;
pub const STA32X_CONFD_PSL_SHIFT: c_int = 3;
pub const STA32X_CONFD_BQL: c_uint = 0x10;
pub const STA32X_CONFD_BQL_SHIFT: c_int = 4;
pub const STA32X_CONFD_DRC: c_uint = 0x20;
pub const STA32X_CONFD_DRC_SHIFT: c_int = 5;
pub const STA32X_CONFD_ZDE: c_uint = 0x40;
pub const STA32X_CONFD_ZDE_SHIFT: c_int = 6;
pub const STA32X_CONFD_MME: c_uint = 0x80;
pub const STA32X_CONFD_MME_SHIFT: c_int = 7;
// 0x04 CONFE
pub const STA32X_CONFE_MPCV: c_uint = 0x01;
pub const STA32X_CONFE_MPCV_SHIFT: c_int = 0;
pub const STA32X_CONFE_MPC: c_uint = 0x02;
pub const STA32X_CONFE_MPC_SHIFT: c_int = 1;
pub const STA32X_CONFE_AME: c_uint = 0x08;
pub const STA32X_CONFE_AME_SHIFT: c_int = 3;
pub const STA32X_CONFE_PWMS: c_uint = 0x10;
pub const STA32X_CONFE_PWMS_SHIFT: c_int = 4;
pub const STA32X_CONFE_ZCE: c_uint = 0x40;
pub const STA32X_CONFE_ZCE_SHIFT: c_int = 6;
pub const STA32X_CONFE_SVE: c_uint = 0x80;
pub const STA32X_CONFE_SVE_SHIFT: c_int = 7;
// 0x05 CONFF
pub const STA32X_CONFF_OCFG_MASK: c_uint = 0x03;
pub const STA32X_CONFF_OCFG_SHIFT: c_int = 0;
pub const STA32X_CONFF_IDE: c_uint = 0x04;
pub const STA32X_CONFF_IDE_SHIFT: c_int = 2;
pub const STA32X_CONFF_BCLE: c_uint = 0x08;
pub const STA32X_CONFF_ECLE: c_uint = 0x20;
pub const STA32X_CONFF_PWDN: c_uint = 0x40;
pub const STA32X_CONFF_EAPD: c_uint = 0x80;
// 0x06 MMUTE
pub const STA32X_MMUTE_MMUTE: c_uint = 0x01;
// 0x0b AUTO1
pub const STA32X_AUTO1_AMEQ_MASK: c_uint = 0x03;
pub const STA32X_AUTO1_AMEQ_SHIFT: c_int = 0;
pub const STA32X_AUTO1_AMV_MASK: c_uint = 0xc0;
pub const STA32X_AUTO1_AMV_SHIFT: c_int = 2;
pub const STA32X_AUTO1_AMGC_MASK: c_uint = 0x30;
pub const STA32X_AUTO1_AMGC_SHIFT: c_int = 4;
pub const STA32X_AUTO1_AMPS: c_uint = 0x80;
// 0x0c AUTO2
pub const STA32X_AUTO2_AMAME: c_uint = 0x01;
pub const STA32X_AUTO2_AMAM_MASK: c_uint = 0x0e;
pub const STA32X_AUTO2_AMAM_SHIFT: c_int = 1;
pub const STA32X_AUTO2_XO_MASK: c_uint = 0xf0;
pub const STA32X_AUTO2_XO_SHIFT: c_int = 4;
// 0x0d AUTO3
pub const STA32X_AUTO3_PEQ_MASK: c_uint = 0x1f;
pub const STA32X_AUTO3_PEQ_SHIFT: c_int = 0;
// 0x0e 0x0f 0x10 CxCFG
pub const STA32X_CxCFG_TCB: c_uint = 0x01	/* only C1 and C2 */;
pub const STA32X_CxCFG_TCB_SHIFT: c_int = 0;
pub const STA32X_CxCFG_EQBP: c_uint = 0x02	/* only C1 and C2 */;
pub const STA32X_CxCFG_EQBP_SHIFT: c_int = 1;
pub const STA32X_CxCFG_VBP: c_uint = 0x03;
pub const STA32X_CxCFG_VBP_SHIFT: c_int = 2;
pub const STA32X_CxCFG_BO: c_uint = 0x04;
pub const STA32X_CxCFG_LS_MASK: c_uint = 0x30;
pub const STA32X_CxCFG_LS_SHIFT: c_int = 4;
pub const STA32X_CxCFG_OM_MASK: c_uint = 0xc0;
pub const STA32X_CxCFG_OM_SHIFT: c_int = 6;
// 0x11 TONE
pub const STA32X_TONE_BTC_SHIFT: c_int = 0;
pub const STA32X_TONE_TTC_SHIFT: c_int = 4;
// 0x12 0x13 0x14 0x15 limiter attack/release
pub const STA32X_LxA_SHIFT: c_int = 0;
pub const STA32X_LxR_SHIFT: c_int = 4;
// 0x26 CFUD
pub const STA32X_CFUD_W1: c_uint = 0x01;
pub const STA32X_CFUD_WA: c_uint = 0x02;
pub const STA32X_CFUD_R1: c_uint = 0x04;
pub const STA32X_CFUD_RA: c_uint = 0x08;
// biquad filter coefficient table offsets
pub const STA32X_C1_BQ_BASE: c_int = 0;
pub const STA32X_C2_BQ_BASE: c_int = 20;
pub const STA32X_CH_BQ_NUM: c_int = 4;
pub const STA32X_BQ_NUM_COEF: c_int = 5;
pub const STA32X_XO_HP_BQ_BASE: c_int = 40;
pub const STA32X_XO_LP_BQ_BASE: c_int = 45;
pub const STA32X_C1_PRESCALE: c_int = 50;
pub const STA32X_C2_PRESCALE: c_int = 51;
pub const STA32X_C1_POSTSCALE: c_int = 52;
pub const STA32X_C2_POSTSCALE: c_int = 53;
pub const STA32X_C3_POSTSCALE: c_int = 54;
pub const STA32X_TW_POSTSCALE: c_int = 55;
pub const STA32X_C1_MIX1: c_int = 56;
pub const STA32X_C1_MIX2: c_int = 57;
pub const STA32X_C2_MIX1: c_int = 58;
pub const STA32X_C2_MIX2: c_int = 59;
pub const STA32X_C3_MIX1: c_int = 60;
pub const STA32X_C3_MIX2: c_int = 61;
