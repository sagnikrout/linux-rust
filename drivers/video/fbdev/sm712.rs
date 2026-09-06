//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/sm712.h
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


//
// Silicon Motion SM712 frame buffer device
//
// Copyright (C) 2006 Silicon Motion Technology Corp.
// Authors:	Ge Wang, gewang@siliconmotion.com
// Boyod boyod.yang@siliconmotion.com.cn
//
// Copyright (C) 2009 Lemote, Inc.
// Author: Wu Zhangjin, wuzhangjin@gmail.com
//
// This file is subject to the terms and conditions of the GNU General Public
// License. See the file COPYING in the main directory of this archive for
// more details.
//
pub const FB_ACCEL_SMI_LYNX: c_int = 88;
pub const SCREEN_X_RES: c_int = 1024;
pub const SCREEN_Y_RES_PC: c_int = 768;
pub const SCREEN_Y_RES_NETBOOK: c_int = 600;
pub const SCREEN_BPP: c_int = 16;

extern "C" {
    pub fn smtc_mmiorb(_arg: 0x3c5) -> return;
}
// The next structure holds all information relevant for a specific video mode.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct modeinit {
    pub mmsizex: c_int,
    pub mmsizey: c_int,
    pub bpp: c_int,
    pub hz: c_int,
    pub init_misc: c_uchar,
    pub init_sr00_sr04: [c_uchar; SIZE_SR00_SR04],
    pub init_sr10_sr24: [c_uchar; SIZE_SR10_SR24],
    pub init_sr30_sr75: [c_uchar; SIZE_SR30_SR75],
    pub init_sr80_sr93: [c_uchar; SIZE_SR80_SR93],
    pub init_sra0_sraf: [c_uchar; SIZE_SRA0_SRAF],
    pub init_gr00_gr08: [c_uchar; SIZE_GR00_GR08],
    pub init_ar00_ar14: [c_uchar; SIZE_AR00_AR14],
    pub init_cr00_cr18: [c_uchar; SIZE_CR00_CR18],
    pub init_cr30_cr4d: [c_uchar; SIZE_CR30_CR4D],
    pub init_cr90_cra7: [c_uchar; SIZE_CR90_CRA7],
}

pub const big_addr: c_uint = 0x800000;
pub const mmio_addr: c_uint = 0x00800000;

pub const big_addr: c_int = 0;
pub const mmio_addr: c_uint = 0x00c00000;

