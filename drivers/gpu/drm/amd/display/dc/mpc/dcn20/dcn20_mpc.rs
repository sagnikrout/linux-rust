//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/mpc/dcn20/dcn20_mpc.h
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


// Copyright 2012-15 Advanced Micro Devices, Inc.
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
// Authors: AMD
//

// Macro flag: #define MPC_REG_LIST_DCN2_0(inst)\

//
// DCN2 MPC_OCSC debug status register:
//
// Status index including current OCSC Mode is 1
// OCSC Mode: [1..0]
//
pub const MPC_OCSC_TEST_DEBUG_DATA_STATUS_IDX: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn20_mpc_registers {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn20_mpc_shift {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn20_mpc_mask {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn20_mpc {
    pub base: mpc,
    pub mpcc_in_use_mask: c_int,
    pub num_mpcc: c_int,
    pub mpc_regs: *const dcn20_mpc_registers,
    pub mpc_shift: *const dcn20_mpc_shift,
    pub mpc_mask: *const dcn20_mpc_mask,
}

extern "C" {
    pub fn mpc2_assert_idle_mpcc(mpc: *mut mpc, id: c_int);
}
extern "C" {
    pub fn mpc2_assert_mpcc_idle_before_connect(mpc: *mut mpc, mpcc_id: c_int);
}
extern "C" {
    pub fn mpc20_power_on_ogam_lut(mpc: *mut mpc, mpcc_id: c_int, power_on: bool);
}
