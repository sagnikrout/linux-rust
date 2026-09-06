//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dce/dce_mem_input.h
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
// Copyright 2016 Advanced Micro Devices, Inc.
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

// Macro flag: #define TO_DCE_MEM_INPUT(mem_input)\
// Macro flag: #define MI_DCE_BASE_REG_LIST(id)\
// Macro flag: #define MI_DCE_PTE_REG_LIST(id)\

// Macro flag: #define MI_DCE6_REG_LIST(id)\

// Macro flag: #define MI_DCE8_REG_LIST(id)\
// Macro flag: #define MI_DCE11_2_REG_LIST(id)\
// Macro flag: #define MI_DCE11_REG_LIST(id)\
// Macro flag: #define MI_DCE12_REG_LIST(id)\
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dce_mem_input_registers {
// DCP
    pub GRPH_ENABLE: u32,
    pub GRPH_CONTROL: u32,
    pub GRPH_X_START: u32,
    pub GRPH_Y_START: u32,
    pub GRPH_X_END: u32,
    pub GRPH_Y_END: u32,
    pub GRPH_PITCH: u32,
    pub HW_ROTATION: u32,
    pub GRPH_SWAP_CNTL: u32,
    pub PRESCALE_GRPH_CONTROL: u32,
    pub GRPH_PIPE_OUTSTANDING_REQUEST_LIMIT: u32,
    pub DVMM_PTE_CONTROL: u32,
    pub DVMM_PTE_ARB_CONTROL: u32,
    pub GRPH_UPDATE: u32,
    pub GRPH_FLIP_CONTROL: u32,
    pub GRPH_PRIMARY_SURFACE_ADDRESS: u32,
    pub GRPH_PRIMARY_SURFACE_ADDRESS_HIGH: u32,
    pub GRPH_SECONDARY_SURFACE_ADDRESS: u32,
    pub GRPH_SECONDARY_SURFACE_ADDRESS_HIGH: u32,
// DMIF_PG
    pub DPG_PIPE_ARBITRATION_CONTROL1: u32,

    pub DPG_PIPE_ARBITRATION_CONTROL3: u32,

    pub DPG_WATERMARK_MASK_CONTROL: u32,
    pub DPG_PIPE_URGENCY_CONTROL: u32,
    pub DPG_PIPE_URGENT_LEVEL_CONTROL: u32,
    pub DPG_PIPE_NB_PSTATE_CHANGE_CONTROL: u32,
    pub DPG_PIPE_LOW_POWER_CONTROL: u32,
    pub DPG_PIPE_STUTTER_CONTROL: u32,
    pub DPG_PIPE_STUTTER_CONTROL2: u32,
// DCI
    pub DMIF_BUFFER_CONTROL: u32,
// MC_HUB
    pub MC_HUB_RDREQ_DMIF_LIMIT: u32,
// DCHUB
    pub DCHUB_FB_LOCATION: u32,
    pub DCHUB_AGP_BASE: u32,
    pub DCHUB_AGP_BOT: u32,
    pub DCHUB_AGP_TOP: u32,
}

// Set_Filed_for_Block

// Macro flag: #define MI_DCE6_MASK_SH_LIST(mask_sh)\

// Macro flag: #define MI_DCE8_MASK_SH_LIST(mask_sh)\
// Macro flag: #define MI_DCE11_2_MASK_SH_LIST(mask_sh)\
// Macro flag: #define MI_DCE11_MASK_SH_LIST(mask_sh)\

// Macro flag: #define MI_GFX9_DCHUB_MASK_SH_LIST(mask_sh)\
// Macro flag: #define MI_DCE12_MASK_SH_LIST(mask_sh)\

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dce_mem_input_shift {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dce_mem_input_mask {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dce_mem_input_wa {
    pub single_head_rdreq_dmif_limit: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dce_mem_input {
    pub base: mem_input,
    pub regs: *const dce_mem_input_registers,
    pub shifts: *const dce_mem_input_shift,
    pub masks: *const dce_mem_input_mask,
    pub wa: dce_mem_input_wa,
}

