//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dce/dce_aux.h
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

// Macro flag: #define AUX_COMMON_REG_LIST0(id)\
// Macro flag: #define AUX_COMMON_REG_LIST(id)\
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dce110_aux_registers {
    pub AUX_CONTROL: u32,
    pub AUX_ARB_CONTROL: u32,
    pub AUX_SW_DATA: u32,
    pub AUX_SW_CONTROL: u32,
    pub AUX_INTERRUPT_CONTROL: u32,
    pub AUX_DPHY_RX_CONTROL1: u32,
    pub AUX_SW_STATUS: u32,
    pub AUXN_IMPCAL: u32,
    pub AUXP_IMPCAL: u32,
    pub AUX_RESET_MASK: u32,
}

// Macro flag: #define DCE_AUX_REG_FIELD_LIST(type)\
// Macro flag: #define DCE10_AUX_MASK_SH_LIST(mask_sh)\
// Macro flag: #define DCE_AUX_MASK_SH_LIST(mask_sh)\
// Macro flag: #define DCE12_AUX_MASK_SH_LIST(mask_sh)\
// DCN10 MASK
// Macro flag: #define DCN10_AUX_MASK_SH_LIST(mask_sh)\
// for all other DCN
// Macro flag: #define DCN_AUX_MASK_SH_LIST(mask_sh)\

// 2.3.4 "Detailed uPacket TX AUX CH State Description".
//
// Ideally, the SW timeout should be just above 550usec
// which is programmed in HW.
// But the SW timeout of 600usec is not reliable,
// because on some systems, delay_in_microseconds()
// returns faster than it should.
// EPR #379763: by trial-and-error on different systems,
// 700usec is the minimum reliable SW timeout for polling
// the AUX_SW_STATUS.AUX_SW_DONE bit.
// This timeout expires *only* when there is
// AUX Error or AUX Timeout conditions - not during normal operation.
// During normal operation, AUX_SW_STATUS.AUX_SW_DONE bit is set
// at most within ~240usec. That means,
// increasing this timeout will not affect normal operation,
// and we'll timeout after
// SW_AUX_TIMEOUT_PERIOD_MULTIPLIER * AUX_TIMEOUT_PERIOD = 2400usec.
// This timeout is especially important for
// converters, resume from S3, and CTS.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dce_aux {
    pub inst: u32,
    pub ddc: *mut ddc,
    pub ctx: *mut dc_context,
// following values are expressed in milliseconds
    pub delay: u32,
    pub max_defer_write_retry: u32,
    pub acquire_reset: bool,
    pub funcs: *mut dce_aux_funcs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dce110_aux_registers_mask {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dce110_aux_registers_shift {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aux_engine_dce110 {
    pub base: dce_aux,
    pub regs: *const dce110_aux_registers,
    pub mask: *const dce110_aux_registers_mask,
    pub shift: *const dce110_aux_registers_shift,
    pub aux_control: u32,
    pub aux_arb_control: u32,
    pub aux_sw_data: u32,
    pub aux_sw_control: u32,
    pub aux_interrupt_control: u32,
    pub aux_dphy_rx_control1: u32,
    pub aux_dphy_rx_control0: u32,
    pub aux_sw_status: u32,
    pub addr: },
    pub polling_timeout_period: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aux_engine_dce110_init_data {
    pub engine_id: u32,
    pub timeout_period: u32,
    pub ctx: *mut dc_context,
    pub regs: *const dce110_aux_registers,
}

extern "C" {
    pub fn dce110_engine_destroy(engine: *mut dce_aux);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dce_aux_funcs {
    pub timeout): u32,
    pub ptr): *mut (struct aux_engine,
}
