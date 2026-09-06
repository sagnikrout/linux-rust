//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/ras/core/ras_umc_v12_0.h
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


// SPDX-License-Identifier: MIT
//
// Copyright 2025 Advanced Micro Devices, Inc.
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

// MCA_UMC_UMC0_MCUMC_ADDRT0
pub const MCA_UMC_UMC0_MCUMC_ADDRT0__ErrorAddr__SHIFT: c_uint = 0x0;
pub const MCA_UMC_UMC0_MCUMC_ADDRT0__Reserved__SHIFT: c_uint = 0x38;
pub const MCA_UMC_UMC0_MCUMC_ADDRT0__ErrorAddr_MASK: c_uint = 0x00FFFFFFFFFFFFFFL;
pub const MCA_UMC_UMC0_MCUMC_ADDRT0__Reserved_MASK: c_uint = 0xFF00000000000000L;
// MCMP1_IPIDT0
pub const MCMP1_IPIDT0__InstanceIdLo__SHIFT: c_uint = 0x0;
pub const MCMP1_IPIDT0__HardwareID__SHIFT: c_uint = 0x20;
pub const MCMP1_IPIDT0__InstanceIdHi__SHIFT: c_uint = 0x2c;
pub const MCMP1_IPIDT0__McaType__SHIFT: c_uint = 0x30;
pub const MCMP1_IPIDT0__InstanceIdLo_MASK: c_uint = 0x00000000FFFFFFFFL;
pub const MCMP1_IPIDT0__HardwareID_MASK: c_uint = 0x00000FFF00000000L;
pub const MCMP1_IPIDT0__InstanceIdHi_MASK: c_uint = 0x0000F00000000000L;
pub const MCMP1_IPIDT0__McaType_MASK: c_uint = 0xFFFF000000000000L;
// number of umc channel instance with memory map register access
pub const UMC_V12_0_CHANNEL_INSTANCE_NUM: c_int = 8;
// number of umc instance with memory map register access
pub const UMC_V12_0_UMC_INSTANCE_NUM: c_int = 4;
// one piece of normalized address is mapped to 8 pieces of physical address
pub const UMC_V12_0_NA_MAP_PA_NUM: c_int = 8;
// bank bits in MCA error address
pub const UMC_V12_0_MCA_B0_BIT: c_int = 6;
pub const UMC_V12_0_MCA_B1_BIT: c_int = 7;
pub const UMC_V12_0_MCA_B2_BIT: c_int = 8;
pub const UMC_V12_0_MCA_B3_BIT: c_int = 9;
// row bits in MCA address
pub const UMC_V12_0_MCA_R0_BIT: c_int = 10;
// Stack ID bits in SOC physical address
pub const UMC_V12_0_PA_SID1_BIT: c_int = 37;
pub const UMC_V12_0_PA_SID0_BIT: c_int = 36;
// bank bits in SOC physical address
pub const UMC_V12_0_PA_B3_BIT: c_int = 18;
pub const UMC_V12_0_PA_B2_BIT: c_int = 17;
pub const UMC_V12_0_PA_B1_BIT: c_int = 20;
pub const UMC_V12_0_PA_B0_BIT: c_int = 19;
// row bits in SOC physical address
pub const UMC_V12_0_PA_R13_BIT: c_int = 35;
pub const UMC_V12_0_PA_R12_BIT: c_int = 34;
pub const UMC_V12_0_PA_R11_BIT: c_int = 33;
pub const UMC_V12_0_PA_R10_BIT: c_int = 32;
pub const UMC_V12_0_PA_R9_BIT: c_int = 31;
pub const UMC_V12_0_PA_R8_BIT: c_int = 30;
pub const UMC_V12_0_PA_R7_BIT: c_int = 29;
pub const UMC_V12_0_PA_R6_BIT: c_int = 28;
pub const UMC_V12_0_PA_R5_BIT: c_int = 27;
pub const UMC_V12_0_PA_R4_BIT: c_int = 26;
pub const UMC_V12_0_PA_R3_BIT: c_int = 25;
pub const UMC_V12_0_PA_R2_BIT: c_int = 24;
pub const UMC_V12_0_PA_R1_BIT: c_int = 23;
pub const UMC_V12_0_PA_R0_BIT: c_int = 22;
// column bits in SOC physical address
pub const UMC_V12_0_PA_C4_BIT: c_int = 21;
pub const UMC_V12_0_PA_C3_BIT: c_int = 16;
pub const UMC_V12_0_PA_C2_BIT: c_int = 15;
pub const UMC_V12_0_PA_C1_BIT: c_int = 6;
pub const UMC_V12_0_PA_C0_BIT: c_int = 5;
// channel index bits in SOC physical address
pub const UMC_V12_0_PA_CH6_BIT: c_int = 14;
pub const UMC_V12_0_PA_CH5_BIT: c_int = 13;
pub const UMC_V12_0_PA_CH4_BIT: c_int = 12;
pub const UMC_V12_0_PA_CH3_BIT: c_int = 11;
pub const UMC_V12_0_PA_CH2_BIT: c_int = 10;
pub const UMC_V12_0_PA_CH1_BIT: c_int = 9;
pub const UMC_V12_0_PA_CH0_BIT: c_int = 8;
// Pseudochannel index bits in SOC physical address
pub const UMC_V12_0_PA_PC0_BIT: c_int = 7;
pub const UMC_V12_0_NA_C2_BIT: c_int = 8;

// bank hash settings
pub const UMC_V12_0_XOR_EN0: c_int = 1;
pub const UMC_V12_0_XOR_EN1: c_int = 1;
pub const UMC_V12_0_XOR_EN2: c_int = 1;
pub const UMC_V12_0_XOR_EN3: c_int = 1;
pub const UMC_V12_0_COL_XOR0: c_uint = 0x0;
pub const UMC_V12_0_COL_XOR1: c_uint = 0x0;
pub const UMC_V12_0_COL_XOR2: c_uint = 0x800;
pub const UMC_V12_0_COL_XOR3: c_uint = 0x1000;
pub const UMC_V12_0_ROW_XOR0: c_uint = 0x11111;
pub const UMC_V12_0_ROW_XOR1: c_uint = 0x22222;
pub const UMC_V12_0_ROW_XOR2: c_uint = 0x4444;
pub const UMC_V12_0_ROW_XOR3: c_uint = 0x8888;
// channel hash settings
pub const UMC_V12_0_HASH_4K: c_int = 0;
pub const UMC_V12_0_HASH_64K: c_int = 1;
pub const UMC_V12_0_HASH_2M: c_int = 1;
pub const UMC_V12_0_HASH_1G: c_int = 1;
pub const UMC_V12_0_HASH_1T: c_int = 1;
// XOR some bits of PA into CH4~CH6 bits (bits 12~14 of PA),
// hash bit is only effective when related setting is enabled
//

//
// (addr / 256) * 4096, the higher 26 bits in ErrorAddr
// is the index of 4KB block
//

//
// (addr / 256) * 8192, the higher 26 bits in ErrorAddr
// is the index of 8KB block
//

//
// (addr / 256) * 32768, the higher 26 bits in ErrorAddr
// is the index of 8KB block
//

// channel index is the index of 256B block

// offset in 256B block

// R13 bit shift should be considered, double the number

// UMC register per channel offset
pub const UMC_V12_0_PER_CHANNEL_OFFSET: c_uint = 0x400;
// C2, C3, C4, R13, four MCA bits are looped in page retirement
pub const UMC_V12_0_RETIRE_LOOP_BITS: c_int = 4;
pub const UMC_V12_0_AID_NUM_MAX: c_int = 4;
pub const UMC_V12_0_SOCKET_NUM_MAX: c_int = 8;

// one device has 192GB HBM
pub const SOCKET_LFB_SIZE: c_uint = 0x3000000000ULL;
extern "C" {
    pub fn ras_umc_get_badpage_count(ras_core: *mut ras_core_context) -> c_int;
}
extern "C" {
    pub fn ras_umc_get_badpage_record(ras_core: *mut ras_core_context, index: u32, record: *mut c_void) -> c_int;
}
