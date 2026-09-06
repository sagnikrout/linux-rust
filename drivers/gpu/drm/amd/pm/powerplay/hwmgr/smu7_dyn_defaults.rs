//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/powerplay/hwmgr/smu7_dyn_defaults.h
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
// Copyright 2015 Advanced Micro Devices, Inc.
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
// We need to fill in the default values
pub const SMU7_VOTINGRIGHTSCLIENTS_DFLT0: c_uint = 0x3FFFC102;
pub const SMU7_VOTINGRIGHTSCLIENTS_DFLT1: c_uint = 0x000400;
pub const SMU7_VOTINGRIGHTSCLIENTS_DFLT2: c_uint = 0xC00080;
pub const SMU7_VOTINGRIGHTSCLIENTS_DFLT3: c_uint = 0xC00200;
pub const SMU7_VOTINGRIGHTSCLIENTS_DFLT4: c_uint = 0xC01680;
pub const SMU7_VOTINGRIGHTSCLIENTS_DFLT5: c_uint = 0xC00033;
pub const SMU7_VOTINGRIGHTSCLIENTS_DFLT6: c_uint = 0xC00033;
pub const SMU7_VOTINGRIGHTSCLIENTS_DFLT7: c_uint = 0x3FFFC000;
pub const SMU7_THERMALPROTECTCOUNTER_DFLT: c_uint = 0x200;
pub const SMU7_STATICSCREENTHRESHOLDUNIT_DFLT: c_int = 0;
pub const SMU7_STATICSCREENTHRESHOLD_DFLT: c_uint = 0x00C8;
pub const SMU7_GFXIDLECLOCKSTOPTHRESHOLD_DFLT: c_uint = 0x200;
pub const SMU7_REFERENCEDIVIDER_DFLT: c_int = 4;
pub const SMU7_ULVVOLTAGECHANGEDELAY_DFLT: c_int = 1687;
pub const SMU7_CGULVPARAMETER_DFLT: c_uint = 0x00040035;
pub const SMU7_CGULVCONTROL_DFLT: c_uint = 0x00007450;
pub const SMU7_TARGETACTIVITY_DFLT: c_int = 50;
pub const SMU7_MCLK_TARGETACTIVITY_DFLT: c_int = 10;
pub const SMU7_SCLK_TARGETACTIVITY_DFLT: c_int = 30;
