//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/include/asic_reg/uvd/uvd_3_1_d.h
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
// UVD_3_1 Register documentation
//
// Copyright (C) 2020  Advanced Micro Devices, Inc.
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
pub const mmUVD_SEMA_ADDR_LOW: c_uint = 0x3bc0;
pub const mmUVD_SEMA_ADDR_HIGH: c_uint = 0x3bc1;
pub const mmUVD_SEMA_CMD: c_uint = 0x3bc2;
pub const mmUVD_GPCOM_VCPU_CMD: c_uint = 0x3bc3;
pub const mmUVD_GPCOM_VCPU_DATA0: c_uint = 0x3bc4;
pub const mmUVD_GPCOM_VCPU_DATA1: c_uint = 0x3bc5;
pub const mmUVD_ENGINE_CNTL: c_uint = 0x3bc6;
pub const mmUVD_UDEC_ADDR_CONFIG: c_uint = 0x3bd3;
pub const mmUVD_UDEC_DB_ADDR_CONFIG: c_uint = 0x3bd4;
pub const mmUVD_UDEC_DBW_ADDR_CONFIG: c_uint = 0x3bd5;
pub const mmUVD_NO_OP: c_uint = 0x3bff;
pub const mmUVD_SEMA_CNTL: c_uint = 0x3d00;
pub const mmUVD_LMI_EXT40_ADDR: c_uint = 0x3d26;
pub const mmUVD_CTX_INDEX: c_uint = 0x3d28;
pub const mmUVD_CTX_DATA: c_uint = 0x3d29;
pub const mmUVD_CGC_GATE: c_uint = 0x3d2a;
pub const mmUVD_CGC_STATUS: c_uint = 0x3d2b;
pub const mmUVD_CGC_CTRL: c_uint = 0x3d2c;
pub const mmUVD_CGC_UDEC_STATUS: c_uint = 0x3d2d;
pub const mmUVD_LMI_CTRL2: c_uint = 0x3d3d;
pub const mmUVD_MASTINT_EN: c_uint = 0x3d40;
pub const mmUVD_FW_START: c_uint = 0x3d47;
pub const mmUVD_FW_STATUS: c_uint = 0x3d57;
pub const mmUVD_LMI_ADDR_EXT: c_uint = 0x3d65;
pub const mmUVD_LMI_CTRL: c_uint = 0x3d66;
pub const mmUVD_LMI_STATUS: c_uint = 0x3d67;
pub const mmUVD_LMI_SWAP_CNTL: c_uint = 0x3d6d;
pub const mmUVD_MP_SWAP_CNTL: c_uint = 0x3d6f;
pub const mmUVD_MPC_CNTL: c_uint = 0x3d77;
pub const mmUVD_MPC_SET_MUXA0: c_uint = 0x3d79;
pub const mmUVD_MPC_SET_MUXA1: c_uint = 0x3d7a;
pub const mmUVD_MPC_SET_MUXB0: c_uint = 0x3d7b;
pub const mmUVD_MPC_SET_MUXB1: c_uint = 0x3d7c;
pub const mmUVD_MPC_SET_MUX: c_uint = 0x3d7d;
pub const mmUVD_MPC_SET_ALU: c_uint = 0x3d7e;
pub const mmUVD_VCPU_CACHE_OFFSET0: c_uint = 0x3d82;
pub const mmUVD_VCPU_CACHE_SIZE0: c_uint = 0x3d83;
pub const mmUVD_VCPU_CACHE_OFFSET1: c_uint = 0x3d84;
pub const mmUVD_VCPU_CACHE_SIZE1: c_uint = 0x3d85;
pub const mmUVD_VCPU_CACHE_OFFSET2: c_uint = 0x3d86;
pub const mmUVD_VCPU_CACHE_SIZE2: c_uint = 0x3d87;
pub const mmUVD_VCPU_CNTL: c_uint = 0x3d98;
pub const mmUVD_SOFT_RESET: c_uint = 0x3da0;
pub const mmUVD_RBC_IB_BASE: c_uint = 0x3da1;
pub const mmUVD_RBC_IB_SIZE: c_uint = 0x3da2;
pub const mmUVD_RBC_RB_BASE: c_uint = 0x3da3;
pub const mmUVD_RBC_RB_RPTR: c_uint = 0x3da4;
pub const mmUVD_RBC_RB_WPTR: c_uint = 0x3da5;
pub const mmUVD_RBC_RB_WPTR_CNTL: c_uint = 0x3da6;
pub const mmUVD_RBC_RB_CNTL: c_uint = 0x3da9;
pub const mmUVD_RBC_RB_RPTR_ADDR: c_uint = 0x3daa;
pub const mmUVD_STATUS: c_uint = 0x3daf;
pub const mmUVD_SEMA_TIMEOUT_STATUS: c_uint = 0x3db0;
pub const mmUVD_SEMA_WAIT_INCOMPLETE_TIMEOUT_CNTL: c_uint = 0x3db1;
pub const mmUVD_SEMA_WAIT_FAULT_TIMEOUT_CNTL: c_uint = 0x3db2;
pub const mmUVD_SEMA_SIGNAL_INCOMPLETE_TIMEOUT_CNTL: c_uint = 0x3db3;
pub const mmUVD_CONTEXT_ID: c_uint = 0x3dbd;
pub const mmUVD_RBC_IB_SIZE_UPDATE: c_uint = 0x3df1;
pub const ixUVD_LMI_CACHE_CTRL: c_uint = 0x9b;
pub const ixUVD_LMI_SWAP_CNTL2: c_uint = 0xaa;
pub const ixUVD_LMI_ADDR_EXT2: c_uint = 0xab;
pub const ixUVD_CGC_MEM_CTRL: c_uint = 0xc0;
pub const ixUVD_CGC_CTRL2: c_uint = 0xc1;
pub const mmUVD_PGFSM_CONFIG: c_uint = 0x38f8;
pub const mmUVD_PGFSM_READ_TILE1: c_uint = 0x38fa;
pub const mmUVD_PGFSM_READ_TILE2: c_uint = 0x38fb;
pub const mmUVD_POWER_STATUS: c_uint = 0x38fc;
pub const ixUVD_MIF_CURR_ADDR_CONFIG: c_uint = 0x48;
pub const ixUVD_MIF_REF_ADDR_CONFIG: c_uint = 0x4c;
pub const ixUVD_MIF_RECON1_ADDR_CONFIG: c_uint = 0x114;
