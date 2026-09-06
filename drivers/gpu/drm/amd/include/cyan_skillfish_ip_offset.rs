//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/include/cyan_skillfish_ip_offset.h
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
// Copyright (C) 2018  Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included
// in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
// OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN
// AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//

// Macro flag: #define _cyan_skillfish_ip_offset_HEADER
pub const MAX_INSTANCE: c_int = 6;
pub const MAX_SEGMENT: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct IP_BASE_INSTANCE {
    pub segment: [c_uint; MAX_SEGMENT],
    pub __maybe_unused: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct IP_BASE {
    pub instance: [IP_BASE_INSTANCE; MAX_INSTANCE],
    pub __maybe_unused: },
    pub }: { { 0, 0, 0, 0, 0 } } },
    pub }: { { 0x0001B000, 0, 0, 0, 0 } } },
    pub }: { { 0, 0, 0, 0, 0 } } },
    pub }: { { 0, 0, 0, 0, 0 } } },
    pub }: { { 0, 0, 0, 0, 0 } } },
    pub }: { { 0, 0, 0, 0, 0 } } },
    pub }: { { 0, 0, 0, 0, 0 } } },
    pub }: { { 0, 0, 0, 0, 0 } } },
    pub }: { { 0, 0, 0, 0, 0 } } },
    pub }: { { 0, 0, 0, 0, 0 } } },
    pub }: { { 0, 0, 0, 0, 0 } } },
    pub }: { { 0, 0, 0, 0, 0 } } },
    pub }: { { 0, 0, 0, 0, 0 } } },
    pub }: { { 0, 0, 0, 0, 0 } } },
    pub }: { { 0, 0, 0, 0, 0 } } },
    pub }: { { 0, 0, 0, 0, 0 } } },
pub const ATHUB_BASE__INST0_SEG0: c_uint = 0x00000C00;
pub const ATHUB_BASE__INST0_SEG1: c_int = 0;
pub const ATHUB_BASE__INST0_SEG2: c_int = 0;
pub const ATHUB_BASE__INST0_SEG3: c_int = 0;
pub const ATHUB_BASE__INST0_SEG4: c_int = 0;
pub const ATHUB_BASE__INST1_SEG0: c_int = 0;
pub const ATHUB_BASE__INST1_SEG1: c_int = 0;
pub const ATHUB_BASE__INST1_SEG2: c_int = 0;
pub const ATHUB_BASE__INST1_SEG3: c_int = 0;
pub const ATHUB_BASE__INST1_SEG4: c_int = 0;
pub const ATHUB_BASE__INST2_SEG0: c_int = 0;
pub const ATHUB_BASE__INST2_SEG1: c_int = 0;
pub const ATHUB_BASE__INST2_SEG2: c_int = 0;
pub const ATHUB_BASE__INST2_SEG3: c_int = 0;
pub const ATHUB_BASE__INST2_SEG4: c_int = 0;
pub const ATHUB_BASE__INST3_SEG0: c_int = 0;
pub const ATHUB_BASE__INST3_SEG1: c_int = 0;
pub const ATHUB_BASE__INST3_SEG2: c_int = 0;
pub const ATHUB_BASE__INST3_SEG3: c_int = 0;
pub const ATHUB_BASE__INST3_SEG4: c_int = 0;
pub const ATHUB_BASE__INST4_SEG0: c_int = 0;
pub const ATHUB_BASE__INST4_SEG1: c_int = 0;
pub const ATHUB_BASE__INST4_SEG2: c_int = 0;
pub const ATHUB_BASE__INST4_SEG3: c_int = 0;
pub const ATHUB_BASE__INST4_SEG4: c_int = 0;
pub const ATHUB_BASE__INST5_SEG0: c_int = 0;
pub const ATHUB_BASE__INST5_SEG1: c_int = 0;
pub const ATHUB_BASE__INST5_SEG2: c_int = 0;
pub const ATHUB_BASE__INST5_SEG3: c_int = 0;
pub const ATHUB_BASE__INST5_SEG4: c_int = 0;
pub const CLK_BASE__INST0_SEG0: c_uint = 0x00016C00;
pub const CLK_BASE__INST0_SEG1: c_int = 0;
pub const CLK_BASE__INST0_SEG2: c_int = 0;
pub const CLK_BASE__INST0_SEG3: c_int = 0;
pub const CLK_BASE__INST0_SEG4: c_int = 0;
pub const CLK_BASE__INST1_SEG0: c_uint = 0x00016E00;
pub const CLK_BASE__INST1_SEG1: c_int = 0;
pub const CLK_BASE__INST1_SEG2: c_int = 0;
pub const CLK_BASE__INST1_SEG3: c_int = 0;
pub const CLK_BASE__INST1_SEG4: c_int = 0;
pub const CLK_BASE__INST2_SEG0: c_uint = 0x00017000;
pub const CLK_BASE__INST2_SEG1: c_int = 0;
pub const CLK_BASE__INST2_SEG2: c_int = 0;
pub const CLK_BASE__INST2_SEG3: c_int = 0;
pub const CLK_BASE__INST2_SEG4: c_int = 0;
pub const CLK_BASE__INST3_SEG0: c_uint = 0x00017200;
pub const CLK_BASE__INST3_SEG1: c_int = 0;
pub const CLK_BASE__INST3_SEG2: c_int = 0;
pub const CLK_BASE__INST3_SEG3: c_int = 0;
pub const CLK_BASE__INST3_SEG4: c_int = 0;
pub const CLK_BASE__INST4_SEG0: c_uint = 0x00017E00;
pub const CLK_BASE__INST4_SEG1: c_int = 0;
pub const CLK_BASE__INST4_SEG2: c_int = 0;
pub const CLK_BASE__INST4_SEG3: c_int = 0;
pub const CLK_BASE__INST4_SEG4: c_int = 0;
pub const CLK_BASE__INST5_SEG0: c_uint = 0x0001B000;
pub const CLK_BASE__INST5_SEG1: c_int = 0;
pub const CLK_BASE__INST5_SEG2: c_int = 0;
pub const CLK_BASE__INST5_SEG3: c_int = 0;
pub const CLK_BASE__INST5_SEG4: c_int = 0;
pub const DF_BASE__INST0_SEG0: c_uint = 0x00007000;
pub const DF_BASE__INST0_SEG1: c_int = 0;
pub const DF_BASE__INST0_SEG2: c_int = 0;
pub const DF_BASE__INST0_SEG3: c_int = 0;
pub const DF_BASE__INST0_SEG4: c_int = 0;
pub const DF_BASE__INST1_SEG0: c_int = 0;
pub const DF_BASE__INST1_SEG1: c_int = 0;
pub const DF_BASE__INST1_SEG2: c_int = 0;
pub const DF_BASE__INST1_SEG3: c_int = 0;
pub const DF_BASE__INST1_SEG4: c_int = 0;
pub const DF_BASE__INST2_SEG0: c_int = 0;
pub const DF_BASE__INST2_SEG1: c_int = 0;
pub const DF_BASE__INST2_SEG2: c_int = 0;
pub const DF_BASE__INST2_SEG3: c_int = 0;
pub const DF_BASE__INST2_SEG4: c_int = 0;
pub const DF_BASE__INST3_SEG0: c_int = 0;
pub const DF_BASE__INST3_SEG1: c_int = 0;
pub const DF_BASE__INST3_SEG2: c_int = 0;
pub const DF_BASE__INST3_SEG3: c_int = 0;
pub const DF_BASE__INST3_SEG4: c_int = 0;
pub const DF_BASE__INST4_SEG0: c_int = 0;
pub const DF_BASE__INST4_SEG1: c_int = 0;
pub const DF_BASE__INST4_SEG2: c_int = 0;
pub const DF_BASE__INST4_SEG3: c_int = 0;
pub const DF_BASE__INST4_SEG4: c_int = 0;
pub const DF_BASE__INST5_SEG0: c_int = 0;
pub const DF_BASE__INST5_SEG1: c_int = 0;
pub const DF_BASE__INST5_SEG2: c_int = 0;
pub const DF_BASE__INST5_SEG3: c_int = 0;
pub const DF_BASE__INST5_SEG4: c_int = 0;
pub const DMU_BASE__INST0_SEG0: c_uint = 0x00000012;
pub const DMU_BASE__INST0_SEG1: c_uint = 0x000000C0;
pub const DMU_BASE__INST0_SEG2: c_uint = 0x000034C0;
pub const DMU_BASE__INST0_SEG3: c_uint = 0x00009000;
pub const DMU_BASE__INST0_SEG4: c_int = 0;
pub const DMU_BASE__INST1_SEG0: c_int = 0;
pub const DMU_BASE__INST1_SEG1: c_int = 0;
pub const DMU_BASE__INST1_SEG2: c_int = 0;
pub const DMU_BASE__INST1_SEG3: c_int = 0;
pub const DMU_BASE__INST1_SEG4: c_int = 0;
pub const DMU_BASE__INST2_SEG0: c_int = 0;
pub const DMU_BASE__INST2_SEG1: c_int = 0;
pub const DMU_BASE__INST2_SEG2: c_int = 0;
pub const DMU_BASE__INST2_SEG3: c_int = 0;
pub const DMU_BASE__INST2_SEG4: c_int = 0;
pub const DMU_BASE__INST3_SEG0: c_int = 0;
pub const DMU_BASE__INST3_SEG1: c_int = 0;
pub const DMU_BASE__INST3_SEG2: c_int = 0;
pub const DMU_BASE__INST3_SEG3: c_int = 0;
pub const DMU_BASE__INST3_SEG4: c_int = 0;
pub const DMU_BASE__INST4_SEG0: c_int = 0;
pub const DMU_BASE__INST4_SEG1: c_int = 0;
pub const DMU_BASE__INST4_SEG2: c_int = 0;
pub const DMU_BASE__INST4_SEG3: c_int = 0;
pub const DMU_BASE__INST4_SEG4: c_int = 0;
pub const DMU_BASE__INST5_SEG0: c_int = 0;
pub const DMU_BASE__INST5_SEG1: c_int = 0;
pub const DMU_BASE__INST5_SEG2: c_int = 0;
pub const DMU_BASE__INST5_SEG3: c_int = 0;
pub const DMU_BASE__INST5_SEG4: c_int = 0;
pub const FUSE_BASE__INST0_SEG0: c_uint = 0x00017400;
pub const FUSE_BASE__INST0_SEG1: c_int = 0;
pub const FUSE_BASE__INST0_SEG2: c_int = 0;
pub const FUSE_BASE__INST0_SEG3: c_int = 0;
pub const FUSE_BASE__INST0_SEG4: c_int = 0;
pub const FUSE_BASE__INST1_SEG0: c_int = 0;
pub const FUSE_BASE__INST1_SEG1: c_int = 0;
pub const FUSE_BASE__INST1_SEG2: c_int = 0;
pub const FUSE_BASE__INST1_SEG3: c_int = 0;
pub const FUSE_BASE__INST1_SEG4: c_int = 0;
pub const FUSE_BASE__INST2_SEG0: c_int = 0;
pub const FUSE_BASE__INST2_SEG1: c_int = 0;
pub const FUSE_BASE__INST2_SEG2: c_int = 0;
pub const FUSE_BASE__INST2_SEG3: c_int = 0;
pub const FUSE_BASE__INST2_SEG4: c_int = 0;
pub const FUSE_BASE__INST3_SEG0: c_int = 0;
pub const FUSE_BASE__INST3_SEG1: c_int = 0;
pub const FUSE_BASE__INST3_SEG2: c_int = 0;
pub const FUSE_BASE__INST3_SEG3: c_int = 0;
pub const FUSE_BASE__INST3_SEG4: c_int = 0;
pub const FUSE_BASE__INST4_SEG0: c_int = 0;
pub const FUSE_BASE__INST4_SEG1: c_int = 0;
pub const FUSE_BASE__INST4_SEG2: c_int = 0;
pub const FUSE_BASE__INST4_SEG3: c_int = 0;
pub const FUSE_BASE__INST4_SEG4: c_int = 0;
pub const FUSE_BASE__INST5_SEG0: c_int = 0;
pub const FUSE_BASE__INST5_SEG1: c_int = 0;
pub const FUSE_BASE__INST5_SEG2: c_int = 0;
pub const FUSE_BASE__INST5_SEG3: c_int = 0;
pub const FUSE_BASE__INST5_SEG4: c_int = 0;
pub const GC_BASE__INST0_SEG0: c_uint = 0x00001260;
pub const GC_BASE__INST0_SEG1: c_uint = 0x0000A000;
pub const GC_BASE__INST0_SEG2: c_int = 0;
pub const GC_BASE__INST0_SEG3: c_int = 0;
pub const GC_BASE__INST0_SEG4: c_int = 0;
pub const GC_BASE__INST1_SEG0: c_int = 0;
pub const GC_BASE__INST1_SEG1: c_int = 0;
pub const GC_BASE__INST1_SEG2: c_int = 0;
pub const GC_BASE__INST1_SEG3: c_int = 0;
pub const GC_BASE__INST1_SEG4: c_int = 0;
pub const GC_BASE__INST2_SEG0: c_int = 0;
pub const GC_BASE__INST2_SEG1: c_int = 0;
pub const GC_BASE__INST2_SEG2: c_int = 0;
pub const GC_BASE__INST2_SEG3: c_int = 0;
pub const GC_BASE__INST2_SEG4: c_int = 0;
pub const GC_BASE__INST3_SEG0: c_int = 0;
pub const GC_BASE__INST3_SEG1: c_int = 0;
pub const GC_BASE__INST3_SEG2: c_int = 0;
pub const GC_BASE__INST3_SEG3: c_int = 0;
pub const GC_BASE__INST3_SEG4: c_int = 0;
pub const GC_BASE__INST4_SEG0: c_int = 0;
pub const GC_BASE__INST4_SEG1: c_int = 0;
pub const GC_BASE__INST4_SEG2: c_int = 0;
pub const GC_BASE__INST4_SEG3: c_int = 0;
pub const GC_BASE__INST4_SEG4: c_int = 0;
pub const GC_BASE__INST5_SEG0: c_int = 0;
pub const GC_BASE__INST5_SEG1: c_int = 0;
pub const GC_BASE__INST5_SEG2: c_int = 0;
pub const GC_BASE__INST5_SEG3: c_int = 0;
pub const GC_BASE__INST5_SEG4: c_int = 0;
pub const HDP_BASE__INST0_SEG0: c_uint = 0x00000F20;
pub const HDP_BASE__INST0_SEG1: c_int = 0;
pub const HDP_BASE__INST0_SEG2: c_int = 0;
pub const HDP_BASE__INST0_SEG3: c_int = 0;
pub const HDP_BASE__INST0_SEG4: c_int = 0;
pub const HDP_BASE__INST1_SEG0: c_int = 0;
pub const HDP_BASE__INST1_SEG1: c_int = 0;
pub const HDP_BASE__INST1_SEG2: c_int = 0;
pub const HDP_BASE__INST1_SEG3: c_int = 0;
pub const HDP_BASE__INST1_SEG4: c_int = 0;
pub const HDP_BASE__INST2_SEG0: c_int = 0;
pub const HDP_BASE__INST2_SEG1: c_int = 0;
pub const HDP_BASE__INST2_SEG2: c_int = 0;
pub const HDP_BASE__INST2_SEG3: c_int = 0;
pub const HDP_BASE__INST2_SEG4: c_int = 0;
pub const HDP_BASE__INST3_SEG0: c_int = 0;
pub const HDP_BASE__INST3_SEG1: c_int = 0;
pub const HDP_BASE__INST3_SEG2: c_int = 0;
pub const HDP_BASE__INST3_SEG3: c_int = 0;
pub const HDP_BASE__INST3_SEG4: c_int = 0;
pub const HDP_BASE__INST4_SEG0: c_int = 0;
pub const HDP_BASE__INST4_SEG1: c_int = 0;
pub const HDP_BASE__INST4_SEG2: c_int = 0;
pub const HDP_BASE__INST4_SEG3: c_int = 0;
pub const HDP_BASE__INST4_SEG4: c_int = 0;
pub const HDP_BASE__INST5_SEG0: c_int = 0;
pub const HDP_BASE__INST5_SEG1: c_int = 0;
pub const HDP_BASE__INST5_SEG2: c_int = 0;
pub const HDP_BASE__INST5_SEG3: c_int = 0;
pub const HDP_BASE__INST5_SEG4: c_int = 0;
pub const MMHUB_BASE__INST0_SEG0: c_uint = 0x0001A000;
pub const MMHUB_BASE__INST0_SEG1: c_int = 0;
pub const MMHUB_BASE__INST0_SEG2: c_int = 0;
pub const MMHUB_BASE__INST0_SEG3: c_int = 0;
pub const MMHUB_BASE__INST0_SEG4: c_int = 0;
pub const MMHUB_BASE__INST1_SEG0: c_int = 0;
pub const MMHUB_BASE__INST1_SEG1: c_int = 0;
pub const MMHUB_BASE__INST1_SEG2: c_int = 0;
pub const MMHUB_BASE__INST1_SEG3: c_int = 0;
pub const MMHUB_BASE__INST1_SEG4: c_int = 0;
pub const MMHUB_BASE__INST2_SEG0: c_int = 0;
pub const MMHUB_BASE__INST2_SEG1: c_int = 0;
pub const MMHUB_BASE__INST2_SEG2: c_int = 0;
pub const MMHUB_BASE__INST2_SEG3: c_int = 0;
pub const MMHUB_BASE__INST2_SEG4: c_int = 0;
pub const MMHUB_BASE__INST3_SEG0: c_int = 0;
pub const MMHUB_BASE__INST3_SEG1: c_int = 0;
pub const MMHUB_BASE__INST3_SEG2: c_int = 0;
pub const MMHUB_BASE__INST3_SEG3: c_int = 0;
pub const MMHUB_BASE__INST3_SEG4: c_int = 0;
pub const MMHUB_BASE__INST4_SEG0: c_int = 0;
pub const MMHUB_BASE__INST4_SEG1: c_int = 0;
pub const MMHUB_BASE__INST4_SEG2: c_int = 0;
pub const MMHUB_BASE__INST4_SEG3: c_int = 0;
pub const MMHUB_BASE__INST4_SEG4: c_int = 0;
pub const MMHUB_BASE__INST5_SEG0: c_int = 0;
pub const MMHUB_BASE__INST5_SEG1: c_int = 0;
pub const MMHUB_BASE__INST5_SEG2: c_int = 0;
pub const MMHUB_BASE__INST5_SEG3: c_int = 0;
pub const MMHUB_BASE__INST5_SEG4: c_int = 0;
pub const MP0_BASE__INST0_SEG0: c_uint = 0x00016000;
pub const MP0_BASE__INST0_SEG1: c_int = 0;
pub const MP0_BASE__INST0_SEG2: c_int = 0;
pub const MP0_BASE__INST0_SEG3: c_int = 0;
pub const MP0_BASE__INST0_SEG4: c_int = 0;
pub const MP0_BASE__INST1_SEG0: c_int = 0;
pub const MP0_BASE__INST1_SEG1: c_int = 0;
pub const MP0_BASE__INST1_SEG2: c_int = 0;
pub const MP0_BASE__INST1_SEG3: c_int = 0;
pub const MP0_BASE__INST1_SEG4: c_int = 0;
pub const MP0_BASE__INST2_SEG0: c_int = 0;
pub const MP0_BASE__INST2_SEG1: c_int = 0;
pub const MP0_BASE__INST2_SEG2: c_int = 0;
pub const MP0_BASE__INST2_SEG3: c_int = 0;
pub const MP0_BASE__INST2_SEG4: c_int = 0;
pub const MP0_BASE__INST3_SEG0: c_int = 0;
pub const MP0_BASE__INST3_SEG1: c_int = 0;
pub const MP0_BASE__INST3_SEG2: c_int = 0;
pub const MP0_BASE__INST3_SEG3: c_int = 0;
pub const MP0_BASE__INST3_SEG4: c_int = 0;
pub const MP0_BASE__INST4_SEG0: c_int = 0;
pub const MP0_BASE__INST4_SEG1: c_int = 0;
pub const MP0_BASE__INST4_SEG2: c_int = 0;
pub const MP0_BASE__INST4_SEG3: c_int = 0;
pub const MP0_BASE__INST4_SEG4: c_int = 0;
pub const MP0_BASE__INST5_SEG0: c_int = 0;
pub const MP0_BASE__INST5_SEG1: c_int = 0;
pub const MP0_BASE__INST5_SEG2: c_int = 0;
pub const MP0_BASE__INST5_SEG3: c_int = 0;
pub const MP0_BASE__INST5_SEG4: c_int = 0;
pub const MP1_BASE__INST0_SEG0: c_uint = 0x00016000;
pub const MP1_BASE__INST0_SEG1: c_int = 0;
pub const MP1_BASE__INST0_SEG2: c_int = 0;
pub const MP1_BASE__INST0_SEG3: c_int = 0;
pub const MP1_BASE__INST0_SEG4: c_int = 0;
pub const MP1_BASE__INST1_SEG0: c_int = 0;
pub const MP1_BASE__INST1_SEG1: c_int = 0;
pub const MP1_BASE__INST1_SEG2: c_int = 0;
pub const MP1_BASE__INST1_SEG3: c_int = 0;
pub const MP1_BASE__INST1_SEG4: c_int = 0;
pub const MP1_BASE__INST2_SEG0: c_int = 0;
pub const MP1_BASE__INST2_SEG1: c_int = 0;
pub const MP1_BASE__INST2_SEG2: c_int = 0;
pub const MP1_BASE__INST2_SEG3: c_int = 0;
pub const MP1_BASE__INST2_SEG4: c_int = 0;
pub const MP1_BASE__INST3_SEG0: c_int = 0;
pub const MP1_BASE__INST3_SEG1: c_int = 0;
pub const MP1_BASE__INST3_SEG2: c_int = 0;
pub const MP1_BASE__INST3_SEG3: c_int = 0;
pub const MP1_BASE__INST3_SEG4: c_int = 0;
pub const MP1_BASE__INST4_SEG0: c_int = 0;
pub const MP1_BASE__INST4_SEG1: c_int = 0;
pub const MP1_BASE__INST4_SEG2: c_int = 0;
pub const MP1_BASE__INST4_SEG3: c_int = 0;
pub const MP1_BASE__INST4_SEG4: c_int = 0;
pub const MP1_BASE__INST5_SEG0: c_int = 0;
pub const MP1_BASE__INST5_SEG1: c_int = 0;
pub const MP1_BASE__INST5_SEG2: c_int = 0;
pub const MP1_BASE__INST5_SEG3: c_int = 0;
pub const MP1_BASE__INST5_SEG4: c_int = 0;
pub const NBIO_BASE__INST0_SEG0: c_uint = 0x00000000;
pub const NBIO_BASE__INST0_SEG1: c_uint = 0x00000014;
pub const NBIO_BASE__INST0_SEG2: c_uint = 0x00000D20;
pub const NBIO_BASE__INST0_SEG3: c_uint = 0x00010400;
pub const NBIO_BASE__INST0_SEG4: c_int = 0;
pub const NBIO_BASE__INST1_SEG0: c_int = 0;
pub const NBIO_BASE__INST1_SEG1: c_int = 0;
pub const NBIO_BASE__INST1_SEG2: c_int = 0;
pub const NBIO_BASE__INST1_SEG3: c_int = 0;
pub const NBIO_BASE__INST1_SEG4: c_int = 0;
pub const NBIO_BASE__INST2_SEG0: c_int = 0;
pub const NBIO_BASE__INST2_SEG1: c_int = 0;
pub const NBIO_BASE__INST2_SEG2: c_int = 0;
pub const NBIO_BASE__INST2_SEG3: c_int = 0;
pub const NBIO_BASE__INST2_SEG4: c_int = 0;
pub const NBIO_BASE__INST3_SEG0: c_int = 0;
pub const NBIO_BASE__INST3_SEG1: c_int = 0;
pub const NBIO_BASE__INST3_SEG2: c_int = 0;
pub const NBIO_BASE__INST3_SEG3: c_int = 0;
pub const NBIO_BASE__INST3_SEG4: c_int = 0;
pub const NBIO_BASE__INST4_SEG0: c_int = 0;
pub const NBIO_BASE__INST4_SEG1: c_int = 0;
pub const NBIO_BASE__INST4_SEG2: c_int = 0;
pub const NBIO_BASE__INST4_SEG3: c_int = 0;
pub const NBIO_BASE__INST4_SEG4: c_int = 0;
pub const NBIO_BASE__INST5_SEG0: c_int = 0;
pub const NBIO_BASE__INST5_SEG1: c_int = 0;
pub const NBIO_BASE__INST5_SEG2: c_int = 0;
pub const NBIO_BASE__INST5_SEG3: c_int = 0;
pub const NBIO_BASE__INST5_SEG4: c_int = 0;
pub const OSSSYS_BASE__INST0_SEG0: c_uint = 0x000010A0;
pub const OSSSYS_BASE__INST0_SEG1: c_int = 0;
pub const OSSSYS_BASE__INST0_SEG2: c_int = 0;
pub const OSSSYS_BASE__INST0_SEG3: c_int = 0;
pub const OSSSYS_BASE__INST0_SEG4: c_int = 0;
pub const OSSSYS_BASE__INST1_SEG0: c_int = 0;
pub const OSSSYS_BASE__INST1_SEG1: c_int = 0;
pub const OSSSYS_BASE__INST1_SEG2: c_int = 0;
pub const OSSSYS_BASE__INST1_SEG3: c_int = 0;
pub const OSSSYS_BASE__INST1_SEG4: c_int = 0;
pub const OSSSYS_BASE__INST2_SEG0: c_int = 0;
pub const OSSSYS_BASE__INST2_SEG1: c_int = 0;
pub const OSSSYS_BASE__INST2_SEG2: c_int = 0;
pub const OSSSYS_BASE__INST2_SEG3: c_int = 0;
pub const OSSSYS_BASE__INST2_SEG4: c_int = 0;
pub const OSSSYS_BASE__INST3_SEG0: c_int = 0;
pub const OSSSYS_BASE__INST3_SEG1: c_int = 0;
pub const OSSSYS_BASE__INST3_SEG2: c_int = 0;
pub const OSSSYS_BASE__INST3_SEG3: c_int = 0;
pub const OSSSYS_BASE__INST3_SEG4: c_int = 0;
pub const OSSSYS_BASE__INST4_SEG0: c_int = 0;
pub const OSSSYS_BASE__INST4_SEG1: c_int = 0;
pub const OSSSYS_BASE__INST4_SEG2: c_int = 0;
pub const OSSSYS_BASE__INST4_SEG3: c_int = 0;
pub const OSSSYS_BASE__INST4_SEG4: c_int = 0;
pub const OSSSYS_BASE__INST5_SEG0: c_int = 0;
pub const OSSSYS_BASE__INST5_SEG1: c_int = 0;
pub const OSSSYS_BASE__INST5_SEG2: c_int = 0;
pub const OSSSYS_BASE__INST5_SEG3: c_int = 0;
pub const OSSSYS_BASE__INST5_SEG4: c_int = 0;
pub const SMUIO_BASE__INST0_SEG0: c_uint = 0x00016800;
pub const SMUIO_BASE__INST0_SEG1: c_uint = 0x00016A00;
pub const SMUIO_BASE__INST0_SEG2: c_int = 0;
pub const SMUIO_BASE__INST0_SEG3: c_int = 0;
pub const SMUIO_BASE__INST0_SEG4: c_int = 0;
pub const SMUIO_BASE__INST1_SEG0: c_int = 0;
pub const SMUIO_BASE__INST1_SEG1: c_int = 0;
pub const SMUIO_BASE__INST1_SEG2: c_int = 0;
pub const SMUIO_BASE__INST1_SEG3: c_int = 0;
pub const SMUIO_BASE__INST1_SEG4: c_int = 0;
pub const SMUIO_BASE__INST2_SEG0: c_int = 0;
pub const SMUIO_BASE__INST2_SEG1: c_int = 0;
pub const SMUIO_BASE__INST2_SEG2: c_int = 0;
pub const SMUIO_BASE__INST2_SEG3: c_int = 0;
pub const SMUIO_BASE__INST2_SEG4: c_int = 0;
pub const SMUIO_BASE__INST3_SEG0: c_int = 0;
pub const SMUIO_BASE__INST3_SEG1: c_int = 0;
pub const SMUIO_BASE__INST3_SEG2: c_int = 0;
pub const SMUIO_BASE__INST3_SEG3: c_int = 0;
pub const SMUIO_BASE__INST3_SEG4: c_int = 0;
pub const SMUIO_BASE__INST4_SEG0: c_int = 0;
pub const SMUIO_BASE__INST4_SEG1: c_int = 0;
pub const SMUIO_BASE__INST4_SEG2: c_int = 0;
pub const SMUIO_BASE__INST4_SEG3: c_int = 0;
pub const SMUIO_BASE__INST4_SEG4: c_int = 0;
pub const SMUIO_BASE__INST5_SEG0: c_int = 0;
pub const SMUIO_BASE__INST5_SEG1: c_int = 0;
pub const SMUIO_BASE__INST5_SEG2: c_int = 0;
pub const SMUIO_BASE__INST5_SEG3: c_int = 0;
pub const SMUIO_BASE__INST5_SEG4: c_int = 0;
pub const THM_BASE__INST0_SEG0: c_uint = 0x00016600;
pub const THM_BASE__INST0_SEG1: c_int = 0;
pub const THM_BASE__INST0_SEG2: c_int = 0;
pub const THM_BASE__INST0_SEG3: c_int = 0;
pub const THM_BASE__INST0_SEG4: c_int = 0;
pub const THM_BASE__INST1_SEG0: c_int = 0;
pub const THM_BASE__INST1_SEG1: c_int = 0;
pub const THM_BASE__INST1_SEG2: c_int = 0;
pub const THM_BASE__INST1_SEG3: c_int = 0;
pub const THM_BASE__INST1_SEG4: c_int = 0;
pub const THM_BASE__INST2_SEG0: c_int = 0;
pub const THM_BASE__INST2_SEG1: c_int = 0;
pub const THM_BASE__INST2_SEG2: c_int = 0;
pub const THM_BASE__INST2_SEG3: c_int = 0;
pub const THM_BASE__INST2_SEG4: c_int = 0;
pub const THM_BASE__INST3_SEG0: c_int = 0;
pub const THM_BASE__INST3_SEG1: c_int = 0;
pub const THM_BASE__INST3_SEG2: c_int = 0;
pub const THM_BASE__INST3_SEG3: c_int = 0;
pub const THM_BASE__INST3_SEG4: c_int = 0;
pub const THM_BASE__INST4_SEG0: c_int = 0;
pub const THM_BASE__INST4_SEG1: c_int = 0;
pub const THM_BASE__INST4_SEG2: c_int = 0;
pub const THM_BASE__INST4_SEG3: c_int = 0;
pub const THM_BASE__INST4_SEG4: c_int = 0;
pub const THM_BASE__INST5_SEG0: c_int = 0;
pub const THM_BASE__INST5_SEG1: c_int = 0;
pub const THM_BASE__INST5_SEG2: c_int = 0;
pub const THM_BASE__INST5_SEG3: c_int = 0;
pub const THM_BASE__INST5_SEG4: c_int = 0;
pub const UMC0_BASE__INST0_SEG0: c_uint = 0x00014000;
pub const UMC0_BASE__INST0_SEG1: c_int = 0;
pub const UMC0_BASE__INST0_SEG2: c_int = 0;
pub const UMC0_BASE__INST0_SEG3: c_int = 0;
pub const UMC0_BASE__INST0_SEG4: c_int = 0;
pub const UMC0_BASE__INST1_SEG0: c_int = 0;
pub const UMC0_BASE__INST1_SEG1: c_int = 0;
pub const UMC0_BASE__INST1_SEG2: c_int = 0;
pub const UMC0_BASE__INST1_SEG3: c_int = 0;
pub const UMC0_BASE__INST1_SEG4: c_int = 0;
pub const UMC0_BASE__INST2_SEG0: c_int = 0;
pub const UMC0_BASE__INST2_SEG1: c_int = 0;
pub const UMC0_BASE__INST2_SEG2: c_int = 0;
pub const UMC0_BASE__INST2_SEG3: c_int = 0;
pub const UMC0_BASE__INST2_SEG4: c_int = 0;
pub const UMC0_BASE__INST3_SEG0: c_int = 0;
pub const UMC0_BASE__INST3_SEG1: c_int = 0;
pub const UMC0_BASE__INST3_SEG2: c_int = 0;
pub const UMC0_BASE__INST3_SEG3: c_int = 0;
pub const UMC0_BASE__INST3_SEG4: c_int = 0;
pub const UMC0_BASE__INST4_SEG0: c_int = 0;
pub const UMC0_BASE__INST4_SEG1: c_int = 0;
pub const UMC0_BASE__INST4_SEG2: c_int = 0;
pub const UMC0_BASE__INST4_SEG3: c_int = 0;
pub const UMC0_BASE__INST4_SEG4: c_int = 0;
pub const UMC0_BASE__INST5_SEG0: c_int = 0;
pub const UMC0_BASE__INST5_SEG1: c_int = 0;
pub const UMC0_BASE__INST5_SEG2: c_int = 0;
pub const UMC0_BASE__INST5_SEG3: c_int = 0;
pub const UMC0_BASE__INST5_SEG4: c_int = 0;
pub const UVD0_BASE__INST0_SEG0: c_uint = 0x00007800;
pub const UVD0_BASE__INST0_SEG1: c_uint = 0x00007E00;
pub const UVD0_BASE__INST0_SEG2: c_int = 0;
pub const UVD0_BASE__INST0_SEG3: c_int = 0;
pub const UVD0_BASE__INST0_SEG4: c_int = 0;
pub const UVD0_BASE__INST1_SEG0: c_int = 0;
pub const UVD0_BASE__INST1_SEG1: c_int = 0;
pub const UVD0_BASE__INST1_SEG2: c_int = 0;
pub const UVD0_BASE__INST1_SEG3: c_int = 0;
pub const UVD0_BASE__INST1_SEG4: c_int = 0;
pub const UVD0_BASE__INST2_SEG0: c_int = 0;
pub const UVD0_BASE__INST2_SEG1: c_int = 0;
pub const UVD0_BASE__INST2_SEG2: c_int = 0;
pub const UVD0_BASE__INST2_SEG3: c_int = 0;
pub const UVD0_BASE__INST2_SEG4: c_int = 0;
pub const UVD0_BASE__INST3_SEG0: c_int = 0;
pub const UVD0_BASE__INST3_SEG1: c_int = 0;
pub const UVD0_BASE__INST3_SEG2: c_int = 0;
pub const UVD0_BASE__INST3_SEG3: c_int = 0;
pub const UVD0_BASE__INST3_SEG4: c_int = 0;
pub const UVD0_BASE__INST4_SEG0: c_int = 0;
pub const UVD0_BASE__INST4_SEG1: c_int = 0;
pub const UVD0_BASE__INST4_SEG2: c_int = 0;
pub const UVD0_BASE__INST4_SEG3: c_int = 0;
pub const UVD0_BASE__INST4_SEG4: c_int = 0;
pub const UVD0_BASE__INST5_SEG0: c_int = 0;
pub const UVD0_BASE__INST5_SEG1: c_int = 0;
pub const UVD0_BASE__INST5_SEG2: c_int = 0;
pub const UVD0_BASE__INST5_SEG3: c_int = 0;
pub const UVD0_BASE__INST5_SEG4: c_int = 0;
