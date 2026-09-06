//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/include/asic_reg/nbio/nbio_7_4_0_smn.h
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
// Copyright (C) 2019  Advanced Micro Devices, Inc.
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

// Macro flag: #define _nbio_7_4_0_SMN_HEADER
// addressBlock: nbio_nbif0_bif_ras_bif_ras_regblk
// base address: 0x10100000
pub const smnBIFL_RAS_CENTRAL_STATUS: c_uint = 0x10139040;
pub const smnNBIF_MGCG_CTRL_LCLK: c_uint = 0x1013a21c;
pub const smnCPM_CONTROL: c_uint = 0x11180460;
pub const smnPCIE_CNTL2: c_uint = 0x11180070;
pub const smnPCIE_CI_CNTL: c_uint = 0x11180080;
pub const smnPCIE_PERF_COUNT_CNTL: c_uint = 0x11180200;
pub const smnPCIE_PERF_CNTL_TXCLK1: c_uint = 0x11180204;
pub const smnPCIE_PERF_COUNT0_TXCLK1: c_uint = 0x11180208;
pub const smnPCIE_PERF_COUNT1_TXCLK1: c_uint = 0x1118020c;
pub const smnPCIE_PERF_CNTL_TXCLK2: c_uint = 0x11180210;
pub const smnPCIE_PERF_COUNT0_TXCLK2: c_uint = 0x11180214;
pub const smnPCIE_PERF_COUNT1_TXCLK2: c_uint = 0x11180218;
pub const smnPCIE_PERF_CNTL_TXCLK3: c_uint = 0x1118021c;
pub const smnPCIE_PERF_COUNT0_TXCLK3: c_uint = 0x11180220;
pub const smnPCIE_PERF_COUNT1_TXCLK3: c_uint = 0x11180224;
pub const smnPCIE_PERF_CNTL_TXCLK4: c_uint = 0x11180228;
pub const smnPCIE_PERF_COUNT0_TXCLK4: c_uint = 0x1118022c;
pub const smnPCIE_PERF_COUNT1_TXCLK4: c_uint = 0x11180230;
pub const smnPCIE_PERF_CNTL_SCLK1: c_uint = 0x11180234;
pub const smnPCIE_PERF_COUNT0_SCLK1: c_uint = 0x11180238;
pub const smnPCIE_PERF_COUNT1_SCLK1: c_uint = 0x1118023c;
pub const smnPCIE_PERF_CNTL_SCLK2: c_uint = 0x11180240;
pub const smnPCIE_PERF_COUNT0_SCLK2: c_uint = 0x11180244;
pub const smnPCIE_PERF_COUNT1_SCLK2: c_uint = 0x11180248;
pub const smnPCIE_PERF_CNTL_EVENT_LC_PORT_SEL: c_uint = 0x1118024c;
pub const smnPCIE_PERF_CNTL_EVENT_CI_PORT_SEL: c_uint = 0x11180250;
pub const smnPCIE_RX_NUM_NAK: c_uint = 0x11180038;
pub const smnPCIE_RX_NUM_NAK_GENERATED: c_uint = 0x1118003c;
// addressBlock: nbio_iohub_nb_misc_misc_cfgdec
// base address: 0x13a10000
pub const smnIOHC_INTERRUPT_EOI: c_uint = 0x13a10120;
// addressBlock: nbio_iohub_nb_rascfg_ras_cfgdec
// base address: 0x13a20000
pub const smnRAS_GLOBAL_STATUS_LO: c_uint = 0x13a20020;
pub const smnRAS_GLOBAL_STATUS_HI: c_uint = 0x13a20024;
