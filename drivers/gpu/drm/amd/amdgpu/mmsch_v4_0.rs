//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/mmsch_v4_0.h
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
// Copyright 2022 Advanced Micro Devices, Inc.
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

pub const MMSCH_VERSION_MAJOR: c_int = 4;
pub const MMSCH_VERSION_MINOR: c_int = 0;

pub const MMSCH_VF_ENGINE_STATUS__PASS: c_uint = 0x1;
pub const MMSCH_VF_MAILBOX_RESP__OK: c_uint = 0x1;
pub const MMSCH_VF_MAILBOX_RESP__INCOMPLETE: c_uint = 0x2;
pub const MMSCH_VF_MAILBOX_RESP__FAILED: c_uint = 0x3;
pub const MMSCH_VF_MAILBOX_RESP__FAILED_SMALL_CTX_SIZE: c_uint = 0x4;
pub const MMSCH_VF_MAILBOX_RESP__UNKNOWN_CMD: c_uint = 0x5;
pub const MMSCH_V4_0_VCN_INSTANCES: c_uint = 0x2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mmsch_v4_0_command_type {
    MMSCH_COMMAND__DIRECT_REG_WRITE = 0,
    MMSCH_COMMAND__DIRECT_REG_POLLING = 2,
    MMSCH_COMMAND__DIRECT_REG_READ_MODIFY_WRITE = 3,
    MMSCH_COMMAND__INDIRECT_REG_WRITE = 8,
    MMSCH_COMMAND__END = 0xf
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmsch_v4_0_table_info {
    pub init_status: u32,
    pub table_offset: u32,
    pub table_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmsch_v4_0_init_header {
    pub version: u32,
    pub total_size: u32,
    pub inst: [mmsch_v4_0_table_info; MMSCH_V4_0_VCN_INSTANCES],
    pub jpegdec: mmsch_v4_0_table_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmsch_v4_0_cmd_direct_reg_header {
    pub 28: uint32_t reg_offset :,
    pub 4: uint32_t command_type :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmsch_v4_0_cmd_indirect_reg_header {
    pub 20: uint32_t reg_offset :,
    pub 8: uint32_t reg_idx_space :,
    pub 4: uint32_t command_type :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmsch_v4_0_cmd_direct_write {
    pub cmd_header: mmsch_v4_0_cmd_direct_reg_header,
    pub reg_value: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmsch_v4_0_cmd_direct_read_modify_write {
    pub cmd_header: mmsch_v4_0_cmd_direct_reg_header,
    pub write_data: u32,
    pub mask_value: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmsch_v4_0_cmd_direct_polling {
    pub cmd_header: mmsch_v4_0_cmd_direct_reg_header,
    pub mask_value: u32,
    pub wait_value: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmsch_v4_0_cmd_end {
    pub cmd_header: mmsch_v4_0_cmd_direct_reg_header,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmsch_v4_0_cmd_indirect_write {
    pub cmd_header: mmsch_v4_0_cmd_indirect_reg_header,
    pub reg_value: u32,
}

