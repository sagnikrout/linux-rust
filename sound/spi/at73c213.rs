//! Automatically rewritten from C Header to Rust Module
//! Source: sound/spi/at73c213.h
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
// Driver for the AT73C213 16-bit stereo DAC on Atmel ATSTK1000
//
// Copyright (C) 2006 - 2007 Atmel Corporation
//
// DAC control register
pub const DAC_CTRL: c_uint = 0x00;
pub const DAC_CTRL_ONPADRV: c_int = 7;
pub const DAC_CTRL_ONAUXIN: c_int = 6;
pub const DAC_CTRL_ONDACR: c_int = 5;
pub const DAC_CTRL_ONDACL: c_int = 4;
pub const DAC_CTRL_ONLNOR: c_int = 3;
pub const DAC_CTRL_ONLNOL: c_int = 2;
pub const DAC_CTRL_ONLNIR: c_int = 1;
pub const DAC_CTRL_ONLNIL: c_int = 0;
// DAC left line in gain register
pub const DAC_LLIG: c_uint = 0x01;
pub const DAC_LLIG_LLIG: c_int = 0;
// DAC right line in gain register
pub const DAC_RLIG: c_uint = 0x02;
pub const DAC_RLIG_RLIG: c_int = 0;
// DAC Left Master Playback Gain Register
pub const DAC_LMPG: c_uint = 0x03;
pub const DAC_LMPG_LMPG: c_int = 0;
// DAC Right Master Playback Gain Register
pub const DAC_RMPG: c_uint = 0x04;
pub const DAC_RMPG_RMPG: c_int = 0;
// DAC Left Line Out Gain Register
pub const DAC_LLOG: c_uint = 0x05;
pub const DAC_LLOG_LLOG: c_int = 0;
// DAC Right Line Out Gain Register
pub const DAC_RLOG: c_uint = 0x06;
pub const DAC_RLOG_RLOG: c_int = 0;
// DAC Output Level Control Register
pub const DAC_OLC: c_uint = 0x07;
pub const DAC_OLC_RSHORT: c_int = 7;
pub const DAC_OLC_ROLC: c_int = 4;
pub const DAC_OLC_LSHORT: c_int = 3;
pub const DAC_OLC_LOLC: c_int = 0;
// DAC Mixer Control Register
pub const DAC_MC: c_uint = 0x08;
pub const DAC_MC_INVR: c_int = 5;
pub const DAC_MC_INVL: c_int = 4;
pub const DAC_MC_RMSMIN2: c_int = 3;
pub const DAC_MC_RMSMIN1: c_int = 2;
pub const DAC_MC_LMSMIN2: c_int = 1;
pub const DAC_MC_LMSMIN1: c_int = 0;
// DAC Clock and Sampling Frequency Control Register
pub const DAC_CSFC: c_uint = 0x09;
pub const DAC_CSFC_OVRSEL: c_int = 4;
// DAC Miscellaneous Register
pub const DAC_MISC: c_uint = 0x0A;
pub const DAC_MISC_VCMCAPSEL: c_int = 7;
pub const DAC_MISC_DINTSEL: c_int = 4;
pub const DAC_MISC_DITHEN: c_int = 3;
pub const DAC_MISC_DEEMPEN: c_int = 2;
pub const DAC_MISC_NBITS: c_int = 0;
// DAC Precharge Control Register
pub const DAC_PRECH: c_uint = 0x0C;
pub const DAC_PRECH_PRCHGPDRV: c_int = 7;
pub const DAC_PRECH_PRCHGAUX1: c_int = 6;
pub const DAC_PRECH_PRCHGLNOR: c_int = 5;
pub const DAC_PRECH_PRCHGLNOL: c_int = 4;
pub const DAC_PRECH_PRCHGLNIR: c_int = 3;
pub const DAC_PRECH_PRCHGLNIL: c_int = 2;
pub const DAC_PRECH_PRCHG: c_int = 1;
pub const DAC_PRECH_ONMSTR: c_int = 0;
// DAC Auxiliary Input Gain Control Register
pub const DAC_AUXG: c_uint = 0x0D;
pub const DAC_AUXG_AUXG: c_int = 0;
// DAC Reset Register
pub const DAC_RST: c_uint = 0x10;
pub const DAC_RST_RESMASK: c_int = 2;
pub const DAC_RST_RESFILZ: c_int = 1;
pub const DAC_RST_RSTZ: c_int = 0;
// Power Amplifier Control Register
pub const PA_CTRL: c_uint = 0x11;
pub const PA_CTRL_APAON: c_int = 6;
pub const PA_CTRL_APAPRECH: c_int = 5;
pub const PA_CTRL_APALP: c_int = 4;
pub const PA_CTRL_APAGAIN: c_int = 0;
