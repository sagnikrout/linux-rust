//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/jpeg_v5_0_1.h
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
// Copyright 2024 Advanced Micro Devices, Inc.
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
pub const regUVD_JRBC0_UVD_JRBC_SCRATCH0_INTERNAL_OFFSET: c_uint = 0x4094;
pub const regUVD_JRBC_EXTERNAL_MCM_ADDR_INTERNAL_OFFSET: c_uint = 0x1bffe;
pub const regUVD_JRBC0_UVD_JRBC_RB_WPTR: c_uint = 0x0640;
pub const regUVD_JRBC0_UVD_JRBC_RB_WPTR_BASE_IDX: c_int = 1;
pub const regUVD_JRBC0_UVD_JRBC_STATUS: c_uint = 0x0649;
pub const regUVD_JRBC0_UVD_JRBC_STATUS_BASE_IDX: c_int = 1;
pub const regUVD_JRBC0_UVD_JRBC_RB_RPTR: c_uint = 0x064a;
pub const regUVD_JRBC0_UVD_JRBC_RB_RPTR_BASE_IDX: c_int = 1;
pub const regUVD_JRBC1_UVD_JRBC_RB_WPTR: c_uint = 0x0000;
pub const regUVD_JRBC1_UVD_JRBC_RB_WPTR_BASE_IDX: c_int = 0;
pub const regUVD_JRBC1_UVD_JRBC_STATUS: c_uint = 0x0009;
pub const regUVD_JRBC1_UVD_JRBC_STATUS_BASE_IDX: c_int = 0;
pub const regUVD_JRBC1_UVD_JRBC_RB_RPTR: c_uint = 0x000a;
pub const regUVD_JRBC1_UVD_JRBC_RB_RPTR_BASE_IDX: c_int = 0;
pub const regUVD_JRBC2_UVD_JRBC_RB_WPTR: c_uint = 0x0040;
pub const regUVD_JRBC2_UVD_JRBC_RB_WPTR_BASE_IDX: c_int = 0;
pub const regUVD_JRBC2_UVD_JRBC_STATUS: c_uint = 0x0049;
pub const regUVD_JRBC2_UVD_JRBC_STATUS_BASE_IDX: c_int = 0;
pub const regUVD_JRBC2_UVD_JRBC_RB_RPTR: c_uint = 0x004a;
pub const regUVD_JRBC2_UVD_JRBC_RB_RPTR_BASE_IDX: c_int = 0;
pub const regUVD_JRBC3_UVD_JRBC_RB_WPTR: c_uint = 0x0080;
pub const regUVD_JRBC3_UVD_JRBC_RB_WPTR_BASE_IDX: c_int = 0;
pub const regUVD_JRBC3_UVD_JRBC_STATUS: c_uint = 0x0089;
pub const regUVD_JRBC3_UVD_JRBC_STATUS_BASE_IDX: c_int = 0;
pub const regUVD_JRBC3_UVD_JRBC_RB_RPTR: c_uint = 0x008a;
pub const regUVD_JRBC3_UVD_JRBC_RB_RPTR_BASE_IDX: c_int = 0;
pub const regUVD_JRBC4_UVD_JRBC_RB_WPTR: c_uint = 0x00c0;
pub const regUVD_JRBC4_UVD_JRBC_RB_WPTR_BASE_IDX: c_int = 0;
pub const regUVD_JRBC4_UVD_JRBC_STATUS: c_uint = 0x00c9;
pub const regUVD_JRBC4_UVD_JRBC_STATUS_BASE_IDX: c_int = 0;
pub const regUVD_JRBC4_UVD_JRBC_RB_RPTR: c_uint = 0x00ca;
pub const regUVD_JRBC4_UVD_JRBC_RB_RPTR_BASE_IDX: c_int = 0;
pub const regUVD_JRBC5_UVD_JRBC_RB_WPTR: c_uint = 0x0100;
pub const regUVD_JRBC5_UVD_JRBC_RB_WPTR_BASE_IDX: c_int = 0;
pub const regUVD_JRBC5_UVD_JRBC_STATUS: c_uint = 0x0109;
pub const regUVD_JRBC5_UVD_JRBC_STATUS_BASE_IDX: c_int = 0;
pub const regUVD_JRBC5_UVD_JRBC_RB_RPTR: c_uint = 0x010a;
pub const regUVD_JRBC5_UVD_JRBC_RB_RPTR_BASE_IDX: c_int = 0;
pub const regUVD_JRBC6_UVD_JRBC_RB_WPTR: c_uint = 0x0140;
pub const regUVD_JRBC6_UVD_JRBC_RB_WPTR_BASE_IDX: c_int = 0;
pub const regUVD_JRBC6_UVD_JRBC_STATUS: c_uint = 0x0149;
pub const regUVD_JRBC6_UVD_JRBC_STATUS_BASE_IDX: c_int = 0;
pub const regUVD_JRBC6_UVD_JRBC_RB_RPTR: c_uint = 0x014a;
pub const regUVD_JRBC6_UVD_JRBC_RB_RPTR_BASE_IDX: c_int = 0;
pub const regUVD_JRBC7_UVD_JRBC_RB_WPTR: c_uint = 0x0180;
pub const regUVD_JRBC7_UVD_JRBC_RB_WPTR_BASE_IDX: c_int = 0;
pub const regUVD_JRBC7_UVD_JRBC_STATUS: c_uint = 0x0189;
pub const regUVD_JRBC7_UVD_JRBC_STATUS_BASE_IDX: c_int = 0;
pub const regUVD_JRBC7_UVD_JRBC_RB_RPTR: c_uint = 0x018a;
pub const regUVD_JRBC7_UVD_JRBC_RB_RPTR_BASE_IDX: c_int = 0;
pub const regUVD_JRBC8_UVD_JRBC_RB_WPTR: c_uint = 0x01c0;
pub const regUVD_JRBC8_UVD_JRBC_RB_WPTR_BASE_IDX: c_int = 0;
pub const regUVD_JRBC8_UVD_JRBC_STATUS: c_uint = 0x01c9;
pub const regUVD_JRBC8_UVD_JRBC_STATUS_BASE_IDX: c_int = 0;
pub const regUVD_JRBC8_UVD_JRBC_RB_RPTR: c_uint = 0x01ca;
pub const regUVD_JRBC8_UVD_JRBC_RB_RPTR_BASE_IDX: c_int = 0;
pub const regUVD_JRBC9_UVD_JRBC_RB_WPTR: c_uint = 0x0440;
pub const regUVD_JRBC9_UVD_JRBC_RB_WPTR_BASE_IDX: c_int = 1;
pub const regUVD_JRBC9_UVD_JRBC_STATUS: c_uint = 0x0449;
pub const regUVD_JRBC9_UVD_JRBC_STATUS_BASE_IDX: c_int = 1;
pub const regUVD_JRBC9_UVD_JRBC_RB_RPTR: c_uint = 0x044a;
pub const regUVD_JRBC9_UVD_JRBC_RB_RPTR_BASE_IDX: c_int = 1;
pub const regUVD_JMI0_JPEG_LMI_DROP: c_uint = 0x0663;
pub const regUVD_JMI0_JPEG_LMI_DROP_BASE_IDX: c_int = 1;
pub const regUVD_JMI0_UVD_JMI_CLIENT_STALL: c_uint = 0x067a;
pub const regUVD_JMI0_UVD_JMI_CLIENT_STALL_BASE_IDX: c_int = 1;
pub const regUVD_JMI0_UVD_JMI_CLIENT_CLEAN_STATUS: c_uint = 0x067b;
pub const regUVD_JMI0_UVD_JMI_CLIENT_CLEAN_STATUS_BASE_IDX: c_int = 1;
pub const regJPEG_CORE_RST_CTRL: c_uint = 0x072e;
pub const regJPEG_CORE_RST_CTRL_BASE_IDX: c_int = 1;
pub const regVCN_RRMT_CNTL: c_uint = 0x0940;
pub const regVCN_RRMT_CNTL_BASE_IDX: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_jpeg_v5_0_1_sub_block {
    AMDGPU_JPEG_V5_0_1_JPEG0 = 0,
    AMDGPU_JPEG_V5_0_1_JPEG1,

    AMDGPU_JPEG_V5_0_1_MAX_SUB_BLOCK,
}
