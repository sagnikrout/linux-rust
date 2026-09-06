//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/include/asic_reg/df/df_3_6_offset.h
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

// Macro flag: #define _df_3_6_OFFSET_HEADER
pub const mmFabricConfigAccessControl: c_uint = 0x0410;
pub const mmFabricConfigAccessControl_BASE_IDX: c_int = 0;
pub const mmDF_PIE_AON0_DfGlobalClkGater: c_uint = 0x00fc;
pub const mmDF_PIE_AON0_DfGlobalClkGater_BASE_IDX: c_int = 0;
pub const mmDF_CS_UMC_AON0_DfGlobalCtrl: c_uint = 0x00fe;
pub const mmDF_CS_UMC_AON0_DfGlobalCtrl_BASE_IDX: c_int = 0;
pub const mmDF_CS_UMC_AON0_DramBaseAddress0: c_uint = 0x0044;
pub const mmDF_CS_UMC_AON0_DramBaseAddress0_BASE_IDX: c_int = 0;
pub const mmDF_GCM_AON0_DramMegaBaseAddress0: c_uint = 0x0064;
pub const mmDF_GCM_AON0_DramMegaBaseAddress0_BASE_IDX: c_int = 0;
pub const smnPerfMonCtlLo0: c_uint = 0x01d440UL;
pub const smnPerfMonCtlHi0: c_uint = 0x01d444UL;
pub const smnPerfMonCtlLo1: c_uint = 0x01d450UL;
pub const smnPerfMonCtlHi1: c_uint = 0x01d454UL;
pub const smnPerfMonCtlLo2: c_uint = 0x01d460UL;
pub const smnPerfMonCtlHi2: c_uint = 0x01d464UL;
pub const smnPerfMonCtlLo3: c_uint = 0x01d470UL;
pub const smnPerfMonCtlHi3: c_uint = 0x01d474UL;
pub const smnPerfMonCtlLo4: c_uint = 0x01d880UL;
pub const smnPerfMonCtlHi4: c_uint = 0x01d884UL;
pub const smnPerfMonCtlLo5: c_uint = 0x01d888UL;
pub const smnPerfMonCtlHi5: c_uint = 0x01d88cUL;
pub const smnPerfMonCtlLo6: c_uint = 0x01d890UL;
pub const smnPerfMonCtlHi6: c_uint = 0x01d894UL;
pub const smnPerfMonCtlLo7: c_uint = 0x01d898UL;
pub const smnPerfMonCtlHi7: c_uint = 0x01d89cUL;
pub const smnPerfMonCtrLo0: c_uint = 0x01d448UL;
pub const smnPerfMonCtrHi0: c_uint = 0x01d44cUL;
pub const smnPerfMonCtrLo1: c_uint = 0x01d458UL;
pub const smnPerfMonCtrHi1: c_uint = 0x01d45cUL;
pub const smnPerfMonCtrLo2: c_uint = 0x01d468UL;
pub const smnPerfMonCtrHi2: c_uint = 0x01d46cUL;
pub const smnPerfMonCtrLo3: c_uint = 0x01d478UL;
pub const smnPerfMonCtrHi3: c_uint = 0x01d47cUL;
pub const smnPerfMonCtrLo4: c_uint = 0x01d790UL;
pub const smnPerfMonCtrHi4: c_uint = 0x01d794UL;
pub const smnPerfMonCtrLo5: c_uint = 0x01d798UL;
pub const smnPerfMonCtrHi5: c_uint = 0x01d79cUL;
pub const smnPerfMonCtrLo6: c_uint = 0x01d7a0UL;
pub const smnPerfMonCtrHi6: c_uint = 0x01d7a4UL;
pub const smnPerfMonCtrLo7: c_uint = 0x01d7a8UL;
pub const smnPerfMonCtrHi7: c_uint = 0x01d7acUL;
pub const smnDF_PIE_AON_FabricIndirectConfigAccessAddress3: c_uint = 0x1d05cUL;
pub const smnDF_PIE_AON_FabricIndirectConfigAccessDataLo3: c_uint = 0x1d098UL;
pub const smnDF_PIE_AON_FabricIndirectConfigAccessDataHi3: c_uint = 0x1d09cUL;
pub const smnDF_CS_UMC_AON0_DramBaseAddress0: c_uint = 0x1c110UL;
pub const smnDF_CS_UMC_AON0_DramLimitAddress0: c_uint = 0x1c114UL;
pub const mmDF_CS_UMC_AON0_HardwareAssertMaskLow: c_uint = 0x067e;
pub const mmDF_CS_UMC_AON0_HardwareAssertMaskLow_BASE_IDX: c_int = 0;
pub const mmDF_NCS_PG0_HardwareAssertMaskHigh: c_uint = 0x067f;
pub const mmDF_NCS_PG0_HardwareAssertMaskHigh_BASE_IDX: c_int = 0;
