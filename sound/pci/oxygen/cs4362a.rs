//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/oxygen/cs4362a.h
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
// register 01h
pub const CS4362A_PDN: c_uint = 0x01;
pub const CS4362A_DAC1_DIS: c_uint = 0x02;
pub const CS4362A_DAC2_DIS: c_uint = 0x04;
pub const CS4362A_DAC3_DIS: c_uint = 0x08;
pub const CS4362A_MCLKDIV: c_uint = 0x20;
pub const CS4362A_FREEZE: c_uint = 0x40;
pub const CS4362A_CPEN: c_uint = 0x80;
// register 02h
pub const CS4362A_DIF_MASK: c_uint = 0x70;
pub const CS4362A_DIF_LJUST: c_uint = 0x00;
pub const CS4362A_DIF_I2S: c_uint = 0x10;
pub const CS4362A_DIF_RJUST_16: c_uint = 0x20;
pub const CS4362A_DIF_RJUST_24: c_uint = 0x30;
pub const CS4362A_DIF_RJUST_20: c_uint = 0x40;
pub const CS4362A_DIF_RJUST_18: c_uint = 0x50;
// register 03h
pub const CS4362A_MUTEC_MASK: c_uint = 0x03;
pub const CS4362A_MUTEC_6: c_uint = 0x00;
pub const CS4362A_MUTEC_1: c_uint = 0x01;
pub const CS4362A_MUTEC_3: c_uint = 0x03;
pub const CS4362A_AMUTE: c_uint = 0x04;
pub const CS4362A_MUTEC_POL: c_uint = 0x08;
pub const CS4362A_RMP_UP: c_uint = 0x10;
pub const CS4362A_SNGLVOL: c_uint = 0x20;
pub const CS4362A_ZERO_CROSS: c_uint = 0x40;
pub const CS4362A_SOFT_RAMP: c_uint = 0x80;
// register 04h
pub const CS4362A_RMP_DN: c_uint = 0x01;
pub const CS4362A_DEM_MASK: c_uint = 0x06;
pub const CS4362A_DEM_NONE: c_uint = 0x00;
pub const CS4362A_DEM_44100: c_uint = 0x02;
pub const CS4362A_DEM_48000: c_uint = 0x04;
pub const CS4362A_DEM_32000: c_uint = 0x06;
pub const CS4362A_FILT_SEL: c_uint = 0x10;
// register 05h
pub const CS4362A_INV_A1: c_uint = 0x01;
pub const CS4362A_INV_B1: c_uint = 0x02;
pub const CS4362A_INV_A2: c_uint = 0x04;
pub const CS4362A_INV_B2: c_uint = 0x08;
pub const CS4362A_INV_A3: c_uint = 0x10;
pub const CS4362A_INV_B3: c_uint = 0x20;
// register 06h
pub const CS4362A_FM_MASK: c_uint = 0x03;
pub const CS4362A_FM_SINGLE: c_uint = 0x00;
pub const CS4362A_FM_DOUBLE: c_uint = 0x01;
pub const CS4362A_FM_QUAD: c_uint = 0x02;
pub const CS4362A_FM_DSD: c_uint = 0x03;
pub const CS4362A_ATAPI_MASK: c_uint = 0x7c;
pub const CS4362A_ATAPI_B_MUTE: c_uint = 0x00;
pub const CS4362A_ATAPI_B_R: c_uint = 0x04;
pub const CS4362A_ATAPI_B_L: c_uint = 0x08;
pub const CS4362A_ATAPI_B_LR: c_uint = 0x0c;
pub const CS4362A_ATAPI_A_MUTE: c_uint = 0x00;
pub const CS4362A_ATAPI_A_R: c_uint = 0x10;
pub const CS4362A_ATAPI_A_L: c_uint = 0x20;
pub const CS4362A_ATAPI_A_LR: c_uint = 0x30;
pub const CS4362A_ATAPI_MIX_LR_VOL: c_uint = 0x40;
pub const CS4362A_A_EQ_B: c_uint = 0x80;
// register 07h
pub const CS4362A_VOL_MASK: c_uint = 0x7f;
pub const CS4362A_MUTE: c_uint = 0x80;
// register 08h: like 07h
// registers 09h..0Bh: like 06h..08h
// registers 0Ch..0Eh: like 06h..08h
// register 12h
pub const CS4362A_REV_MASK: c_uint = 0x07;
pub const CS4362A_PART_MASK: c_uint = 0xf8;
pub const CS4362A_PART_CS4362A: c_uint = 0x50;
