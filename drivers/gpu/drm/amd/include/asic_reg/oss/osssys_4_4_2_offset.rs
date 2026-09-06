//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/include/asic_reg/oss/osssys_4_4_2_offset.h
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

// Macro flag: #define _osssys_4_4_2_OFFSET_HEADER
// addressBlock: aid_osssys_osssysdec
// base address: 0x4280
pub const regIH_VMID_0_LUT: c_uint = 0x0000;
pub const regIH_VMID_0_LUT_BASE_IDX: c_int = 0;
pub const regIH_VMID_1_LUT: c_uint = 0x0001;
pub const regIH_VMID_1_LUT_BASE_IDX: c_int = 0;
pub const regIH_VMID_2_LUT: c_uint = 0x0002;
pub const regIH_VMID_2_LUT_BASE_IDX: c_int = 0;
pub const regIH_VMID_3_LUT: c_uint = 0x0003;
pub const regIH_VMID_3_LUT_BASE_IDX: c_int = 0;
pub const regIH_VMID_4_LUT: c_uint = 0x0004;
pub const regIH_VMID_4_LUT_BASE_IDX: c_int = 0;
pub const regIH_VMID_5_LUT: c_uint = 0x0005;
pub const regIH_VMID_5_LUT_BASE_IDX: c_int = 0;
pub const regIH_VMID_6_LUT: c_uint = 0x0006;
pub const regIH_VMID_6_LUT_BASE_IDX: c_int = 0;
pub const regIH_VMID_7_LUT: c_uint = 0x0007;
pub const regIH_VMID_7_LUT_BASE_IDX: c_int = 0;
pub const regIH_VMID_8_LUT: c_uint = 0x0008;
pub const regIH_VMID_8_LUT_BASE_IDX: c_int = 0;
pub const regIH_VMID_9_LUT: c_uint = 0x0009;
pub const regIH_VMID_9_LUT_BASE_IDX: c_int = 0;
pub const regIH_VMID_10_LUT: c_uint = 0x000a;
pub const regIH_VMID_10_LUT_BASE_IDX: c_int = 0;
pub const regIH_VMID_11_LUT: c_uint = 0x000b;
pub const regIH_VMID_11_LUT_BASE_IDX: c_int = 0;
pub const regIH_VMID_12_LUT: c_uint = 0x000c;
pub const regIH_VMID_12_LUT_BASE_IDX: c_int = 0;
pub const regIH_VMID_13_LUT: c_uint = 0x000d;
pub const regIH_VMID_13_LUT_BASE_IDX: c_int = 0;
pub const regIH_VMID_14_LUT: c_uint = 0x000e;
pub const regIH_VMID_14_LUT_BASE_IDX: c_int = 0;
pub const regIH_VMID_15_LUT: c_uint = 0x000f;
pub const regIH_VMID_15_LUT_BASE_IDX: c_int = 0;
pub const regIH_VMID_0_LUT_MM: c_uint = 0x0010;
pub const regIH_VMID_0_LUT_MM_BASE_IDX: c_int = 0;
pub const regIH_VMID_1_LUT_MM: c_uint = 0x0011;
pub const regIH_VMID_1_LUT_MM_BASE_IDX: c_int = 0;
pub const regIH_VMID_2_LUT_MM: c_uint = 0x0012;
pub const regIH_VMID_2_LUT_MM_BASE_IDX: c_int = 0;
pub const regIH_VMID_3_LUT_MM: c_uint = 0x0013;
pub const regIH_VMID_3_LUT_MM_BASE_IDX: c_int = 0;
pub const regIH_VMID_4_LUT_MM: c_uint = 0x0014;
pub const regIH_VMID_4_LUT_MM_BASE_IDX: c_int = 0;
pub const regIH_VMID_5_LUT_MM: c_uint = 0x0015;
pub const regIH_VMID_5_LUT_MM_BASE_IDX: c_int = 0;
pub const regIH_VMID_6_LUT_MM: c_uint = 0x0016;
pub const regIH_VMID_6_LUT_MM_BASE_IDX: c_int = 0;
pub const regIH_VMID_7_LUT_MM: c_uint = 0x0017;
pub const regIH_VMID_7_LUT_MM_BASE_IDX: c_int = 0;
pub const regIH_VMID_8_LUT_MM: c_uint = 0x0018;
pub const regIH_VMID_8_LUT_MM_BASE_IDX: c_int = 0;
pub const regIH_VMID_9_LUT_MM: c_uint = 0x0019;
pub const regIH_VMID_9_LUT_MM_BASE_IDX: c_int = 0;
pub const regIH_VMID_10_LUT_MM: c_uint = 0x001a;
pub const regIH_VMID_10_LUT_MM_BASE_IDX: c_int = 0;
pub const regIH_VMID_11_LUT_MM: c_uint = 0x001b;
pub const regIH_VMID_11_LUT_MM_BASE_IDX: c_int = 0;
pub const regIH_VMID_12_LUT_MM: c_uint = 0x001c;
pub const regIH_VMID_12_LUT_MM_BASE_IDX: c_int = 0;
pub const regIH_VMID_13_LUT_MM: c_uint = 0x001d;
pub const regIH_VMID_13_LUT_MM_BASE_IDX: c_int = 0;
pub const regIH_VMID_14_LUT_MM: c_uint = 0x001e;
pub const regIH_VMID_14_LUT_MM_BASE_IDX: c_int = 0;
pub const regIH_VMID_15_LUT_MM: c_uint = 0x001f;
pub const regIH_VMID_15_LUT_MM_BASE_IDX: c_int = 0;
pub const regIH_COOKIE_0: c_uint = 0x0020;
pub const regIH_COOKIE_0_BASE_IDX: c_int = 0;
pub const regIH_COOKIE_1: c_uint = 0x0021;
pub const regIH_COOKIE_1_BASE_IDX: c_int = 0;
pub const regIH_COOKIE_2: c_uint = 0x0022;
pub const regIH_COOKIE_2_BASE_IDX: c_int = 0;
pub const regIH_COOKIE_3: c_uint = 0x0023;
pub const regIH_COOKIE_3_BASE_IDX: c_int = 0;
pub const regIH_COOKIE_4: c_uint = 0x0024;
pub const regIH_COOKIE_4_BASE_IDX: c_int = 0;
pub const regIH_COOKIE_5: c_uint = 0x0025;
pub const regIH_COOKIE_5_BASE_IDX: c_int = 0;
pub const regIH_COOKIE_6: c_uint = 0x0026;
pub const regIH_COOKIE_6_BASE_IDX: c_int = 0;
pub const regIH_COOKIE_7: c_uint = 0x0027;
pub const regIH_COOKIE_7_BASE_IDX: c_int = 0;
pub const regIH_REGISTER_LAST_PART0: c_uint = 0x003f;
pub const regIH_REGISTER_LAST_PART0_BASE_IDX: c_int = 0;
pub const regIH_RB_CNTL: c_uint = 0x0080;
pub const regIH_RB_CNTL_BASE_IDX: c_int = 0;
pub const regIH_RB_BASE: c_uint = 0x0081;
pub const regIH_RB_BASE_BASE_IDX: c_int = 0;
pub const regIH_RB_BASE_HI: c_uint = 0x0082;
pub const regIH_RB_BASE_HI_BASE_IDX: c_int = 0;
pub const regIH_RB_RPTR: c_uint = 0x0083;
pub const regIH_RB_RPTR_BASE_IDX: c_int = 0;
pub const regIH_RB_WPTR: c_uint = 0x0084;
pub const regIH_RB_WPTR_BASE_IDX: c_int = 0;
pub const regIH_RB_WPTR_ADDR_HI: c_uint = 0x0085;
pub const regIH_RB_WPTR_ADDR_HI_BASE_IDX: c_int = 0;
pub const regIH_RB_WPTR_ADDR_LO: c_uint = 0x0086;
pub const regIH_RB_WPTR_ADDR_LO_BASE_IDX: c_int = 0;
pub const regIH_DOORBELL_RPTR: c_uint = 0x0087;
pub const regIH_DOORBELL_RPTR_BASE_IDX: c_int = 0;
pub const regIH_DOORBELL_RETRY_CAM: c_uint = 0x0088;
pub const regIH_DOORBELL_RETRY_CAM_BASE_IDX: c_int = 0;
pub const regIH_RB_CNTL_RING1: c_uint = 0x008c;
pub const regIH_RB_CNTL_RING1_BASE_IDX: c_int = 0;
pub const regIH_RB_BASE_RING1: c_uint = 0x008d;
pub const regIH_RB_BASE_RING1_BASE_IDX: c_int = 0;
pub const regIH_RB_BASE_HI_RING1: c_uint = 0x008e;
pub const regIH_RB_BASE_HI_RING1_BASE_IDX: c_int = 0;
pub const regIH_RB_RPTR_RING1: c_uint = 0x008f;
pub const regIH_RB_RPTR_RING1_BASE_IDX: c_int = 0;
pub const regIH_RB_WPTR_RING1: c_uint = 0x0090;
pub const regIH_RB_WPTR_RING1_BASE_IDX: c_int = 0;
pub const regIH_DOORBELL_RPTR_RING1: c_uint = 0x0093;
pub const regIH_DOORBELL_RPTR_RING1_BASE_IDX: c_int = 0;
pub const regIH_RETRY_CAM_ACK: c_uint = 0x00a4;
pub const regIH_RETRY_CAM_ACK_BASE_IDX: c_int = 0;
pub const regIH_VERSION: c_uint = 0x00a5;
pub const regIH_VERSION_BASE_IDX: c_int = 0;
pub const regIH_CNTL: c_uint = 0x00c0;
pub const regIH_CNTL_BASE_IDX: c_int = 0;
pub const regIH_CNTL2: c_uint = 0x00c1;
pub const regIH_CNTL2_BASE_IDX: c_int = 0;
pub const regIH_STATUS: c_uint = 0x00c2;
pub const regIH_STATUS_BASE_IDX: c_int = 0;
pub const regIH_PERFMON_CNTL: c_uint = 0x00c3;
pub const regIH_PERFMON_CNTL_BASE_IDX: c_int = 0;
pub const regIH_PERFCOUNTER0_RESULT: c_uint = 0x00c4;
pub const regIH_PERFCOUNTER0_RESULT_BASE_IDX: c_int = 0;
pub const regIH_PERFCOUNTER1_RESULT: c_uint = 0x00c5;
pub const regIH_PERFCOUNTER1_RESULT_BASE_IDX: c_int = 0;
pub const regIH_DSM_MATCH_VALUE_BIT_31_0: c_uint = 0x00c7;
pub const regIH_DSM_MATCH_VALUE_BIT_31_0_BASE_IDX: c_int = 0;
pub const regIH_DSM_MATCH_VALUE_BIT_63_32: c_uint = 0x00c8;
pub const regIH_DSM_MATCH_VALUE_BIT_63_32_BASE_IDX: c_int = 0;
pub const regIH_DSM_MATCH_VALUE_BIT_95_64: c_uint = 0x00c9;
pub const regIH_DSM_MATCH_VALUE_BIT_95_64_BASE_IDX: c_int = 0;
pub const regIH_DSM_MATCH_FIELD_CONTROL: c_uint = 0x00ca;
pub const regIH_DSM_MATCH_FIELD_CONTROL_BASE_IDX: c_int = 0;
pub const regIH_DSM_MATCH_DATA_CONTROL: c_uint = 0x00cb;
pub const regIH_DSM_MATCH_DATA_CONTROL_BASE_IDX: c_int = 0;
pub const regIH_DSM_MATCH_FCN_ID: c_uint = 0x00cc;
pub const regIH_DSM_MATCH_FCN_ID_BASE_IDX: c_int = 0;
pub const regIH_LIMIT_INT_RATE_CNTL: c_uint = 0x00cd;
pub const regIH_LIMIT_INT_RATE_CNTL_BASE_IDX: c_int = 0;
pub const regIH_VF_RB_STATUS: c_uint = 0x00ce;
pub const regIH_VF_RB_STATUS_BASE_IDX: c_int = 0;
pub const regIH_VF_RB_STATUS2: c_uint = 0x00cf;
pub const regIH_VF_RB_STATUS2_BASE_IDX: c_int = 0;
pub const regIH_VF_RB1_STATUS: c_uint = 0x00d0;
pub const regIH_VF_RB1_STATUS_BASE_IDX: c_int = 0;
pub const regIH_VF_RB1_STATUS2: c_uint = 0x00d1;
pub const regIH_VF_RB1_STATUS2_BASE_IDX: c_int = 0;
pub const regIH_INT_FLOOD_CNTL: c_uint = 0x00d5;
pub const regIH_INT_FLOOD_CNTL_BASE_IDX: c_int = 0;
pub const regIH_RB0_INT_FLOOD_STATUS: c_uint = 0x00d6;
pub const regIH_RB0_INT_FLOOD_STATUS_BASE_IDX: c_int = 0;
pub const regIH_RB1_INT_FLOOD_STATUS: c_uint = 0x00d7;
pub const regIH_RB1_INT_FLOOD_STATUS_BASE_IDX: c_int = 0;
pub const regIH_INT_FLOOD_STATUS: c_uint = 0x00d9;
pub const regIH_INT_FLOOD_STATUS_BASE_IDX: c_int = 0;
pub const regIH_STORM_CLIENT_LIST_CNTL: c_uint = 0x00da;
pub const regIH_STORM_CLIENT_LIST_CNTL_BASE_IDX: c_int = 0;
pub const regIH_CLK_CTRL: c_uint = 0x00db;
pub const regIH_CLK_CTRL_BASE_IDX: c_int = 0;
pub const regIH_INT_FLAGS: c_uint = 0x00dc;
pub const regIH_INT_FLAGS_BASE_IDX: c_int = 0;
pub const regIH_LAST_INT_INFO0: c_uint = 0x00dd;
pub const regIH_LAST_INT_INFO0_BASE_IDX: c_int = 0;
pub const regIH_LAST_INT_INFO1: c_uint = 0x00de;
pub const regIH_LAST_INT_INFO1_BASE_IDX: c_int = 0;
pub const regIH_LAST_INT_INFO2: c_uint = 0x00df;
pub const regIH_LAST_INT_INFO2_BASE_IDX: c_int = 0;
pub const regIH_SCRATCH: c_uint = 0x00e0;
pub const regIH_SCRATCH_BASE_IDX: c_int = 0;
pub const regIH_CLIENT_CREDIT_ERROR: c_uint = 0x00e1;
pub const regIH_CLIENT_CREDIT_ERROR_BASE_IDX: c_int = 0;
pub const regIH_GPU_IOV_VIOLATION_LOG: c_uint = 0x00e2;
pub const regIH_GPU_IOV_VIOLATION_LOG_BASE_IDX: c_int = 0;
pub const regIH_GPU_IOV_VIOLATION_LOG2: c_uint = 0x00e3;
pub const regIH_GPU_IOV_VIOLATION_LOG2_BASE_IDX: c_int = 0;
pub const regIH_COOKIE_REC_VIOLATION_LOG: c_uint = 0x00e4;
pub const regIH_COOKIE_REC_VIOLATION_LOG_BASE_IDX: c_int = 0;
pub const regIH_CREDIT_STATUS: c_uint = 0x00e5;
pub const regIH_CREDIT_STATUS_BASE_IDX: c_int = 0;
pub const regIH_MMHUB_ERROR: c_uint = 0x00e6;
pub const regIH_MMHUB_ERROR_BASE_IDX: c_int = 0;
pub const regIH_MEM_POWER_CTRL: c_uint = 0x00e9;
pub const regIH_MEM_POWER_CTRL_BASE_IDX: c_int = 0;
pub const regIH_RETRY_INT_CAM_CNTL: c_uint = 0x00ea;
pub const regIH_RETRY_INT_CAM_CNTL_BASE_IDX: c_int = 0;
pub const regIH_VMID_LUT_INDEX: c_uint = 0x00ec;
pub const regIH_VMID_LUT_INDEX_BASE_IDX: c_int = 0;
pub const regIH_MEM_POWER_CTRL2: c_uint = 0x00f1;
pub const regIH_MEM_POWER_CTRL2_BASE_IDX: c_int = 0;
pub const regIH_REGISTER_LAST_PART2: c_uint = 0x00ff;
pub const regIH_REGISTER_LAST_PART2_BASE_IDX: c_int = 0;
pub const regSEM_MAILBOX: c_uint = 0x010a;
pub const regSEM_MAILBOX_BASE_IDX: c_int = 0;
pub const regSEM_MAILBOX_CLEAR: c_uint = 0x010b;
pub const regSEM_MAILBOX_CLEAR_BASE_IDX: c_int = 0;
pub const regSEM_REGISTER_LAST_PART2: c_uint = 0x017f;
pub const regSEM_REGISTER_LAST_PART2_BASE_IDX: c_int = 0;
pub const regIH_ACTIVE_FCN_ID: c_uint = 0x0180;
pub const regIH_ACTIVE_FCN_ID_BASE_IDX: c_int = 0;
pub const regIH_VIRT_RESET_REQ: c_uint = 0x0181;
pub const regIH_VIRT_RESET_REQ_BASE_IDX: c_int = 0;
pub const regIH_CLIENT_CFG: c_uint = 0x0184;
pub const regIH_CLIENT_CFG_BASE_IDX: c_int = 0;
pub const regIH_CLIENT_CFG_INDEX: c_uint = 0x0188;
pub const regIH_CLIENT_CFG_INDEX_BASE_IDX: c_int = 0;
pub const regIH_CLIENT_CFG_DATA: c_uint = 0x0189;
pub const regIH_CLIENT_CFG_DATA_BASE_IDX: c_int = 0;
pub const regIH_CLIENT_CFG_DATA2: c_uint = 0x018a;
pub const regIH_CLIENT_CFG_DATA2_BASE_IDX: c_int = 0;
pub const regIH_CID_REMAP_INDEX: c_uint = 0x018b;
pub const regIH_CID_REMAP_INDEX_BASE_IDX: c_int = 0;
pub const regIH_CID_REMAP_DATA: c_uint = 0x018c;
pub const regIH_CID_REMAP_DATA_BASE_IDX: c_int = 0;
pub const regIH_CHICKEN: c_uint = 0x018d;
pub const regIH_CHICKEN_BASE_IDX: c_int = 0;
pub const regIH_INT_DROP_CNTL: c_uint = 0x018f;
pub const regIH_INT_DROP_CNTL_BASE_IDX: c_int = 0;
pub const regIH_INT_DROP_MATCH_VALUE0: c_uint = 0x0190;
pub const regIH_INT_DROP_MATCH_VALUE0_BASE_IDX: c_int = 0;
pub const regIH_INT_DROP_MATCH_VALUE1: c_uint = 0x0191;
pub const regIH_INT_DROP_MATCH_VALUE1_BASE_IDX: c_int = 0;
pub const regIH_INT_DROP_MATCH_MASK0: c_uint = 0x0192;
pub const regIH_INT_DROP_MATCH_MASK0_BASE_IDX: c_int = 0;
pub const regIH_INT_DROP_MATCH_MASK1: c_uint = 0x0193;
pub const regIH_INT_DROP_MATCH_MASK1_BASE_IDX: c_int = 0;
pub const regIH_MMHUB_CNTL: c_uint = 0x019e;
pub const regIH_MMHUB_CNTL_BASE_IDX: c_int = 0;
pub const regIH_REGISTER_LAST_PART1: c_uint = 0x019f;
pub const regIH_REGISTER_LAST_PART1_BASE_IDX: c_int = 0;
