//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/include/asic_reg/mp/mp_10_0_default.h
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

// Macro flag: #define _mp_10_0_DEFAULT_HEADER
// addressBlock: mp_SmuMp0_SmnDec
pub const mmMP0_SMN_C2PMSG_32_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_33_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_34_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_35_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_36_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_37_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_38_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_39_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_40_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_41_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_42_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_43_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_44_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_45_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_46_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_47_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_48_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_49_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_50_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_51_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_52_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_53_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_54_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_55_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_56_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_57_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_58_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_59_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_60_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_61_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_62_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_63_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_64_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_65_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_66_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_67_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_68_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_69_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_70_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_71_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_72_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_73_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_74_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_75_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_76_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_77_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_78_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_79_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_80_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_81_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_82_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_83_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_84_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_85_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_86_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_87_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_88_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_89_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_90_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_91_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_92_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_93_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_94_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_95_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_96_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_97_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_98_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_99_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_100_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_101_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_102_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_C2PMSG_103_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_IH_CREDIT_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_IH_SW_INT_DEFAULT: c_uint = 0x00000000;
pub const mmMP0_SMN_IH_SW_INT_CTRL_DEFAULT: c_uint = 0x00000000;
// addressBlock: mp_SmuMp1_SmnDec
pub const mmMP1_SMN_C2PMSG_32_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_33_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_34_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_35_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_36_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_37_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_38_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_39_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_40_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_41_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_42_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_43_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_44_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_45_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_46_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_47_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_48_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_49_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_50_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_51_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_52_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_53_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_54_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_55_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_56_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_57_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_58_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_59_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_60_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_61_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_62_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_63_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_64_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_65_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_66_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_67_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_68_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_69_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_70_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_71_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_72_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_73_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_74_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_75_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_76_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_77_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_78_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_79_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_80_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_81_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_82_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_83_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_84_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_85_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_86_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_87_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_88_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_89_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_90_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_91_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_92_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_93_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_94_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_95_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_96_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_97_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_98_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_99_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_100_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_101_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_102_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_C2PMSG_103_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_IH_CREDIT_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_IH_SW_INT_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_IH_SW_INT_CTRL_DEFAULT: c_uint = 0x00000000;
pub const mmMP1_SMN_FPS_CNT_DEFAULT: c_uint = 0x00000000;
