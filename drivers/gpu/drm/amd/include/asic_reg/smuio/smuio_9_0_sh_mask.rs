//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/include/asic_reg/smuio/smuio_9_0_sh_mask.h
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
// Copyright (C) 2017  Advanced Micro Devices, Inc.
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

// Macro flag: #define _smuio_9_0_SH_MASK_HEADER
// addressBlock: smuio_smuio_SmuSmuioDec
// ROM_CNTL
pub const ROM_CNTL__CLOCK_GATING_EN__SHIFT: c_uint = 0x0;
pub const ROM_CNTL__CLOCK_GATING_EN_MASK: c_uint = 0x00000001L;
// ROM_STATUS
pub const ROM_STATUS__ROM_BUSY__SHIFT: c_uint = 0x0;
pub const ROM_STATUS__ROM_BUSY_MASK: c_uint = 0x00000001L;
// CGTT_ROM_CLK_CTRL0
pub const CGTT_ROM_CLK_CTRL0__ON_DELAY__SHIFT: c_uint = 0x0;
pub const CGTT_ROM_CLK_CTRL0__OFF_HYSTERESIS__SHIFT: c_uint = 0x4;
pub const CGTT_ROM_CLK_CTRL0__SOFT_OVERRIDE1__SHIFT: c_uint = 0x1e;
pub const CGTT_ROM_CLK_CTRL0__SOFT_OVERRIDE0__SHIFT: c_uint = 0x1f;
pub const CGTT_ROM_CLK_CTRL0__ON_DELAY_MASK: c_uint = 0x0000000FL;
pub const CGTT_ROM_CLK_CTRL0__OFF_HYSTERESIS_MASK: c_uint = 0x00000FF0L;
pub const CGTT_ROM_CLK_CTRL0__SOFT_OVERRIDE1_MASK: c_uint = 0x40000000L;
pub const CGTT_ROM_CLK_CTRL0__SOFT_OVERRIDE0_MASK: c_uint = 0x80000000L;
// ROM_INDEX
pub const ROM_INDEX__ROM_INDEX__SHIFT: c_uint = 0x0;
pub const ROM_INDEX__ROM_INDEX_MASK: c_uint = 0x00FFFFFFL;
// ROM_DATA
pub const ROM_DATA__ROM_DATA__SHIFT: c_uint = 0x0;
pub const ROM_DATA__ROM_DATA_MASK: c_uint = 0xFFFFFFFFL;
// ROM_START
pub const ROM_START__ROM_START__SHIFT: c_uint = 0x0;
pub const ROM_START__ROM_START_MASK: c_uint = 0x00FFFFFFL;
// ROM_SW_CNTL
pub const ROM_SW_CNTL__DATA_SIZE__SHIFT: c_uint = 0x0;
pub const ROM_SW_CNTL__COMMAND_SIZE__SHIFT: c_uint = 0x10;
pub const ROM_SW_CNTL__ROM_SW_RETURN_DATA_ENABLE__SHIFT: c_uint = 0x12;
pub const ROM_SW_CNTL__DATA_SIZE_MASK: c_uint = 0x0000FFFFL;
pub const ROM_SW_CNTL__COMMAND_SIZE_MASK: c_uint = 0x00030000L;
pub const ROM_SW_CNTL__ROM_SW_RETURN_DATA_ENABLE_MASK: c_uint = 0x00040000L;
// ROM_SW_STATUS
pub const ROM_SW_STATUS__ROM_SW_DONE__SHIFT: c_uint = 0x0;
pub const ROM_SW_STATUS__ROM_SW_DONE_MASK: c_uint = 0x00000001L;
// ROM_SW_COMMAND
pub const ROM_SW_COMMAND__ROM_SW_INSTRUCTION__SHIFT: c_uint = 0x0;
pub const ROM_SW_COMMAND__ROM_SW_ADDRESS__SHIFT: c_uint = 0x8;
pub const ROM_SW_COMMAND__ROM_SW_INSTRUCTION_MASK: c_uint = 0x000000FFL;
pub const ROM_SW_COMMAND__ROM_SW_ADDRESS_MASK: c_uint = 0xFFFFFF00L;
// ROM_SW_DATA_1
pub const ROM_SW_DATA_1__ROM_SW_DATA__SHIFT: c_uint = 0x0;
pub const ROM_SW_DATA_1__ROM_SW_DATA_MASK: c_uint = 0xFFFFFFFFL;
// ROM_SW_DATA_2
pub const ROM_SW_DATA_2__ROM_SW_DATA__SHIFT: c_uint = 0x0;
pub const ROM_SW_DATA_2__ROM_SW_DATA_MASK: c_uint = 0xFFFFFFFFL;
// ROM_SW_DATA_3
pub const ROM_SW_DATA_3__ROM_SW_DATA__SHIFT: c_uint = 0x0;
pub const ROM_SW_DATA_3__ROM_SW_DATA_MASK: c_uint = 0xFFFFFFFFL;
// ROM_SW_DATA_4
pub const ROM_SW_DATA_4__ROM_SW_DATA__SHIFT: c_uint = 0x0;
pub const ROM_SW_DATA_4__ROM_SW_DATA_MASK: c_uint = 0xFFFFFFFFL;
// ROM_SW_DATA_5
pub const ROM_SW_DATA_5__ROM_SW_DATA__SHIFT: c_uint = 0x0;
pub const ROM_SW_DATA_5__ROM_SW_DATA_MASK: c_uint = 0xFFFFFFFFL;
// ROM_SW_DATA_6
pub const ROM_SW_DATA_6__ROM_SW_DATA__SHIFT: c_uint = 0x0;
pub const ROM_SW_DATA_6__ROM_SW_DATA_MASK: c_uint = 0xFFFFFFFFL;
// ROM_SW_DATA_7
pub const ROM_SW_DATA_7__ROM_SW_DATA__SHIFT: c_uint = 0x0;
pub const ROM_SW_DATA_7__ROM_SW_DATA_MASK: c_uint = 0xFFFFFFFFL;
// ROM_SW_DATA_8
pub const ROM_SW_DATA_8__ROM_SW_DATA__SHIFT: c_uint = 0x0;
pub const ROM_SW_DATA_8__ROM_SW_DATA_MASK: c_uint = 0xFFFFFFFFL;
// ROM_SW_DATA_9
pub const ROM_SW_DATA_9__ROM_SW_DATA__SHIFT: c_uint = 0x0;
pub const ROM_SW_DATA_9__ROM_SW_DATA_MASK: c_uint = 0xFFFFFFFFL;
// ROM_SW_DATA_10
pub const ROM_SW_DATA_10__ROM_SW_DATA__SHIFT: c_uint = 0x0;
pub const ROM_SW_DATA_10__ROM_SW_DATA_MASK: c_uint = 0xFFFFFFFFL;
// ROM_SW_DATA_11
pub const ROM_SW_DATA_11__ROM_SW_DATA__SHIFT: c_uint = 0x0;
pub const ROM_SW_DATA_11__ROM_SW_DATA_MASK: c_uint = 0xFFFFFFFFL;
// ROM_SW_DATA_12
pub const ROM_SW_DATA_12__ROM_SW_DATA__SHIFT: c_uint = 0x0;
pub const ROM_SW_DATA_12__ROM_SW_DATA_MASK: c_uint = 0xFFFFFFFFL;
// ROM_SW_DATA_13
pub const ROM_SW_DATA_13__ROM_SW_DATA__SHIFT: c_uint = 0x0;
pub const ROM_SW_DATA_13__ROM_SW_DATA_MASK: c_uint = 0xFFFFFFFFL;
// ROM_SW_DATA_14
pub const ROM_SW_DATA_14__ROM_SW_DATA__SHIFT: c_uint = 0x0;
pub const ROM_SW_DATA_14__ROM_SW_DATA_MASK: c_uint = 0xFFFFFFFFL;
// ROM_SW_DATA_15
pub const ROM_SW_DATA_15__ROM_SW_DATA__SHIFT: c_uint = 0x0;
pub const ROM_SW_DATA_15__ROM_SW_DATA_MASK: c_uint = 0xFFFFFFFFL;
// ROM_SW_DATA_16
pub const ROM_SW_DATA_16__ROM_SW_DATA__SHIFT: c_uint = 0x0;
pub const ROM_SW_DATA_16__ROM_SW_DATA_MASK: c_uint = 0xFFFFFFFFL;
// ROM_SW_DATA_17
pub const ROM_SW_DATA_17__ROM_SW_DATA__SHIFT: c_uint = 0x0;
pub const ROM_SW_DATA_17__ROM_SW_DATA_MASK: c_uint = 0xFFFFFFFFL;
// ROM_SW_DATA_18
pub const ROM_SW_DATA_18__ROM_SW_DATA__SHIFT: c_uint = 0x0;
pub const ROM_SW_DATA_18__ROM_SW_DATA_MASK: c_uint = 0xFFFFFFFFL;
// ROM_SW_DATA_19
pub const ROM_SW_DATA_19__ROM_SW_DATA__SHIFT: c_uint = 0x0;
pub const ROM_SW_DATA_19__ROM_SW_DATA_MASK: c_uint = 0xFFFFFFFFL;
// ROM_SW_DATA_20
pub const ROM_SW_DATA_20__ROM_SW_DATA__SHIFT: c_uint = 0x0;
pub const ROM_SW_DATA_20__ROM_SW_DATA_MASK: c_uint = 0xFFFFFFFFL;
// ROM_SW_DATA_21
pub const ROM_SW_DATA_21__ROM_SW_DATA__SHIFT: c_uint = 0x0;
pub const ROM_SW_DATA_21__ROM_SW_DATA_MASK: c_uint = 0xFFFFFFFFL;
// ROM_SW_DATA_22
pub const ROM_SW_DATA_22__ROM_SW_DATA__SHIFT: c_uint = 0x0;
pub const ROM_SW_DATA_22__ROM_SW_DATA_MASK: c_uint = 0xFFFFFFFFL;
// ROM_SW_DATA_23
pub const ROM_SW_DATA_23__ROM_SW_DATA__SHIFT: c_uint = 0x0;
pub const ROM_SW_DATA_23__ROM_SW_DATA_MASK: c_uint = 0xFFFFFFFFL;
// ROM_SW_DATA_24
pub const ROM_SW_DATA_24__ROM_SW_DATA__SHIFT: c_uint = 0x0;
pub const ROM_SW_DATA_24__ROM_SW_DATA_MASK: c_uint = 0xFFFFFFFFL;
// ROM_SW_DATA_25
pub const ROM_SW_DATA_25__ROM_SW_DATA__SHIFT: c_uint = 0x0;
pub const ROM_SW_DATA_25__ROM_SW_DATA_MASK: c_uint = 0xFFFFFFFFL;
// ROM_SW_DATA_26
pub const ROM_SW_DATA_26__ROM_SW_DATA__SHIFT: c_uint = 0x0;
pub const ROM_SW_DATA_26__ROM_SW_DATA_MASK: c_uint = 0xFFFFFFFFL;
// ROM_SW_DATA_27
pub const ROM_SW_DATA_27__ROM_SW_DATA__SHIFT: c_uint = 0x0;
pub const ROM_SW_DATA_27__ROM_SW_DATA_MASK: c_uint = 0xFFFFFFFFL;
// ROM_SW_DATA_28
pub const ROM_SW_DATA_28__ROM_SW_DATA__SHIFT: c_uint = 0x0;
pub const ROM_SW_DATA_28__ROM_SW_DATA_MASK: c_uint = 0xFFFFFFFFL;
// ROM_SW_DATA_29
pub const ROM_SW_DATA_29__ROM_SW_DATA__SHIFT: c_uint = 0x0;
pub const ROM_SW_DATA_29__ROM_SW_DATA_MASK: c_uint = 0xFFFFFFFFL;
// ROM_SW_DATA_30
pub const ROM_SW_DATA_30__ROM_SW_DATA__SHIFT: c_uint = 0x0;
pub const ROM_SW_DATA_30__ROM_SW_DATA_MASK: c_uint = 0xFFFFFFFFL;
// ROM_SW_DATA_31
pub const ROM_SW_DATA_31__ROM_SW_DATA__SHIFT: c_uint = 0x0;
pub const ROM_SW_DATA_31__ROM_SW_DATA_MASK: c_uint = 0xFFFFFFFFL;
// ROM_SW_DATA_32
pub const ROM_SW_DATA_32__ROM_SW_DATA__SHIFT: c_uint = 0x0;
pub const ROM_SW_DATA_32__ROM_SW_DATA_MASK: c_uint = 0xFFFFFFFFL;
// ROM_SW_DATA_33
pub const ROM_SW_DATA_33__ROM_SW_DATA__SHIFT: c_uint = 0x0;
pub const ROM_SW_DATA_33__ROM_SW_DATA_MASK: c_uint = 0xFFFFFFFFL;
// ROM_SW_DATA_34
pub const ROM_SW_DATA_34__ROM_SW_DATA__SHIFT: c_uint = 0x0;
pub const ROM_SW_DATA_34__ROM_SW_DATA_MASK: c_uint = 0xFFFFFFFFL;
// ROM_SW_DATA_35
pub const ROM_SW_DATA_35__ROM_SW_DATA__SHIFT: c_uint = 0x0;
pub const ROM_SW_DATA_35__ROM_SW_DATA_MASK: c_uint = 0xFFFFFFFFL;
// ROM_SW_DATA_36
pub const ROM_SW_DATA_36__ROM_SW_DATA__SHIFT: c_uint = 0x0;
pub const ROM_SW_DATA_36__ROM_SW_DATA_MASK: c_uint = 0xFFFFFFFFL;
// ROM_SW_DATA_37
pub const ROM_SW_DATA_37__ROM_SW_DATA__SHIFT: c_uint = 0x0;
pub const ROM_SW_DATA_37__ROM_SW_DATA_MASK: c_uint = 0xFFFFFFFFL;
// ROM_SW_DATA_38
pub const ROM_SW_DATA_38__ROM_SW_DATA__SHIFT: c_uint = 0x0;
pub const ROM_SW_DATA_38__ROM_SW_DATA_MASK: c_uint = 0xFFFFFFFFL;
// ROM_SW_DATA_39
pub const ROM_SW_DATA_39__ROM_SW_DATA__SHIFT: c_uint = 0x0;
pub const ROM_SW_DATA_39__ROM_SW_DATA_MASK: c_uint = 0xFFFFFFFFL;
// ROM_SW_DATA_40
pub const ROM_SW_DATA_40__ROM_SW_DATA__SHIFT: c_uint = 0x0;
pub const ROM_SW_DATA_40__ROM_SW_DATA_MASK: c_uint = 0xFFFFFFFFL;
// ROM_SW_DATA_41
pub const ROM_SW_DATA_41__ROM_SW_DATA__SHIFT: c_uint = 0x0;
pub const ROM_SW_DATA_41__ROM_SW_DATA_MASK: c_uint = 0xFFFFFFFFL;
// ROM_SW_DATA_42
pub const ROM_SW_DATA_42__ROM_SW_DATA__SHIFT: c_uint = 0x0;
pub const ROM_SW_DATA_42__ROM_SW_DATA_MASK: c_uint = 0xFFFFFFFFL;
// ROM_SW_DATA_43
pub const ROM_SW_DATA_43__ROM_SW_DATA__SHIFT: c_uint = 0x0;
pub const ROM_SW_DATA_43__ROM_SW_DATA_MASK: c_uint = 0xFFFFFFFFL;
// ROM_SW_DATA_44
pub const ROM_SW_DATA_44__ROM_SW_DATA__SHIFT: c_uint = 0x0;
pub const ROM_SW_DATA_44__ROM_SW_DATA_MASK: c_uint = 0xFFFFFFFFL;
// ROM_SW_DATA_45
pub const ROM_SW_DATA_45__ROM_SW_DATA__SHIFT: c_uint = 0x0;
pub const ROM_SW_DATA_45__ROM_SW_DATA_MASK: c_uint = 0xFFFFFFFFL;
// ROM_SW_DATA_46
pub const ROM_SW_DATA_46__ROM_SW_DATA__SHIFT: c_uint = 0x0;
pub const ROM_SW_DATA_46__ROM_SW_DATA_MASK: c_uint = 0xFFFFFFFFL;
// ROM_SW_DATA_47
pub const ROM_SW_DATA_47__ROM_SW_DATA__SHIFT: c_uint = 0x0;
pub const ROM_SW_DATA_47__ROM_SW_DATA_MASK: c_uint = 0xFFFFFFFFL;
// ROM_SW_DATA_48
pub const ROM_SW_DATA_48__ROM_SW_DATA__SHIFT: c_uint = 0x0;
pub const ROM_SW_DATA_48__ROM_SW_DATA_MASK: c_uint = 0xFFFFFFFFL;
// ROM_SW_DATA_49
pub const ROM_SW_DATA_49__ROM_SW_DATA__SHIFT: c_uint = 0x0;
pub const ROM_SW_DATA_49__ROM_SW_DATA_MASK: c_uint = 0xFFFFFFFFL;
// ROM_SW_DATA_50
pub const ROM_SW_DATA_50__ROM_SW_DATA__SHIFT: c_uint = 0x0;
pub const ROM_SW_DATA_50__ROM_SW_DATA_MASK: c_uint = 0xFFFFFFFFL;
// ROM_SW_DATA_51
pub const ROM_SW_DATA_51__ROM_SW_DATA__SHIFT: c_uint = 0x0;
pub const ROM_SW_DATA_51__ROM_SW_DATA_MASK: c_uint = 0xFFFFFFFFL;
// ROM_SW_DATA_52
pub const ROM_SW_DATA_52__ROM_SW_DATA__SHIFT: c_uint = 0x0;
pub const ROM_SW_DATA_52__ROM_SW_DATA_MASK: c_uint = 0xFFFFFFFFL;
// ROM_SW_DATA_53
pub const ROM_SW_DATA_53__ROM_SW_DATA__SHIFT: c_uint = 0x0;
pub const ROM_SW_DATA_53__ROM_SW_DATA_MASK: c_uint = 0xFFFFFFFFL;
// ROM_SW_DATA_54
pub const ROM_SW_DATA_54__ROM_SW_DATA__SHIFT: c_uint = 0x0;
pub const ROM_SW_DATA_54__ROM_SW_DATA_MASK: c_uint = 0xFFFFFFFFL;
// ROM_SW_DATA_55
pub const ROM_SW_DATA_55__ROM_SW_DATA__SHIFT: c_uint = 0x0;
pub const ROM_SW_DATA_55__ROM_SW_DATA_MASK: c_uint = 0xFFFFFFFFL;
// ROM_SW_DATA_56
pub const ROM_SW_DATA_56__ROM_SW_DATA__SHIFT: c_uint = 0x0;
pub const ROM_SW_DATA_56__ROM_SW_DATA_MASK: c_uint = 0xFFFFFFFFL;
// ROM_SW_DATA_57
pub const ROM_SW_DATA_57__ROM_SW_DATA__SHIFT: c_uint = 0x0;
pub const ROM_SW_DATA_57__ROM_SW_DATA_MASK: c_uint = 0xFFFFFFFFL;
// ROM_SW_DATA_58
pub const ROM_SW_DATA_58__ROM_SW_DATA__SHIFT: c_uint = 0x0;
pub const ROM_SW_DATA_58__ROM_SW_DATA_MASK: c_uint = 0xFFFFFFFFL;
// ROM_SW_DATA_59
pub const ROM_SW_DATA_59__ROM_SW_DATA__SHIFT: c_uint = 0x0;
pub const ROM_SW_DATA_59__ROM_SW_DATA_MASK: c_uint = 0xFFFFFFFFL;
// ROM_SW_DATA_60
pub const ROM_SW_DATA_60__ROM_SW_DATA__SHIFT: c_uint = 0x0;
pub const ROM_SW_DATA_60__ROM_SW_DATA_MASK: c_uint = 0xFFFFFFFFL;
// ROM_SW_DATA_61
pub const ROM_SW_DATA_61__ROM_SW_DATA__SHIFT: c_uint = 0x0;
pub const ROM_SW_DATA_61__ROM_SW_DATA_MASK: c_uint = 0xFFFFFFFFL;
// ROM_SW_DATA_62
pub const ROM_SW_DATA_62__ROM_SW_DATA__SHIFT: c_uint = 0x0;
pub const ROM_SW_DATA_62__ROM_SW_DATA_MASK: c_uint = 0xFFFFFFFFL;
// ROM_SW_DATA_63
pub const ROM_SW_DATA_63__ROM_SW_DATA__SHIFT: c_uint = 0x0;
pub const ROM_SW_DATA_63__ROM_SW_DATA_MASK: c_uint = 0xFFFFFFFFL;
// ROM_SW_DATA_64
pub const ROM_SW_DATA_64__ROM_SW_DATA__SHIFT: c_uint = 0x0;
pub const ROM_SW_DATA_64__ROM_SW_DATA_MASK: c_uint = 0xFFFFFFFFL;
// SMUSVI0_PLANE0_CURRENTVID
pub const SMUSVI0_PLANE0_CURRENTVID__CURRENT_SVI0_PLANE0_VID__SHIFT: c_uint = 0x18;
pub const SMUSVI0_PLANE0_CURRENTVID__CURRENT_SVI0_PLANE0_VID_MASK: c_uint = 0xFF000000L;
pub const SMUSVI0_TEL_PLANE0__SVI0_PLANE0_VDDCOR__SHIFT: c_uint = 0x10;
pub const SMUSVI0_TEL_PLANE0__SVI0_PLANE0_VDDCOR_MASK: c_uint = 0x01FF0000L;
