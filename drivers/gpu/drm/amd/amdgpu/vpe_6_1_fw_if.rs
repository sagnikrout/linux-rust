//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/vpe_6_1_fw_if.h
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


// Copyright 2023 Advanced Micro Devices, Inc.
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
// VPE OP Codes
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum VPE_CMD_OPCODE {
    VPE_CMD_OPCODE_NOP          = 0x0,
    VPE_CMD_OPCODE_VPE_DESC     = 0x1,
    VPE_CMD_OPCODE_PLANE_CFG    = 0x2,
    VPE_CMD_OPCODE_VPEP_CFG     = 0x3,
    VPE_CMD_OPCODE_INDIRECT     = 0x4,
    VPE_CMD_OPCODE_FENCE        = 0x5,
    VPE_CMD_OPCODE_TRAP         = 0x6,
    VPE_CMD_OPCODE_REG_WRITE    = 0x7,
    VPE_CMD_OPCODE_POLL_REGMEM  = 0x8,
    VPE_CMD_OPCODE_COND_EXE     = 0x9,
    VPE_CMD_OPCODE_ATOMIC       = 0xA,
    VPE_CMD_OPCODE_PRED_EXE     = 0xB,
    VPE_CMD_OPCODE_COLLAB_SYNC  = 0xC,
    VPE_CMD_OPCODE_TIMESTAMP    = 0xD
}

// Generic Command Header
// Generic Commands include:
// Noop, Fence, Trap,
// RegisterWrite, PollRegisterWriteMemory,
// SetLocalTimestamp, GetLocalTimestamp
// GetGlobalGPUTimestamp
pub const VPE_HEADER_SUB_OPCODE__SHIFT: c_int = 8;
pub const VPE_HEADER_SUB_OPCODE_MASK: c_uint = 0x0000FF00;
pub const VPE_HEADER_OPCODE__SHIFT: c_int = 0;
pub const VPE_HEADER_OPCODE_MASK: c_uint = 0x000000FF;

//
// VPE NOP
//
pub const VPE_CMD_NOP_HEADER_COUNT__SHIFT: c_int = 16;
pub const VPE_CMD_NOP_HEADER_COUNT_MASK: c_uint = 0x00003FFF;

//
// VPE Descriptor
//
pub const VPE_DESC_CD__SHIFT: c_int = 16;
pub const VPE_DESC_CD_MASK: c_uint = 0x000F0000;

//
// VPE Plane Config
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum VPE_PLANE_CFG_SUBOP {
    VPE_PLANE_CFG_SUBOP_1_TO_1 = 0x0,
    VPE_PLANE_CFG_SUBOP_2_TO_1 = 0x1,
    VPE_PLANE_CFG_SUBOP_2_TO_2 = 0x2
}

pub const VPE_PLANE_CFG_ONE_PLANE: c_int = 0;
pub const VPE_PLANE_CFG_TWO_PLANES: c_int = 1;
pub const VPE_PLANE_CFG_NPS0__SHIFT: c_int = 16;
pub const VPE_PLANE_CFG_NPS0_MASK: c_uint = 0x00030000;
pub const VPE_PLANE_CFG_NPD0__SHIFT: c_int = 18;
pub const VPE_PLANE_CFG_NPD0_MASK: c_uint = 0x000C0000;
pub const VPE_PLANE_CFG_NPS1__SHIFT: c_int = 20;
pub const VPE_PLANE_CFG_NPS1_MASK: c_uint = 0x00300000;
pub const VPE_PLANE_CFG_NPD1__SHIFT: c_int = 22;
pub const VPE_PLANE_CFG_NPD1_MASK: c_uint = 0x00C00000;
pub const VPE_PLANE_CFG_TMZ__SHIFT: c_int = 16;
pub const VPE_PLANE_CFG_TMZ_MASK: c_uint = 0x00010000;
pub const VPE_PLANE_CFG_SWIZZLE_MODE__SHIFT: c_int = 3;
pub const VPE_PLANE_CFG_SWIZZLE_MODE_MASK: c_uint = 0x000000F8;
pub const VPE_PLANE_CFG_ROTATION__SHIFT: c_int = 0;
pub const VPE_PLANE_CFG_ROTATION_MASK: c_uint = 0x00000003;
pub const VPE_PLANE_ADDR_LO__SHIFT: c_int = 0;
pub const VPE_PLANE_ADDR_LO_MASK: c_uint = 0xFFFFFF00;
pub const VPE_PLANE_CFG_PITCH__SHIFT: c_int = 0;
pub const VPE_PLANE_CFG_PITCH_MASK: c_uint = 0x00003FFF;
pub const VPE_PLANE_CFG_VIEWPORT_Y__SHIFT: c_int = 16;
pub const VPE_PLANE_CFG_VIEWPORT_Y_MASK: c_uint = 0x3FFF0000;
pub const VPE_PLANE_CFG_VIEWPORT_X__SHIFT: c_int = 0;
pub const VPE_PLANE_CFG_VIEWPORT_X_MASK: c_uint = 0x00003FFF;
pub const VPE_PLANE_CFG_VIEWPORT_HEIGHT__SHIFT: c_int = 16;
pub const VPE_PLANE_CFG_VIEWPORT_HEIGHT_MASK: c_uint = 0x1FFF0000;
pub const VPE_PLANE_CFG_VIEWPORT_ELEMENT_SIZE__SHIFT: c_int = 13;
pub const VPE_PLANE_CFG_VIEWPORT_ELEMENT_SIZE_MASK: c_uint = 0x0000E000;
pub const VPE_PLANE_CFG_VIEWPORT_WIDTH__SHIFT: c_int = 0;
pub const VPE_PLANE_CFG_VIEWPORT_WIDTH_MASK: c_uint = 0x00001FFF;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum VPE_PLANE_CFG_ELEMENT_SIZE {
    VPE_PLANE_CFG_ELEMENT_SIZE_8BPE     = 0,
    VPE_PLANE_CFG_ELEMENT_SIZE_16BPE    = 1,
    VPE_PLANE_CFG_ELEMENT_SIZE_32BPE    = 2,
    VPE_PLANE_CFG_ELEMENT_SIZE_64BPE    = 3
}

//
// VPEP Config
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum VPE_VPEP_CFG_SUBOP {
    VPE_VPEP_CFG_SUBOP_DIR_CFG = 0x0,
    VPE_VPEP_CFG_SUBOP_IND_CFG = 0x1
}

// Direct Config Command Header
pub const VPE_DIR_CFG_HEADER_ARRAY_SIZE__SHIFT: c_int = 16;
pub const VPE_DIR_CFG_HEADER_ARRAY_SIZE_MASK: c_uint = 0xFFFF0000;

pub const VPE_DIR_CFG_PKT_REGISTER_OFFSET__SHIFT: c_int = 2;
pub const VPE_DIR_CFG_PKT_REGISTER_OFFSET_MASK: c_uint = 0x000FFFFC;
pub const VPE_DIR_CFG_PKT_DATA_SIZE__SHIFT: c_int = 20;
pub const VPE_DIR_CFG_PKT_DATA_SIZE_MASK: c_uint = 0xFFF00000;
// InDirect Config Command Header
pub const VPE_IND_CFG_HEADER_NUM_DST__SHIFT: c_int = 28;
pub const VPE_IND_CFG_HEADER_NUM_DST_MASK: c_uint = 0xF0000000;

// Indirect Buffer Command Header
pub const VPE_CMD_INDIRECT_HEADER_VMID__SHIFT: c_int = 16;
pub const VPE_CMD_INDIRECT_HEADER_VMID_MASK: c_uint = 0x0000000F;

//
// Poll Reg/Mem Sub-OpCode
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum VPE_POLL_REGMEM_SUBOP {
    VPE_POLL_REGMEM_SUBOP_REGMEM = 0x0,
    VPE_POLL_REGMEM_SUBOP_REGMEM_WRITE = 0x1
}

pub const VPE_CMD_POLL_REGMEM_HEADER_FUNC__SHIFT: c_int = 28;
pub const VPE_CMD_POLL_REGMEM_HEADER_FUNC_MASK: c_uint = 0x00000007;

pub const VPE_CMD_POLL_REGMEM_HEADER_MEM__SHIFT: c_int = 31;
pub const VPE_CMD_POLL_REGMEM_HEADER_MEM_MASK: c_uint = 0x00000001;

pub const VPE_CMD_POLL_REGMEM_DW5_INTERVAL__SHIFT: c_int = 0;
pub const VPE_CMD_POLL_REGMEM_DW5_INTERVAL_MASK: c_uint = 0x0000FFFF;

pub const VPE_CMD_POLL_REGMEM_DW5_RETRY_COUNT__SHIFT: c_int = 16;
pub const VPE_CMD_POLL_REGMEM_DW5_RETRY_COUNT_MASK: c_uint = 0x00000FFF;

