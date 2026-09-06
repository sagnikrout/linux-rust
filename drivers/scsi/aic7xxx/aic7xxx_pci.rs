//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/aic7xxx/aic7xxx_pci.h
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
// Adaptec AIC7xxx device driver for Linux.
//
// Copyright (c) 2000-2001 Adaptec Inc.
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
// 1. Redistributions of source code must retain the above copyright
// notice, this list of conditions, and the following disclaimer,
// without modification.
// 2. Redistributions in binary form must reproduce at minimum a disclaimer
// substantially similar to the "NO WARRANTY" disclaimer below
// ("Disclaimer") and any redistribution must be conditioned upon
// including a substantially similar Disclaimer requirement for further
// binary redistribution.
// 3. Neither the names of the above-listed copyright holders nor the names
// of any contributors may be used to endorse or promote products derived
// from this software without specific prior written permission.
//
// Alternatively, this software may be distributed under the terms of the
// GNU General Public License ("GPL") version 2 as published by the Free
// Software Foundation.
//
// NO WARRANTY
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTIBILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// HOLDERS OR CONTRIBUTORS BE LIABLE FOR SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
// OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
// HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
// STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING
// IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
// POSSIBILITY OF SUCH DAMAGES.
//
// $Id$
//
pub const ID_ALL_MASK: c_uint = 0xFFFFFFFFFFFFFFFFull;
pub const ID_DEV_VENDOR_MASK: c_uint = 0xFFFFFFFF00000000ull;
pub const ID_9005_GENERIC_MASK: c_uint = 0xFFF0FFFF00000000ull;
pub const ID_9005_SISL_MASK: c_uint = 0x000FFFFF00000000ull;
pub const ID_9005_SISL_ID: c_uint = 0x0005900500000000ull;
pub const ID_AIC7850: c_uint = 0x5078900400000000ull;
pub const ID_AHA_2902_04_10_15_20C_30C: c_uint = 0x5078900478509004ull;
pub const ID_AIC7855: c_uint = 0x5578900400000000ull;
pub const ID_AIC7859: c_uint = 0x3860900400000000ull;
pub const ID_AHA_2930CU: c_uint = 0x3860900438699004ull;
pub const ID_AIC7860: c_uint = 0x6078900400000000ull;
pub const ID_AIC7860C: c_uint = 0x6078900478609004ull;
pub const ID_AHA_1480A: c_uint = 0x6075900400000000ull;
pub const ID_AHA_2940AU_0: c_uint = 0x6178900400000000ull;
pub const ID_AHA_2940AU_1: c_uint = 0x6178900478619004ull;
pub const ID_AHA_2940AU_CN: c_uint = 0x2178900478219004ull;
pub const ID_AHA_2930C_VAR: c_uint = 0x6038900438689004ull;
pub const ID_AIC7870: c_uint = 0x7078900400000000ull;
pub const ID_AHA_2940: c_uint = 0x7178900400000000ull;
pub const ID_AHA_3940: c_uint = 0x7278900400000000ull;
pub const ID_AHA_398X: c_uint = 0x7378900400000000ull;
pub const ID_AHA_2944: c_uint = 0x7478900400000000ull;
pub const ID_AHA_3944: c_uint = 0x7578900400000000ull;
pub const ID_AHA_4944: c_uint = 0x7678900400000000ull;
pub const ID_AIC7880: c_uint = 0x8078900400000000ull;
pub const ID_AIC7880_B: c_uint = 0x8078900478809004ull;
pub const ID_AHA_2940U: c_uint = 0x8178900400000000ull;
pub const ID_AHA_3940U: c_uint = 0x8278900400000000ull;
pub const ID_AHA_2944U: c_uint = 0x8478900400000000ull;
pub const ID_AHA_3944U: c_uint = 0x8578900400000000ull;
pub const ID_AHA_398XU: c_uint = 0x8378900400000000ull;
pub const ID_AHA_4944U: c_uint = 0x8678900400000000ull;
pub const ID_AHA_2940UB: c_uint = 0x8178900478819004ull;
pub const ID_AHA_2930U: c_uint = 0x8878900478889004ull;
pub const ID_AHA_2940U_PRO: c_uint = 0x8778900478879004ull;
pub const ID_AHA_2940U_CN: c_uint = 0x0078900478009004ull;
pub const ID_AIC7895: c_uint = 0x7895900478959004ull;
pub const ID_AIC7895_ARO: c_uint = 0x7890900478939004ull;
pub const ID_AIC7895_ARO_MASK: c_uint = 0xFFF0FFFFFFFFFFFFull;
pub const ID_AHA_2940U_DUAL: c_uint = 0x7895900478919004ull;
pub const ID_AHA_3940AU: c_uint = 0x7895900478929004ull;
pub const ID_AHA_3944AU: c_uint = 0x7895900478949004ull;
pub const ID_AIC7890: c_uint = 0x001F9005000F9005ull;
pub const ID_AIC7890_ARO: c_uint = 0x00139005000F9005ull;
pub const ID_AAA_131U2: c_uint = 0x0013900500039005ull;
pub const ID_AHA_2930U2: c_uint = 0x0011900501819005ull;
pub const ID_AHA_2940U2B: c_uint = 0x00109005A1009005ull;
pub const ID_AHA_2940U2_OEM: c_uint = 0x0010900521809005ull;
pub const ID_AHA_2940U2: c_uint = 0x00109005A1809005ull;
pub const ID_AHA_2950U2B: c_uint = 0x00109005E1009005ull;
pub const ID_AIC7892: c_uint = 0x008F9005FFFF9005ull;
pub const ID_AIC7892_ARO: c_uint = 0x00839005FFFF9005ull;
pub const ID_AHA_29160: c_uint = 0x00809005E2A09005ull;
pub const ID_AHA_29160_CPQ: c_uint = 0x00809005E2A00E11ull;
pub const ID_AHA_29160N: c_uint = 0x0080900562A09005ull;
pub const ID_AHA_29160C: c_uint = 0x0080900562209005ull;
pub const ID_AHA_29160B: c_uint = 0x00809005E2209005ull;
pub const ID_AHA_19160B: c_uint = 0x0081900562A19005ull;
pub const ID_AHA_2915_30LP: c_uint = 0x0082900502109005ull;
pub const ID_AIC7896: c_uint = 0x005F9005FFFF9005ull;
pub const ID_AIC7896_ARO: c_uint = 0x00539005FFFF9005ull;
pub const ID_AHA_3950U2B_0: c_uint = 0x00509005FFFF9005ull;
pub const ID_AHA_3950U2B_1: c_uint = 0x00509005F5009005ull;
pub const ID_AHA_3950U2D_0: c_uint = 0x00519005FFFF9005ull;
pub const ID_AHA_3950U2D_1: c_uint = 0x00519005B5009005ull;
pub const ID_AIC7899: c_uint = 0x00CF9005FFFF9005ull;
pub const ID_AIC7899_ARO: c_uint = 0x00C39005FFFF9005ull;
pub const ID_AHA_3960D: c_uint = 0x00C09005F6209005ull;
pub const ID_AHA_3960D_CPQ: c_uint = 0x00C09005F6200E11ull;
pub const ID_AIC7810: c_uint = 0x1078900400000000ull;
pub const ID_AIC7815: c_uint = 0x7815900400000000ull;
