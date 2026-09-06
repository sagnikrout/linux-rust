//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/bios/command_table_helper_struct.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct command_table_helper {
    pub atom_id): *mut *mut bool (controller_id_to_atom)(enum controller_id id, uint8_t,
    pub action): bp_encoder_control_action,
    pub enable_dp_audio): bool,
    pub atom_engine_id): *mut u32,
    pub ctrl_param): *mut _DIG_ENCODER_CONTROL_PARAMETERS_V2,
    pub atom_pll_id): *mut u32,
    pub ref_clk_src_id): *mut u32,
    pub t): *mut *mut uint8_t (transmitter_bp_to_atom)(enum transmitter,
    pub id): *mut *mut uint8_t (encoder_id_to_atom)(enum encoder_id,
    pub id): clock_source_id,
    pub s): *mut *mut uint8_t (signal_type_to_atom_dig_mode)(enum signal_type,
    pub id): *mut *mut uint8_t (hpd_sel_to_atom)(enum hpd_source_id,
    pub engine_id): *mut *mut uint8_t (dig_encoder_sel_to_atom)(enum engine_id,
    pub t): *mut *mut uint8_t (phy_id_to_atom)(enum transmitter,
    pub action): bp_pipe_control_action,
    pub atom_clock_type): *mut u32,
    pub id): *mut *mut uint8_t (transmitter_color_depth_to_atom)(enum transmitter_color_depth,
}
