//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/oxygen/xonar_dg.h
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

// Macro flag: #define XONAR_DG_H_INCLUDED

pub const GPIO_MAGIC: c_uint = 0x0008;
pub const GPIO_HP_DETECT: c_uint = 0x0010;
pub const GPIO_INPUT_ROUTE: c_uint = 0x0060;
pub const GPIO_HP_REAR: c_uint = 0x0080;
pub const GPIO_OUTPUT_ENABLE: c_uint = 0x0100;
pub const CAPTURE_SRC_MIC: c_int = 0;
pub const CAPTURE_SRC_FP_MIC: c_int = 1;
pub const CAPTURE_SRC_LINE: c_int = 2;
pub const CAPTURE_SRC_AUX: c_int = 3;
pub const PLAYBACK_DST_HP: c_int = 0;
pub const PLAYBACK_DST_HP_FP: c_int = 1;
pub const PLAYBACK_DST_MULTICH: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cs4245_shadow_operation {
    CS4245_SAVE_TO_SHADOW,
    CS4245_LOAD_FROM_SHADOW
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dg {
// shadow copy of the CS4245 register space
    pub cs4245_shadow: [c_uchar; 17],
// output select: headphone/speakers
    pub output_sel: c_uchar,
// volumes for all capture sources
    pub input_vol: [c_char; 4][2],
// input select: mic/fp mic/line/aux
    pub input_sel: c_uchar,
}

// Xonar DG control routines
extern "C" {
    pub fn cs4245_write_spi(chip: *mut oxygen, reg: u8) -> c_int;
}
extern "C" {
    pub fn cs4245_read_spi(chip: *mut oxygen, reg: u8) -> c_int;
}
extern "C" {
    pub fn cs4245_shadow_control(chip: *mut oxygen, op: cs4245_shadow_operation) -> c_int;
}
extern "C" {
    pub fn dg_init(chip: *mut oxygen);
}
extern "C" {
    pub fn dg_suspend(chip: *mut oxygen);
}
extern "C" {
    pub fn dg_resume(chip: *mut oxygen);
}
extern "C" {
    pub fn dg_cleanup(chip: *mut oxygen);
}
