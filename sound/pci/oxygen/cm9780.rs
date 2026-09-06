//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/oxygen/cm9780.h
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

// Macro flag: #define CM9780_H_INCLUDED
pub const CM9780_JACK: c_uint = 0x62;
pub const CM9780_MIXER: c_uint = 0x64;
pub const CM9780_GPIO_SETUP: c_uint = 0x70;
pub const CM9780_GPIO_STATUS: c_uint = 0x72;
// jack control
pub const CM9780_RSOE: c_uint = 0x0001;
pub const CM9780_CBOE: c_uint = 0x0002;
pub const CM9780_SSOE: c_uint = 0x0004;
pub const CM9780_FROE: c_uint = 0x0008;
pub const CM9780_HP2FMICOE: c_uint = 0x0010;
pub const CM9780_CB2MICOE: c_uint = 0x0020;
pub const CM9780_FMIC2LI: c_uint = 0x0040;
pub const CM9780_FMIC2MIC: c_uint = 0x0080;
pub const CM9780_HP2LI: c_uint = 0x0100;
pub const CM9780_HP2MIC: c_uint = 0x0200;
pub const CM9780_MIC2LI: c_uint = 0x0400;
pub const CM9780_MIC2MIC: c_uint = 0x0800;
pub const CM9780_LI2LI: c_uint = 0x1000;
pub const CM9780_LI2MIC: c_uint = 0x2000;
pub const CM9780_LO2LI: c_uint = 0x4000;
pub const CM9780_LO2MIC: c_uint = 0x8000;
// mixer control
pub const CM9780_BSTSEL: c_uint = 0x0001;
pub const CM9780_STRO_MIC: c_uint = 0x0002;
pub const CM9780_SPDI_FREX: c_uint = 0x0004;
pub const CM9780_SPDI_SSEX: c_uint = 0x0008;
pub const CM9780_SPDI_CBEX: c_uint = 0x0010;
pub const CM9780_SPDI_RSEX: c_uint = 0x0020;
pub const CM9780_MIX2FR: c_uint = 0x0040;
pub const CM9780_MIX2SS: c_uint = 0x0080;
pub const CM9780_MIX2CB: c_uint = 0x0100;
pub const CM9780_MIX2RS: c_uint = 0x0200;
pub const CM9780_MIX2FR_EX: c_uint = 0x0400;
pub const CM9780_MIX2SS_EX: c_uint = 0x0800;
pub const CM9780_MIX2CB_EX: c_uint = 0x1000;
pub const CM9780_MIX2RS_EX: c_uint = 0x2000;
pub const CM9780_P47_IO: c_uint = 0x4000;
pub const CM9780_PCBSW: c_uint = 0x8000;
// GPIO setup
pub const CM9780_GPI0EN: c_uint = 0x0001;
pub const CM9780_GPI1EN: c_uint = 0x0002;
pub const CM9780_SENSE_P: c_uint = 0x0004;
pub const CM9780_LOCK_P: c_uint = 0x0008;
pub const CM9780_GPIO0P: c_uint = 0x0010;
pub const CM9780_GPIO1P: c_uint = 0x0020;
pub const CM9780_GPIO0IO: c_uint = 0x0100;
pub const CM9780_GPIO1IO: c_uint = 0x0200;
// GPIO status
pub const CM9780_GPO0: c_uint = 0x0001;
pub const CM9780_GPO1: c_uint = 0x0002;
pub const CM9780_GPIO0S: c_uint = 0x0010;
pub const CM9780_GPIO1S: c_uint = 0x0020;
pub const CM9780_GPII0S: c_uint = 0x0100;
pub const CM9780_GPII1S: c_uint = 0x0200;
