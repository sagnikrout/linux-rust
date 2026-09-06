//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/amd_asic_type.h
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
// Copyright 2017 Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//

//
// Supported ASIC types
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amd_asic_type {
    CHIP_TAHITI = 0,
    CHIP_PITCAIRN,	/* 1 */
    CHIP_VERDE,	/* 2 */
    CHIP_OLAND,	/* 3 */
    CHIP_HAINAN,	/* 4 */
    CHIP_BONAIRE,	/* 5 */
    CHIP_KAVERI,	/* 6 */
    CHIP_KABINI,	/* 7 */
    CHIP_HAWAII,	/* 8 */
    CHIP_MULLINS,	/* 9 */
    CHIP_TOPAZ,	/* 10 */
    CHIP_TONGA,	/* 11 */
    CHIP_FIJI,	/* 12 */
    CHIP_CARRIZO,	/* 13 */
    CHIP_STONEY,	/* 14 */
    CHIP_POLARIS10,	/* 15 */
    CHIP_POLARIS11,	/* 16 */
    CHIP_POLARIS12,	/* 17 */
    CHIP_VEGAM,	/* 18 */
    CHIP_VEGA10,	/* 19 */
    CHIP_VEGA12,	/* 20 */
    CHIP_VEGA20,	/* 21 */
    CHIP_RAVEN,	/* 22 */
    CHIP_ARCTURUS,	/* 23 */
    CHIP_RENOIR,	/* 24 */
    CHIP_ALDEBARAN, /* 25 */
    CHIP_NAVI10,	/* 26 */
    CHIP_CYAN_SKILLFISH,	/* 27 */
    CHIP_NAVI14,	/* 28 */
    CHIP_NAVI12,	/* 29 */
    CHIP_SIENNA_CICHLID,	/* 30 */
    CHIP_NAVY_FLOUNDER,	/* 31 */
    CHIP_VANGOGH,	/* 32 */
    CHIP_DIMGREY_CAVEFISH,	/* 33 */
    CHIP_BEIGE_GOBY,	/* 34 */
    CHIP_YELLOW_CARP,	/* 35 */
    CHIP_IP_DISCOVERY,	/* 36 */
    CHIP_LAST,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_asic_type_quirk {
    pub /: *mut *mut unsigned short device; / PCI device ID,
    pub /: *mut *mut u8 revision; / revision ID,
    pub /: *mut *mut unsigned short type; / real ASIC type,
}
