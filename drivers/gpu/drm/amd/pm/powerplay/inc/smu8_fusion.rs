//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/powerplay/inc/smu8_fusion.h
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
// Copyright 2014 Advanced Micro Devices, Inc.
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

pub const SMU8_MAX_CUS: c_int = 2;
pub const SMU8_PSMS_PER_CU: c_int = 4;
pub const SMU8_CACS_PER_CU: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU8_GfxCuPgScoreboard {
    pub Enabled: u8,
    pub spare: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU8_Port80MonitorTable {
    pub MmioAddress: u32,
    pub MemoryBaseHi: u32,
    pub MemoryBaseLo: u32,
    pub MemoryBufferSize: u16,
    pub MemoryPosition: u16,
    pub PollingInterval: u16,
    pub EnableCsrShadow: u8,
    pub EnableDramShadow: u8,
}

// Display specific power management parameters
pub const PWRMGT_SEPARATION_TIME_SHIFT: c_int = 0;
pub const PWRMGT_SEPARATION_TIME_MASK: c_uint = 0xFFFF;
pub const PWRMGT_DISABLE_CPU_CSTATES_SHIFT: c_int = 16;
pub const PWRMGT_DISABLE_CPU_CSTATES_MASK: c_uint = 0x1;
pub const PWRMGT_DISABLE_CPU_PSTATES_SHIFT: c_int = 24;
pub const PWRMGT_DISABLE_CPU_PSTATES_MASK: c_uint = 0x1;
// Clock Table Definitions
pub const NUM_SCLK_LEVELS: c_int = 8;
pub const NUM_LCLK_LEVELS: c_int = 8;
pub const NUM_UVD_LEVELS: c_int = 8;
pub const NUM_ECLK_LEVELS: c_int = 8;
pub const NUM_ACLK_LEVELS: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU8_Fusion_ClkLevel {
    pub GnbVid: u8,
    pub GfxVid: u8,
    pub DfsDid: u8,
    pub DeepSleepDid: u8,
    pub DfsBypass: u32,
    pub Frequency: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU8_Fusion_SclkBreakdownTable {
    pub ClkLevel: [SMU8_Fusion_ClkLevel; NUM_SCLK_LEVELS],
    pub DpmOffLevel: SMU8_Fusion_ClkLevel,
// SMU8_Fusion_ClkLevel PwrOffLevel;
    pub SclkValidMask: u32,
    pub MaxSclkIndex: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU8_Fusion_LclkBreakdownTable {
    pub ClkLevel: [SMU8_Fusion_ClkLevel; NUM_LCLK_LEVELS],
    pub DpmOffLevel: SMU8_Fusion_ClkLevel,
// SMU8_Fusion_ClkLevel PwrOffLevel;
    pub LclkValidMask: u32,
    pub MaxLclkIndex: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU8_Fusion_EclkBreakdownTable {
    pub ClkLevel: [SMU8_Fusion_ClkLevel; NUM_ECLK_LEVELS],
    pub DpmOffLevel: SMU8_Fusion_ClkLevel,
    pub PwrOffLevel: SMU8_Fusion_ClkLevel,
    pub EclkValidMask: u32,
    pub MaxEclkIndex: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU8_Fusion_VclkBreakdownTable {
    pub ClkLevel: [SMU8_Fusion_ClkLevel; NUM_UVD_LEVELS],
    pub DpmOffLevel: SMU8_Fusion_ClkLevel,
    pub PwrOffLevel: SMU8_Fusion_ClkLevel,
    pub VclkValidMask: u32,
    pub MaxVclkIndex: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU8_Fusion_DclkBreakdownTable {
    pub ClkLevel: [SMU8_Fusion_ClkLevel; NUM_UVD_LEVELS],
    pub DpmOffLevel: SMU8_Fusion_ClkLevel,
    pub PwrOffLevel: SMU8_Fusion_ClkLevel,
    pub DclkValidMask: u32,
    pub MaxDclkIndex: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU8_Fusion_AclkBreakdownTable {
    pub ClkLevel: [SMU8_Fusion_ClkLevel; NUM_ACLK_LEVELS],
    pub DpmOffLevel: SMU8_Fusion_ClkLevel,
    pub PwrOffLevel: SMU8_Fusion_ClkLevel,
    pub AclkValidMask: u32,
    pub MaxAclkIndex: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU8_Fusion_ClkTable {
    pub SclkBreakdownTable: SMU8_Fusion_SclkBreakdownTable,
    pub LclkBreakdownTable: SMU8_Fusion_LclkBreakdownTable,
    pub EclkBreakdownTable: SMU8_Fusion_EclkBreakdownTable,
    pub VclkBreakdownTable: SMU8_Fusion_VclkBreakdownTable,
    pub DclkBreakdownTable: SMU8_Fusion_DclkBreakdownTable,
    pub AclkBreakdownTable: SMU8_Fusion_AclkBreakdownTable,
}

