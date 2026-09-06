//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/include/asic_reg/uvd/uvd_7_0_offset.h
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
// Copyright (C) 2017  Advanced Micro Devices, Inc.
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

// Macro flag: #define _uvd_7_0_OFFSET_HEADER
// addressBlock: uvd0_uvd_pg_dec
// base address: 0x1fb00
pub const mmUVD_POWER_STATUS: c_uint = 0x00c4;
pub const mmUVD_POWER_STATUS_BASE_IDX: c_int = 1;
pub const mmUVD_DPG_RBC_RB_CNTL: c_uint = 0x00cb;
pub const mmUVD_DPG_RBC_RB_CNTL_BASE_IDX: c_int = 1;
pub const mmUVD_DPG_RBC_RB_BASE_LOW: c_uint = 0x00cc;
pub const mmUVD_DPG_RBC_RB_BASE_LOW_BASE_IDX: c_int = 1;
pub const mmUVD_DPG_RBC_RB_BASE_HIGH: c_uint = 0x00cd;
pub const mmUVD_DPG_RBC_RB_BASE_HIGH_BASE_IDX: c_int = 1;
pub const mmUVD_DPG_RBC_RB_WPTR_CNTL: c_uint = 0x00ce;
pub const mmUVD_DPG_RBC_RB_WPTR_CNTL_BASE_IDX: c_int = 1;
pub const mmUVD_DPG_RBC_RB_RPTR: c_uint = 0x00cf;
pub const mmUVD_DPG_RBC_RB_RPTR_BASE_IDX: c_int = 1;
pub const mmUVD_DPG_RBC_RB_WPTR: c_uint = 0x00d0;
pub const mmUVD_DPG_RBC_RB_WPTR_BASE_IDX: c_int = 1;
pub const mmUVD_DPG_LMI_VCPU_CACHE_64BIT_BAR_LOW: c_uint = 0x00e5;
pub const mmUVD_DPG_LMI_VCPU_CACHE_64BIT_BAR_LOW_BASE_IDX: c_int = 1;
pub const mmUVD_DPG_LMI_VCPU_CACHE_64BIT_BAR_HIGH: c_uint = 0x00e6;
pub const mmUVD_DPG_LMI_VCPU_CACHE_64BIT_BAR_HIGH_BASE_IDX: c_int = 1;
pub const mmUVD_DPG_VCPU_CACHE_OFFSET0: c_uint = 0x00e7;
pub const mmUVD_DPG_VCPU_CACHE_OFFSET0_BASE_IDX: c_int = 1;
// addressBlock: uvd0_uvdnpdec
// base address: 0x20000
pub const mmUVD_JPEG_ADDR_CONFIG: c_uint = 0x021f;
pub const mmUVD_JPEG_ADDR_CONFIG_BASE_IDX: c_int = 1;
pub const mmUVD_GPCOM_VCPU_CMD: c_uint = 0x03c3;
pub const mmUVD_GPCOM_VCPU_CMD_BASE_IDX: c_int = 1;
pub const mmUVD_GPCOM_VCPU_DATA0: c_uint = 0x03c4;
pub const mmUVD_GPCOM_VCPU_DATA0_BASE_IDX: c_int = 1;
pub const mmUVD_GPCOM_VCPU_DATA1: c_uint = 0x03c5;
pub const mmUVD_GPCOM_VCPU_DATA1_BASE_IDX: c_int = 1;
pub const mmUVD_UDEC_ADDR_CONFIG: c_uint = 0x03d3;
pub const mmUVD_UDEC_ADDR_CONFIG_BASE_IDX: c_int = 1;
pub const mmUVD_UDEC_DB_ADDR_CONFIG: c_uint = 0x03d4;
pub const mmUVD_UDEC_DB_ADDR_CONFIG_BASE_IDX: c_int = 1;
pub const mmUVD_UDEC_DBW_ADDR_CONFIG: c_uint = 0x03d5;
pub const mmUVD_UDEC_DBW_ADDR_CONFIG_BASE_IDX: c_int = 1;
pub const mmUVD_SUVD_CGC_GATE: c_uint = 0x03e4;
pub const mmUVD_SUVD_CGC_GATE_BASE_IDX: c_int = 1;
pub const mmUVD_SUVD_CGC_CTRL: c_uint = 0x03e6;
pub const mmUVD_SUVD_CGC_CTRL_BASE_IDX: c_int = 1;
pub const mmUVD_LMI_VCPU_CACHE1_64BIT_BAR_LOW: c_uint = 0x03ec;
pub const mmUVD_LMI_VCPU_CACHE1_64BIT_BAR_LOW_BASE_IDX: c_int = 1;
pub const mmUVD_LMI_VCPU_CACHE1_64BIT_BAR_HIGH: c_uint = 0x03ed;
pub const mmUVD_LMI_VCPU_CACHE1_64BIT_BAR_HIGH_BASE_IDX: c_int = 1;
pub const mmUVD_LMI_VCPU_CACHE2_64BIT_BAR_LOW: c_uint = 0x03f0;
pub const mmUVD_LMI_VCPU_CACHE2_64BIT_BAR_LOW_BASE_IDX: c_int = 1;
pub const mmUVD_LMI_VCPU_CACHE2_64BIT_BAR_HIGH: c_uint = 0x03f1;
pub const mmUVD_LMI_VCPU_CACHE2_64BIT_BAR_HIGH_BASE_IDX: c_int = 1;
pub const mmUVD_POWER_STATUS_U: c_uint = 0x03fd;
pub const mmUVD_POWER_STATUS_U_BASE_IDX: c_int = 1;
pub const mmUVD_NO_OP: c_uint = 0x03ff;
pub const mmUVD_NO_OP_BASE_IDX: c_int = 1;
pub const mmUVD_GP_SCRATCH8: c_uint = 0x040a;
pub const mmUVD_GP_SCRATCH8_BASE_IDX: c_int = 1;
pub const mmUVD_RB_BASE_LO2: c_uint = 0x0421;
pub const mmUVD_RB_BASE_LO2_BASE_IDX: c_int = 1;
pub const mmUVD_RB_BASE_HI2: c_uint = 0x0422;
pub const mmUVD_RB_BASE_HI2_BASE_IDX: c_int = 1;
pub const mmUVD_RB_SIZE2: c_uint = 0x0423;
pub const mmUVD_RB_SIZE2_BASE_IDX: c_int = 1;
pub const mmUVD_RB_RPTR2: c_uint = 0x0424;
pub const mmUVD_RB_RPTR2_BASE_IDX: c_int = 1;
pub const mmUVD_RB_WPTR2: c_uint = 0x0425;
pub const mmUVD_RB_WPTR2_BASE_IDX: c_int = 1;
pub const mmUVD_RB_BASE_LO: c_uint = 0x0426;
pub const mmUVD_RB_BASE_LO_BASE_IDX: c_int = 1;
pub const mmUVD_RB_BASE_HI: c_uint = 0x0427;
pub const mmUVD_RB_BASE_HI_BASE_IDX: c_int = 1;
pub const mmUVD_RB_SIZE: c_uint = 0x0428;
pub const mmUVD_RB_SIZE_BASE_IDX: c_int = 1;
pub const mmUVD_RB_RPTR: c_uint = 0x0429;
pub const mmUVD_RB_RPTR_BASE_IDX: c_int = 1;
pub const mmUVD_RB_WPTR: c_uint = 0x042a;
pub const mmUVD_RB_WPTR_BASE_IDX: c_int = 1;
pub const mmUVD_JRBC_RB_RPTR: c_uint = 0x0457;
pub const mmUVD_JRBC_RB_RPTR_BASE_IDX: c_int = 1;
pub const mmUVD_LMI_VCPU_CACHE_64BIT_BAR_HIGH: c_uint = 0x045e;
pub const mmUVD_LMI_VCPU_CACHE_64BIT_BAR_HIGH_BASE_IDX: c_int = 1;
pub const mmUVD_LMI_VCPU_CACHE_64BIT_BAR_LOW: c_uint = 0x045f;
pub const mmUVD_LMI_VCPU_CACHE_64BIT_BAR_LOW_BASE_IDX: c_int = 1;
pub const mmUVD_LMI_RBC_IB_64BIT_BAR_HIGH: c_uint = 0x0466;
pub const mmUVD_LMI_RBC_IB_64BIT_BAR_HIGH_BASE_IDX: c_int = 1;
pub const mmUVD_LMI_RBC_IB_64BIT_BAR_LOW: c_uint = 0x0467;
pub const mmUVD_LMI_RBC_IB_64BIT_BAR_LOW_BASE_IDX: c_int = 1;
pub const mmUVD_LMI_RBC_RB_64BIT_BAR_HIGH: c_uint = 0x0468;
pub const mmUVD_LMI_RBC_RB_64BIT_BAR_HIGH_BASE_IDX: c_int = 1;
pub const mmUVD_LMI_RBC_RB_64BIT_BAR_LOW: c_uint = 0x0469;
pub const mmUVD_LMI_RBC_RB_64BIT_BAR_LOW_BASE_IDX: c_int = 1;
// addressBlock: uvd0_uvddec
// base address: 0x20c00
pub const mmUVD_SEMA_CNTL: c_uint = 0x0500;
pub const mmUVD_SEMA_CNTL_BASE_IDX: c_int = 1;
pub const mmUVD_LMI_JRBC_RB_64BIT_BAR_LOW: c_uint = 0x0503;
pub const mmUVD_LMI_JRBC_RB_64BIT_BAR_LOW_BASE_IDX: c_int = 1;
pub const mmUVD_JRBC_RB_WPTR: c_uint = 0x0509;
pub const mmUVD_JRBC_RB_WPTR_BASE_IDX: c_int = 1;
pub const mmUVD_RB_RPTR3: c_uint = 0x051b;
pub const mmUVD_RB_RPTR3_BASE_IDX: c_int = 1;
pub const mmUVD_RB_WPTR3: c_uint = 0x051c;
pub const mmUVD_RB_WPTR3_BASE_IDX: c_int = 1;
pub const mmUVD_RB_BASE_LO3: c_uint = 0x051d;
pub const mmUVD_RB_BASE_LO3_BASE_IDX: c_int = 1;
pub const mmUVD_RB_BASE_HI3: c_uint = 0x051e;
pub const mmUVD_RB_BASE_HI3_BASE_IDX: c_int = 1;
pub const mmUVD_RB_SIZE3: c_uint = 0x051f;
pub const mmUVD_RB_SIZE3_BASE_IDX: c_int = 1;
pub const mmJPEG_CGC_GATE: c_uint = 0x0526;
pub const mmJPEG_CGC_GATE_BASE_IDX: c_int = 1;
pub const mmUVD_CTX_INDEX: c_uint = 0x0528;
pub const mmUVD_CTX_INDEX_BASE_IDX: c_int = 1;
pub const mmUVD_CTX_DATA: c_uint = 0x0529;
pub const mmUVD_CTX_DATA_BASE_IDX: c_int = 1;
pub const mmUVD_CGC_GATE: c_uint = 0x052a;
pub const mmUVD_CGC_GATE_BASE_IDX: c_int = 1;
pub const mmUVD_CGC_CTRL: c_uint = 0x052c;
pub const mmUVD_CGC_CTRL_BASE_IDX: c_int = 1;
pub const mmUVD_GP_SCRATCH4: c_uint = 0x0538;
pub const mmUVD_GP_SCRATCH4_BASE_IDX: c_int = 1;
pub const mmUVD_LMI_CTRL2: c_uint = 0x053d;
pub const mmUVD_LMI_CTRL2_BASE_IDX: c_int = 1;
pub const mmUVD_MASTINT_EN: c_uint = 0x0540;
pub const mmUVD_MASTINT_EN_BASE_IDX: c_int = 1;
pub const mmUVD_FW_STATUS: c_uint = 0x0557;
pub const mmUVD_FW_STATUS_BASE_IDX: c_int = 1;
pub const mmJPEG_CGC_CTRL: c_uint = 0x0565;
pub const mmJPEG_CGC_CTRL_BASE_IDX: c_int = 1;
pub const mmUVD_LMI_CTRL: c_uint = 0x0566;
pub const mmUVD_LMI_CTRL_BASE_IDX: c_int = 1;
pub const mmUVD_LMI_VM_CTRL: c_uint = 0x0568;
pub const mmUVD_LMI_VM_CTRL_BASE_IDX: c_int = 1;
pub const mmUVD_LMI_SWAP_CNTL: c_uint = 0x056d;
pub const mmUVD_LMI_SWAP_CNTL_BASE_IDX: c_int = 1;
pub const mmUVD_MP_SWAP_CNTL: c_uint = 0x056f;
pub const mmUVD_MP_SWAP_CNTL_BASE_IDX: c_int = 1;
pub const mmUVD_MPC_SET_MUXA0: c_uint = 0x0579;
pub const mmUVD_MPC_SET_MUXA0_BASE_IDX: c_int = 1;
pub const mmUVD_MPC_SET_MUXA1: c_uint = 0x057a;
pub const mmUVD_MPC_SET_MUXA1_BASE_IDX: c_int = 1;
pub const mmUVD_MPC_SET_MUXB0: c_uint = 0x057b;
pub const mmUVD_MPC_SET_MUXB0_BASE_IDX: c_int = 1;
pub const mmUVD_MPC_SET_MUXB1: c_uint = 0x057c;
pub const mmUVD_MPC_SET_MUXB1_BASE_IDX: c_int = 1;
pub const mmUVD_MPC_SET_MUX: c_uint = 0x057d;
pub const mmUVD_MPC_SET_MUX_BASE_IDX: c_int = 1;
pub const mmUVD_MPC_SET_ALU: c_uint = 0x057e;
pub const mmUVD_MPC_SET_ALU_BASE_IDX: c_int = 1;
pub const mmUVD_VCPU_CACHE_OFFSET0: c_uint = 0x0582;
pub const mmUVD_VCPU_CACHE_OFFSET0_BASE_IDX: c_int = 1;
pub const mmUVD_VCPU_CACHE_SIZE0: c_uint = 0x0583;
pub const mmUVD_VCPU_CACHE_SIZE0_BASE_IDX: c_int = 1;
pub const mmUVD_VCPU_CACHE_OFFSET1: c_uint = 0x0584;
pub const mmUVD_VCPU_CACHE_OFFSET1_BASE_IDX: c_int = 1;
pub const mmUVD_VCPU_CACHE_SIZE1: c_uint = 0x0585;
pub const mmUVD_VCPU_CACHE_SIZE1_BASE_IDX: c_int = 1;
pub const mmUVD_VCPU_CACHE_OFFSET2: c_uint = 0x0586;
pub const mmUVD_VCPU_CACHE_OFFSET2_BASE_IDX: c_int = 1;
pub const mmUVD_VCPU_CACHE_SIZE2: c_uint = 0x0587;
pub const mmUVD_VCPU_CACHE_SIZE2_BASE_IDX: c_int = 1;
pub const mmUVD_VCPU_CNTL: c_uint = 0x0598;
pub const mmUVD_VCPU_CNTL_BASE_IDX: c_int = 1;
pub const mmUVD_SOFT_RESET: c_uint = 0x05a0;
pub const mmUVD_SOFT_RESET_BASE_IDX: c_int = 1;
pub const mmUVD_LMI_RBC_IB_VMID: c_uint = 0x05a1;
pub const mmUVD_LMI_RBC_IB_VMID_BASE_IDX: c_int = 1;
pub const mmUVD_RBC_IB_SIZE: c_uint = 0x05a2;
pub const mmUVD_RBC_IB_SIZE_BASE_IDX: c_int = 1;
pub const mmUVD_RBC_RB_RPTR: c_uint = 0x05a4;
pub const mmUVD_RBC_RB_RPTR_BASE_IDX: c_int = 1;
pub const mmUVD_RBC_RB_WPTR: c_uint = 0x05a5;
pub const mmUVD_RBC_RB_WPTR_BASE_IDX: c_int = 1;
pub const mmUVD_RBC_RB_WPTR_CNTL: c_uint = 0x05a6;
pub const mmUVD_RBC_RB_WPTR_CNTL_BASE_IDX: c_int = 1;
pub const mmUVD_RBC_RB_CNTL: c_uint = 0x05a9;
pub const mmUVD_RBC_RB_CNTL_BASE_IDX: c_int = 1;
pub const mmUVD_RBC_RB_RPTR_ADDR: c_uint = 0x05aa;
pub const mmUVD_RBC_RB_RPTR_ADDR_BASE_IDX: c_int = 1;
pub const mmUVD_STATUS: c_uint = 0x05af;
pub const mmUVD_STATUS_BASE_IDX: c_int = 1;
pub const mmUVD_SEMA_TIMEOUT_STATUS: c_uint = 0x05b0;
pub const mmUVD_SEMA_TIMEOUT_STATUS_BASE_IDX: c_int = 1;
pub const mmUVD_SEMA_WAIT_INCOMPLETE_TIMEOUT_CNTL: c_uint = 0x05b1;
pub const mmUVD_SEMA_WAIT_INCOMPLETE_TIMEOUT_CNTL_BASE_IDX: c_int = 1;
pub const mmUVD_SEMA_WAIT_FAULT_TIMEOUT_CNTL: c_uint = 0x05b2;
pub const mmUVD_SEMA_WAIT_FAULT_TIMEOUT_CNTL_BASE_IDX: c_int = 1;
pub const mmUVD_SEMA_SIGNAL_INCOMPLETE_TIMEOUT_CNTL: c_uint = 0x05b3;
pub const mmUVD_SEMA_SIGNAL_INCOMPLETE_TIMEOUT_CNTL_BASE_IDX: c_int = 1;
pub const mmUVD_CONTEXT_ID: c_uint = 0x05bd;
pub const mmUVD_CONTEXT_ID_BASE_IDX: c_int = 1;
pub const mmUVD_CONTEXT_ID2: c_uint = 0x05bf;
pub const mmUVD_CONTEXT_ID2_BASE_IDX: c_int = 1;
