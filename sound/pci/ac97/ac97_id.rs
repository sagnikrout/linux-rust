//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/ac97/ac97_id.h
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
// Copyright (c) by Jaroslav Kysela <perex@perex.cz>
// Universal interface for Audio Codec '97
//
// For more details look to AC '97 component specification revision 2.2
// by Intel Corporation (http://developer.intel.com).
//
pub const AC97_ID_AK4540: c_uint = 0x414b4d00;
pub const AC97_ID_AK4542: c_uint = 0x414b4d01;
pub const AC97_ID_AD1819: c_uint = 0x41445303;
pub const AC97_ID_AD1881: c_uint = 0x41445340;
pub const AC97_ID_AD1881A: c_uint = 0x41445348;
pub const AC97_ID_AD1885: c_uint = 0x41445360;
pub const AC97_ID_AD1886: c_uint = 0x41445361;
pub const AC97_ID_AD1887: c_uint = 0x41445362;
pub const AC97_ID_AD1886A: c_uint = 0x41445363;
pub const AC97_ID_AD1980: c_uint = 0x41445370;
pub const AC97_ID_TR28028: c_uint = 0x54524108;
pub const AC97_ID_STAC9700: c_uint = 0x83847600;
pub const AC97_ID_STAC9704: c_uint = 0x83847604;
pub const AC97_ID_STAC9705: c_uint = 0x83847605;
pub const AC97_ID_STAC9708: c_uint = 0x83847608;
pub const AC97_ID_STAC9721: c_uint = 0x83847609;
pub const AC97_ID_STAC9744: c_uint = 0x83847644;
pub const AC97_ID_STAC9756: c_uint = 0x83847656;
pub const AC97_ID_CS4297A: c_uint = 0x43525910;
pub const AC97_ID_CS4299: c_uint = 0x43525930;
pub const AC97_ID_CS4201: c_uint = 0x43525948;
pub const AC97_ID_CS4205: c_uint = 0x43525958;
pub const AC97_ID_CS_MASK: c_uint = 0xfffffff8	/* bit 0-2: rev */;
pub const AC97_ID_ALC100: c_uint = 0x414c4300;
pub const AC97_ID_ALC650: c_uint = 0x414c4720;
pub const AC97_ID_ALC650D: c_uint = 0x414c4721;
pub const AC97_ID_ALC650E: c_uint = 0x414c4722;
pub const AC97_ID_ALC650F: c_uint = 0x414c4723;
pub const AC97_ID_ALC655: c_uint = 0x414c4760;
pub const AC97_ID_ALC658: c_uint = 0x414c4780;
pub const AC97_ID_ALC658D: c_uint = 0x414c4781;
pub const AC97_ID_ALC850: c_uint = 0x414c4790;
pub const AC97_ID_YMF743: c_uint = 0x594d4800;
pub const AC97_ID_YMF753: c_uint = 0x594d4803;
pub const AC97_ID_VT1616: c_uint = 0x49434551;
pub const AC97_ID_CM9738: c_uint = 0x434d4941;
pub const AC97_ID_CM9739: c_uint = 0x434d4961;
pub const AC97_ID_CM9761_78: c_uint = 0x434d4978;
pub const AC97_ID_CM9761_82: c_uint = 0x434d4982;
pub const AC97_ID_CM9761_83: c_uint = 0x434d4983;
pub const AC97_ID_ST7597: c_uint = 0x53544d02;
pub const AC97_ID_ST_AC97_ID4: c_uint = 0x53544d04;
