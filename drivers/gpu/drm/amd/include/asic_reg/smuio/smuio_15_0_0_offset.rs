//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/include/asic_reg/smuio/smuio_15_0_0_offset.h
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
// Copyright (C) 2025  Advanced Micro Devices, Inc.
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

// Macro flag: #define _smuio_15_0_0_OFFSET_HEADER
// addressBlock: smuio_smuio_misc_SmuSmuioDec
// base address: 0x5a000
pub const regSMUIO_MCM_CONFIG: c_uint = 0x0023;
pub const regSMUIO_MCM_CONFIG_BASE_IDX: c_int = 0;
pub const regIP_DISCOVERY_VERSION: c_uint = 0x0000;
pub const regIP_DISCOVERY_VERSION_BASE_IDX: c_int = 1;
pub const regSCRATCH_REGISTER0: c_uint = 0x01c6;
pub const regSCRATCH_REGISTER0_BASE_IDX: c_int = 1;
pub const regSCRATCH_REGISTER1: c_uint = 0x01c7;
pub const regSCRATCH_REGISTER1_BASE_IDX: c_int = 1;
pub const regSCRATCH_REGISTER2: c_uint = 0x01c8;
pub const regSCRATCH_REGISTER2_BASE_IDX: c_int = 1;
pub const regSCRATCH_REGISTER3: c_uint = 0x01c9;
pub const regSCRATCH_REGISTER3_BASE_IDX: c_int = 1;
pub const regSCRATCH_REGISTER4: c_uint = 0x01ca;
pub const regSCRATCH_REGISTER4_BASE_IDX: c_int = 1;
pub const regSCRATCH_REGISTER5: c_uint = 0x01cb;
pub const regSCRATCH_REGISTER5_BASE_IDX: c_int = 1;
pub const regSCRATCH_REGISTER6: c_uint = 0x01cc;
pub const regSCRATCH_REGISTER6_BASE_IDX: c_int = 1;
pub const regSCRATCH_REGISTER7: c_uint = 0x01cd;
pub const regSCRATCH_REGISTER7_BASE_IDX: c_int = 1;
pub const regIO_SMUIO_PINSTRAP: c_uint = 0x01ce;
pub const regIO_SMUIO_PINSTRAP_BASE_IDX: c_int = 1;
// addressBlock: smuio_smuio_reset_SmuSmuioDec
// base address: 0x5a300
pub const regSMUIO_GFX_MISC_CNTL: c_uint = 0x00c5;
pub const regSMUIO_GFX_MISC_CNTL_BASE_IDX: c_int = 0;
// addressBlock: smuio_smuio_tsc_SmuSmuioDec
// base address: 0x5a8a0
pub const regPWROK_REFCLK_GAP_CYCLES: c_uint = 0x0028;
pub const regPWROK_REFCLK_GAP_CYCLES_BASE_IDX: c_int = 1;
pub const regGOLDEN_TSC_INCREMENT_UPPER: c_uint = 0x002b;
pub const regGOLDEN_TSC_INCREMENT_UPPER_BASE_IDX: c_int = 1;
pub const regGOLDEN_TSC_INCREMENT_LOWER: c_uint = 0x002c;
pub const regGOLDEN_TSC_INCREMENT_LOWER_BASE_IDX: c_int = 1;
pub const regGOLDEN_TSC_COUNT_UPPER: c_uint = 0x0030;
pub const regGOLDEN_TSC_COUNT_UPPER_BASE_IDX: c_int = 1;
pub const regGOLDEN_TSC_COUNT_LOWER: c_uint = 0x0031;
pub const regGOLDEN_TSC_COUNT_LOWER_BASE_IDX: c_int = 1;
pub const regSOC_GOLDEN_TSC_SHADOW_UPPER: c_uint = 0x0032;
pub const regSOC_GOLDEN_TSC_SHADOW_UPPER_BASE_IDX: c_int = 1;
pub const regSOC_GOLDEN_TSC_SHADOW_LOWER: c_uint = 0x0033;
pub const regSOC_GOLDEN_TSC_SHADOW_LOWER_BASE_IDX: c_int = 1;
pub const regSOC_GAP_PWROK: c_uint = 0x0034;
pub const regSOC_GAP_PWROK_BASE_IDX: c_int = 1;
// addressBlock: smuio_smuio_swtimer_SmuSmuioDec
// base address: 0x5aca8
pub const regPWR_VIRT_RESET_REQ: c_uint = 0x012a;
pub const regPWR_VIRT_RESET_REQ_BASE_IDX: c_int = 1;
pub const regPWR_DISP_TIMER_CONTROL: c_uint = 0x012b;
pub const regPWR_DISP_TIMER_CONTROL_BASE_IDX: c_int = 1;
pub const regPWR_DISP_TIMER_DEBUG: c_uint = 0x012c;
pub const regPWR_DISP_TIMER_DEBUG_BASE_IDX: c_int = 1;
pub const regPWR_DISP_TIMER_ELAPSED_CONTROL: c_uint = 0x012d;
pub const regPWR_DISP_TIMER_ELAPSED_CONTROL_BASE_IDX: c_int = 1;
pub const regPWR_DISP_TIMER2_CONTROL: c_uint = 0x012e;
pub const regPWR_DISP_TIMER2_CONTROL_BASE_IDX: c_int = 1;
pub const regPWR_DISP_TIMER2_DEBUG: c_uint = 0x012f;
pub const regPWR_DISP_TIMER2_DEBUG_BASE_IDX: c_int = 1;
pub const regPWR_DISP_TIMER2_ELAPSED_CONTROL: c_uint = 0x0130;
pub const regPWR_DISP_TIMER2_ELAPSED_CONTROL_BASE_IDX: c_int = 1;
pub const regPWR_DISP_TIMER_GLOBAL_CONTROL: c_uint = 0x0131;
pub const regPWR_DISP_TIMER_GLOBAL_CONTROL_BASE_IDX: c_int = 1;
pub const regPWR_IH_CONTROL: c_uint = 0x0132;
pub const regPWR_IH_CONTROL_BASE_IDX: c_int = 1;
