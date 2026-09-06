//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/tuners/mc44s803_priv.h
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
// Driver for Freescale MC44S803 Low Power CMOS Broadband Tuner
//
// Copyright (c) 2009 Jochen Friedrich <jochen@scram.de>
//
// This driver is based on the information available in the datasheet
//

pub const MC44S803_REG_POWER: c_int = 0;
pub const MC44S803_REG_REFOSC: c_int = 1;
pub const MC44S803_REG_REFDIV: c_int = 2;
pub const MC44S803_REG_MIXER: c_int = 3;
pub const MC44S803_REG_RESET: c_int = 4;
pub const MC44S803_REG_LO1: c_int = 5;
pub const MC44S803_REG_LO2: c_int = 6;
pub const MC44S803_REG_CIRCADJ: c_int = 7;
pub const MC44S803_REG_TEST: c_int = 8;
pub const MC44S803_REG_DIGTUNE: c_int = 9;
pub const MC44S803_REG_LNAAGC: c_uint = 0x0A;
pub const MC44S803_REG_DATAREG: c_uint = 0x0B;
pub const MC44S803_REG_REGTEST: c_uint = 0x0C;
pub const MC44S803_REG_VCOTEST: c_uint = 0x0D;
pub const MC44S803_REG_LNAGAIN: c_uint = 0x0E;
pub const MC44S803_REG_ID: c_uint = 0x0F;
// Register definitions
pub const MC44S803_ADDR: c_uint = 0x0F;
pub const MC44S803_ADDR_S: c_int = 0;
// REG_POWER
pub const MC44S803_POWER: c_uint = 0xFFFFF0;
pub const MC44S803_POWER_S: c_int = 4;
// REG_REFOSC
pub const MC44S803_REFOSC: c_uint = 0x1FF0;
pub const MC44S803_REFOSC_S: c_int = 4;
pub const MC44S803_OSCSEL: c_uint = 0x2000;
pub const MC44S803_OSCSEL_S: c_int = 13;
// REG_REFDIV
pub const MC44S803_R2: c_uint = 0x1FF0;
pub const MC44S803_R2_S: c_int = 4;
pub const MC44S803_REFBUF_EN: c_uint = 0x2000;
pub const MC44S803_REFBUF_EN_S: c_int = 13;
pub const MC44S803_R1: c_uint = 0x7C000;
pub const MC44S803_R1_S: c_int = 14;
// REG_MIXER
pub const MC44S803_R3: c_uint = 0x70;
pub const MC44S803_R3_S: c_int = 4;
pub const MC44S803_MUX3: c_uint = 0x80;
pub const MC44S803_MUX3_S: c_int = 7;
pub const MC44S803_MUX4: c_uint = 0x100;
pub const MC44S803_MUX4_S: c_int = 8;
pub const MC44S803_OSC_SCR: c_uint = 0x200;
pub const MC44S803_OSC_SCR_S: c_int = 9;
pub const MC44S803_TRI_STATE: c_uint = 0x400;
pub const MC44S803_TRI_STATE_S: c_int = 10;
pub const MC44S803_BUF_GAIN: c_uint = 0x800;
pub const MC44S803_BUF_GAIN_S: c_int = 11;
pub const MC44S803_BUF_IO: c_uint = 0x1000;
pub const MC44S803_BUF_IO_S: c_int = 12;
pub const MC44S803_MIXER_RES: c_uint = 0xFE000;
pub const MC44S803_MIXER_RES_S: c_int = 13;
// REG_RESET
pub const MC44S803_RS: c_uint = 0x10;
pub const MC44S803_RS_S: c_int = 4;
pub const MC44S803_SO: c_uint = 0x20;
pub const MC44S803_SO_S: c_int = 5;
// REG_LO1
pub const MC44S803_LO1: c_uint = 0xFFF0;
pub const MC44S803_LO1_S: c_int = 4;
// REG_LO2
pub const MC44S803_LO2: c_uint = 0x7FFF0;
pub const MC44S803_LO2_S: c_int = 4;
// REG_CIRCADJ
pub const MC44S803_G1: c_uint = 0x20;
pub const MC44S803_G1_S: c_int = 5;
pub const MC44S803_G3: c_uint = 0x80;
pub const MC44S803_G3_S: c_int = 7;
pub const MC44S803_CIRCADJ_RES: c_uint = 0x300;
pub const MC44S803_CIRCADJ_RES_S: c_int = 8;
pub const MC44S803_G6: c_uint = 0x400;
pub const MC44S803_G6_S: c_int = 10;
pub const MC44S803_G7: c_uint = 0x800;
pub const MC44S803_G7_S: c_int = 11;
pub const MC44S803_S1: c_uint = 0x1000;
pub const MC44S803_S1_S: c_int = 12;
pub const MC44S803_LP: c_uint = 0x7E000;
pub const MC44S803_LP_S: c_int = 13;
pub const MC44S803_CLRF: c_uint = 0x80000;
pub const MC44S803_CLRF_S: c_int = 19;
pub const MC44S803_CLIF: c_uint = 0x100000;
pub const MC44S803_CLIF_S: c_int = 20;
// REG_TEST
// REG_DIGTUNE
pub const MC44S803_DA: c_uint = 0xF0;
pub const MC44S803_DA_S: c_int = 4;
pub const MC44S803_XOD: c_uint = 0x300;
pub const MC44S803_XOD_S: c_int = 8;
pub const MC44S803_RST: c_uint = 0x10000;
pub const MC44S803_RST_S: c_int = 16;
pub const MC44S803_LO_REF: c_uint = 0x1FFF00;
pub const MC44S803_LO_REF_S: c_int = 8;
pub const MC44S803_AT: c_uint = 0x200000;
pub const MC44S803_AT_S: c_int = 21;
pub const MC44S803_MT: c_uint = 0x400000;
pub const MC44S803_MT_S: c_int = 22;
// REG_LNAAGC
pub const MC44S803_G: c_uint = 0x3F0;
pub const MC44S803_G_S: c_int = 4;
pub const MC44S803_AT1: c_uint = 0x400;
pub const MC44S803_AT1_S: c_int = 10;
pub const MC44S803_AT2: c_uint = 0x800;
pub const MC44S803_AT2_S: c_int = 11;
pub const MC44S803_HL_GR_EN: c_uint = 0x8000;
pub const MC44S803_HL_GR_EN_S: c_int = 15;
pub const MC44S803_AGC_AN_DIG: c_uint = 0x10000;
pub const MC44S803_AGC_AN_DIG_S: c_int = 16;
pub const MC44S803_ATTEN_EN: c_uint = 0x20000;
pub const MC44S803_ATTEN_EN_S: c_int = 17;
pub const MC44S803_AGC_READ_EN: c_uint = 0x40000;
pub const MC44S803_AGC_READ_EN_S: c_int = 18;
pub const MC44S803_LNA0: c_uint = 0x80000;
pub const MC44S803_LNA0_S: c_int = 19;
pub const MC44S803_AGC_SEL: c_uint = 0x100000;
pub const MC44S803_AGC_SEL_S: c_int = 20;
pub const MC44S803_AT0: c_uint = 0x200000;
pub const MC44S803_AT0_S: c_int = 21;
pub const MC44S803_B: c_uint = 0xC00000;
pub const MC44S803_B_S: c_int = 22;
// REG_DATAREG
pub const MC44S803_D: c_uint = 0xF0;
pub const MC44S803_D_S: c_int = 4;
// REG_REGTEST
// REG_VCOTEST
// REG_LNAGAIN
pub const MC44S803_IF_PWR: c_uint = 0x700;
pub const MC44S803_IF_PWR_S: c_int = 8;
pub const MC44S803_RF_PWR: c_uint = 0x3800;
pub const MC44S803_RF_PWR_S: c_int = 11;
pub const MC44S803_LNA_GAIN: c_uint = 0xFC000;
pub const MC44S803_LNA_GAIN_S: c_int = 14;
// REG_ID
pub const MC44S803_ID: c_uint = 0x3E00;
pub const MC44S803_ID_S: c_int = 9;
// Some macros to read/write fields
// First shift, then mask

// First mask, then shift

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mc44s803_priv {
    pub cfg: *mut mc44s803_config,
    pub i2c: *mut i2c_adapter,
    pub fe: *mut dvb_frontend,
    pub frequency: u32,
}
