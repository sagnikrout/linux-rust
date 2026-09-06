//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/nvidia/nv_proto.h
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
// $XFree86: xc/programs/Xserver/hw/xfree86/drivers/nv/nv_proto.h,v 1.10 2003/07/31 20:24:29 mvojkovi Exp $
// in nv_setup.c
extern "C" {
    pub fn NVCommonSetup(info: *mut fb_info) -> c_int;
}
extern "C" {
    pub fn NVWriteCrtc(par: *mut nvidia_par, index: u8, value: u8);
}
extern "C" {
    pub fn NVReadCrtc(par: *mut nvidia_par, index: u8) -> u8;
}
extern "C" {
    pub fn NVWriteGr(par: *mut nvidia_par, index: u8, value: u8);
}
extern "C" {
    pub fn NVReadGr(par: *mut nvidia_par, index: u8) -> u8;
}
extern "C" {
    pub fn NVWriteSeq(par: *mut nvidia_par, index: u8, value: u8);
}
extern "C" {
    pub fn NVReadSeq(par: *mut nvidia_par, index: u8) -> u8;
}
extern "C" {
    pub fn NVWriteAttr(par: *mut nvidia_par, index: u8, value: u8);
}
extern "C" {
    pub fn NVReadAttr(par: *mut nvidia_par, index: u8) -> u8;
}
extern "C" {
    pub fn NVWriteMiscOut(par: *mut nvidia_par, value: u8);
}
extern "C" {
    pub fn NVReadMiscOut(par: *mut nvidia_par) -> u8;
}
extern "C" {
    pub fn NVWriteDacMask(par: *mut nvidia_par, value: u8);
}
extern "C" {
    pub fn NVWriteDacReadAddr(par: *mut nvidia_par, value: u8);
}
extern "C" {
    pub fn NVWriteDacWriteAddr(par: *mut nvidia_par, value: u8);
}
extern "C" {
    pub fn NVWriteDacData(par: *mut nvidia_par, value: u8);
}
extern "C" {
    pub fn NVReadDacData(par: *mut nvidia_par) -> u8;
}
// in nv_hw.c
extern "C" {
    pub fn NVLoadStateExt(par: *mut nvidia_par, : *mut _riva_hw_state);
}
extern "C" {
    pub fn NVUnloadStateExt(par: *mut nvidia_par, : *mut _riva_hw_state);
}
extern "C" {
    pub fn NVSetStartAddress(par: *mut nvidia_par, _arg: u32);
}
extern "C" {
    pub fn NVShowHideCursor(par: *mut nvidia_par, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn NVLockUnlock(par: *mut nvidia_par, _arg: c_int);
}
// in nvidia-i2c.c

extern "C" {
    pub fn nvidia_create_i2c_busses(par: *mut nvidia_par);
}
extern "C" {
    pub fn nvidia_delete_i2c_busses(par: *mut nvidia_par);
}

// Macro flag: #define nvidia_create_i2c_busses(...)
// Macro flag: #define nvidia_delete_i2c_busses(...)

// in nv_accel.c
extern "C" {
    pub fn NVResetGraphics(info: *mut fb_info);
}
extern "C" {
    pub fn nvidiafb_sync(info: *mut fb_info) -> c_int;
}
// in nv_backlight.h

extern "C" {
    pub fn nvidia_bl_init(par: *mut nvidia_par);
}
extern "C" {
    pub fn nvidia_bl_exit(par: *mut nvidia_par);
}

