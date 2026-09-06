//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/powerplay/inc/smu_ucode_xfer_vi.h
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
// Copyright 2014 Advanced Micro Devices, Inc.
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
pub const SMU_DRAMData_TOC_VERSION: c_int = 1;
pub const MAX_IH_REGISTER_COUNT: c_int = 65535;
pub const SMU_DIGEST_SIZE_BYTES: c_int = 20;
pub const SMU_FB_SIZE_BYTES: c_int = 1048576;
pub const SMU_MAX_ENTRIES: c_int = 12;
pub const UCODE_ID_SMU: c_int = 0;
pub const UCODE_ID_SDMA0: c_int = 1;
pub const UCODE_ID_SDMA1: c_int = 2;
pub const UCODE_ID_CP_CE: c_int = 3;
pub const UCODE_ID_CP_PFP: c_int = 4;
pub const UCODE_ID_CP_ME: c_int = 5;
pub const UCODE_ID_CP_MEC: c_int = 6;
pub const UCODE_ID_CP_MEC_JT1: c_int = 7;
pub const UCODE_ID_CP_MEC_JT2: c_int = 8;
pub const UCODE_ID_GMCON_RENG: c_int = 9;
pub const UCODE_ID_RLC_G: c_int = 10;
pub const UCODE_ID_IH_REG_RESTORE: c_int = 11;
pub const UCODE_ID_VBIOS: c_int = 12;
pub const UCODE_ID_MISC_METADATA: c_int = 13;
pub const UCODE_ID_SMU_SK: c_int = 14;
pub const UCODE_ID_RLC_SCRATCH: c_int = 32;
pub const UCODE_ID_RLC_SRM_ARAM: c_int = 33;
pub const UCODE_ID_RLC_SRM_DRAM: c_int = 34;
pub const UCODE_ID_MEC_STORAGE: c_int = 35;
pub const UCODE_ID_VBIOS_PARAMETERS: c_int = 36;
pub const UCODE_META_DATA: c_uint = 0xFF;
pub const UCODE_ID_SMU_MASK: c_uint = 0x00000001;
pub const UCODE_ID_SDMA0_MASK: c_uint = 0x00000002;
pub const UCODE_ID_SDMA1_MASK: c_uint = 0x00000004;
pub const UCODE_ID_CP_CE_MASK: c_uint = 0x00000008;
pub const UCODE_ID_CP_PFP_MASK: c_uint = 0x00000010;
pub const UCODE_ID_CP_ME_MASK: c_uint = 0x00000020;
pub const UCODE_ID_CP_MEC_MASK: c_uint = 0x00000040;
pub const UCODE_ID_CP_MEC_JT1_MASK: c_uint = 0x00000080;
pub const UCODE_ID_CP_MEC_JT2_MASK: c_uint = 0x00000100;
pub const UCODE_ID_GMCON_RENG_MASK: c_uint = 0x00000200;
pub const UCODE_ID_RLC_G_MASK: c_uint = 0x00000400;
pub const UCODE_ID_IH_REG_RESTORE_MASK: c_uint = 0x00000800;
pub const UCODE_ID_VBIOS_MASK: c_uint = 0x00001000;
pub const UCODE_FLAG_UNHALT_MASK: c_uint = 0x1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU_Entry {
    pub id: u16,
    pub version: u16,
    pub image_addr_high: u32,
    pub image_addr_low: u32,
    pub meta_data_addr_high: u32,
    pub meta_data_addr_low: u32,
    pub data_size_byte: u32,
    pub flags: u16,
    pub num_register_entries: u16,

    pub version: u16,
    pub id: u16,
    pub image_addr_high: u32,
    pub image_addr_low: u32,
    pub meta_data_addr_high: u32,
    pub meta_data_addr_low: u32,
    pub data_size_byte: u32,
    pub num_register_entries: u16,
    pub flags: u16,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMU_DRAMData_TOC {
    pub structure_version: u32,
    pub num_entries: u32,
    pub entry: [SMU_Entry; SMU_MAX_ENTRIES],
}
