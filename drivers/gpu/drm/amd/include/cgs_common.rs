//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/include/cgs_common.h
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
// Copyright 2015 Advanced Micro Devices, Inc.
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

//
// enum cgs_ind_reg - Indirect register spaces
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cgs_ind_reg {
    CGS_IND_REG__PCIE,
    CGS_IND_REG__SMC,
    CGS_IND_REG__UVD_CTX,
    CGS_IND_REG__DIDT,
    CGS_IND_REG_GC_CAC,
    CGS_IND_REG_SE_CAC,
    CGS_IND_REG__AUDIO_ENDPT
}

//
// enum cgs_ucode_id - Firmware types for different IPs
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cgs_ucode_id {
    CGS_UCODE_ID_SMU = 0,
    CGS_UCODE_ID_SMU_SK,
    CGS_UCODE_ID_SDMA0,
    CGS_UCODE_ID_SDMA1,
    CGS_UCODE_ID_CP_CE,
    CGS_UCODE_ID_CP_PFP,
    CGS_UCODE_ID_CP_ME,
    CGS_UCODE_ID_CP_MEC,
    CGS_UCODE_ID_CP_MEC_JT1,
    CGS_UCODE_ID_CP_MEC_JT2,
    CGS_UCODE_ID_GMCON_RENG,
    CGS_UCODE_ID_RLC_G,
    CGS_UCODE_ID_STORAGE,
    CGS_UCODE_ID_MAXIMUM,
}

//
// struct cgs_firmware_info - Firmware information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgs_firmware_info {
    pub version: u16,
    pub fw_version: u16,
    pub feature_version: u16,
    pub image_size: u32,
    pub mc_addr: u64,
// only for smc firmware
    pub ucode_start_address: u32,
    pub kptr: *mut c_void,
    pub is_kicker: bool,
}

pub type cgs_handle_t = c_ulong;
//
// cgs_read_register() - Read an MMIO register
// @cgs_device:	opaque device handle
// @offset:	register offset
//
// Return:  register value
//
extern "C" {
    pub fn uint32_t(cgs_device: *mut *mut cgs_read_register_t)(struct cgs_device, offset: unsigned) -> typedef;
}
//
// cgs_write_register() - Write an MMIO register
// @cgs_device:	opaque device handle
// @offset:	register offset
// @value:	register value
//
// cgs_read_ind_register() - Read an indirect register
// @cgs_device:	opaque device handle
// @offset:	register offset
//
// Return:  register value
//
// cgs_write_ind_register() - Write an indirect register
// @cgs_device:	opaque device handle
// @offset:	register offset
// @value:	register value
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgs_ops {
// MMIO access
    pub read_register: cgs_read_register_t,
    pub write_register: cgs_write_register_t,
    pub read_ind_register: cgs_read_ind_register_t,
    pub write_ind_register: cgs_write_ind_register_t,
// Firmware Info
    pub get_firmware_info: cgs_get_firmware_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgs_device {
    pub ops: *const cgs_ops,
// to be embedded at the start of driver private structure
}

// Convenience macros that make CGS indirect function calls look like
// normal function calls

