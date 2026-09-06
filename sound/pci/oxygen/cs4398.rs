//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/oxygen/cs4398.h
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
// register 1
pub const CS4398_REV_MASK: c_uint = 0x07;
pub const CS4398_PART_MASK: c_uint = 0xf8;
pub const CS4398_PART_CS4398: c_uint = 0x70;
// register 2
pub const CS4398_FM_MASK: c_uint = 0x03;
pub const CS4398_FM_SINGLE: c_uint = 0x00;
pub const CS4398_FM_DOUBLE: c_uint = 0x01;
pub const CS4398_FM_QUAD: c_uint = 0x02;
pub const CS4398_FM_DSD: c_uint = 0x03;
pub const CS4398_DEM_MASK: c_uint = 0x0c;
pub const CS4398_DEM_NONE: c_uint = 0x00;
pub const CS4398_DEM_44100: c_uint = 0x04;
pub const CS4398_DEM_48000: c_uint = 0x08;
pub const CS4398_DEM_32000: c_uint = 0x0c;
pub const CS4398_DIF_MASK: c_uint = 0x70;
pub const CS4398_DIF_LJUST: c_uint = 0x00;
pub const CS4398_DIF_I2S: c_uint = 0x10;
pub const CS4398_DIF_RJUST_16: c_uint = 0x20;
pub const CS4398_DIF_RJUST_24: c_uint = 0x30;
pub const CS4398_DIF_RJUST_20: c_uint = 0x40;
pub const CS4398_DIF_RJUST_18: c_uint = 0x50;
pub const CS4398_DSD_SRC: c_uint = 0x80;
// register 3
pub const CS4398_ATAPI_MASK: c_uint = 0x1f;
pub const CS4398_ATAPI_B_MUTE: c_uint = 0x00;
pub const CS4398_ATAPI_B_R: c_uint = 0x01;
pub const CS4398_ATAPI_B_L: c_uint = 0x02;
pub const CS4398_ATAPI_B_LR: c_uint = 0x03;
pub const CS4398_ATAPI_A_MUTE: c_uint = 0x00;
pub const CS4398_ATAPI_A_R: c_uint = 0x04;
pub const CS4398_ATAPI_A_L: c_uint = 0x08;
pub const CS4398_ATAPI_A_LR: c_uint = 0x0c;
pub const CS4398_ATAPI_MIX_LR_VOL: c_uint = 0x10;
pub const CS4398_INVERT_B: c_uint = 0x20;
pub const CS4398_INVERT_A: c_uint = 0x40;
pub const CS4398_VOL_B_EQ_A: c_uint = 0x80;
// register 4
pub const CS4398_MUTEP_MASK: c_uint = 0x03;
pub const CS4398_MUTEP_AUTO: c_uint = 0x00;
pub const CS4398_MUTEP_LOW: c_uint = 0x02;
pub const CS4398_MUTEP_HIGH: c_uint = 0x03;
pub const CS4398_MUTE_B: c_uint = 0x08;
pub const CS4398_MUTE_A: c_uint = 0x10;
pub const CS4398_MUTEC_A_EQ_B: c_uint = 0x20;
pub const CS4398_DAMUTE: c_uint = 0x40;
pub const CS4398_PAMUTE: c_uint = 0x80;
// register 5
pub const CS4398_VOL_A_MASK: c_uint = 0xff;
// register 6
pub const CS4398_VOL_B_MASK: c_uint = 0xff;
// register 7
pub const CS4398_DIR_DSD: c_uint = 0x01;
pub const CS4398_FILT_SEL: c_uint = 0x04;
pub const CS4398_RMP_DN: c_uint = 0x10;
pub const CS4398_RMP_UP: c_uint = 0x20;
pub const CS4398_ZERO_CROSS: c_uint = 0x40;
pub const CS4398_SOFT_RAMP: c_uint = 0x80;
// register 8
pub const CS4398_MCLKDIV3: c_uint = 0x08;
pub const CS4398_MCLKDIV2: c_uint = 0x10;
pub const CS4398_FREEZE: c_uint = 0x20;
pub const CS4398_CPEN: c_uint = 0x40;
pub const CS4398_PDN: c_uint = 0x80;
// register 9
pub const CS4398_DSD_PM_EN: c_uint = 0x01;
pub const CS4398_DSD_PM_MODE: c_uint = 0x02;
pub const CS4398_INVALID_DSD: c_uint = 0x04;
pub const CS4398_STATIC_DSD: c_uint = 0x08;
