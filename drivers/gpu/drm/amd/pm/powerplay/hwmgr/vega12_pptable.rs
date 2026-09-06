//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/powerplay/hwmgr/vega12_pptable.h
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

pub const ATOM_VEGA12_PP_THERMALCONTROLLER_NONE: c_int = 0;
pub const ATOM_VEGA12_PP_THERMALCONTROLLER_VEGA12: c_int = 25;
pub const ATOM_VEGA12_PP_PLATFORM_CAP_POWERPLAY: c_uint = 0x1;
pub const ATOM_VEGA12_PP_PLATFORM_CAP_SBIOSPOWERSOURCE: c_uint = 0x2;
pub const ATOM_VEGA12_PP_PLATFORM_CAP_HARDWAREDC: c_uint = 0x4;
pub const ATOM_VEGA12_PP_PLATFORM_CAP_BACO: c_uint = 0x8;
pub const ATOM_VEGA12_PP_PLATFORM_CAP_BAMACO: c_uint = 0x10;
pub const ATOM_VEGA12_PP_PLATFORM_CAP_ENABLESHADOWPSTATE: c_uint = 0x20;
pub const ATOM_VEGA12_TABLE_REVISION_VEGA12: c_int = 9;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ATOM_VEGA12_ODSETTING_ID {
    ATOM_VEGA12_ODSETTING_GFXCLKFMAX = 0,
    ATOM_VEGA12_ODSETTING_GFXCLKFMIN,
    ATOM_VEGA12_ODSETTING_VDDGFXCURVEFREQ_P1,
    ATOM_VEGA12_ODSETTING_VDDGFXCURVEVOLTAGEOFFSET_P1,
    ATOM_VEGA12_ODSETTING_VDDGFXCURVEFREQ_P2,
    ATOM_VEGA12_ODSETTING_VDDGFXCURVEVOLTAGEOFFSET_P2,
    ATOM_VEGA12_ODSETTING_VDDGFXCURVEFREQ_P3,
    ATOM_VEGA12_ODSETTING_VDDGFXCURVEVOLTAGEOFFSET_P3,
    ATOM_VEGA12_ODSETTING_UCLKFMAX,
    ATOM_VEGA12_ODSETTING_POWERPERCENTAGE,
    ATOM_VEGA12_ODSETTING_FANRPMMIN,
    ATOM_VEGA12_ODSETTING_FANRPMACOUSTICLIMIT,
    ATOM_VEGA12_ODSETTING_FANTARGETTEMPERATURE,
    ATOM_VEGA12_ODSETTING_OPERATINGTEMPMAX,
    ATOM_VEGA12_ODSETTING_COUNT,
}

pub type ATOM_VEGA12_ODSETTING_ID = ATOM_VEGA12_ODSETTING_ID;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ATOM_VEGA12_PPCLOCK_ID {
    ATOM_VEGA12_PPCLOCK_GFXCLK = 0,
    ATOM_VEGA12_PPCLOCK_VCLK,
    ATOM_VEGA12_PPCLOCK_DCLK,
    ATOM_VEGA12_PPCLOCK_ECLK,
    ATOM_VEGA12_PPCLOCK_SOCCLK,
    ATOM_VEGA12_PPCLOCK_UCLK,
    ATOM_VEGA12_PPCLOCK_DCEFCLK,
    ATOM_VEGA12_PPCLOCK_DISPCLK,
    ATOM_VEGA12_PPCLOCK_PIXCLK,
    ATOM_VEGA12_PPCLOCK_PHYCLK,
    ATOM_VEGA12_PPCLOCK_COUNT,
}

pub type ATOM_VEGA12_PPCLOCK_ID = ATOM_VEGA12_PPCLOCK_ID;

