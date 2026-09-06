//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/include/asic_reg/dpcs/dpcs_2_0_3_offset.h
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
// Copyright (C) 2021 Advanced Micro Devices, Inc.
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

// Macro flag: #define _dpcs_2_0_3_OFFSET_HEADER
// addressBlock: dpcssysa_dpcs0_dpcstx0_dispdec
// base address: 0x0
pub const mmDPCSTX0_DPCSTX_TX_CLOCK_CNTL: c_uint = 0x2928;
pub const mmDPCSTX0_DPCSTX_TX_CLOCK_CNTL_BASE_IDX: c_int = 2;
pub const mmDPCSTX0_DPCSTX_TX_CNTL: c_uint = 0x2929;
pub const mmDPCSTX0_DPCSTX_TX_CNTL_BASE_IDX: c_int = 2;
pub const mmDPCSTX0_DPCSTX_CBUS_CNTL: c_uint = 0x292a;
pub const mmDPCSTX0_DPCSTX_CBUS_CNTL_BASE_IDX: c_int = 2;
pub const mmDPCSTX0_DPCSTX_INTERRUPT_CNTL: c_uint = 0x292b;
pub const mmDPCSTX0_DPCSTX_INTERRUPT_CNTL_BASE_IDX: c_int = 2;
pub const mmDPCSTX0_DPCSTX_PLL_UPDATE_ADDR: c_uint = 0x292c;
pub const mmDPCSTX0_DPCSTX_PLL_UPDATE_ADDR_BASE_IDX: c_int = 2;
pub const mmDPCSTX0_DPCSTX_PLL_UPDATE_DATA: c_uint = 0x292d;
pub const mmDPCSTX0_DPCSTX_PLL_UPDATE_DATA_BASE_IDX: c_int = 2;
// addressBlock: dpcssysa_dpcs0_rdpcstx0_dispdec
// base address: 0x0
pub const mmRDPCSTX0_RDPCSTX_CNTL: c_uint = 0x2930;
pub const mmRDPCSTX0_RDPCSTX_CNTL_BASE_IDX: c_int = 2;
pub const mmRDPCSTX0_RDPCSTX_CLOCK_CNTL: c_uint = 0x2931;
pub const mmRDPCSTX0_RDPCSTX_CLOCK_CNTL_BASE_IDX: c_int = 2;
pub const mmRDPCSTX0_RDPCSTX_INTERRUPT_CONTROL: c_uint = 0x2932;
pub const mmRDPCSTX0_RDPCSTX_INTERRUPT_CONTROL_BASE_IDX: c_int = 2;
pub const mmRDPCSTX0_RDPCSTX_PLL_UPDATE_DATA: c_uint = 0x2933;
pub const mmRDPCSTX0_RDPCSTX_PLL_UPDATE_DATA_BASE_IDX: c_int = 2;
pub const mmRDPCSTX0_RDPCS_TX_CR_ADDR: c_uint = 0x2934;
pub const mmRDPCSTX0_RDPCS_TX_CR_ADDR_BASE_IDX: c_int = 2;
pub const mmRDPCSTX0_RDPCS_TX_CR_DATA: c_uint = 0x2935;
pub const mmRDPCSTX0_RDPCS_TX_CR_DATA_BASE_IDX: c_int = 2;
pub const mmRDPCSTX0_RDPCSTX_SCRATCH: c_uint = 0x2939;
pub const mmRDPCSTX0_RDPCSTX_SCRATCH_BASE_IDX: c_int = 2;
pub const mmRDPCSTX0_RDPCSTX_PHY_CNTL0: c_uint = 0x2940;
pub const mmRDPCSTX0_RDPCSTX_PHY_CNTL0_BASE_IDX: c_int = 2;
pub const mmRDPCSTX0_RDPCSTX_PHY_CNTL1: c_uint = 0x2941;
pub const mmRDPCSTX0_RDPCSTX_PHY_CNTL1_BASE_IDX: c_int = 2;
pub const mmRDPCSTX0_RDPCSTX_PHY_CNTL2: c_uint = 0x2942;
pub const mmRDPCSTX0_RDPCSTX_PHY_CNTL2_BASE_IDX: c_int = 2;
pub const mmRDPCSTX0_RDPCSTX_PHY_CNTL3: c_uint = 0x2943;
pub const mmRDPCSTX0_RDPCSTX_PHY_CNTL3_BASE_IDX: c_int = 2;
pub const mmRDPCSTX0_RDPCSTX_PHY_CNTL4: c_uint = 0x2944;
pub const mmRDPCSTX0_RDPCSTX_PHY_CNTL4_BASE_IDX: c_int = 2;
pub const mmRDPCSTX0_RDPCSTX_PHY_CNTL5: c_uint = 0x2945;
pub const mmRDPCSTX0_RDPCSTX_PHY_CNTL5_BASE_IDX: c_int = 2;
pub const mmRDPCSTX0_RDPCSTX_PHY_CNTL6: c_uint = 0x2946;
pub const mmRDPCSTX0_RDPCSTX_PHY_CNTL6_BASE_IDX: c_int = 2;
pub const mmRDPCSTX0_RDPCSTX_PHY_CNTL7: c_uint = 0x2947;
pub const mmRDPCSTX0_RDPCSTX_PHY_CNTL7_BASE_IDX: c_int = 2;
pub const mmRDPCSTX0_RDPCSTX_PHY_CNTL8: c_uint = 0x2948;
pub const mmRDPCSTX0_RDPCSTX_PHY_CNTL8_BASE_IDX: c_int = 2;
pub const mmRDPCSTX0_RDPCSTX_PHY_CNTL9: c_uint = 0x2949;
pub const mmRDPCSTX0_RDPCSTX_PHY_CNTL9_BASE_IDX: c_int = 2;
pub const mmRDPCSTX0_RDPCSTX_PHY_CNTL10: c_uint = 0x294a;
pub const mmRDPCSTX0_RDPCSTX_PHY_CNTL10_BASE_IDX: c_int = 2;
pub const mmRDPCSTX0_RDPCSTX_PHY_CNTL11: c_uint = 0x294b;
pub const mmRDPCSTX0_RDPCSTX_PHY_CNTL11_BASE_IDX: c_int = 2;
pub const mmRDPCSTX0_RDPCSTX_PHY_CNTL12: c_uint = 0x294c;
pub const mmRDPCSTX0_RDPCSTX_PHY_CNTL12_BASE_IDX: c_int = 2;
pub const mmRDPCSTX0_RDPCSTX_PHY_CNTL13: c_uint = 0x294d;
pub const mmRDPCSTX0_RDPCSTX_PHY_CNTL13_BASE_IDX: c_int = 2;
pub const mmRDPCSTX0_RDPCSTX_PHY_CNTL14: c_uint = 0x294e;
pub const mmRDPCSTX0_RDPCSTX_PHY_CNTL14_BASE_IDX: c_int = 2;
// addressBlock: dpcssysa_dpcs0_dpcstx1_dispdec
// base address: 0x360
pub const mmDPCSTX1_DPCSTX_TX_CLOCK_CNTL: c_uint = 0x2a00;
pub const mmDPCSTX1_DPCSTX_TX_CLOCK_CNTL_BASE_IDX: c_int = 2;
pub const mmDPCSTX1_DPCSTX_TX_CNTL: c_uint = 0x2a01;
pub const mmDPCSTX1_DPCSTX_TX_CNTL_BASE_IDX: c_int = 2;
pub const mmDPCSTX1_DPCSTX_CBUS_CNTL: c_uint = 0x2a02;
pub const mmDPCSTX1_DPCSTX_CBUS_CNTL_BASE_IDX: c_int = 2;
pub const mmDPCSTX1_DPCSTX_INTERRUPT_CNTL: c_uint = 0x2a03;
pub const mmDPCSTX1_DPCSTX_INTERRUPT_CNTL_BASE_IDX: c_int = 2;
pub const mmDPCSTX1_DPCSTX_PLL_UPDATE_ADDR: c_uint = 0x2a04;
pub const mmDPCSTX1_DPCSTX_PLL_UPDATE_ADDR_BASE_IDX: c_int = 2;
pub const mmDPCSTX1_DPCSTX_PLL_UPDATE_DATA: c_uint = 0x2a05;
pub const mmDPCSTX1_DPCSTX_PLL_UPDATE_DATA_BASE_IDX: c_int = 2;
// addressBlock: dpcssysa_dpcs0_rdpcstx1_dispdec
// base address: 0x360
pub const mmRDPCSTX1_RDPCSTX_CNTL: c_uint = 0x2a08;
pub const mmRDPCSTX1_RDPCSTX_CNTL_BASE_IDX: c_int = 2;
pub const mmRDPCSTX1_RDPCSTX_CLOCK_CNTL: c_uint = 0x2a09;
pub const mmRDPCSTX1_RDPCSTX_CLOCK_CNTL_BASE_IDX: c_int = 2;
pub const mmRDPCSTX1_RDPCSTX_INTERRUPT_CONTROL: c_uint = 0x2a0a;
pub const mmRDPCSTX1_RDPCSTX_INTERRUPT_CONTROL_BASE_IDX: c_int = 2;
pub const mmRDPCSTX1_RDPCSTX_PLL_UPDATE_DATA: c_uint = 0x2a0b;
pub const mmRDPCSTX1_RDPCSTX_PLL_UPDATE_DATA_BASE_IDX: c_int = 2;
pub const mmRDPCSTX1_RDPCS_TX_CR_ADDR: c_uint = 0x2a0c;
pub const mmRDPCSTX1_RDPCS_TX_CR_ADDR_BASE_IDX: c_int = 2;
pub const mmRDPCSTX1_RDPCS_TX_CR_DATA: c_uint = 0x2a0d;
pub const mmRDPCSTX1_RDPCS_TX_CR_DATA_BASE_IDX: c_int = 2;
pub const mmRDPCSTX1_RDPCSTX_SCRATCH: c_uint = 0x2a11;
pub const mmRDPCSTX1_RDPCSTX_SCRATCH_BASE_IDX: c_int = 2;
pub const mmRDPCSTX1_RDPCSTX_PHY_CNTL0: c_uint = 0x2a18;
pub const mmRDPCSTX1_RDPCSTX_PHY_CNTL0_BASE_IDX: c_int = 2;
pub const mmRDPCSTX1_RDPCSTX_PHY_CNTL1: c_uint = 0x2a19;
pub const mmRDPCSTX1_RDPCSTX_PHY_CNTL1_BASE_IDX: c_int = 2;
pub const mmRDPCSTX1_RDPCSTX_PHY_CNTL2: c_uint = 0x2a1a;
pub const mmRDPCSTX1_RDPCSTX_PHY_CNTL2_BASE_IDX: c_int = 2;
pub const mmRDPCSTX1_RDPCSTX_PHY_CNTL3: c_uint = 0x2a1b;
pub const mmRDPCSTX1_RDPCSTX_PHY_CNTL3_BASE_IDX: c_int = 2;
pub const mmRDPCSTX1_RDPCSTX_PHY_CNTL4: c_uint = 0x2a1c;
pub const mmRDPCSTX1_RDPCSTX_PHY_CNTL4_BASE_IDX: c_int = 2;
pub const mmRDPCSTX1_RDPCSTX_PHY_CNTL5: c_uint = 0x2a1d;
pub const mmRDPCSTX1_RDPCSTX_PHY_CNTL5_BASE_IDX: c_int = 2;
pub const mmRDPCSTX1_RDPCSTX_PHY_CNTL6: c_uint = 0x2a1e;
pub const mmRDPCSTX1_RDPCSTX_PHY_CNTL6_BASE_IDX: c_int = 2;
pub const mmRDPCSTX1_RDPCSTX_PHY_CNTL7: c_uint = 0x2a1f;
pub const mmRDPCSTX1_RDPCSTX_PHY_CNTL7_BASE_IDX: c_int = 2;
pub const mmRDPCSTX1_RDPCSTX_PHY_CNTL8: c_uint = 0x2a20;
pub const mmRDPCSTX1_RDPCSTX_PHY_CNTL8_BASE_IDX: c_int = 2;
pub const mmRDPCSTX1_RDPCSTX_PHY_CNTL9: c_uint = 0x2a21;
pub const mmRDPCSTX1_RDPCSTX_PHY_CNTL9_BASE_IDX: c_int = 2;
pub const mmRDPCSTX1_RDPCSTX_PHY_CNTL10: c_uint = 0x2a22;
pub const mmRDPCSTX1_RDPCSTX_PHY_CNTL10_BASE_IDX: c_int = 2;
pub const mmRDPCSTX1_RDPCSTX_PHY_CNTL11: c_uint = 0x2a23;
pub const mmRDPCSTX1_RDPCSTX_PHY_CNTL11_BASE_IDX: c_int = 2;
pub const mmRDPCSTX1_RDPCSTX_PHY_CNTL12: c_uint = 0x2a24;
pub const mmRDPCSTX1_RDPCSTX_PHY_CNTL12_BASE_IDX: c_int = 2;
pub const mmRDPCSTX1_RDPCSTX_PHY_CNTL13: c_uint = 0x2a25;
pub const mmRDPCSTX1_RDPCSTX_PHY_CNTL13_BASE_IDX: c_int = 2;
pub const mmRDPCSTX1_RDPCSTX_PHY_CNTL14: c_uint = 0x2a26;
pub const mmRDPCSTX1_RDPCSTX_PHY_CNTL14_BASE_IDX: c_int = 2;
