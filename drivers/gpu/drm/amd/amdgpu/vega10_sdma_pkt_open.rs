//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/vega10_sdma_pkt_open.h
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
// Copyright (C) 2016  Advanced Micro Devices, Inc.
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
pub const SDMA_OP_NOP: c_int = 0;
pub const SDMA_OP_COPY: c_int = 1;
pub const SDMA_OP_WRITE: c_int = 2;
pub const SDMA_OP_INDIRECT: c_int = 4;
pub const SDMA_OP_FENCE: c_int = 5;
pub const SDMA_OP_TRAP: c_int = 6;
pub const SDMA_OP_SEM: c_int = 7;
pub const SDMA_OP_POLL_REGMEM: c_int = 8;
pub const SDMA_OP_COND_EXE: c_int = 9;
pub const SDMA_OP_ATOMIC: c_int = 10;
pub const SDMA_OP_CONST_FILL: c_int = 11;
pub const SDMA_OP_PTEPDE: c_int = 12;
pub const SDMA_OP_TIMESTAMP: c_int = 13;
pub const SDMA_OP_SRBM_WRITE: c_int = 14;
pub const SDMA_OP_PRE_EXE: c_int = 15;
pub const SDMA_OP_DUMMY_TRAP: c_int = 16;
pub const SDMA_SUBOP_TIMESTAMP_SET: c_int = 0;
pub const SDMA_SUBOP_TIMESTAMP_GET: c_int = 1;
pub const SDMA_SUBOP_TIMESTAMP_GET_GLOBAL: c_int = 2;
pub const SDMA_SUBOP_COPY_LINEAR: c_int = 0;
pub const SDMA_SUBOP_COPY_LINEAR_SUB_WIND: c_int = 4;
pub const SDMA_SUBOP_COPY_TILED: c_int = 1;
pub const SDMA_SUBOP_COPY_TILED_SUB_WIND: c_int = 5;
pub const SDMA_SUBOP_COPY_T2T_SUB_WIND: c_int = 6;
pub const SDMA_SUBOP_COPY_SOA: c_int = 3;
pub const SDMA_SUBOP_COPY_DIRTY_PAGE: c_int = 7;
pub const SDMA_SUBOP_COPY_LINEAR_PHY: c_int = 8;
pub const SDMA_SUBOP_WRITE_LINEAR: c_int = 0;
pub const SDMA_SUBOP_WRITE_TILED: c_int = 1;
pub const SDMA_SUBOP_PTEPDE_GEN: c_int = 0;
pub const SDMA_SUBOP_PTEPDE_COPY: c_int = 1;
pub const SDMA_SUBOP_PTEPDE_RMW: c_int = 2;
pub const SDMA_SUBOP_PTEPDE_COPY_BACKWARDS: c_int = 3;
pub const SDMA_SUBOP_DATA_FILL_MULTI: c_int = 1;
pub const SDMA_SUBOP_POLL_REG_WRITE_MEM: c_int = 1;
pub const SDMA_SUBOP_POLL_DBIT_WRITE_MEM: c_int = 2;
pub const SDMA_SUBOP_POLL_MEM_VERIFY: c_int = 3;
pub const HEADER_AGENT_DISPATCH: c_int = 4;
pub const HEADER_BARRIER: c_int = 5;
pub const SDMA_OP_AQL_COPY: c_int = 0;
pub const SDMA_OP_AQL_BARRIER_OR: c_int = 0;
// define for op field
pub const SDMA_PKT_HEADER_op_offset: c_int = 0;
pub const SDMA_PKT_HEADER_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_HEADER_op_shift: c_int = 0;

// define for sub_op field
pub const SDMA_PKT_HEADER_sub_op_offset: c_int = 0;
pub const SDMA_PKT_HEADER_sub_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_HEADER_sub_op_shift: c_int = 8;

//
// Definitions for SDMA_PKT_COPY_LINEAR packet
//
// define for HEADER word
// define for op field
pub const SDMA_PKT_COPY_LINEAR_HEADER_op_offset: c_int = 0;
pub const SDMA_PKT_COPY_LINEAR_HEADER_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_COPY_LINEAR_HEADER_op_shift: c_int = 0;

// define for sub_op field
pub const SDMA_PKT_COPY_LINEAR_HEADER_sub_op_offset: c_int = 0;
pub const SDMA_PKT_COPY_LINEAR_HEADER_sub_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_COPY_LINEAR_HEADER_sub_op_shift: c_int = 8;

// define for encrypt field
pub const SDMA_PKT_COPY_LINEAR_HEADER_encrypt_offset: c_int = 0;
pub const SDMA_PKT_COPY_LINEAR_HEADER_encrypt_mask: c_uint = 0x00000001;
pub const SDMA_PKT_COPY_LINEAR_HEADER_encrypt_shift: c_int = 16;

// define for tmz field
pub const SDMA_PKT_COPY_LINEAR_HEADER_tmz_offset: c_int = 0;
pub const SDMA_PKT_COPY_LINEAR_HEADER_tmz_mask: c_uint = 0x00000001;
pub const SDMA_PKT_COPY_LINEAR_HEADER_tmz_shift: c_int = 18;

// define for broadcast field
pub const SDMA_PKT_COPY_LINEAR_HEADER_broadcast_offset: c_int = 0;
pub const SDMA_PKT_COPY_LINEAR_HEADER_broadcast_mask: c_uint = 0x00000001;
pub const SDMA_PKT_COPY_LINEAR_HEADER_broadcast_shift: c_int = 27;

// define for COUNT word
// define for count field
pub const SDMA_PKT_COPY_LINEAR_COUNT_count_offset: c_int = 1;
pub const SDMA_PKT_COPY_LINEAR_COUNT_count_mask: c_uint = 0x003FFFFF;
pub const SDMA_PKT_COPY_LINEAR_COUNT_count_shift: c_int = 0;

// define for PARAMETER word
// define for dst_sw field
pub const SDMA_PKT_COPY_LINEAR_PARAMETER_dst_sw_offset: c_int = 2;
pub const SDMA_PKT_COPY_LINEAR_PARAMETER_dst_sw_mask: c_uint = 0x00000003;
pub const SDMA_PKT_COPY_LINEAR_PARAMETER_dst_sw_shift: c_int = 16;

// define for src_sw field
pub const SDMA_PKT_COPY_LINEAR_PARAMETER_src_sw_offset: c_int = 2;
pub const SDMA_PKT_COPY_LINEAR_PARAMETER_src_sw_mask: c_uint = 0x00000003;
pub const SDMA_PKT_COPY_LINEAR_PARAMETER_src_sw_shift: c_int = 24;

// define for SRC_ADDR_LO word
// define for src_addr_31_0 field
pub const SDMA_PKT_COPY_LINEAR_SRC_ADDR_LO_src_addr_31_0_offset: c_int = 3;
pub const SDMA_PKT_COPY_LINEAR_SRC_ADDR_LO_src_addr_31_0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_LINEAR_SRC_ADDR_LO_src_addr_31_0_shift: c_int = 0;

// define for SRC_ADDR_HI word
// define for src_addr_63_32 field
pub const SDMA_PKT_COPY_LINEAR_SRC_ADDR_HI_src_addr_63_32_offset: c_int = 4;
pub const SDMA_PKT_COPY_LINEAR_SRC_ADDR_HI_src_addr_63_32_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_LINEAR_SRC_ADDR_HI_src_addr_63_32_shift: c_int = 0;

// define for DST_ADDR_LO word
// define for dst_addr_31_0 field
pub const SDMA_PKT_COPY_LINEAR_DST_ADDR_LO_dst_addr_31_0_offset: c_int = 5;
pub const SDMA_PKT_COPY_LINEAR_DST_ADDR_LO_dst_addr_31_0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_LINEAR_DST_ADDR_LO_dst_addr_31_0_shift: c_int = 0;

// define for DST_ADDR_HI word
// define for dst_addr_63_32 field
pub const SDMA_PKT_COPY_LINEAR_DST_ADDR_HI_dst_addr_63_32_offset: c_int = 6;
pub const SDMA_PKT_COPY_LINEAR_DST_ADDR_HI_dst_addr_63_32_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_LINEAR_DST_ADDR_HI_dst_addr_63_32_shift: c_int = 0;

//
// Definitions for SDMA_PKT_COPY_DIRTY_PAGE packet
//
// define for HEADER word
// define for op field
pub const SDMA_PKT_COPY_DIRTY_PAGE_HEADER_op_offset: c_int = 0;
pub const SDMA_PKT_COPY_DIRTY_PAGE_HEADER_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_COPY_DIRTY_PAGE_HEADER_op_shift: c_int = 0;

// define for sub_op field
pub const SDMA_PKT_COPY_DIRTY_PAGE_HEADER_sub_op_offset: c_int = 0;
pub const SDMA_PKT_COPY_DIRTY_PAGE_HEADER_sub_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_COPY_DIRTY_PAGE_HEADER_sub_op_shift: c_int = 8;

// define for tmz field
pub const SDMA_PKT_COPY_DIRTY_PAGE_HEADER_tmz_offset: c_int = 0;
pub const SDMA_PKT_COPY_DIRTY_PAGE_HEADER_tmz_mask: c_uint = 0x00000001;
pub const SDMA_PKT_COPY_DIRTY_PAGE_HEADER_tmz_shift: c_int = 18;

// define for all field
pub const SDMA_PKT_COPY_DIRTY_PAGE_HEADER_all_offset: c_int = 0;
pub const SDMA_PKT_COPY_DIRTY_PAGE_HEADER_all_mask: c_uint = 0x00000001;
pub const SDMA_PKT_COPY_DIRTY_PAGE_HEADER_all_shift: c_int = 31;

// define for COUNT word
// define for count field
pub const SDMA_PKT_COPY_DIRTY_PAGE_COUNT_count_offset: c_int = 1;
pub const SDMA_PKT_COPY_DIRTY_PAGE_COUNT_count_mask: c_uint = 0x003FFFFF;
pub const SDMA_PKT_COPY_DIRTY_PAGE_COUNT_count_shift: c_int = 0;

// define for PARAMETER word
// define for dst_sw field
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_dst_sw_offset: c_int = 2;
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_dst_sw_mask: c_uint = 0x00000003;
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_dst_sw_shift: c_int = 16;

// define for dst_gcc field
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_dst_gcc_offset: c_int = 2;
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_dst_gcc_mask: c_uint = 0x00000001;
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_dst_gcc_shift: c_int = 19;

// define for dst_sys field
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_dst_sys_offset: c_int = 2;
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_dst_sys_mask: c_uint = 0x00000001;
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_dst_sys_shift: c_int = 20;

// define for dst_snoop field
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_dst_snoop_offset: c_int = 2;
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_dst_snoop_mask: c_uint = 0x00000001;
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_dst_snoop_shift: c_int = 22;

// define for dst_gpa field
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_dst_gpa_offset: c_int = 2;
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_dst_gpa_mask: c_uint = 0x00000001;
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_dst_gpa_shift: c_int = 23;

// define for src_sw field
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_src_sw_offset: c_int = 2;
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_src_sw_mask: c_uint = 0x00000003;
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_src_sw_shift: c_int = 24;

// define for src_sys field
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_src_sys_offset: c_int = 2;
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_src_sys_mask: c_uint = 0x00000001;
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_src_sys_shift: c_int = 28;

// define for src_snoop field
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_src_snoop_offset: c_int = 2;
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_src_snoop_mask: c_uint = 0x00000001;
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_src_snoop_shift: c_int = 30;

// define for src_gpa field
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_src_gpa_offset: c_int = 2;
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_src_gpa_mask: c_uint = 0x00000001;
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_src_gpa_shift: c_int = 31;

// define for SRC_ADDR_LO word
// define for src_addr_31_0 field
pub const SDMA_PKT_COPY_DIRTY_PAGE_SRC_ADDR_LO_src_addr_31_0_offset: c_int = 3;
pub const SDMA_PKT_COPY_DIRTY_PAGE_SRC_ADDR_LO_src_addr_31_0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_DIRTY_PAGE_SRC_ADDR_LO_src_addr_31_0_shift: c_int = 0;

// define for SRC_ADDR_HI word
// define for src_addr_63_32 field
pub const SDMA_PKT_COPY_DIRTY_PAGE_SRC_ADDR_HI_src_addr_63_32_offset: c_int = 4;
pub const SDMA_PKT_COPY_DIRTY_PAGE_SRC_ADDR_HI_src_addr_63_32_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_DIRTY_PAGE_SRC_ADDR_HI_src_addr_63_32_shift: c_int = 0;

// define for DST_ADDR_LO word
// define for dst_addr_31_0 field
pub const SDMA_PKT_COPY_DIRTY_PAGE_DST_ADDR_LO_dst_addr_31_0_offset: c_int = 5;
pub const SDMA_PKT_COPY_DIRTY_PAGE_DST_ADDR_LO_dst_addr_31_0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_DIRTY_PAGE_DST_ADDR_LO_dst_addr_31_0_shift: c_int = 0;

// define for DST_ADDR_HI word
// define for dst_addr_63_32 field
pub const SDMA_PKT_COPY_DIRTY_PAGE_DST_ADDR_HI_dst_addr_63_32_offset: c_int = 6;
pub const SDMA_PKT_COPY_DIRTY_PAGE_DST_ADDR_HI_dst_addr_63_32_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_DIRTY_PAGE_DST_ADDR_HI_dst_addr_63_32_shift: c_int = 0;

//
// Definitions for SDMA_PKT_COPY_PHYSICAL_LINEAR packet
//
// define for HEADER word
// define for op field
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_HEADER_op_offset: c_int = 0;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_HEADER_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_HEADER_op_shift: c_int = 0;

// define for sub_op field
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_HEADER_sub_op_offset: c_int = 0;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_HEADER_sub_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_HEADER_sub_op_shift: c_int = 8;

// define for tmz field
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_HEADER_tmz_offset: c_int = 0;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_HEADER_tmz_mask: c_uint = 0x00000001;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_HEADER_tmz_shift: c_int = 18;

// define for COUNT word
// define for count field
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_COUNT_count_offset: c_int = 1;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_COUNT_count_mask: c_uint = 0x003FFFFF;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_COUNT_count_shift: c_int = 0;

// define for PARAMETER word
// define for dst_sw field
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_dst_sw_offset: c_int = 2;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_dst_sw_mask: c_uint = 0x00000003;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_dst_sw_shift: c_int = 16;

// define for dst_gcc field
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_dst_gcc_offset: c_int = 2;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_dst_gcc_mask: c_uint = 0x00000001;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_dst_gcc_shift: c_int = 19;

// define for dst_sys field
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_dst_sys_offset: c_int = 2;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_dst_sys_mask: c_uint = 0x00000001;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_dst_sys_shift: c_int = 20;

// define for dst_log field
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_dst_log_offset: c_int = 2;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_dst_log_mask: c_uint = 0x00000001;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_dst_log_shift: c_int = 21;

// define for dst_snoop field
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_dst_snoop_offset: c_int = 2;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_dst_snoop_mask: c_uint = 0x00000001;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_dst_snoop_shift: c_int = 22;

// define for dst_gpa field
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_dst_gpa_offset: c_int = 2;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_dst_gpa_mask: c_uint = 0x00000001;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_dst_gpa_shift: c_int = 23;

// define for src_sw field
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_src_sw_offset: c_int = 2;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_src_sw_mask: c_uint = 0x00000003;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_src_sw_shift: c_int = 24;

// define for src_gcc field
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_src_gcc_offset: c_int = 2;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_src_gcc_mask: c_uint = 0x00000001;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_src_gcc_shift: c_int = 27;

// define for src_sys field
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_src_sys_offset: c_int = 2;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_src_sys_mask: c_uint = 0x00000001;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_src_sys_shift: c_int = 28;

// define for src_snoop field
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_src_snoop_offset: c_int = 2;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_src_snoop_mask: c_uint = 0x00000001;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_src_snoop_shift: c_int = 30;

// define for src_gpa field
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_src_gpa_offset: c_int = 2;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_src_gpa_mask: c_uint = 0x00000001;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_src_gpa_shift: c_int = 31;

// define for SRC_ADDR_LO word
// define for src_addr_31_0 field
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_SRC_ADDR_LO_src_addr_31_0_offset: c_int = 3;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_SRC_ADDR_LO_src_addr_31_0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_SRC_ADDR_LO_src_addr_31_0_shift: c_int = 0;

// define for SRC_ADDR_HI word
// define for src_addr_63_32 field
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_SRC_ADDR_HI_src_addr_63_32_offset: c_int = 4;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_SRC_ADDR_HI_src_addr_63_32_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_SRC_ADDR_HI_src_addr_63_32_shift: c_int = 0;

// define for DST_ADDR_LO word
// define for dst_addr_31_0 field
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_DST_ADDR_LO_dst_addr_31_0_offset: c_int = 5;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_DST_ADDR_LO_dst_addr_31_0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_DST_ADDR_LO_dst_addr_31_0_shift: c_int = 0;

// define for DST_ADDR_HI word
// define for dst_addr_63_32 field
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_DST_ADDR_HI_dst_addr_63_32_offset: c_int = 6;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_DST_ADDR_HI_dst_addr_63_32_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_DST_ADDR_HI_dst_addr_63_32_shift: c_int = 0;

//
// Definitions for SDMA_PKT_COPY_BROADCAST_LINEAR packet
//
// define for HEADER word
// define for op field
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_HEADER_op_offset: c_int = 0;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_HEADER_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_HEADER_op_shift: c_int = 0;

// define for sub_op field
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_HEADER_sub_op_offset: c_int = 0;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_HEADER_sub_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_HEADER_sub_op_shift: c_int = 8;

// define for encrypt field
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_HEADER_encrypt_offset: c_int = 0;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_HEADER_encrypt_mask: c_uint = 0x00000001;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_HEADER_encrypt_shift: c_int = 16;

// define for tmz field
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_HEADER_tmz_offset: c_int = 0;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_HEADER_tmz_mask: c_uint = 0x00000001;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_HEADER_tmz_shift: c_int = 18;

// define for broadcast field
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_HEADER_broadcast_offset: c_int = 0;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_HEADER_broadcast_mask: c_uint = 0x00000001;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_HEADER_broadcast_shift: c_int = 27;

// define for COUNT word
// define for count field
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_COUNT_count_offset: c_int = 1;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_COUNT_count_mask: c_uint = 0x003FFFFF;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_COUNT_count_shift: c_int = 0;

// define for PARAMETER word
// define for dst2_sw field
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_PARAMETER_dst2_sw_offset: c_int = 2;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_PARAMETER_dst2_sw_mask: c_uint = 0x00000003;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_PARAMETER_dst2_sw_shift: c_int = 8;

// define for dst1_sw field
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_PARAMETER_dst1_sw_offset: c_int = 2;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_PARAMETER_dst1_sw_mask: c_uint = 0x00000003;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_PARAMETER_dst1_sw_shift: c_int = 16;

// define for src_sw field
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_PARAMETER_src_sw_offset: c_int = 2;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_PARAMETER_src_sw_mask: c_uint = 0x00000003;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_PARAMETER_src_sw_shift: c_int = 24;

// define for SRC_ADDR_LO word
// define for src_addr_31_0 field
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_SRC_ADDR_LO_src_addr_31_0_offset: c_int = 3;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_SRC_ADDR_LO_src_addr_31_0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_SRC_ADDR_LO_src_addr_31_0_shift: c_int = 0;

// define for SRC_ADDR_HI word
// define for src_addr_63_32 field
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_SRC_ADDR_HI_src_addr_63_32_offset: c_int = 4;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_SRC_ADDR_HI_src_addr_63_32_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_SRC_ADDR_HI_src_addr_63_32_shift: c_int = 0;

// define for DST1_ADDR_LO word
// define for dst1_addr_31_0 field
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_DST1_ADDR_LO_dst1_addr_31_0_offset: c_int = 5;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_DST1_ADDR_LO_dst1_addr_31_0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_DST1_ADDR_LO_dst1_addr_31_0_shift: c_int = 0;

// define for DST1_ADDR_HI word
// define for dst1_addr_63_32 field
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_DST1_ADDR_HI_dst1_addr_63_32_offset: c_int = 6;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_DST1_ADDR_HI_dst1_addr_63_32_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_DST1_ADDR_HI_dst1_addr_63_32_shift: c_int = 0;

// define for DST2_ADDR_LO word
// define for dst2_addr_31_0 field
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_DST2_ADDR_LO_dst2_addr_31_0_offset: c_int = 7;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_DST2_ADDR_LO_dst2_addr_31_0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_DST2_ADDR_LO_dst2_addr_31_0_shift: c_int = 0;

// define for DST2_ADDR_HI word
// define for dst2_addr_63_32 field
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_DST2_ADDR_HI_dst2_addr_63_32_offset: c_int = 8;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_DST2_ADDR_HI_dst2_addr_63_32_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_DST2_ADDR_HI_dst2_addr_63_32_shift: c_int = 0;

//
// Definitions for SDMA_PKT_COPY_LINEAR_SUBWIN packet
//
// define for HEADER word
// define for op field
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_HEADER_op_offset: c_int = 0;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_HEADER_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_HEADER_op_shift: c_int = 0;

// define for sub_op field
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_HEADER_sub_op_offset: c_int = 0;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_HEADER_sub_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_HEADER_sub_op_shift: c_int = 8;

// define for tmz field
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_HEADER_tmz_offset: c_int = 0;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_HEADER_tmz_mask: c_uint = 0x00000001;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_HEADER_tmz_shift: c_int = 18;

// define for elementsize field
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_HEADER_elementsize_offset: c_int = 0;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_HEADER_elementsize_mask: c_uint = 0x00000007;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_HEADER_elementsize_shift: c_int = 29;

// define for SRC_ADDR_LO word
// define for src_addr_31_0 field
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_SRC_ADDR_LO_src_addr_31_0_offset: c_int = 1;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_SRC_ADDR_LO_src_addr_31_0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_SRC_ADDR_LO_src_addr_31_0_shift: c_int = 0;

// define for SRC_ADDR_HI word
// define for src_addr_63_32 field
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_SRC_ADDR_HI_src_addr_63_32_offset: c_int = 2;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_SRC_ADDR_HI_src_addr_63_32_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_SRC_ADDR_HI_src_addr_63_32_shift: c_int = 0;

// define for DW_3 word
// define for src_x field
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_3_src_x_offset: c_int = 3;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_3_src_x_mask: c_uint = 0x00003FFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_3_src_x_shift: c_int = 0;

// define for src_y field
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_3_src_y_offset: c_int = 3;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_3_src_y_mask: c_uint = 0x00003FFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_3_src_y_shift: c_int = 16;

// define for DW_4 word
// define for src_z field
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_4_src_z_offset: c_int = 4;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_4_src_z_mask: c_uint = 0x000007FF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_4_src_z_shift: c_int = 0;

// define for src_pitch field
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_4_src_pitch_offset: c_int = 4;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_4_src_pitch_mask: c_uint = 0x0007FFFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_4_src_pitch_shift: c_int = 13;

// define for DW_5 word
// define for src_slice_pitch field
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_5_src_slice_pitch_offset: c_int = 5;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_5_src_slice_pitch_mask: c_uint = 0x0FFFFFFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_5_src_slice_pitch_shift: c_int = 0;

// define for DST_ADDR_LO word
// define for dst_addr_31_0 field
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DST_ADDR_LO_dst_addr_31_0_offset: c_int = 6;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DST_ADDR_LO_dst_addr_31_0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DST_ADDR_LO_dst_addr_31_0_shift: c_int = 0;

// define for DST_ADDR_HI word
// define for dst_addr_63_32 field
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DST_ADDR_HI_dst_addr_63_32_offset: c_int = 7;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DST_ADDR_HI_dst_addr_63_32_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DST_ADDR_HI_dst_addr_63_32_shift: c_int = 0;

// define for DW_8 word
// define for dst_x field
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_8_dst_x_offset: c_int = 8;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_8_dst_x_mask: c_uint = 0x00003FFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_8_dst_x_shift: c_int = 0;

// define for dst_y field
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_8_dst_y_offset: c_int = 8;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_8_dst_y_mask: c_uint = 0x00003FFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_8_dst_y_shift: c_int = 16;

// define for DW_9 word
// define for dst_z field
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_9_dst_z_offset: c_int = 9;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_9_dst_z_mask: c_uint = 0x000007FF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_9_dst_z_shift: c_int = 0;

// define for dst_pitch field
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_9_dst_pitch_offset: c_int = 9;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_9_dst_pitch_mask: c_uint = 0x0007FFFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_9_dst_pitch_shift: c_int = 13;

// define for DW_10 word
// define for dst_slice_pitch field
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_10_dst_slice_pitch_offset: c_int = 10;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_10_dst_slice_pitch_mask: c_uint = 0x0FFFFFFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_10_dst_slice_pitch_shift: c_int = 0;

// define for DW_11 word
// define for rect_x field
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_11_rect_x_offset: c_int = 11;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_11_rect_x_mask: c_uint = 0x00003FFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_11_rect_x_shift: c_int = 0;

// define for rect_y field
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_11_rect_y_offset: c_int = 11;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_11_rect_y_mask: c_uint = 0x00003FFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_11_rect_y_shift: c_int = 16;

// define for DW_12 word
// define for rect_z field
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_12_rect_z_offset: c_int = 12;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_12_rect_z_mask: c_uint = 0x000007FF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_12_rect_z_shift: c_int = 0;

// define for dst_sw field
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_12_dst_sw_offset: c_int = 12;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_12_dst_sw_mask: c_uint = 0x00000003;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_12_dst_sw_shift: c_int = 16;

// define for src_sw field
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_12_src_sw_offset: c_int = 12;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_12_src_sw_mask: c_uint = 0x00000003;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_12_src_sw_shift: c_int = 24;

//
// Definitions for SDMA_PKT_COPY_TILED packet
//
// define for HEADER word
// define for op field
pub const SDMA_PKT_COPY_TILED_HEADER_op_offset: c_int = 0;
pub const SDMA_PKT_COPY_TILED_HEADER_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_COPY_TILED_HEADER_op_shift: c_int = 0;

// define for sub_op field
pub const SDMA_PKT_COPY_TILED_HEADER_sub_op_offset: c_int = 0;
pub const SDMA_PKT_COPY_TILED_HEADER_sub_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_COPY_TILED_HEADER_sub_op_shift: c_int = 8;

// define for encrypt field
pub const SDMA_PKT_COPY_TILED_HEADER_encrypt_offset: c_int = 0;
pub const SDMA_PKT_COPY_TILED_HEADER_encrypt_mask: c_uint = 0x00000001;
pub const SDMA_PKT_COPY_TILED_HEADER_encrypt_shift: c_int = 16;

// define for tmz field
pub const SDMA_PKT_COPY_TILED_HEADER_tmz_offset: c_int = 0;
pub const SDMA_PKT_COPY_TILED_HEADER_tmz_mask: c_uint = 0x00000001;
pub const SDMA_PKT_COPY_TILED_HEADER_tmz_shift: c_int = 18;

// define for mip_max field
pub const SDMA_PKT_COPY_TILED_HEADER_mip_max_offset: c_int = 0;
pub const SDMA_PKT_COPY_TILED_HEADER_mip_max_mask: c_uint = 0x0000000F;
pub const SDMA_PKT_COPY_TILED_HEADER_mip_max_shift: c_int = 20;

// define for detile field
pub const SDMA_PKT_COPY_TILED_HEADER_detile_offset: c_int = 0;
pub const SDMA_PKT_COPY_TILED_HEADER_detile_mask: c_uint = 0x00000001;
pub const SDMA_PKT_COPY_TILED_HEADER_detile_shift: c_int = 31;

// define for TILED_ADDR_LO word
// define for tiled_addr_31_0 field
pub const SDMA_PKT_COPY_TILED_TILED_ADDR_LO_tiled_addr_31_0_offset: c_int = 1;
pub const SDMA_PKT_COPY_TILED_TILED_ADDR_LO_tiled_addr_31_0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_TILED_TILED_ADDR_LO_tiled_addr_31_0_shift: c_int = 0;

// define for TILED_ADDR_HI word
// define for tiled_addr_63_32 field
pub const SDMA_PKT_COPY_TILED_TILED_ADDR_HI_tiled_addr_63_32_offset: c_int = 2;
pub const SDMA_PKT_COPY_TILED_TILED_ADDR_HI_tiled_addr_63_32_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_TILED_TILED_ADDR_HI_tiled_addr_63_32_shift: c_int = 0;

// define for DW_3 word
// define for width field
pub const SDMA_PKT_COPY_TILED_DW_3_width_offset: c_int = 3;
pub const SDMA_PKT_COPY_TILED_DW_3_width_mask: c_uint = 0x00003FFF;
pub const SDMA_PKT_COPY_TILED_DW_3_width_shift: c_int = 0;

// define for DW_4 word
// define for height field
pub const SDMA_PKT_COPY_TILED_DW_4_height_offset: c_int = 4;
pub const SDMA_PKT_COPY_TILED_DW_4_height_mask: c_uint = 0x00003FFF;
pub const SDMA_PKT_COPY_TILED_DW_4_height_shift: c_int = 0;

// define for depth field
pub const SDMA_PKT_COPY_TILED_DW_4_depth_offset: c_int = 4;
pub const SDMA_PKT_COPY_TILED_DW_4_depth_mask: c_uint = 0x000007FF;
pub const SDMA_PKT_COPY_TILED_DW_4_depth_shift: c_int = 16;

// define for DW_5 word
// define for element_size field
pub const SDMA_PKT_COPY_TILED_DW_5_element_size_offset: c_int = 5;
pub const SDMA_PKT_COPY_TILED_DW_5_element_size_mask: c_uint = 0x00000007;
pub const SDMA_PKT_COPY_TILED_DW_5_element_size_shift: c_int = 0;

// define for swizzle_mode field
pub const SDMA_PKT_COPY_TILED_DW_5_swizzle_mode_offset: c_int = 5;
pub const SDMA_PKT_COPY_TILED_DW_5_swizzle_mode_mask: c_uint = 0x0000001F;
pub const SDMA_PKT_COPY_TILED_DW_5_swizzle_mode_shift: c_int = 3;

// define for dimension field
pub const SDMA_PKT_COPY_TILED_DW_5_dimension_offset: c_int = 5;
pub const SDMA_PKT_COPY_TILED_DW_5_dimension_mask: c_uint = 0x00000003;
pub const SDMA_PKT_COPY_TILED_DW_5_dimension_shift: c_int = 9;

// define for epitch field
pub const SDMA_PKT_COPY_TILED_DW_5_epitch_offset: c_int = 5;
pub const SDMA_PKT_COPY_TILED_DW_5_epitch_mask: c_uint = 0x0000FFFF;
pub const SDMA_PKT_COPY_TILED_DW_5_epitch_shift: c_int = 16;

// define for DW_6 word
// define for x field
pub const SDMA_PKT_COPY_TILED_DW_6_x_offset: c_int = 6;
pub const SDMA_PKT_COPY_TILED_DW_6_x_mask: c_uint = 0x00003FFF;
pub const SDMA_PKT_COPY_TILED_DW_6_x_shift: c_int = 0;

// define for y field
pub const SDMA_PKT_COPY_TILED_DW_6_y_offset: c_int = 6;
pub const SDMA_PKT_COPY_TILED_DW_6_y_mask: c_uint = 0x00003FFF;
pub const SDMA_PKT_COPY_TILED_DW_6_y_shift: c_int = 16;

// define for DW_7 word
// define for z field
pub const SDMA_PKT_COPY_TILED_DW_7_z_offset: c_int = 7;
pub const SDMA_PKT_COPY_TILED_DW_7_z_mask: c_uint = 0x000007FF;
pub const SDMA_PKT_COPY_TILED_DW_7_z_shift: c_int = 0;

// define for linear_sw field
pub const SDMA_PKT_COPY_TILED_DW_7_linear_sw_offset: c_int = 7;
pub const SDMA_PKT_COPY_TILED_DW_7_linear_sw_mask: c_uint = 0x00000003;
pub const SDMA_PKT_COPY_TILED_DW_7_linear_sw_shift: c_int = 16;

// define for tile_sw field
pub const SDMA_PKT_COPY_TILED_DW_7_tile_sw_offset: c_int = 7;
pub const SDMA_PKT_COPY_TILED_DW_7_tile_sw_mask: c_uint = 0x00000003;
pub const SDMA_PKT_COPY_TILED_DW_7_tile_sw_shift: c_int = 24;

// define for LINEAR_ADDR_LO word
// define for linear_addr_31_0 field
pub const SDMA_PKT_COPY_TILED_LINEAR_ADDR_LO_linear_addr_31_0_offset: c_int = 8;
pub const SDMA_PKT_COPY_TILED_LINEAR_ADDR_LO_linear_addr_31_0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_TILED_LINEAR_ADDR_LO_linear_addr_31_0_shift: c_int = 0;

// define for LINEAR_ADDR_HI word
// define for linear_addr_63_32 field
pub const SDMA_PKT_COPY_TILED_LINEAR_ADDR_HI_linear_addr_63_32_offset: c_int = 9;
pub const SDMA_PKT_COPY_TILED_LINEAR_ADDR_HI_linear_addr_63_32_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_TILED_LINEAR_ADDR_HI_linear_addr_63_32_shift: c_int = 0;

// define for LINEAR_PITCH word
// define for linear_pitch field
pub const SDMA_PKT_COPY_TILED_LINEAR_PITCH_linear_pitch_offset: c_int = 10;
pub const SDMA_PKT_COPY_TILED_LINEAR_PITCH_linear_pitch_mask: c_uint = 0x0007FFFF;
pub const SDMA_PKT_COPY_TILED_LINEAR_PITCH_linear_pitch_shift: c_int = 0;

// define for LINEAR_SLICE_PITCH word
// define for linear_slice_pitch field
pub const SDMA_PKT_COPY_TILED_LINEAR_SLICE_PITCH_linear_slice_pitch_offset: c_int = 11;
pub const SDMA_PKT_COPY_TILED_LINEAR_SLICE_PITCH_linear_slice_pitch_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_TILED_LINEAR_SLICE_PITCH_linear_slice_pitch_shift: c_int = 0;

// define for COUNT word
// define for count field
pub const SDMA_PKT_COPY_TILED_COUNT_count_offset: c_int = 12;
pub const SDMA_PKT_COPY_TILED_COUNT_count_mask: c_uint = 0x000FFFFF;
pub const SDMA_PKT_COPY_TILED_COUNT_count_shift: c_int = 0;

//
// Definitions for SDMA_PKT_COPY_L2T_BROADCAST packet
//
// define for HEADER word
// define for op field
pub const SDMA_PKT_COPY_L2T_BROADCAST_HEADER_op_offset: c_int = 0;
pub const SDMA_PKT_COPY_L2T_BROADCAST_HEADER_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_COPY_L2T_BROADCAST_HEADER_op_shift: c_int = 0;

// define for sub_op field
pub const SDMA_PKT_COPY_L2T_BROADCAST_HEADER_sub_op_offset: c_int = 0;
pub const SDMA_PKT_COPY_L2T_BROADCAST_HEADER_sub_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_COPY_L2T_BROADCAST_HEADER_sub_op_shift: c_int = 8;

// define for encrypt field
pub const SDMA_PKT_COPY_L2T_BROADCAST_HEADER_encrypt_offset: c_int = 0;
pub const SDMA_PKT_COPY_L2T_BROADCAST_HEADER_encrypt_mask: c_uint = 0x00000001;
pub const SDMA_PKT_COPY_L2T_BROADCAST_HEADER_encrypt_shift: c_int = 16;

// define for tmz field
pub const SDMA_PKT_COPY_L2T_BROADCAST_HEADER_tmz_offset: c_int = 0;
pub const SDMA_PKT_COPY_L2T_BROADCAST_HEADER_tmz_mask: c_uint = 0x00000001;
pub const SDMA_PKT_COPY_L2T_BROADCAST_HEADER_tmz_shift: c_int = 18;

// define for mip_max field
pub const SDMA_PKT_COPY_L2T_BROADCAST_HEADER_mip_max_offset: c_int = 0;
pub const SDMA_PKT_COPY_L2T_BROADCAST_HEADER_mip_max_mask: c_uint = 0x0000000F;
pub const SDMA_PKT_COPY_L2T_BROADCAST_HEADER_mip_max_shift: c_int = 20;

// define for videocopy field
pub const SDMA_PKT_COPY_L2T_BROADCAST_HEADER_videocopy_offset: c_int = 0;
pub const SDMA_PKT_COPY_L2T_BROADCAST_HEADER_videocopy_mask: c_uint = 0x00000001;
pub const SDMA_PKT_COPY_L2T_BROADCAST_HEADER_videocopy_shift: c_int = 26;

// define for broadcast field
pub const SDMA_PKT_COPY_L2T_BROADCAST_HEADER_broadcast_offset: c_int = 0;
pub const SDMA_PKT_COPY_L2T_BROADCAST_HEADER_broadcast_mask: c_uint = 0x00000001;
pub const SDMA_PKT_COPY_L2T_BROADCAST_HEADER_broadcast_shift: c_int = 27;

// define for TILED_ADDR_LO_0 word
// define for tiled_addr0_31_0 field
pub const SDMA_PKT_COPY_L2T_BROADCAST_TILED_ADDR_LO_0_tiled_addr0_31_0_offset: c_int = 1;
pub const SDMA_PKT_COPY_L2T_BROADCAST_TILED_ADDR_LO_0_tiled_addr0_31_0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_L2T_BROADCAST_TILED_ADDR_LO_0_tiled_addr0_31_0_shift: c_int = 0;

// define for TILED_ADDR_HI_0 word
// define for tiled_addr0_63_32 field
pub const SDMA_PKT_COPY_L2T_BROADCAST_TILED_ADDR_HI_0_tiled_addr0_63_32_offset: c_int = 2;
pub const SDMA_PKT_COPY_L2T_BROADCAST_TILED_ADDR_HI_0_tiled_addr0_63_32_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_L2T_BROADCAST_TILED_ADDR_HI_0_tiled_addr0_63_32_shift: c_int = 0;

// define for TILED_ADDR_LO_1 word
// define for tiled_addr1_31_0 field
pub const SDMA_PKT_COPY_L2T_BROADCAST_TILED_ADDR_LO_1_tiled_addr1_31_0_offset: c_int = 3;
pub const SDMA_PKT_COPY_L2T_BROADCAST_TILED_ADDR_LO_1_tiled_addr1_31_0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_L2T_BROADCAST_TILED_ADDR_LO_1_tiled_addr1_31_0_shift: c_int = 0;

// define for TILED_ADDR_HI_1 word
// define for tiled_addr1_63_32 field
pub const SDMA_PKT_COPY_L2T_BROADCAST_TILED_ADDR_HI_1_tiled_addr1_63_32_offset: c_int = 4;
pub const SDMA_PKT_COPY_L2T_BROADCAST_TILED_ADDR_HI_1_tiled_addr1_63_32_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_L2T_BROADCAST_TILED_ADDR_HI_1_tiled_addr1_63_32_shift: c_int = 0;

// define for DW_5 word
// define for width field
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_5_width_offset: c_int = 5;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_5_width_mask: c_uint = 0x00003FFF;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_5_width_shift: c_int = 0;

// define for DW_6 word
// define for height field
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_6_height_offset: c_int = 6;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_6_height_mask: c_uint = 0x00003FFF;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_6_height_shift: c_int = 0;

// define for depth field
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_6_depth_offset: c_int = 6;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_6_depth_mask: c_uint = 0x000007FF;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_6_depth_shift: c_int = 16;

// define for DW_7 word
// define for element_size field
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_7_element_size_offset: c_int = 7;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_7_element_size_mask: c_uint = 0x00000007;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_7_element_size_shift: c_int = 0;

// define for swizzle_mode field
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_7_swizzle_mode_offset: c_int = 7;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_7_swizzle_mode_mask: c_uint = 0x0000001F;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_7_swizzle_mode_shift: c_int = 3;

// define for dimension field
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_7_dimension_offset: c_int = 7;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_7_dimension_mask: c_uint = 0x00000003;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_7_dimension_shift: c_int = 9;

// define for epitch field
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_7_epitch_offset: c_int = 7;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_7_epitch_mask: c_uint = 0x0000FFFF;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_7_epitch_shift: c_int = 16;

// define for DW_8 word
// define for x field
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_8_x_offset: c_int = 8;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_8_x_mask: c_uint = 0x00003FFF;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_8_x_shift: c_int = 0;

// define for y field
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_8_y_offset: c_int = 8;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_8_y_mask: c_uint = 0x00003FFF;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_8_y_shift: c_int = 16;

// define for DW_9 word
// define for z field
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_9_z_offset: c_int = 9;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_9_z_mask: c_uint = 0x000007FF;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_9_z_shift: c_int = 0;

// define for DW_10 word
// define for dst2_sw field
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_10_dst2_sw_offset: c_int = 10;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_10_dst2_sw_mask: c_uint = 0x00000003;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_10_dst2_sw_shift: c_int = 8;

// define for linear_sw field
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_10_linear_sw_offset: c_int = 10;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_10_linear_sw_mask: c_uint = 0x00000003;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_10_linear_sw_shift: c_int = 16;

// define for tile_sw field
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_10_tile_sw_offset: c_int = 10;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_10_tile_sw_mask: c_uint = 0x00000003;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_10_tile_sw_shift: c_int = 24;

// define for LINEAR_ADDR_LO word
// define for linear_addr_31_0 field
pub const SDMA_PKT_COPY_L2T_BROADCAST_LINEAR_ADDR_LO_linear_addr_31_0_offset: c_int = 11;
pub const SDMA_PKT_COPY_L2T_BROADCAST_LINEAR_ADDR_LO_linear_addr_31_0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_L2T_BROADCAST_LINEAR_ADDR_LO_linear_addr_31_0_shift: c_int = 0;

// define for LINEAR_ADDR_HI word
// define for linear_addr_63_32 field
pub const SDMA_PKT_COPY_L2T_BROADCAST_LINEAR_ADDR_HI_linear_addr_63_32_offset: c_int = 12;
pub const SDMA_PKT_COPY_L2T_BROADCAST_LINEAR_ADDR_HI_linear_addr_63_32_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_L2T_BROADCAST_LINEAR_ADDR_HI_linear_addr_63_32_shift: c_int = 0;

// define for LINEAR_PITCH word
// define for linear_pitch field
pub const SDMA_PKT_COPY_L2T_BROADCAST_LINEAR_PITCH_linear_pitch_offset: c_int = 13;
pub const SDMA_PKT_COPY_L2T_BROADCAST_LINEAR_PITCH_linear_pitch_mask: c_uint = 0x0007FFFF;
pub const SDMA_PKT_COPY_L2T_BROADCAST_LINEAR_PITCH_linear_pitch_shift: c_int = 0;

// define for LINEAR_SLICE_PITCH word
// define for linear_slice_pitch field
pub const SDMA_PKT_COPY_L2T_BROADCAST_LINEAR_SLICE_PITCH_linear_slice_pitch_offset: c_int = 14;
pub const SDMA_PKT_COPY_L2T_BROADCAST_LINEAR_SLICE_PITCH_linear_slice_pitch_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_L2T_BROADCAST_LINEAR_SLICE_PITCH_linear_slice_pitch_shift: c_int = 0;

// define for COUNT word
// define for count field
pub const SDMA_PKT_COPY_L2T_BROADCAST_COUNT_count_offset: c_int = 15;
pub const SDMA_PKT_COPY_L2T_BROADCAST_COUNT_count_mask: c_uint = 0x000FFFFF;
pub const SDMA_PKT_COPY_L2T_BROADCAST_COUNT_count_shift: c_int = 0;

//
// Definitions for SDMA_PKT_COPY_T2T packet
//
// define for HEADER word
// define for op field
pub const SDMA_PKT_COPY_T2T_HEADER_op_offset: c_int = 0;
pub const SDMA_PKT_COPY_T2T_HEADER_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_COPY_T2T_HEADER_op_shift: c_int = 0;

// define for sub_op field
pub const SDMA_PKT_COPY_T2T_HEADER_sub_op_offset: c_int = 0;
pub const SDMA_PKT_COPY_T2T_HEADER_sub_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_COPY_T2T_HEADER_sub_op_shift: c_int = 8;

// define for tmz field
pub const SDMA_PKT_COPY_T2T_HEADER_tmz_offset: c_int = 0;
pub const SDMA_PKT_COPY_T2T_HEADER_tmz_mask: c_uint = 0x00000001;
pub const SDMA_PKT_COPY_T2T_HEADER_tmz_shift: c_int = 18;

// define for mip_max field
pub const SDMA_PKT_COPY_T2T_HEADER_mip_max_offset: c_int = 0;
pub const SDMA_PKT_COPY_T2T_HEADER_mip_max_mask: c_uint = 0x0000000F;
pub const SDMA_PKT_COPY_T2T_HEADER_mip_max_shift: c_int = 20;

// define for SRC_ADDR_LO word
// define for src_addr_31_0 field
pub const SDMA_PKT_COPY_T2T_SRC_ADDR_LO_src_addr_31_0_offset: c_int = 1;
pub const SDMA_PKT_COPY_T2T_SRC_ADDR_LO_src_addr_31_0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_T2T_SRC_ADDR_LO_src_addr_31_0_shift: c_int = 0;

// define for SRC_ADDR_HI word
// define for src_addr_63_32 field
pub const SDMA_PKT_COPY_T2T_SRC_ADDR_HI_src_addr_63_32_offset: c_int = 2;
pub const SDMA_PKT_COPY_T2T_SRC_ADDR_HI_src_addr_63_32_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_T2T_SRC_ADDR_HI_src_addr_63_32_shift: c_int = 0;

// define for DW_3 word
// define for src_x field
pub const SDMA_PKT_COPY_T2T_DW_3_src_x_offset: c_int = 3;
pub const SDMA_PKT_COPY_T2T_DW_3_src_x_mask: c_uint = 0x00003FFF;
pub const SDMA_PKT_COPY_T2T_DW_3_src_x_shift: c_int = 0;

// define for src_y field
pub const SDMA_PKT_COPY_T2T_DW_3_src_y_offset: c_int = 3;
pub const SDMA_PKT_COPY_T2T_DW_3_src_y_mask: c_uint = 0x00003FFF;
pub const SDMA_PKT_COPY_T2T_DW_3_src_y_shift: c_int = 16;

// define for DW_4 word
// define for src_z field
pub const SDMA_PKT_COPY_T2T_DW_4_src_z_offset: c_int = 4;
pub const SDMA_PKT_COPY_T2T_DW_4_src_z_mask: c_uint = 0x000007FF;
pub const SDMA_PKT_COPY_T2T_DW_4_src_z_shift: c_int = 0;

// define for src_width field
pub const SDMA_PKT_COPY_T2T_DW_4_src_width_offset: c_int = 4;
pub const SDMA_PKT_COPY_T2T_DW_4_src_width_mask: c_uint = 0x00003FFF;
pub const SDMA_PKT_COPY_T2T_DW_4_src_width_shift: c_int = 16;

// define for DW_5 word
// define for src_height field
pub const SDMA_PKT_COPY_T2T_DW_5_src_height_offset: c_int = 5;
pub const SDMA_PKT_COPY_T2T_DW_5_src_height_mask: c_uint = 0x00003FFF;
pub const SDMA_PKT_COPY_T2T_DW_5_src_height_shift: c_int = 0;

// define for src_depth field
pub const SDMA_PKT_COPY_T2T_DW_5_src_depth_offset: c_int = 5;
pub const SDMA_PKT_COPY_T2T_DW_5_src_depth_mask: c_uint = 0x000007FF;
pub const SDMA_PKT_COPY_T2T_DW_5_src_depth_shift: c_int = 16;

// define for DW_6 word
// define for src_element_size field
pub const SDMA_PKT_COPY_T2T_DW_6_src_element_size_offset: c_int = 6;
pub const SDMA_PKT_COPY_T2T_DW_6_src_element_size_mask: c_uint = 0x00000007;
pub const SDMA_PKT_COPY_T2T_DW_6_src_element_size_shift: c_int = 0;

// define for src_swizzle_mode field
pub const SDMA_PKT_COPY_T2T_DW_6_src_swizzle_mode_offset: c_int = 6;
pub const SDMA_PKT_COPY_T2T_DW_6_src_swizzle_mode_mask: c_uint = 0x0000001F;
pub const SDMA_PKT_COPY_T2T_DW_6_src_swizzle_mode_shift: c_int = 3;

// define for src_dimension field
pub const SDMA_PKT_COPY_T2T_DW_6_src_dimension_offset: c_int = 6;
pub const SDMA_PKT_COPY_T2T_DW_6_src_dimension_mask: c_uint = 0x00000003;
pub const SDMA_PKT_COPY_T2T_DW_6_src_dimension_shift: c_int = 9;

// define for src_epitch field
pub const SDMA_PKT_COPY_T2T_DW_6_src_epitch_offset: c_int = 6;
pub const SDMA_PKT_COPY_T2T_DW_6_src_epitch_mask: c_uint = 0x0000FFFF;
pub const SDMA_PKT_COPY_T2T_DW_6_src_epitch_shift: c_int = 16;

// define for DST_ADDR_LO word
// define for dst_addr_31_0 field
pub const SDMA_PKT_COPY_T2T_DST_ADDR_LO_dst_addr_31_0_offset: c_int = 7;
pub const SDMA_PKT_COPY_T2T_DST_ADDR_LO_dst_addr_31_0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_T2T_DST_ADDR_LO_dst_addr_31_0_shift: c_int = 0;

// define for DST_ADDR_HI word
// define for dst_addr_63_32 field
pub const SDMA_PKT_COPY_T2T_DST_ADDR_HI_dst_addr_63_32_offset: c_int = 8;
pub const SDMA_PKT_COPY_T2T_DST_ADDR_HI_dst_addr_63_32_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_T2T_DST_ADDR_HI_dst_addr_63_32_shift: c_int = 0;

// define for DW_9 word
// define for dst_x field
pub const SDMA_PKT_COPY_T2T_DW_9_dst_x_offset: c_int = 9;
pub const SDMA_PKT_COPY_T2T_DW_9_dst_x_mask: c_uint = 0x00003FFF;
pub const SDMA_PKT_COPY_T2T_DW_9_dst_x_shift: c_int = 0;

// define for dst_y field
pub const SDMA_PKT_COPY_T2T_DW_9_dst_y_offset: c_int = 9;
pub const SDMA_PKT_COPY_T2T_DW_9_dst_y_mask: c_uint = 0x00003FFF;
pub const SDMA_PKT_COPY_T2T_DW_9_dst_y_shift: c_int = 16;

// define for DW_10 word
// define for dst_z field
pub const SDMA_PKT_COPY_T2T_DW_10_dst_z_offset: c_int = 10;
pub const SDMA_PKT_COPY_T2T_DW_10_dst_z_mask: c_uint = 0x000007FF;
pub const SDMA_PKT_COPY_T2T_DW_10_dst_z_shift: c_int = 0;

// define for dst_width field
pub const SDMA_PKT_COPY_T2T_DW_10_dst_width_offset: c_int = 10;
pub const SDMA_PKT_COPY_T2T_DW_10_dst_width_mask: c_uint = 0x00003FFF;
pub const SDMA_PKT_COPY_T2T_DW_10_dst_width_shift: c_int = 16;

// define for DW_11 word
// define for dst_height field
pub const SDMA_PKT_COPY_T2T_DW_11_dst_height_offset: c_int = 11;
pub const SDMA_PKT_COPY_T2T_DW_11_dst_height_mask: c_uint = 0x00003FFF;
pub const SDMA_PKT_COPY_T2T_DW_11_dst_height_shift: c_int = 0;

// define for dst_depth field
pub const SDMA_PKT_COPY_T2T_DW_11_dst_depth_offset: c_int = 11;
pub const SDMA_PKT_COPY_T2T_DW_11_dst_depth_mask: c_uint = 0x000007FF;
pub const SDMA_PKT_COPY_T2T_DW_11_dst_depth_shift: c_int = 16;

// define for DW_12 word
// define for dst_element_size field
pub const SDMA_PKT_COPY_T2T_DW_12_dst_element_size_offset: c_int = 12;
pub const SDMA_PKT_COPY_T2T_DW_12_dst_element_size_mask: c_uint = 0x00000007;
pub const SDMA_PKT_COPY_T2T_DW_12_dst_element_size_shift: c_int = 0;

// define for dst_swizzle_mode field
pub const SDMA_PKT_COPY_T2T_DW_12_dst_swizzle_mode_offset: c_int = 12;
pub const SDMA_PKT_COPY_T2T_DW_12_dst_swizzle_mode_mask: c_uint = 0x0000001F;
pub const SDMA_PKT_COPY_T2T_DW_12_dst_swizzle_mode_shift: c_int = 3;

// define for dst_dimension field
pub const SDMA_PKT_COPY_T2T_DW_12_dst_dimension_offset: c_int = 12;
pub const SDMA_PKT_COPY_T2T_DW_12_dst_dimension_mask: c_uint = 0x00000003;
pub const SDMA_PKT_COPY_T2T_DW_12_dst_dimension_shift: c_int = 9;

// define for dst_epitch field
pub const SDMA_PKT_COPY_T2T_DW_12_dst_epitch_offset: c_int = 12;
pub const SDMA_PKT_COPY_T2T_DW_12_dst_epitch_mask: c_uint = 0x0000FFFF;
pub const SDMA_PKT_COPY_T2T_DW_12_dst_epitch_shift: c_int = 16;

// define for DW_13 word
// define for rect_x field
pub const SDMA_PKT_COPY_T2T_DW_13_rect_x_offset: c_int = 13;
pub const SDMA_PKT_COPY_T2T_DW_13_rect_x_mask: c_uint = 0x00003FFF;
pub const SDMA_PKT_COPY_T2T_DW_13_rect_x_shift: c_int = 0;

// define for rect_y field
pub const SDMA_PKT_COPY_T2T_DW_13_rect_y_offset: c_int = 13;
pub const SDMA_PKT_COPY_T2T_DW_13_rect_y_mask: c_uint = 0x00003FFF;
pub const SDMA_PKT_COPY_T2T_DW_13_rect_y_shift: c_int = 16;

// define for DW_14 word
// define for rect_z field
pub const SDMA_PKT_COPY_T2T_DW_14_rect_z_offset: c_int = 14;
pub const SDMA_PKT_COPY_T2T_DW_14_rect_z_mask: c_uint = 0x000007FF;
pub const SDMA_PKT_COPY_T2T_DW_14_rect_z_shift: c_int = 0;

// define for dst_sw field
pub const SDMA_PKT_COPY_T2T_DW_14_dst_sw_offset: c_int = 14;
pub const SDMA_PKT_COPY_T2T_DW_14_dst_sw_mask: c_uint = 0x00000003;
pub const SDMA_PKT_COPY_T2T_DW_14_dst_sw_shift: c_int = 16;

// define for src_sw field
pub const SDMA_PKT_COPY_T2T_DW_14_src_sw_offset: c_int = 14;
pub const SDMA_PKT_COPY_T2T_DW_14_src_sw_mask: c_uint = 0x00000003;
pub const SDMA_PKT_COPY_T2T_DW_14_src_sw_shift: c_int = 24;

//
// Definitions for SDMA_PKT_COPY_TILED_SUBWIN packet
//
// define for HEADER word
// define for op field
pub const SDMA_PKT_COPY_TILED_SUBWIN_HEADER_op_offset: c_int = 0;
pub const SDMA_PKT_COPY_TILED_SUBWIN_HEADER_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_COPY_TILED_SUBWIN_HEADER_op_shift: c_int = 0;

// define for sub_op field
pub const SDMA_PKT_COPY_TILED_SUBWIN_HEADER_sub_op_offset: c_int = 0;
pub const SDMA_PKT_COPY_TILED_SUBWIN_HEADER_sub_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_COPY_TILED_SUBWIN_HEADER_sub_op_shift: c_int = 8;

// define for tmz field
pub const SDMA_PKT_COPY_TILED_SUBWIN_HEADER_tmz_offset: c_int = 0;
pub const SDMA_PKT_COPY_TILED_SUBWIN_HEADER_tmz_mask: c_uint = 0x00000001;
pub const SDMA_PKT_COPY_TILED_SUBWIN_HEADER_tmz_shift: c_int = 18;

// define for mip_max field
pub const SDMA_PKT_COPY_TILED_SUBWIN_HEADER_mip_max_offset: c_int = 0;
pub const SDMA_PKT_COPY_TILED_SUBWIN_HEADER_mip_max_mask: c_uint = 0x0000000F;
pub const SDMA_PKT_COPY_TILED_SUBWIN_HEADER_mip_max_shift: c_int = 20;

// define for mip_id field
pub const SDMA_PKT_COPY_TILED_SUBWIN_HEADER_mip_id_offset: c_int = 0;
pub const SDMA_PKT_COPY_TILED_SUBWIN_HEADER_mip_id_mask: c_uint = 0x0000000F;
pub const SDMA_PKT_COPY_TILED_SUBWIN_HEADER_mip_id_shift: c_int = 24;

// define for detile field
pub const SDMA_PKT_COPY_TILED_SUBWIN_HEADER_detile_offset: c_int = 0;
pub const SDMA_PKT_COPY_TILED_SUBWIN_HEADER_detile_mask: c_uint = 0x00000001;
pub const SDMA_PKT_COPY_TILED_SUBWIN_HEADER_detile_shift: c_int = 31;

// define for TILED_ADDR_LO word
// define for tiled_addr_31_0 field
pub const SDMA_PKT_COPY_TILED_SUBWIN_TILED_ADDR_LO_tiled_addr_31_0_offset: c_int = 1;
pub const SDMA_PKT_COPY_TILED_SUBWIN_TILED_ADDR_LO_tiled_addr_31_0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_TILED_SUBWIN_TILED_ADDR_LO_tiled_addr_31_0_shift: c_int = 0;

// define for TILED_ADDR_HI word
// define for tiled_addr_63_32 field
pub const SDMA_PKT_COPY_TILED_SUBWIN_TILED_ADDR_HI_tiled_addr_63_32_offset: c_int = 2;
pub const SDMA_PKT_COPY_TILED_SUBWIN_TILED_ADDR_HI_tiled_addr_63_32_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_TILED_SUBWIN_TILED_ADDR_HI_tiled_addr_63_32_shift: c_int = 0;

// define for DW_3 word
// define for tiled_x field
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_3_tiled_x_offset: c_int = 3;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_3_tiled_x_mask: c_uint = 0x00003FFF;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_3_tiled_x_shift: c_int = 0;

// define for tiled_y field
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_3_tiled_y_offset: c_int = 3;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_3_tiled_y_mask: c_uint = 0x00003FFF;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_3_tiled_y_shift: c_int = 16;

// define for DW_4 word
// define for tiled_z field
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_4_tiled_z_offset: c_int = 4;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_4_tiled_z_mask: c_uint = 0x000007FF;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_4_tiled_z_shift: c_int = 0;

// define for width field
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_4_width_offset: c_int = 4;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_4_width_mask: c_uint = 0x00003FFF;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_4_width_shift: c_int = 16;

// define for DW_5 word
// define for height field
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_5_height_offset: c_int = 5;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_5_height_mask: c_uint = 0x00003FFF;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_5_height_shift: c_int = 0;

// define for depth field
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_5_depth_offset: c_int = 5;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_5_depth_mask: c_uint = 0x000007FF;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_5_depth_shift: c_int = 16;

// define for DW_6 word
// define for element_size field
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_6_element_size_offset: c_int = 6;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_6_element_size_mask: c_uint = 0x00000007;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_6_element_size_shift: c_int = 0;

// define for swizzle_mode field
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_6_swizzle_mode_offset: c_int = 6;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_6_swizzle_mode_mask: c_uint = 0x0000001F;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_6_swizzle_mode_shift: c_int = 3;

// define for dimension field
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_6_dimension_offset: c_int = 6;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_6_dimension_mask: c_uint = 0x00000003;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_6_dimension_shift: c_int = 9;

// define for epitch field
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_6_epitch_offset: c_int = 6;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_6_epitch_mask: c_uint = 0x0000FFFF;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_6_epitch_shift: c_int = 16;

// define for LINEAR_ADDR_LO word
// define for linear_addr_31_0 field
pub const SDMA_PKT_COPY_TILED_SUBWIN_LINEAR_ADDR_LO_linear_addr_31_0_offset: c_int = 7;
pub const SDMA_PKT_COPY_TILED_SUBWIN_LINEAR_ADDR_LO_linear_addr_31_0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_TILED_SUBWIN_LINEAR_ADDR_LO_linear_addr_31_0_shift: c_int = 0;

// define for LINEAR_ADDR_HI word
// define for linear_addr_63_32 field
pub const SDMA_PKT_COPY_TILED_SUBWIN_LINEAR_ADDR_HI_linear_addr_63_32_offset: c_int = 8;
pub const SDMA_PKT_COPY_TILED_SUBWIN_LINEAR_ADDR_HI_linear_addr_63_32_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_TILED_SUBWIN_LINEAR_ADDR_HI_linear_addr_63_32_shift: c_int = 0;

// define for DW_9 word
// define for linear_x field
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_9_linear_x_offset: c_int = 9;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_9_linear_x_mask: c_uint = 0x00003FFF;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_9_linear_x_shift: c_int = 0;

// define for linear_y field
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_9_linear_y_offset: c_int = 9;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_9_linear_y_mask: c_uint = 0x00003FFF;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_9_linear_y_shift: c_int = 16;

// define for DW_10 word
// define for linear_z field
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_10_linear_z_offset: c_int = 10;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_10_linear_z_mask: c_uint = 0x000007FF;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_10_linear_z_shift: c_int = 0;

// define for linear_pitch field
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_10_linear_pitch_offset: c_int = 10;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_10_linear_pitch_mask: c_uint = 0x00003FFF;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_10_linear_pitch_shift: c_int = 16;

// define for DW_11 word
// define for linear_slice_pitch field
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_11_linear_slice_pitch_offset: c_int = 11;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_11_linear_slice_pitch_mask: c_uint = 0x0FFFFFFF;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_11_linear_slice_pitch_shift: c_int = 0;

// define for DW_12 word
// define for rect_x field
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_12_rect_x_offset: c_int = 12;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_12_rect_x_mask: c_uint = 0x00003FFF;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_12_rect_x_shift: c_int = 0;

// define for rect_y field
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_12_rect_y_offset: c_int = 12;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_12_rect_y_mask: c_uint = 0x00003FFF;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_12_rect_y_shift: c_int = 16;

// define for DW_13 word
// define for rect_z field
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_13_rect_z_offset: c_int = 13;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_13_rect_z_mask: c_uint = 0x000007FF;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_13_rect_z_shift: c_int = 0;

// define for linear_sw field
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_13_linear_sw_offset: c_int = 13;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_13_linear_sw_mask: c_uint = 0x00000003;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_13_linear_sw_shift: c_int = 16;

// define for tile_sw field
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_13_tile_sw_offset: c_int = 13;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_13_tile_sw_mask: c_uint = 0x00000003;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_13_tile_sw_shift: c_int = 24;

//
// Definitions for SDMA_PKT_COPY_STRUCT packet
//
// define for HEADER word
// define for op field
pub const SDMA_PKT_COPY_STRUCT_HEADER_op_offset: c_int = 0;
pub const SDMA_PKT_COPY_STRUCT_HEADER_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_COPY_STRUCT_HEADER_op_shift: c_int = 0;

// define for sub_op field
pub const SDMA_PKT_COPY_STRUCT_HEADER_sub_op_offset: c_int = 0;
pub const SDMA_PKT_COPY_STRUCT_HEADER_sub_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_COPY_STRUCT_HEADER_sub_op_shift: c_int = 8;

// define for tmz field
pub const SDMA_PKT_COPY_STRUCT_HEADER_tmz_offset: c_int = 0;
pub const SDMA_PKT_COPY_STRUCT_HEADER_tmz_mask: c_uint = 0x00000001;
pub const SDMA_PKT_COPY_STRUCT_HEADER_tmz_shift: c_int = 18;

// define for detile field
pub const SDMA_PKT_COPY_STRUCT_HEADER_detile_offset: c_int = 0;
pub const SDMA_PKT_COPY_STRUCT_HEADER_detile_mask: c_uint = 0x00000001;
pub const SDMA_PKT_COPY_STRUCT_HEADER_detile_shift: c_int = 31;

// define for SB_ADDR_LO word
// define for sb_addr_31_0 field
pub const SDMA_PKT_COPY_STRUCT_SB_ADDR_LO_sb_addr_31_0_offset: c_int = 1;
pub const SDMA_PKT_COPY_STRUCT_SB_ADDR_LO_sb_addr_31_0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_STRUCT_SB_ADDR_LO_sb_addr_31_0_shift: c_int = 0;

// define for SB_ADDR_HI word
// define for sb_addr_63_32 field
pub const SDMA_PKT_COPY_STRUCT_SB_ADDR_HI_sb_addr_63_32_offset: c_int = 2;
pub const SDMA_PKT_COPY_STRUCT_SB_ADDR_HI_sb_addr_63_32_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_STRUCT_SB_ADDR_HI_sb_addr_63_32_shift: c_int = 0;

// define for START_INDEX word
// define for start_index field
pub const SDMA_PKT_COPY_STRUCT_START_INDEX_start_index_offset: c_int = 3;
pub const SDMA_PKT_COPY_STRUCT_START_INDEX_start_index_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_STRUCT_START_INDEX_start_index_shift: c_int = 0;

// define for COUNT word
// define for count field
pub const SDMA_PKT_COPY_STRUCT_COUNT_count_offset: c_int = 4;
pub const SDMA_PKT_COPY_STRUCT_COUNT_count_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_STRUCT_COUNT_count_shift: c_int = 0;

// define for DW_5 word
// define for stride field
pub const SDMA_PKT_COPY_STRUCT_DW_5_stride_offset: c_int = 5;
pub const SDMA_PKT_COPY_STRUCT_DW_5_stride_mask: c_uint = 0x000007FF;
pub const SDMA_PKT_COPY_STRUCT_DW_5_stride_shift: c_int = 0;

// define for linear_sw field
pub const SDMA_PKT_COPY_STRUCT_DW_5_linear_sw_offset: c_int = 5;
pub const SDMA_PKT_COPY_STRUCT_DW_5_linear_sw_mask: c_uint = 0x00000003;
pub const SDMA_PKT_COPY_STRUCT_DW_5_linear_sw_shift: c_int = 16;

// define for struct_sw field
pub const SDMA_PKT_COPY_STRUCT_DW_5_struct_sw_offset: c_int = 5;
pub const SDMA_PKT_COPY_STRUCT_DW_5_struct_sw_mask: c_uint = 0x00000003;
pub const SDMA_PKT_COPY_STRUCT_DW_5_struct_sw_shift: c_int = 24;

// define for LINEAR_ADDR_LO word
// define for linear_addr_31_0 field
pub const SDMA_PKT_COPY_STRUCT_LINEAR_ADDR_LO_linear_addr_31_0_offset: c_int = 6;
pub const SDMA_PKT_COPY_STRUCT_LINEAR_ADDR_LO_linear_addr_31_0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_STRUCT_LINEAR_ADDR_LO_linear_addr_31_0_shift: c_int = 0;

// define for LINEAR_ADDR_HI word
// define for linear_addr_63_32 field
pub const SDMA_PKT_COPY_STRUCT_LINEAR_ADDR_HI_linear_addr_63_32_offset: c_int = 7;
pub const SDMA_PKT_COPY_STRUCT_LINEAR_ADDR_HI_linear_addr_63_32_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_STRUCT_LINEAR_ADDR_HI_linear_addr_63_32_shift: c_int = 0;

//
// Definitions for SDMA_PKT_WRITE_UNTILED packet
//
// define for HEADER word
// define for op field
pub const SDMA_PKT_WRITE_UNTILED_HEADER_op_offset: c_int = 0;
pub const SDMA_PKT_WRITE_UNTILED_HEADER_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_WRITE_UNTILED_HEADER_op_shift: c_int = 0;

// define for sub_op field
pub const SDMA_PKT_WRITE_UNTILED_HEADER_sub_op_offset: c_int = 0;
pub const SDMA_PKT_WRITE_UNTILED_HEADER_sub_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_WRITE_UNTILED_HEADER_sub_op_shift: c_int = 8;

// define for encrypt field
pub const SDMA_PKT_WRITE_UNTILED_HEADER_encrypt_offset: c_int = 0;
pub const SDMA_PKT_WRITE_UNTILED_HEADER_encrypt_mask: c_uint = 0x00000001;
pub const SDMA_PKT_WRITE_UNTILED_HEADER_encrypt_shift: c_int = 16;

// define for tmz field
pub const SDMA_PKT_WRITE_UNTILED_HEADER_tmz_offset: c_int = 0;
pub const SDMA_PKT_WRITE_UNTILED_HEADER_tmz_mask: c_uint = 0x00000001;
pub const SDMA_PKT_WRITE_UNTILED_HEADER_tmz_shift: c_int = 18;

// define for DST_ADDR_LO word
// define for dst_addr_31_0 field
pub const SDMA_PKT_WRITE_UNTILED_DST_ADDR_LO_dst_addr_31_0_offset: c_int = 1;
pub const SDMA_PKT_WRITE_UNTILED_DST_ADDR_LO_dst_addr_31_0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_WRITE_UNTILED_DST_ADDR_LO_dst_addr_31_0_shift: c_int = 0;

// define for DST_ADDR_HI word
// define for dst_addr_63_32 field
pub const SDMA_PKT_WRITE_UNTILED_DST_ADDR_HI_dst_addr_63_32_offset: c_int = 2;
pub const SDMA_PKT_WRITE_UNTILED_DST_ADDR_HI_dst_addr_63_32_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_WRITE_UNTILED_DST_ADDR_HI_dst_addr_63_32_shift: c_int = 0;

// define for DW_3 word
// define for count field
pub const SDMA_PKT_WRITE_UNTILED_DW_3_count_offset: c_int = 3;
pub const SDMA_PKT_WRITE_UNTILED_DW_3_count_mask: c_uint = 0x000FFFFF;
pub const SDMA_PKT_WRITE_UNTILED_DW_3_count_shift: c_int = 0;

// define for sw field
pub const SDMA_PKT_WRITE_UNTILED_DW_3_sw_offset: c_int = 3;
pub const SDMA_PKT_WRITE_UNTILED_DW_3_sw_mask: c_uint = 0x00000003;
pub const SDMA_PKT_WRITE_UNTILED_DW_3_sw_shift: c_int = 24;

// define for DATA0 word
// define for data0 field
pub const SDMA_PKT_WRITE_UNTILED_DATA0_data0_offset: c_int = 4;
pub const SDMA_PKT_WRITE_UNTILED_DATA0_data0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_WRITE_UNTILED_DATA0_data0_shift: c_int = 0;

//
// Definitions for SDMA_PKT_WRITE_TILED packet
//
// define for HEADER word
// define for op field
pub const SDMA_PKT_WRITE_TILED_HEADER_op_offset: c_int = 0;
pub const SDMA_PKT_WRITE_TILED_HEADER_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_WRITE_TILED_HEADER_op_shift: c_int = 0;

// define for sub_op field
pub const SDMA_PKT_WRITE_TILED_HEADER_sub_op_offset: c_int = 0;
pub const SDMA_PKT_WRITE_TILED_HEADER_sub_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_WRITE_TILED_HEADER_sub_op_shift: c_int = 8;

// define for encrypt field
pub const SDMA_PKT_WRITE_TILED_HEADER_encrypt_offset: c_int = 0;
pub const SDMA_PKT_WRITE_TILED_HEADER_encrypt_mask: c_uint = 0x00000001;
pub const SDMA_PKT_WRITE_TILED_HEADER_encrypt_shift: c_int = 16;

// define for tmz field
pub const SDMA_PKT_WRITE_TILED_HEADER_tmz_offset: c_int = 0;
pub const SDMA_PKT_WRITE_TILED_HEADER_tmz_mask: c_uint = 0x00000001;
pub const SDMA_PKT_WRITE_TILED_HEADER_tmz_shift: c_int = 18;

// define for mip_max field
pub const SDMA_PKT_WRITE_TILED_HEADER_mip_max_offset: c_int = 0;
pub const SDMA_PKT_WRITE_TILED_HEADER_mip_max_mask: c_uint = 0x0000000F;
pub const SDMA_PKT_WRITE_TILED_HEADER_mip_max_shift: c_int = 20;

// define for DST_ADDR_LO word
// define for dst_addr_31_0 field
pub const SDMA_PKT_WRITE_TILED_DST_ADDR_LO_dst_addr_31_0_offset: c_int = 1;
pub const SDMA_PKT_WRITE_TILED_DST_ADDR_LO_dst_addr_31_0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_WRITE_TILED_DST_ADDR_LO_dst_addr_31_0_shift: c_int = 0;

// define for DST_ADDR_HI word
// define for dst_addr_63_32 field
pub const SDMA_PKT_WRITE_TILED_DST_ADDR_HI_dst_addr_63_32_offset: c_int = 2;
pub const SDMA_PKT_WRITE_TILED_DST_ADDR_HI_dst_addr_63_32_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_WRITE_TILED_DST_ADDR_HI_dst_addr_63_32_shift: c_int = 0;

// define for DW_3 word
// define for width field
pub const SDMA_PKT_WRITE_TILED_DW_3_width_offset: c_int = 3;
pub const SDMA_PKT_WRITE_TILED_DW_3_width_mask: c_uint = 0x00003FFF;
pub const SDMA_PKT_WRITE_TILED_DW_3_width_shift: c_int = 0;

// define for DW_4 word
// define for height field
pub const SDMA_PKT_WRITE_TILED_DW_4_height_offset: c_int = 4;
pub const SDMA_PKT_WRITE_TILED_DW_4_height_mask: c_uint = 0x00003FFF;
pub const SDMA_PKT_WRITE_TILED_DW_4_height_shift: c_int = 0;

// define for depth field
pub const SDMA_PKT_WRITE_TILED_DW_4_depth_offset: c_int = 4;
pub const SDMA_PKT_WRITE_TILED_DW_4_depth_mask: c_uint = 0x000007FF;
pub const SDMA_PKT_WRITE_TILED_DW_4_depth_shift: c_int = 16;

// define for DW_5 word
// define for element_size field
pub const SDMA_PKT_WRITE_TILED_DW_5_element_size_offset: c_int = 5;
pub const SDMA_PKT_WRITE_TILED_DW_5_element_size_mask: c_uint = 0x00000007;
pub const SDMA_PKT_WRITE_TILED_DW_5_element_size_shift: c_int = 0;

// define for swizzle_mode field
pub const SDMA_PKT_WRITE_TILED_DW_5_swizzle_mode_offset: c_int = 5;
pub const SDMA_PKT_WRITE_TILED_DW_5_swizzle_mode_mask: c_uint = 0x0000001F;
pub const SDMA_PKT_WRITE_TILED_DW_5_swizzle_mode_shift: c_int = 3;

// define for dimension field
pub const SDMA_PKT_WRITE_TILED_DW_5_dimension_offset: c_int = 5;
pub const SDMA_PKT_WRITE_TILED_DW_5_dimension_mask: c_uint = 0x00000003;
pub const SDMA_PKT_WRITE_TILED_DW_5_dimension_shift: c_int = 9;

// define for epitch field
pub const SDMA_PKT_WRITE_TILED_DW_5_epitch_offset: c_int = 5;
pub const SDMA_PKT_WRITE_TILED_DW_5_epitch_mask: c_uint = 0x0000FFFF;
pub const SDMA_PKT_WRITE_TILED_DW_5_epitch_shift: c_int = 16;

// define for DW_6 word
// define for x field
pub const SDMA_PKT_WRITE_TILED_DW_6_x_offset: c_int = 6;
pub const SDMA_PKT_WRITE_TILED_DW_6_x_mask: c_uint = 0x00003FFF;
pub const SDMA_PKT_WRITE_TILED_DW_6_x_shift: c_int = 0;

// define for y field
pub const SDMA_PKT_WRITE_TILED_DW_6_y_offset: c_int = 6;
pub const SDMA_PKT_WRITE_TILED_DW_6_y_mask: c_uint = 0x00003FFF;
pub const SDMA_PKT_WRITE_TILED_DW_6_y_shift: c_int = 16;

// define for DW_7 word
// define for z field
pub const SDMA_PKT_WRITE_TILED_DW_7_z_offset: c_int = 7;
pub const SDMA_PKT_WRITE_TILED_DW_7_z_mask: c_uint = 0x000007FF;
pub const SDMA_PKT_WRITE_TILED_DW_7_z_shift: c_int = 0;

// define for sw field
pub const SDMA_PKT_WRITE_TILED_DW_7_sw_offset: c_int = 7;
pub const SDMA_PKT_WRITE_TILED_DW_7_sw_mask: c_uint = 0x00000003;
pub const SDMA_PKT_WRITE_TILED_DW_7_sw_shift: c_int = 24;

// define for COUNT word
// define for count field
pub const SDMA_PKT_WRITE_TILED_COUNT_count_offset: c_int = 8;
pub const SDMA_PKT_WRITE_TILED_COUNT_count_mask: c_uint = 0x000FFFFF;
pub const SDMA_PKT_WRITE_TILED_COUNT_count_shift: c_int = 0;

// define for DATA0 word
// define for data0 field
pub const SDMA_PKT_WRITE_TILED_DATA0_data0_offset: c_int = 9;
pub const SDMA_PKT_WRITE_TILED_DATA0_data0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_WRITE_TILED_DATA0_data0_shift: c_int = 0;

//
// Definitions for SDMA_PKT_PTEPDE_COPY packet
//
// define for HEADER word
// define for op field
pub const SDMA_PKT_PTEPDE_COPY_HEADER_op_offset: c_int = 0;
pub const SDMA_PKT_PTEPDE_COPY_HEADER_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_PTEPDE_COPY_HEADER_op_shift: c_int = 0;

// define for sub_op field
pub const SDMA_PKT_PTEPDE_COPY_HEADER_sub_op_offset: c_int = 0;
pub const SDMA_PKT_PTEPDE_COPY_HEADER_sub_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_PTEPDE_COPY_HEADER_sub_op_shift: c_int = 8;

// define for ptepde_op field
pub const SDMA_PKT_PTEPDE_COPY_HEADER_ptepde_op_offset: c_int = 0;
pub const SDMA_PKT_PTEPDE_COPY_HEADER_ptepde_op_mask: c_uint = 0x00000001;
pub const SDMA_PKT_PTEPDE_COPY_HEADER_ptepde_op_shift: c_int = 31;

// define for SRC_ADDR_LO word
// define for src_addr_31_0 field
pub const SDMA_PKT_PTEPDE_COPY_SRC_ADDR_LO_src_addr_31_0_offset: c_int = 1;
pub const SDMA_PKT_PTEPDE_COPY_SRC_ADDR_LO_src_addr_31_0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_PTEPDE_COPY_SRC_ADDR_LO_src_addr_31_0_shift: c_int = 0;

// define for SRC_ADDR_HI word
// define for src_addr_63_32 field
pub const SDMA_PKT_PTEPDE_COPY_SRC_ADDR_HI_src_addr_63_32_offset: c_int = 2;
pub const SDMA_PKT_PTEPDE_COPY_SRC_ADDR_HI_src_addr_63_32_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_PTEPDE_COPY_SRC_ADDR_HI_src_addr_63_32_shift: c_int = 0;

// define for DST_ADDR_LO word
// define for dst_addr_31_0 field
pub const SDMA_PKT_PTEPDE_COPY_DST_ADDR_LO_dst_addr_31_0_offset: c_int = 3;
pub const SDMA_PKT_PTEPDE_COPY_DST_ADDR_LO_dst_addr_31_0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_PTEPDE_COPY_DST_ADDR_LO_dst_addr_31_0_shift: c_int = 0;

// define for DST_ADDR_HI word
// define for dst_addr_63_32 field
pub const SDMA_PKT_PTEPDE_COPY_DST_ADDR_HI_dst_addr_63_32_offset: c_int = 4;
pub const SDMA_PKT_PTEPDE_COPY_DST_ADDR_HI_dst_addr_63_32_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_PTEPDE_COPY_DST_ADDR_HI_dst_addr_63_32_shift: c_int = 0;

// define for MASK_DW0 word
// define for mask_dw0 field
pub const SDMA_PKT_PTEPDE_COPY_MASK_DW0_mask_dw0_offset: c_int = 5;
pub const SDMA_PKT_PTEPDE_COPY_MASK_DW0_mask_dw0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_PTEPDE_COPY_MASK_DW0_mask_dw0_shift: c_int = 0;

// define for MASK_DW1 word
// define for mask_dw1 field
pub const SDMA_PKT_PTEPDE_COPY_MASK_DW1_mask_dw1_offset: c_int = 6;
pub const SDMA_PKT_PTEPDE_COPY_MASK_DW1_mask_dw1_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_PTEPDE_COPY_MASK_DW1_mask_dw1_shift: c_int = 0;

// define for COUNT word
// define for count field
pub const SDMA_PKT_PTEPDE_COPY_COUNT_count_offset: c_int = 7;
pub const SDMA_PKT_PTEPDE_COPY_COUNT_count_mask: c_uint = 0x0007FFFF;
pub const SDMA_PKT_PTEPDE_COPY_COUNT_count_shift: c_int = 0;

//
// Definitions for SDMA_PKT_PTEPDE_COPY_BACKWARDS packet
//
// define for HEADER word
// define for op field
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_HEADER_op_offset: c_int = 0;
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_HEADER_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_HEADER_op_shift: c_int = 0;

// define for sub_op field
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_HEADER_sub_op_offset: c_int = 0;
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_HEADER_sub_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_HEADER_sub_op_shift: c_int = 8;

// define for pte_size field
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_HEADER_pte_size_offset: c_int = 0;
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_HEADER_pte_size_mask: c_uint = 0x00000003;
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_HEADER_pte_size_shift: c_int = 28;

// define for direction field
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_HEADER_direction_offset: c_int = 0;
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_HEADER_direction_mask: c_uint = 0x00000001;
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_HEADER_direction_shift: c_int = 30;

// define for ptepde_op field
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_HEADER_ptepde_op_offset: c_int = 0;
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_HEADER_ptepde_op_mask: c_uint = 0x00000001;
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_HEADER_ptepde_op_shift: c_int = 31;

// define for SRC_ADDR_LO word
// define for src_addr_31_0 field
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_SRC_ADDR_LO_src_addr_31_0_offset: c_int = 1;
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_SRC_ADDR_LO_src_addr_31_0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_SRC_ADDR_LO_src_addr_31_0_shift: c_int = 0;

// define for SRC_ADDR_HI word
// define for src_addr_63_32 field
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_SRC_ADDR_HI_src_addr_63_32_offset: c_int = 2;
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_SRC_ADDR_HI_src_addr_63_32_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_SRC_ADDR_HI_src_addr_63_32_shift: c_int = 0;

// define for DST_ADDR_LO word
// define for dst_addr_31_0 field
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_DST_ADDR_LO_dst_addr_31_0_offset: c_int = 3;
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_DST_ADDR_LO_dst_addr_31_0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_DST_ADDR_LO_dst_addr_31_0_shift: c_int = 0;

// define for DST_ADDR_HI word
// define for dst_addr_63_32 field
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_DST_ADDR_HI_dst_addr_63_32_offset: c_int = 4;
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_DST_ADDR_HI_dst_addr_63_32_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_DST_ADDR_HI_dst_addr_63_32_shift: c_int = 0;

// define for MASK_BIT_FOR_DW word
// define for mask_first_xfer field
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_MASK_BIT_FOR_DW_mask_first_xfer_offset: c_int = 5;
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_MASK_BIT_FOR_DW_mask_first_xfer_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_MASK_BIT_FOR_DW_mask_first_xfer_shift: c_int = 0;

// define for mask_last_xfer field
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_MASK_BIT_FOR_DW_mask_last_xfer_offset: c_int = 5;
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_MASK_BIT_FOR_DW_mask_last_xfer_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_MASK_BIT_FOR_DW_mask_last_xfer_shift: c_int = 8;

// define for COUNT_IN_32B_XFER word
// define for count field
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_COUNT_IN_32B_XFER_count_offset: c_int = 6;
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_COUNT_IN_32B_XFER_count_mask: c_uint = 0x0001FFFF;
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_COUNT_IN_32B_XFER_count_shift: c_int = 0;

//
// Definitions for SDMA_PKT_PTEPDE_RMW packet
//
// define for HEADER word
// define for op field
pub const SDMA_PKT_PTEPDE_RMW_HEADER_op_offset: c_int = 0;
pub const SDMA_PKT_PTEPDE_RMW_HEADER_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_PTEPDE_RMW_HEADER_op_shift: c_int = 0;

// define for sub_op field
pub const SDMA_PKT_PTEPDE_RMW_HEADER_sub_op_offset: c_int = 0;
pub const SDMA_PKT_PTEPDE_RMW_HEADER_sub_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_PTEPDE_RMW_HEADER_sub_op_shift: c_int = 8;

// define for gcc field
pub const SDMA_PKT_PTEPDE_RMW_HEADER_gcc_offset: c_int = 0;
pub const SDMA_PKT_PTEPDE_RMW_HEADER_gcc_mask: c_uint = 0x00000001;
pub const SDMA_PKT_PTEPDE_RMW_HEADER_gcc_shift: c_int = 19;

// define for sys field
pub const SDMA_PKT_PTEPDE_RMW_HEADER_sys_offset: c_int = 0;
pub const SDMA_PKT_PTEPDE_RMW_HEADER_sys_mask: c_uint = 0x00000001;
pub const SDMA_PKT_PTEPDE_RMW_HEADER_sys_shift: c_int = 20;

// define for snp field
pub const SDMA_PKT_PTEPDE_RMW_HEADER_snp_offset: c_int = 0;
pub const SDMA_PKT_PTEPDE_RMW_HEADER_snp_mask: c_uint = 0x00000001;
pub const SDMA_PKT_PTEPDE_RMW_HEADER_snp_shift: c_int = 22;

// define for gpa field
pub const SDMA_PKT_PTEPDE_RMW_HEADER_gpa_offset: c_int = 0;
pub const SDMA_PKT_PTEPDE_RMW_HEADER_gpa_mask: c_uint = 0x00000001;
pub const SDMA_PKT_PTEPDE_RMW_HEADER_gpa_shift: c_int = 23;

// define for ADDR_LO word
// define for addr_31_0 field
pub const SDMA_PKT_PTEPDE_RMW_ADDR_LO_addr_31_0_offset: c_int = 1;
pub const SDMA_PKT_PTEPDE_RMW_ADDR_LO_addr_31_0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_PTEPDE_RMW_ADDR_LO_addr_31_0_shift: c_int = 0;

// define for ADDR_HI word
// define for addr_63_32 field
pub const SDMA_PKT_PTEPDE_RMW_ADDR_HI_addr_63_32_offset: c_int = 2;
pub const SDMA_PKT_PTEPDE_RMW_ADDR_HI_addr_63_32_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_PTEPDE_RMW_ADDR_HI_addr_63_32_shift: c_int = 0;

// define for MASK_LO word
// define for mask_31_0 field
pub const SDMA_PKT_PTEPDE_RMW_MASK_LO_mask_31_0_offset: c_int = 3;
pub const SDMA_PKT_PTEPDE_RMW_MASK_LO_mask_31_0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_PTEPDE_RMW_MASK_LO_mask_31_0_shift: c_int = 0;

// define for MASK_HI word
// define for mask_63_32 field
pub const SDMA_PKT_PTEPDE_RMW_MASK_HI_mask_63_32_offset: c_int = 4;
pub const SDMA_PKT_PTEPDE_RMW_MASK_HI_mask_63_32_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_PTEPDE_RMW_MASK_HI_mask_63_32_shift: c_int = 0;

// define for VALUE_LO word
// define for value_31_0 field
pub const SDMA_PKT_PTEPDE_RMW_VALUE_LO_value_31_0_offset: c_int = 5;
pub const SDMA_PKT_PTEPDE_RMW_VALUE_LO_value_31_0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_PTEPDE_RMW_VALUE_LO_value_31_0_shift: c_int = 0;

// define for VALUE_HI word
// define for value_63_32 field
pub const SDMA_PKT_PTEPDE_RMW_VALUE_HI_value_63_32_offset: c_int = 6;
pub const SDMA_PKT_PTEPDE_RMW_VALUE_HI_value_63_32_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_PTEPDE_RMW_VALUE_HI_value_63_32_shift: c_int = 0;

//
// Definitions for SDMA_PKT_WRITE_INCR packet
//
// define for HEADER word
// define for op field
pub const SDMA_PKT_WRITE_INCR_HEADER_op_offset: c_int = 0;
pub const SDMA_PKT_WRITE_INCR_HEADER_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_WRITE_INCR_HEADER_op_shift: c_int = 0;

// define for sub_op field
pub const SDMA_PKT_WRITE_INCR_HEADER_sub_op_offset: c_int = 0;
pub const SDMA_PKT_WRITE_INCR_HEADER_sub_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_WRITE_INCR_HEADER_sub_op_shift: c_int = 8;

// define for DST_ADDR_LO word
// define for dst_addr_31_0 field
pub const SDMA_PKT_WRITE_INCR_DST_ADDR_LO_dst_addr_31_0_offset: c_int = 1;
pub const SDMA_PKT_WRITE_INCR_DST_ADDR_LO_dst_addr_31_0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_WRITE_INCR_DST_ADDR_LO_dst_addr_31_0_shift: c_int = 0;

// define for DST_ADDR_HI word
// define for dst_addr_63_32 field
pub const SDMA_PKT_WRITE_INCR_DST_ADDR_HI_dst_addr_63_32_offset: c_int = 2;
pub const SDMA_PKT_WRITE_INCR_DST_ADDR_HI_dst_addr_63_32_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_WRITE_INCR_DST_ADDR_HI_dst_addr_63_32_shift: c_int = 0;

// define for MASK_DW0 word
// define for mask_dw0 field
pub const SDMA_PKT_WRITE_INCR_MASK_DW0_mask_dw0_offset: c_int = 3;
pub const SDMA_PKT_WRITE_INCR_MASK_DW0_mask_dw0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_WRITE_INCR_MASK_DW0_mask_dw0_shift: c_int = 0;

// define for MASK_DW1 word
// define for mask_dw1 field
pub const SDMA_PKT_WRITE_INCR_MASK_DW1_mask_dw1_offset: c_int = 4;
pub const SDMA_PKT_WRITE_INCR_MASK_DW1_mask_dw1_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_WRITE_INCR_MASK_DW1_mask_dw1_shift: c_int = 0;

// define for INIT_DW0 word
// define for init_dw0 field
pub const SDMA_PKT_WRITE_INCR_INIT_DW0_init_dw0_offset: c_int = 5;
pub const SDMA_PKT_WRITE_INCR_INIT_DW0_init_dw0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_WRITE_INCR_INIT_DW0_init_dw0_shift: c_int = 0;

// define for INIT_DW1 word
// define for init_dw1 field
pub const SDMA_PKT_WRITE_INCR_INIT_DW1_init_dw1_offset: c_int = 6;
pub const SDMA_PKT_WRITE_INCR_INIT_DW1_init_dw1_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_WRITE_INCR_INIT_DW1_init_dw1_shift: c_int = 0;

// define for INCR_DW0 word
// define for incr_dw0 field
pub const SDMA_PKT_WRITE_INCR_INCR_DW0_incr_dw0_offset: c_int = 7;
pub const SDMA_PKT_WRITE_INCR_INCR_DW0_incr_dw0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_WRITE_INCR_INCR_DW0_incr_dw0_shift: c_int = 0;

// define for INCR_DW1 word
// define for incr_dw1 field
pub const SDMA_PKT_WRITE_INCR_INCR_DW1_incr_dw1_offset: c_int = 8;
pub const SDMA_PKT_WRITE_INCR_INCR_DW1_incr_dw1_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_WRITE_INCR_INCR_DW1_incr_dw1_shift: c_int = 0;

// define for COUNT word
// define for count field
pub const SDMA_PKT_WRITE_INCR_COUNT_count_offset: c_int = 9;
pub const SDMA_PKT_WRITE_INCR_COUNT_count_mask: c_uint = 0x0007FFFF;
pub const SDMA_PKT_WRITE_INCR_COUNT_count_shift: c_int = 0;

//
// Definitions for SDMA_PKT_INDIRECT packet
//
// define for HEADER word
// define for op field
pub const SDMA_PKT_INDIRECT_HEADER_op_offset: c_int = 0;
pub const SDMA_PKT_INDIRECT_HEADER_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_INDIRECT_HEADER_op_shift: c_int = 0;

// define for sub_op field
pub const SDMA_PKT_INDIRECT_HEADER_sub_op_offset: c_int = 0;
pub const SDMA_PKT_INDIRECT_HEADER_sub_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_INDIRECT_HEADER_sub_op_shift: c_int = 8;

// define for vmid field
pub const SDMA_PKT_INDIRECT_HEADER_vmid_offset: c_int = 0;
pub const SDMA_PKT_INDIRECT_HEADER_vmid_mask: c_uint = 0x0000000F;
pub const SDMA_PKT_INDIRECT_HEADER_vmid_shift: c_int = 16;

// define for BASE_LO word
// define for ib_base_31_0 field
pub const SDMA_PKT_INDIRECT_BASE_LO_ib_base_31_0_offset: c_int = 1;
pub const SDMA_PKT_INDIRECT_BASE_LO_ib_base_31_0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_INDIRECT_BASE_LO_ib_base_31_0_shift: c_int = 0;

// define for BASE_HI word
// define for ib_base_63_32 field
pub const SDMA_PKT_INDIRECT_BASE_HI_ib_base_63_32_offset: c_int = 2;
pub const SDMA_PKT_INDIRECT_BASE_HI_ib_base_63_32_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_INDIRECT_BASE_HI_ib_base_63_32_shift: c_int = 0;

// define for IB_SIZE word
// define for ib_size field
pub const SDMA_PKT_INDIRECT_IB_SIZE_ib_size_offset: c_int = 3;
pub const SDMA_PKT_INDIRECT_IB_SIZE_ib_size_mask: c_uint = 0x000FFFFF;
pub const SDMA_PKT_INDIRECT_IB_SIZE_ib_size_shift: c_int = 0;

// define for CSA_ADDR_LO word
// define for csa_addr_31_0 field
pub const SDMA_PKT_INDIRECT_CSA_ADDR_LO_csa_addr_31_0_offset: c_int = 4;
pub const SDMA_PKT_INDIRECT_CSA_ADDR_LO_csa_addr_31_0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_INDIRECT_CSA_ADDR_LO_csa_addr_31_0_shift: c_int = 0;

// define for CSA_ADDR_HI word
// define for csa_addr_63_32 field
pub const SDMA_PKT_INDIRECT_CSA_ADDR_HI_csa_addr_63_32_offset: c_int = 5;
pub const SDMA_PKT_INDIRECT_CSA_ADDR_HI_csa_addr_63_32_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_INDIRECT_CSA_ADDR_HI_csa_addr_63_32_shift: c_int = 0;

//
// Definitions for SDMA_PKT_SEMAPHORE packet
//
// define for HEADER word
// define for op field
pub const SDMA_PKT_SEMAPHORE_HEADER_op_offset: c_int = 0;
pub const SDMA_PKT_SEMAPHORE_HEADER_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_SEMAPHORE_HEADER_op_shift: c_int = 0;

// define for sub_op field
pub const SDMA_PKT_SEMAPHORE_HEADER_sub_op_offset: c_int = 0;
pub const SDMA_PKT_SEMAPHORE_HEADER_sub_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_SEMAPHORE_HEADER_sub_op_shift: c_int = 8;

// define for write_one field
pub const SDMA_PKT_SEMAPHORE_HEADER_write_one_offset: c_int = 0;
pub const SDMA_PKT_SEMAPHORE_HEADER_write_one_mask: c_uint = 0x00000001;
pub const SDMA_PKT_SEMAPHORE_HEADER_write_one_shift: c_int = 29;

// define for signal field
pub const SDMA_PKT_SEMAPHORE_HEADER_signal_offset: c_int = 0;
pub const SDMA_PKT_SEMAPHORE_HEADER_signal_mask: c_uint = 0x00000001;
pub const SDMA_PKT_SEMAPHORE_HEADER_signal_shift: c_int = 30;

// define for mailbox field
pub const SDMA_PKT_SEMAPHORE_HEADER_mailbox_offset: c_int = 0;
pub const SDMA_PKT_SEMAPHORE_HEADER_mailbox_mask: c_uint = 0x00000001;
pub const SDMA_PKT_SEMAPHORE_HEADER_mailbox_shift: c_int = 31;

// define for ADDR_LO word
// define for addr_31_0 field
pub const SDMA_PKT_SEMAPHORE_ADDR_LO_addr_31_0_offset: c_int = 1;
pub const SDMA_PKT_SEMAPHORE_ADDR_LO_addr_31_0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_SEMAPHORE_ADDR_LO_addr_31_0_shift: c_int = 0;

// define for ADDR_HI word
// define for addr_63_32 field
pub const SDMA_PKT_SEMAPHORE_ADDR_HI_addr_63_32_offset: c_int = 2;
pub const SDMA_PKT_SEMAPHORE_ADDR_HI_addr_63_32_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_SEMAPHORE_ADDR_HI_addr_63_32_shift: c_int = 0;

//
// Definitions for SDMA_PKT_FENCE packet
//
// define for HEADER word
// define for op field
pub const SDMA_PKT_FENCE_HEADER_op_offset: c_int = 0;
pub const SDMA_PKT_FENCE_HEADER_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_FENCE_HEADER_op_shift: c_int = 0;

// define for sub_op field
pub const SDMA_PKT_FENCE_HEADER_sub_op_offset: c_int = 0;
pub const SDMA_PKT_FENCE_HEADER_sub_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_FENCE_HEADER_sub_op_shift: c_int = 8;

// define for ADDR_LO word
// define for addr_31_0 field
pub const SDMA_PKT_FENCE_ADDR_LO_addr_31_0_offset: c_int = 1;
pub const SDMA_PKT_FENCE_ADDR_LO_addr_31_0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_FENCE_ADDR_LO_addr_31_0_shift: c_int = 0;

// define for ADDR_HI word
// define for addr_63_32 field
pub const SDMA_PKT_FENCE_ADDR_HI_addr_63_32_offset: c_int = 2;
pub const SDMA_PKT_FENCE_ADDR_HI_addr_63_32_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_FENCE_ADDR_HI_addr_63_32_shift: c_int = 0;

// define for DATA word
// define for data field
pub const SDMA_PKT_FENCE_DATA_data_offset: c_int = 3;
pub const SDMA_PKT_FENCE_DATA_data_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_FENCE_DATA_data_shift: c_int = 0;

//
// Definitions for SDMA_PKT_SRBM_WRITE packet
//
// define for HEADER word
// define for op field
pub const SDMA_PKT_SRBM_WRITE_HEADER_op_offset: c_int = 0;
pub const SDMA_PKT_SRBM_WRITE_HEADER_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_SRBM_WRITE_HEADER_op_shift: c_int = 0;

// define for sub_op field
pub const SDMA_PKT_SRBM_WRITE_HEADER_sub_op_offset: c_int = 0;
pub const SDMA_PKT_SRBM_WRITE_HEADER_sub_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_SRBM_WRITE_HEADER_sub_op_shift: c_int = 8;

// define for byte_en field
pub const SDMA_PKT_SRBM_WRITE_HEADER_byte_en_offset: c_int = 0;
pub const SDMA_PKT_SRBM_WRITE_HEADER_byte_en_mask: c_uint = 0x0000000F;
pub const SDMA_PKT_SRBM_WRITE_HEADER_byte_en_shift: c_int = 28;

// define for ADDR word
// define for addr field
pub const SDMA_PKT_SRBM_WRITE_ADDR_addr_offset: c_int = 1;
pub const SDMA_PKT_SRBM_WRITE_ADDR_addr_mask: c_uint = 0x0003FFFF;
pub const SDMA_PKT_SRBM_WRITE_ADDR_addr_shift: c_int = 0;

// define for DATA word
// define for data field
pub const SDMA_PKT_SRBM_WRITE_DATA_data_offset: c_int = 2;
pub const SDMA_PKT_SRBM_WRITE_DATA_data_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_SRBM_WRITE_DATA_data_shift: c_int = 0;

//
// Definitions for SDMA_PKT_PRE_EXE packet
//
// define for HEADER word
// define for op field
pub const SDMA_PKT_PRE_EXE_HEADER_op_offset: c_int = 0;
pub const SDMA_PKT_PRE_EXE_HEADER_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_PRE_EXE_HEADER_op_shift: c_int = 0;

// define for sub_op field
pub const SDMA_PKT_PRE_EXE_HEADER_sub_op_offset: c_int = 0;
pub const SDMA_PKT_PRE_EXE_HEADER_sub_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_PRE_EXE_HEADER_sub_op_shift: c_int = 8;

// define for dev_sel field
pub const SDMA_PKT_PRE_EXE_HEADER_dev_sel_offset: c_int = 0;
pub const SDMA_PKT_PRE_EXE_HEADER_dev_sel_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_PRE_EXE_HEADER_dev_sel_shift: c_int = 16;

// define for EXEC_COUNT word
// define for exec_count field
pub const SDMA_PKT_PRE_EXE_EXEC_COUNT_exec_count_offset: c_int = 1;
pub const SDMA_PKT_PRE_EXE_EXEC_COUNT_exec_count_mask: c_uint = 0x00003FFF;
pub const SDMA_PKT_PRE_EXE_EXEC_COUNT_exec_count_shift: c_int = 0;

//
// Definitions for SDMA_PKT_COND_EXE packet
//
// define for HEADER word
// define for op field
pub const SDMA_PKT_COND_EXE_HEADER_op_offset: c_int = 0;
pub const SDMA_PKT_COND_EXE_HEADER_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_COND_EXE_HEADER_op_shift: c_int = 0;

// define for sub_op field
pub const SDMA_PKT_COND_EXE_HEADER_sub_op_offset: c_int = 0;
pub const SDMA_PKT_COND_EXE_HEADER_sub_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_COND_EXE_HEADER_sub_op_shift: c_int = 8;

// define for ADDR_LO word
// define for addr_31_0 field
pub const SDMA_PKT_COND_EXE_ADDR_LO_addr_31_0_offset: c_int = 1;
pub const SDMA_PKT_COND_EXE_ADDR_LO_addr_31_0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_COND_EXE_ADDR_LO_addr_31_0_shift: c_int = 0;

// define for ADDR_HI word
// define for addr_63_32 field
pub const SDMA_PKT_COND_EXE_ADDR_HI_addr_63_32_offset: c_int = 2;
pub const SDMA_PKT_COND_EXE_ADDR_HI_addr_63_32_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_COND_EXE_ADDR_HI_addr_63_32_shift: c_int = 0;

// define for REFERENCE word
// define for reference field
pub const SDMA_PKT_COND_EXE_REFERENCE_reference_offset: c_int = 3;
pub const SDMA_PKT_COND_EXE_REFERENCE_reference_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_COND_EXE_REFERENCE_reference_shift: c_int = 0;

// define for EXEC_COUNT word
// define for exec_count field
pub const SDMA_PKT_COND_EXE_EXEC_COUNT_exec_count_offset: c_int = 4;
pub const SDMA_PKT_COND_EXE_EXEC_COUNT_exec_count_mask: c_uint = 0x00003FFF;
pub const SDMA_PKT_COND_EXE_EXEC_COUNT_exec_count_shift: c_int = 0;

//
// Definitions for SDMA_PKT_CONSTANT_FILL packet
//
// define for HEADER word
// define for op field
pub const SDMA_PKT_CONSTANT_FILL_HEADER_op_offset: c_int = 0;
pub const SDMA_PKT_CONSTANT_FILL_HEADER_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_CONSTANT_FILL_HEADER_op_shift: c_int = 0;

// define for sub_op field
pub const SDMA_PKT_CONSTANT_FILL_HEADER_sub_op_offset: c_int = 0;
pub const SDMA_PKT_CONSTANT_FILL_HEADER_sub_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_CONSTANT_FILL_HEADER_sub_op_shift: c_int = 8;

// define for sw field
pub const SDMA_PKT_CONSTANT_FILL_HEADER_sw_offset: c_int = 0;
pub const SDMA_PKT_CONSTANT_FILL_HEADER_sw_mask: c_uint = 0x00000003;
pub const SDMA_PKT_CONSTANT_FILL_HEADER_sw_shift: c_int = 16;

// define for fillsize field
pub const SDMA_PKT_CONSTANT_FILL_HEADER_fillsize_offset: c_int = 0;
pub const SDMA_PKT_CONSTANT_FILL_HEADER_fillsize_mask: c_uint = 0x00000003;
pub const SDMA_PKT_CONSTANT_FILL_HEADER_fillsize_shift: c_int = 30;

// define for DST_ADDR_LO word
// define for dst_addr_31_0 field
pub const SDMA_PKT_CONSTANT_FILL_DST_ADDR_LO_dst_addr_31_0_offset: c_int = 1;
pub const SDMA_PKT_CONSTANT_FILL_DST_ADDR_LO_dst_addr_31_0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_CONSTANT_FILL_DST_ADDR_LO_dst_addr_31_0_shift: c_int = 0;

// define for DST_ADDR_HI word
// define for dst_addr_63_32 field
pub const SDMA_PKT_CONSTANT_FILL_DST_ADDR_HI_dst_addr_63_32_offset: c_int = 2;
pub const SDMA_PKT_CONSTANT_FILL_DST_ADDR_HI_dst_addr_63_32_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_CONSTANT_FILL_DST_ADDR_HI_dst_addr_63_32_shift: c_int = 0;

// define for DATA word
// define for src_data_31_0 field
pub const SDMA_PKT_CONSTANT_FILL_DATA_src_data_31_0_offset: c_int = 3;
pub const SDMA_PKT_CONSTANT_FILL_DATA_src_data_31_0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_CONSTANT_FILL_DATA_src_data_31_0_shift: c_int = 0;

// define for COUNT word
// define for count field
pub const SDMA_PKT_CONSTANT_FILL_COUNT_count_offset: c_int = 4;
pub const SDMA_PKT_CONSTANT_FILL_COUNT_count_mask: c_uint = 0x003FFFFF;
pub const SDMA_PKT_CONSTANT_FILL_COUNT_count_shift: c_int = 0;

//
// Definitions for SDMA_PKT_DATA_FILL_MULTI packet
//
// define for HEADER word
// define for op field
pub const SDMA_PKT_DATA_FILL_MULTI_HEADER_op_offset: c_int = 0;
pub const SDMA_PKT_DATA_FILL_MULTI_HEADER_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_DATA_FILL_MULTI_HEADER_op_shift: c_int = 0;

// define for sub_op field
pub const SDMA_PKT_DATA_FILL_MULTI_HEADER_sub_op_offset: c_int = 0;
pub const SDMA_PKT_DATA_FILL_MULTI_HEADER_sub_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_DATA_FILL_MULTI_HEADER_sub_op_shift: c_int = 8;

// define for memlog_clr field
pub const SDMA_PKT_DATA_FILL_MULTI_HEADER_memlog_clr_offset: c_int = 0;
pub const SDMA_PKT_DATA_FILL_MULTI_HEADER_memlog_clr_mask: c_uint = 0x00000001;
pub const SDMA_PKT_DATA_FILL_MULTI_HEADER_memlog_clr_shift: c_int = 31;

// define for BYTE_STRIDE word
// define for byte_stride field
pub const SDMA_PKT_DATA_FILL_MULTI_BYTE_STRIDE_byte_stride_offset: c_int = 1;
pub const SDMA_PKT_DATA_FILL_MULTI_BYTE_STRIDE_byte_stride_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_DATA_FILL_MULTI_BYTE_STRIDE_byte_stride_shift: c_int = 0;

// define for DMA_COUNT word
// define for dma_count field
pub const SDMA_PKT_DATA_FILL_MULTI_DMA_COUNT_dma_count_offset: c_int = 2;
pub const SDMA_PKT_DATA_FILL_MULTI_DMA_COUNT_dma_count_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_DATA_FILL_MULTI_DMA_COUNT_dma_count_shift: c_int = 0;

// define for DST_ADDR_LO word
// define for dst_addr_31_0 field
pub const SDMA_PKT_DATA_FILL_MULTI_DST_ADDR_LO_dst_addr_31_0_offset: c_int = 3;
pub const SDMA_PKT_DATA_FILL_MULTI_DST_ADDR_LO_dst_addr_31_0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_DATA_FILL_MULTI_DST_ADDR_LO_dst_addr_31_0_shift: c_int = 0;

// define for DST_ADDR_HI word
// define for dst_addr_63_32 field
pub const SDMA_PKT_DATA_FILL_MULTI_DST_ADDR_HI_dst_addr_63_32_offset: c_int = 4;
pub const SDMA_PKT_DATA_FILL_MULTI_DST_ADDR_HI_dst_addr_63_32_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_DATA_FILL_MULTI_DST_ADDR_HI_dst_addr_63_32_shift: c_int = 0;

// define for BYTE_COUNT word
// define for count field
pub const SDMA_PKT_DATA_FILL_MULTI_BYTE_COUNT_count_offset: c_int = 5;
pub const SDMA_PKT_DATA_FILL_MULTI_BYTE_COUNT_count_mask: c_uint = 0x03FFFFFF;
pub const SDMA_PKT_DATA_FILL_MULTI_BYTE_COUNT_count_shift: c_int = 0;

//
// Definitions for SDMA_PKT_POLL_REGMEM packet
//
// define for HEADER word
// define for op field
pub const SDMA_PKT_POLL_REGMEM_HEADER_op_offset: c_int = 0;
pub const SDMA_PKT_POLL_REGMEM_HEADER_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_POLL_REGMEM_HEADER_op_shift: c_int = 0;

// define for sub_op field
pub const SDMA_PKT_POLL_REGMEM_HEADER_sub_op_offset: c_int = 0;
pub const SDMA_PKT_POLL_REGMEM_HEADER_sub_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_POLL_REGMEM_HEADER_sub_op_shift: c_int = 8;

// define for hdp_flush field
pub const SDMA_PKT_POLL_REGMEM_HEADER_hdp_flush_offset: c_int = 0;
pub const SDMA_PKT_POLL_REGMEM_HEADER_hdp_flush_mask: c_uint = 0x00000001;
pub const SDMA_PKT_POLL_REGMEM_HEADER_hdp_flush_shift: c_int = 26;

// define for func field
pub const SDMA_PKT_POLL_REGMEM_HEADER_func_offset: c_int = 0;
pub const SDMA_PKT_POLL_REGMEM_HEADER_func_mask: c_uint = 0x00000007;
pub const SDMA_PKT_POLL_REGMEM_HEADER_func_shift: c_int = 28;

// define for mem_poll field
pub const SDMA_PKT_POLL_REGMEM_HEADER_mem_poll_offset: c_int = 0;
pub const SDMA_PKT_POLL_REGMEM_HEADER_mem_poll_mask: c_uint = 0x00000001;
pub const SDMA_PKT_POLL_REGMEM_HEADER_mem_poll_shift: c_int = 31;

// define for ADDR_LO word
// define for addr_31_0 field
pub const SDMA_PKT_POLL_REGMEM_ADDR_LO_addr_31_0_offset: c_int = 1;
pub const SDMA_PKT_POLL_REGMEM_ADDR_LO_addr_31_0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_POLL_REGMEM_ADDR_LO_addr_31_0_shift: c_int = 0;

// define for ADDR_HI word
// define for addr_63_32 field
pub const SDMA_PKT_POLL_REGMEM_ADDR_HI_addr_63_32_offset: c_int = 2;
pub const SDMA_PKT_POLL_REGMEM_ADDR_HI_addr_63_32_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_POLL_REGMEM_ADDR_HI_addr_63_32_shift: c_int = 0;

// define for VALUE word
// define for value field
pub const SDMA_PKT_POLL_REGMEM_VALUE_value_offset: c_int = 3;
pub const SDMA_PKT_POLL_REGMEM_VALUE_value_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_POLL_REGMEM_VALUE_value_shift: c_int = 0;

// define for MASK word
// define for mask field
pub const SDMA_PKT_POLL_REGMEM_MASK_mask_offset: c_int = 4;
pub const SDMA_PKT_POLL_REGMEM_MASK_mask_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_POLL_REGMEM_MASK_mask_shift: c_int = 0;

// define for DW5 word
// define for interval field
pub const SDMA_PKT_POLL_REGMEM_DW5_interval_offset: c_int = 5;
pub const SDMA_PKT_POLL_REGMEM_DW5_interval_mask: c_uint = 0x0000FFFF;
pub const SDMA_PKT_POLL_REGMEM_DW5_interval_shift: c_int = 0;

// define for retry_count field
pub const SDMA_PKT_POLL_REGMEM_DW5_retry_count_offset: c_int = 5;
pub const SDMA_PKT_POLL_REGMEM_DW5_retry_count_mask: c_uint = 0x00000FFF;
pub const SDMA_PKT_POLL_REGMEM_DW5_retry_count_shift: c_int = 16;

//
// Definitions for SDMA_PKT_POLL_REG_WRITE_MEM packet
//
// define for HEADER word
// define for op field
pub const SDMA_PKT_POLL_REG_WRITE_MEM_HEADER_op_offset: c_int = 0;
pub const SDMA_PKT_POLL_REG_WRITE_MEM_HEADER_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_POLL_REG_WRITE_MEM_HEADER_op_shift: c_int = 0;

// define for sub_op field
pub const SDMA_PKT_POLL_REG_WRITE_MEM_HEADER_sub_op_offset: c_int = 0;
pub const SDMA_PKT_POLL_REG_WRITE_MEM_HEADER_sub_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_POLL_REG_WRITE_MEM_HEADER_sub_op_shift: c_int = 8;

// define for SRC_ADDR word
// define for addr_31_2 field
pub const SDMA_PKT_POLL_REG_WRITE_MEM_SRC_ADDR_addr_31_2_offset: c_int = 1;
pub const SDMA_PKT_POLL_REG_WRITE_MEM_SRC_ADDR_addr_31_2_mask: c_uint = 0x3FFFFFFF;
pub const SDMA_PKT_POLL_REG_WRITE_MEM_SRC_ADDR_addr_31_2_shift: c_int = 2;

// define for DST_ADDR_LO word
// define for addr_31_0 field
pub const SDMA_PKT_POLL_REG_WRITE_MEM_DST_ADDR_LO_addr_31_0_offset: c_int = 2;
pub const SDMA_PKT_POLL_REG_WRITE_MEM_DST_ADDR_LO_addr_31_0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_POLL_REG_WRITE_MEM_DST_ADDR_LO_addr_31_0_shift: c_int = 0;

// define for DST_ADDR_HI word
// define for addr_63_32 field
pub const SDMA_PKT_POLL_REG_WRITE_MEM_DST_ADDR_HI_addr_63_32_offset: c_int = 3;
pub const SDMA_PKT_POLL_REG_WRITE_MEM_DST_ADDR_HI_addr_63_32_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_POLL_REG_WRITE_MEM_DST_ADDR_HI_addr_63_32_shift: c_int = 0;

//
// Definitions for SDMA_PKT_POLL_DBIT_WRITE_MEM packet
//
// define for HEADER word
// define for op field
pub const SDMA_PKT_POLL_DBIT_WRITE_MEM_HEADER_op_offset: c_int = 0;
pub const SDMA_PKT_POLL_DBIT_WRITE_MEM_HEADER_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_POLL_DBIT_WRITE_MEM_HEADER_op_shift: c_int = 0;

// define for sub_op field
pub const SDMA_PKT_POLL_DBIT_WRITE_MEM_HEADER_sub_op_offset: c_int = 0;
pub const SDMA_PKT_POLL_DBIT_WRITE_MEM_HEADER_sub_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_POLL_DBIT_WRITE_MEM_HEADER_sub_op_shift: c_int = 8;

// define for ea field
pub const SDMA_PKT_POLL_DBIT_WRITE_MEM_HEADER_ea_offset: c_int = 0;
pub const SDMA_PKT_POLL_DBIT_WRITE_MEM_HEADER_ea_mask: c_uint = 0x00000003;
pub const SDMA_PKT_POLL_DBIT_WRITE_MEM_HEADER_ea_shift: c_int = 16;

// define for DST_ADDR_LO word
// define for addr_31_0 field
pub const SDMA_PKT_POLL_DBIT_WRITE_MEM_DST_ADDR_LO_addr_31_0_offset: c_int = 1;
pub const SDMA_PKT_POLL_DBIT_WRITE_MEM_DST_ADDR_LO_addr_31_0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_POLL_DBIT_WRITE_MEM_DST_ADDR_LO_addr_31_0_shift: c_int = 0;

// define for DST_ADDR_HI word
// define for addr_63_32 field
pub const SDMA_PKT_POLL_DBIT_WRITE_MEM_DST_ADDR_HI_addr_63_32_offset: c_int = 2;
pub const SDMA_PKT_POLL_DBIT_WRITE_MEM_DST_ADDR_HI_addr_63_32_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_POLL_DBIT_WRITE_MEM_DST_ADDR_HI_addr_63_32_shift: c_int = 0;

// define for START_PAGE word
// define for addr_31_4 field
pub const SDMA_PKT_POLL_DBIT_WRITE_MEM_START_PAGE_addr_31_4_offset: c_int = 3;
pub const SDMA_PKT_POLL_DBIT_WRITE_MEM_START_PAGE_addr_31_4_mask: c_uint = 0x0FFFFFFF;
pub const SDMA_PKT_POLL_DBIT_WRITE_MEM_START_PAGE_addr_31_4_shift: c_int = 4;

// define for PAGE_NUM word
// define for page_num_31_0 field
pub const SDMA_PKT_POLL_DBIT_WRITE_MEM_PAGE_NUM_page_num_31_0_offset: c_int = 4;
pub const SDMA_PKT_POLL_DBIT_WRITE_MEM_PAGE_NUM_page_num_31_0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_POLL_DBIT_WRITE_MEM_PAGE_NUM_page_num_31_0_shift: c_int = 0;

//
// Definitions for SDMA_PKT_POLL_MEM_VERIFY packet
//
// define for HEADER word
// define for op field
pub const SDMA_PKT_POLL_MEM_VERIFY_HEADER_op_offset: c_int = 0;
pub const SDMA_PKT_POLL_MEM_VERIFY_HEADER_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_POLL_MEM_VERIFY_HEADER_op_shift: c_int = 0;

// define for sub_op field
pub const SDMA_PKT_POLL_MEM_VERIFY_HEADER_sub_op_offset: c_int = 0;
pub const SDMA_PKT_POLL_MEM_VERIFY_HEADER_sub_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_POLL_MEM_VERIFY_HEADER_sub_op_shift: c_int = 8;

// define for mode field
pub const SDMA_PKT_POLL_MEM_VERIFY_HEADER_mode_offset: c_int = 0;
pub const SDMA_PKT_POLL_MEM_VERIFY_HEADER_mode_mask: c_uint = 0x00000001;
pub const SDMA_PKT_POLL_MEM_VERIFY_HEADER_mode_shift: c_int = 31;

// define for PATTERN word
// define for pattern field
pub const SDMA_PKT_POLL_MEM_VERIFY_PATTERN_pattern_offset: c_int = 1;
pub const SDMA_PKT_POLL_MEM_VERIFY_PATTERN_pattern_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_POLL_MEM_VERIFY_PATTERN_pattern_shift: c_int = 0;

// define for CMP0_ADDR_START_LO word
// define for cmp0_start_31_0 field
pub const SDMA_PKT_POLL_MEM_VERIFY_CMP0_ADDR_START_LO_cmp0_start_31_0_offset: c_int = 2;
pub const SDMA_PKT_POLL_MEM_VERIFY_CMP0_ADDR_START_LO_cmp0_start_31_0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_POLL_MEM_VERIFY_CMP0_ADDR_START_LO_cmp0_start_31_0_shift: c_int = 0;

// define for CMP0_ADDR_START_HI word
// define for cmp0_start_63_32 field
pub const SDMA_PKT_POLL_MEM_VERIFY_CMP0_ADDR_START_HI_cmp0_start_63_32_offset: c_int = 3;
pub const SDMA_PKT_POLL_MEM_VERIFY_CMP0_ADDR_START_HI_cmp0_start_63_32_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_POLL_MEM_VERIFY_CMP0_ADDR_START_HI_cmp0_start_63_32_shift: c_int = 0;

// define for CMP0_ADDR_END_LO word
// define for cmp1_end_31_0 field
pub const SDMA_PKT_POLL_MEM_VERIFY_CMP0_ADDR_END_LO_cmp1_end_31_0_offset: c_int = 4;
pub const SDMA_PKT_POLL_MEM_VERIFY_CMP0_ADDR_END_LO_cmp1_end_31_0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_POLL_MEM_VERIFY_CMP0_ADDR_END_LO_cmp1_end_31_0_shift: c_int = 0;

// define for CMP0_ADDR_END_HI word
// define for cmp1_end_63_32 field
pub const SDMA_PKT_POLL_MEM_VERIFY_CMP0_ADDR_END_HI_cmp1_end_63_32_offset: c_int = 5;
pub const SDMA_PKT_POLL_MEM_VERIFY_CMP0_ADDR_END_HI_cmp1_end_63_32_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_POLL_MEM_VERIFY_CMP0_ADDR_END_HI_cmp1_end_63_32_shift: c_int = 0;

// define for CMP1_ADDR_START_LO word
// define for cmp1_start_31_0 field
pub const SDMA_PKT_POLL_MEM_VERIFY_CMP1_ADDR_START_LO_cmp1_start_31_0_offset: c_int = 6;
pub const SDMA_PKT_POLL_MEM_VERIFY_CMP1_ADDR_START_LO_cmp1_start_31_0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_POLL_MEM_VERIFY_CMP1_ADDR_START_LO_cmp1_start_31_0_shift: c_int = 0;

// define for CMP1_ADDR_START_HI word
// define for cmp1_start_63_32 field
pub const SDMA_PKT_POLL_MEM_VERIFY_CMP1_ADDR_START_HI_cmp1_start_63_32_offset: c_int = 7;
pub const SDMA_PKT_POLL_MEM_VERIFY_CMP1_ADDR_START_HI_cmp1_start_63_32_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_POLL_MEM_VERIFY_CMP1_ADDR_START_HI_cmp1_start_63_32_shift: c_int = 0;

// define for CMP1_ADDR_END_LO word
// define for cmp1_end_31_0 field
pub const SDMA_PKT_POLL_MEM_VERIFY_CMP1_ADDR_END_LO_cmp1_end_31_0_offset: c_int = 8;
pub const SDMA_PKT_POLL_MEM_VERIFY_CMP1_ADDR_END_LO_cmp1_end_31_0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_POLL_MEM_VERIFY_CMP1_ADDR_END_LO_cmp1_end_31_0_shift: c_int = 0;

// define for CMP1_ADDR_END_HI word
// define for cmp1_end_63_32 field
pub const SDMA_PKT_POLL_MEM_VERIFY_CMP1_ADDR_END_HI_cmp1_end_63_32_offset: c_int = 9;
pub const SDMA_PKT_POLL_MEM_VERIFY_CMP1_ADDR_END_HI_cmp1_end_63_32_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_POLL_MEM_VERIFY_CMP1_ADDR_END_HI_cmp1_end_63_32_shift: c_int = 0;

// define for REC_ADDR_LO word
// define for rec_31_0 field
pub const SDMA_PKT_POLL_MEM_VERIFY_REC_ADDR_LO_rec_31_0_offset: c_int = 10;
pub const SDMA_PKT_POLL_MEM_VERIFY_REC_ADDR_LO_rec_31_0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_POLL_MEM_VERIFY_REC_ADDR_LO_rec_31_0_shift: c_int = 0;

// define for REC_ADDR_HI word
// define for rec_63_32 field
pub const SDMA_PKT_POLL_MEM_VERIFY_REC_ADDR_HI_rec_63_32_offset: c_int = 11;
pub const SDMA_PKT_POLL_MEM_VERIFY_REC_ADDR_HI_rec_63_32_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_POLL_MEM_VERIFY_REC_ADDR_HI_rec_63_32_shift: c_int = 0;

// define for RESERVED word
// define for reserved field
pub const SDMA_PKT_POLL_MEM_VERIFY_RESERVED_reserved_offset: c_int = 12;
pub const SDMA_PKT_POLL_MEM_VERIFY_RESERVED_reserved_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_POLL_MEM_VERIFY_RESERVED_reserved_shift: c_int = 0;

//
// Definitions for SDMA_PKT_ATOMIC packet
//
// define for HEADER word
// define for op field
pub const SDMA_PKT_ATOMIC_HEADER_op_offset: c_int = 0;
pub const SDMA_PKT_ATOMIC_HEADER_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_ATOMIC_HEADER_op_shift: c_int = 0;

// define for loop field
pub const SDMA_PKT_ATOMIC_HEADER_loop_offset: c_int = 0;
pub const SDMA_PKT_ATOMIC_HEADER_loop_mask: c_uint = 0x00000001;
pub const SDMA_PKT_ATOMIC_HEADER_loop_shift: c_int = 16;

// define for tmz field
pub const SDMA_PKT_ATOMIC_HEADER_tmz_offset: c_int = 0;
pub const SDMA_PKT_ATOMIC_HEADER_tmz_mask: c_uint = 0x00000001;
pub const SDMA_PKT_ATOMIC_HEADER_tmz_shift: c_int = 18;

// define for atomic_op field
pub const SDMA_PKT_ATOMIC_HEADER_atomic_op_offset: c_int = 0;
pub const SDMA_PKT_ATOMIC_HEADER_atomic_op_mask: c_uint = 0x0000007F;
pub const SDMA_PKT_ATOMIC_HEADER_atomic_op_shift: c_int = 25;

// define for ADDR_LO word
// define for addr_31_0 field
pub const SDMA_PKT_ATOMIC_ADDR_LO_addr_31_0_offset: c_int = 1;
pub const SDMA_PKT_ATOMIC_ADDR_LO_addr_31_0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_ATOMIC_ADDR_LO_addr_31_0_shift: c_int = 0;

// define for ADDR_HI word
// define for addr_63_32 field
pub const SDMA_PKT_ATOMIC_ADDR_HI_addr_63_32_offset: c_int = 2;
pub const SDMA_PKT_ATOMIC_ADDR_HI_addr_63_32_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_ATOMIC_ADDR_HI_addr_63_32_shift: c_int = 0;

// define for SRC_DATA_LO word
// define for src_data_31_0 field
pub const SDMA_PKT_ATOMIC_SRC_DATA_LO_src_data_31_0_offset: c_int = 3;
pub const SDMA_PKT_ATOMIC_SRC_DATA_LO_src_data_31_0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_ATOMIC_SRC_DATA_LO_src_data_31_0_shift: c_int = 0;

// define for SRC_DATA_HI word
// define for src_data_63_32 field
pub const SDMA_PKT_ATOMIC_SRC_DATA_HI_src_data_63_32_offset: c_int = 4;
pub const SDMA_PKT_ATOMIC_SRC_DATA_HI_src_data_63_32_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_ATOMIC_SRC_DATA_HI_src_data_63_32_shift: c_int = 0;

// define for CMP_DATA_LO word
// define for cmp_data_31_0 field
pub const SDMA_PKT_ATOMIC_CMP_DATA_LO_cmp_data_31_0_offset: c_int = 5;
pub const SDMA_PKT_ATOMIC_CMP_DATA_LO_cmp_data_31_0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_ATOMIC_CMP_DATA_LO_cmp_data_31_0_shift: c_int = 0;

// define for CMP_DATA_HI word
// define for cmp_data_63_32 field
pub const SDMA_PKT_ATOMIC_CMP_DATA_HI_cmp_data_63_32_offset: c_int = 6;
pub const SDMA_PKT_ATOMIC_CMP_DATA_HI_cmp_data_63_32_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_ATOMIC_CMP_DATA_HI_cmp_data_63_32_shift: c_int = 0;

// define for LOOP_INTERVAL word
// define for loop_interval field
pub const SDMA_PKT_ATOMIC_LOOP_INTERVAL_loop_interval_offset: c_int = 7;
pub const SDMA_PKT_ATOMIC_LOOP_INTERVAL_loop_interval_mask: c_uint = 0x00001FFF;
pub const SDMA_PKT_ATOMIC_LOOP_INTERVAL_loop_interval_shift: c_int = 0;

//
// Definitions for SDMA_PKT_TIMESTAMP_SET packet
//
// define for HEADER word
// define for op field
pub const SDMA_PKT_TIMESTAMP_SET_HEADER_op_offset: c_int = 0;
pub const SDMA_PKT_TIMESTAMP_SET_HEADER_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_TIMESTAMP_SET_HEADER_op_shift: c_int = 0;

// define for sub_op field
pub const SDMA_PKT_TIMESTAMP_SET_HEADER_sub_op_offset: c_int = 0;
pub const SDMA_PKT_TIMESTAMP_SET_HEADER_sub_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_TIMESTAMP_SET_HEADER_sub_op_shift: c_int = 8;

// define for INIT_DATA_LO word
// define for init_data_31_0 field
pub const SDMA_PKT_TIMESTAMP_SET_INIT_DATA_LO_init_data_31_0_offset: c_int = 1;
pub const SDMA_PKT_TIMESTAMP_SET_INIT_DATA_LO_init_data_31_0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_TIMESTAMP_SET_INIT_DATA_LO_init_data_31_0_shift: c_int = 0;

// define for INIT_DATA_HI word
// define for init_data_63_32 field
pub const SDMA_PKT_TIMESTAMP_SET_INIT_DATA_HI_init_data_63_32_offset: c_int = 2;
pub const SDMA_PKT_TIMESTAMP_SET_INIT_DATA_HI_init_data_63_32_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_TIMESTAMP_SET_INIT_DATA_HI_init_data_63_32_shift: c_int = 0;

//
// Definitions for SDMA_PKT_TIMESTAMP_GET packet
//
// define for HEADER word
// define for op field
pub const SDMA_PKT_TIMESTAMP_GET_HEADER_op_offset: c_int = 0;
pub const SDMA_PKT_TIMESTAMP_GET_HEADER_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_TIMESTAMP_GET_HEADER_op_shift: c_int = 0;

// define for sub_op field
pub const SDMA_PKT_TIMESTAMP_GET_HEADER_sub_op_offset: c_int = 0;
pub const SDMA_PKT_TIMESTAMP_GET_HEADER_sub_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_TIMESTAMP_GET_HEADER_sub_op_shift: c_int = 8;

// define for WRITE_ADDR_LO word
// define for write_addr_31_3 field
pub const SDMA_PKT_TIMESTAMP_GET_WRITE_ADDR_LO_write_addr_31_3_offset: c_int = 1;
pub const SDMA_PKT_TIMESTAMP_GET_WRITE_ADDR_LO_write_addr_31_3_mask: c_uint = 0x1FFFFFFF;
pub const SDMA_PKT_TIMESTAMP_GET_WRITE_ADDR_LO_write_addr_31_3_shift: c_int = 3;

// define for WRITE_ADDR_HI word
// define for write_addr_63_32 field
pub const SDMA_PKT_TIMESTAMP_GET_WRITE_ADDR_HI_write_addr_63_32_offset: c_int = 2;
pub const SDMA_PKT_TIMESTAMP_GET_WRITE_ADDR_HI_write_addr_63_32_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_TIMESTAMP_GET_WRITE_ADDR_HI_write_addr_63_32_shift: c_int = 0;

//
// Definitions for SDMA_PKT_TIMESTAMP_GET_GLOBAL packet
//
// define for HEADER word
// define for op field
pub const SDMA_PKT_TIMESTAMP_GET_GLOBAL_HEADER_op_offset: c_int = 0;
pub const SDMA_PKT_TIMESTAMP_GET_GLOBAL_HEADER_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_TIMESTAMP_GET_GLOBAL_HEADER_op_shift: c_int = 0;

// define for sub_op field
pub const SDMA_PKT_TIMESTAMP_GET_GLOBAL_HEADER_sub_op_offset: c_int = 0;
pub const SDMA_PKT_TIMESTAMP_GET_GLOBAL_HEADER_sub_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_TIMESTAMP_GET_GLOBAL_HEADER_sub_op_shift: c_int = 8;

// define for WRITE_ADDR_LO word
// define for write_addr_31_3 field
pub const SDMA_PKT_TIMESTAMP_GET_GLOBAL_WRITE_ADDR_LO_write_addr_31_3_offset: c_int = 1;
pub const SDMA_PKT_TIMESTAMP_GET_GLOBAL_WRITE_ADDR_LO_write_addr_31_3_mask: c_uint = 0x1FFFFFFF;
pub const SDMA_PKT_TIMESTAMP_GET_GLOBAL_WRITE_ADDR_LO_write_addr_31_3_shift: c_int = 3;

// define for WRITE_ADDR_HI word
// define for write_addr_63_32 field
pub const SDMA_PKT_TIMESTAMP_GET_GLOBAL_WRITE_ADDR_HI_write_addr_63_32_offset: c_int = 2;
pub const SDMA_PKT_TIMESTAMP_GET_GLOBAL_WRITE_ADDR_HI_write_addr_63_32_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_TIMESTAMP_GET_GLOBAL_WRITE_ADDR_HI_write_addr_63_32_shift: c_int = 0;

//
// Definitions for SDMA_PKT_TRAP packet
//
// define for HEADER word
// define for op field
pub const SDMA_PKT_TRAP_HEADER_op_offset: c_int = 0;
pub const SDMA_PKT_TRAP_HEADER_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_TRAP_HEADER_op_shift: c_int = 0;

// define for sub_op field
pub const SDMA_PKT_TRAP_HEADER_sub_op_offset: c_int = 0;
pub const SDMA_PKT_TRAP_HEADER_sub_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_TRAP_HEADER_sub_op_shift: c_int = 8;

// define for INT_CONTEXT word
// define for int_context field
pub const SDMA_PKT_TRAP_INT_CONTEXT_int_context_offset: c_int = 1;
pub const SDMA_PKT_TRAP_INT_CONTEXT_int_context_mask: c_uint = 0x0FFFFFFF;
pub const SDMA_PKT_TRAP_INT_CONTEXT_int_context_shift: c_int = 0;

//
// Definitions for SDMA_PKT_DUMMY_TRAP packet
//
// define for HEADER word
// define for op field
pub const SDMA_PKT_DUMMY_TRAP_HEADER_op_offset: c_int = 0;
pub const SDMA_PKT_DUMMY_TRAP_HEADER_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_DUMMY_TRAP_HEADER_op_shift: c_int = 0;

// define for sub_op field
pub const SDMA_PKT_DUMMY_TRAP_HEADER_sub_op_offset: c_int = 0;
pub const SDMA_PKT_DUMMY_TRAP_HEADER_sub_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_DUMMY_TRAP_HEADER_sub_op_shift: c_int = 8;

// define for INT_CONTEXT word
// define for int_context field
pub const SDMA_PKT_DUMMY_TRAP_INT_CONTEXT_int_context_offset: c_int = 1;
pub const SDMA_PKT_DUMMY_TRAP_INT_CONTEXT_int_context_mask: c_uint = 0x0FFFFFFF;
pub const SDMA_PKT_DUMMY_TRAP_INT_CONTEXT_int_context_shift: c_int = 0;

//
// Definitions for SDMA_PKT_NOP packet
//
// define for HEADER word
// define for op field
pub const SDMA_PKT_NOP_HEADER_op_offset: c_int = 0;
pub const SDMA_PKT_NOP_HEADER_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_NOP_HEADER_op_shift: c_int = 0;

// define for sub_op field
pub const SDMA_PKT_NOP_HEADER_sub_op_offset: c_int = 0;
pub const SDMA_PKT_NOP_HEADER_sub_op_mask: c_uint = 0x000000FF;
pub const SDMA_PKT_NOP_HEADER_sub_op_shift: c_int = 8;

// define for count field
pub const SDMA_PKT_NOP_HEADER_count_offset: c_int = 0;
pub const SDMA_PKT_NOP_HEADER_count_mask: c_uint = 0x00003FFF;
pub const SDMA_PKT_NOP_HEADER_count_shift: c_int = 16;

// define for DATA0 word
// define for data0 field
pub const SDMA_PKT_NOP_DATA0_data0_offset: c_int = 1;
pub const SDMA_PKT_NOP_DATA0_data0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_NOP_DATA0_data0_shift: c_int = 0;

//
// Definitions for SDMA_AQL_PKT_HEADER packet
//
// define for HEADER word
// define for format field
pub const SDMA_AQL_PKT_HEADER_HEADER_format_offset: c_int = 0;
pub const SDMA_AQL_PKT_HEADER_HEADER_format_mask: c_uint = 0x000000FF;
pub const SDMA_AQL_PKT_HEADER_HEADER_format_shift: c_int = 0;

// define for barrier field
pub const SDMA_AQL_PKT_HEADER_HEADER_barrier_offset: c_int = 0;
pub const SDMA_AQL_PKT_HEADER_HEADER_barrier_mask: c_uint = 0x00000001;
pub const SDMA_AQL_PKT_HEADER_HEADER_barrier_shift: c_int = 8;

// define for acquire_fence_scope field
pub const SDMA_AQL_PKT_HEADER_HEADER_acquire_fence_scope_offset: c_int = 0;
pub const SDMA_AQL_PKT_HEADER_HEADER_acquire_fence_scope_mask: c_uint = 0x00000003;
pub const SDMA_AQL_PKT_HEADER_HEADER_acquire_fence_scope_shift: c_int = 9;

// define for release_fence_scope field
pub const SDMA_AQL_PKT_HEADER_HEADER_release_fence_scope_offset: c_int = 0;
pub const SDMA_AQL_PKT_HEADER_HEADER_release_fence_scope_mask: c_uint = 0x00000003;
pub const SDMA_AQL_PKT_HEADER_HEADER_release_fence_scope_shift: c_int = 11;

// define for reserved field
pub const SDMA_AQL_PKT_HEADER_HEADER_reserved_offset: c_int = 0;
pub const SDMA_AQL_PKT_HEADER_HEADER_reserved_mask: c_uint = 0x00000007;
pub const SDMA_AQL_PKT_HEADER_HEADER_reserved_shift: c_int = 13;

// define for op field
pub const SDMA_AQL_PKT_HEADER_HEADER_op_offset: c_int = 0;
pub const SDMA_AQL_PKT_HEADER_HEADER_op_mask: c_uint = 0x0000000F;
pub const SDMA_AQL_PKT_HEADER_HEADER_op_shift: c_int = 16;

// define for subop field
pub const SDMA_AQL_PKT_HEADER_HEADER_subop_offset: c_int = 0;
pub const SDMA_AQL_PKT_HEADER_HEADER_subop_mask: c_uint = 0x00000007;
pub const SDMA_AQL_PKT_HEADER_HEADER_subop_shift: c_int = 20;

//
// Definitions for SDMA_AQL_PKT_COPY_LINEAR packet
//
// define for HEADER word
// define for format field
pub const SDMA_AQL_PKT_COPY_LINEAR_HEADER_format_offset: c_int = 0;
pub const SDMA_AQL_PKT_COPY_LINEAR_HEADER_format_mask: c_uint = 0x000000FF;
pub const SDMA_AQL_PKT_COPY_LINEAR_HEADER_format_shift: c_int = 0;

// define for barrier field
pub const SDMA_AQL_PKT_COPY_LINEAR_HEADER_barrier_offset: c_int = 0;
pub const SDMA_AQL_PKT_COPY_LINEAR_HEADER_barrier_mask: c_uint = 0x00000001;
pub const SDMA_AQL_PKT_COPY_LINEAR_HEADER_barrier_shift: c_int = 8;

// define for acquire_fence_scope field
pub const SDMA_AQL_PKT_COPY_LINEAR_HEADER_acquire_fence_scope_offset: c_int = 0;
pub const SDMA_AQL_PKT_COPY_LINEAR_HEADER_acquire_fence_scope_mask: c_uint = 0x00000003;
pub const SDMA_AQL_PKT_COPY_LINEAR_HEADER_acquire_fence_scope_shift: c_int = 9;

// define for release_fence_scope field
pub const SDMA_AQL_PKT_COPY_LINEAR_HEADER_release_fence_scope_offset: c_int = 0;
pub const SDMA_AQL_PKT_COPY_LINEAR_HEADER_release_fence_scope_mask: c_uint = 0x00000003;
pub const SDMA_AQL_PKT_COPY_LINEAR_HEADER_release_fence_scope_shift: c_int = 11;

// define for reserved field
pub const SDMA_AQL_PKT_COPY_LINEAR_HEADER_reserved_offset: c_int = 0;
pub const SDMA_AQL_PKT_COPY_LINEAR_HEADER_reserved_mask: c_uint = 0x00000007;
pub const SDMA_AQL_PKT_COPY_LINEAR_HEADER_reserved_shift: c_int = 13;

// define for op field
pub const SDMA_AQL_PKT_COPY_LINEAR_HEADER_op_offset: c_int = 0;
pub const SDMA_AQL_PKT_COPY_LINEAR_HEADER_op_mask: c_uint = 0x0000000F;
pub const SDMA_AQL_PKT_COPY_LINEAR_HEADER_op_shift: c_int = 16;

// define for subop field
pub const SDMA_AQL_PKT_COPY_LINEAR_HEADER_subop_offset: c_int = 0;
pub const SDMA_AQL_PKT_COPY_LINEAR_HEADER_subop_mask: c_uint = 0x00000007;
pub const SDMA_AQL_PKT_COPY_LINEAR_HEADER_subop_shift: c_int = 20;

// define for RESERVED_DW1 word
// define for reserved_dw1 field
pub const SDMA_AQL_PKT_COPY_LINEAR_RESERVED_DW1_reserved_dw1_offset: c_int = 1;
pub const SDMA_AQL_PKT_COPY_LINEAR_RESERVED_DW1_reserved_dw1_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_AQL_PKT_COPY_LINEAR_RESERVED_DW1_reserved_dw1_shift: c_int = 0;

// define for RETURN_ADDR_LO word
// define for return_addr_31_0 field
pub const SDMA_AQL_PKT_COPY_LINEAR_RETURN_ADDR_LO_return_addr_31_0_offset: c_int = 2;
pub const SDMA_AQL_PKT_COPY_LINEAR_RETURN_ADDR_LO_return_addr_31_0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_AQL_PKT_COPY_LINEAR_RETURN_ADDR_LO_return_addr_31_0_shift: c_int = 0;

// define for RETURN_ADDR_HI word
// define for return_addr_63_32 field
pub const SDMA_AQL_PKT_COPY_LINEAR_RETURN_ADDR_HI_return_addr_63_32_offset: c_int = 3;
pub const SDMA_AQL_PKT_COPY_LINEAR_RETURN_ADDR_HI_return_addr_63_32_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_AQL_PKT_COPY_LINEAR_RETURN_ADDR_HI_return_addr_63_32_shift: c_int = 0;

// define for COUNT word
// define for count field
pub const SDMA_AQL_PKT_COPY_LINEAR_COUNT_count_offset: c_int = 4;
pub const SDMA_AQL_PKT_COPY_LINEAR_COUNT_count_mask: c_uint = 0x003FFFFF;
pub const SDMA_AQL_PKT_COPY_LINEAR_COUNT_count_shift: c_int = 0;

// define for PARAMETER word
// define for dst_sw field
pub const SDMA_AQL_PKT_COPY_LINEAR_PARAMETER_dst_sw_offset: c_int = 5;
pub const SDMA_AQL_PKT_COPY_LINEAR_PARAMETER_dst_sw_mask: c_uint = 0x00000003;
pub const SDMA_AQL_PKT_COPY_LINEAR_PARAMETER_dst_sw_shift: c_int = 16;

// define for src_sw field
pub const SDMA_AQL_PKT_COPY_LINEAR_PARAMETER_src_sw_offset: c_int = 5;
pub const SDMA_AQL_PKT_COPY_LINEAR_PARAMETER_src_sw_mask: c_uint = 0x00000003;
pub const SDMA_AQL_PKT_COPY_LINEAR_PARAMETER_src_sw_shift: c_int = 24;

// define for SRC_ADDR_LO word
// define for src_addr_31_0 field
pub const SDMA_AQL_PKT_COPY_LINEAR_SRC_ADDR_LO_src_addr_31_0_offset: c_int = 6;
pub const SDMA_AQL_PKT_COPY_LINEAR_SRC_ADDR_LO_src_addr_31_0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_AQL_PKT_COPY_LINEAR_SRC_ADDR_LO_src_addr_31_0_shift: c_int = 0;

// define for SRC_ADDR_HI word
// define for src_addr_63_32 field
pub const SDMA_AQL_PKT_COPY_LINEAR_SRC_ADDR_HI_src_addr_63_32_offset: c_int = 7;
pub const SDMA_AQL_PKT_COPY_LINEAR_SRC_ADDR_HI_src_addr_63_32_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_AQL_PKT_COPY_LINEAR_SRC_ADDR_HI_src_addr_63_32_shift: c_int = 0;

// define for DST_ADDR_LO word
// define for dst_addr_31_0 field
pub const SDMA_AQL_PKT_COPY_LINEAR_DST_ADDR_LO_dst_addr_31_0_offset: c_int = 8;
pub const SDMA_AQL_PKT_COPY_LINEAR_DST_ADDR_LO_dst_addr_31_0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_AQL_PKT_COPY_LINEAR_DST_ADDR_LO_dst_addr_31_0_shift: c_int = 0;

// define for DST_ADDR_HI word
// define for dst_addr_63_32 field
pub const SDMA_AQL_PKT_COPY_LINEAR_DST_ADDR_HI_dst_addr_63_32_offset: c_int = 9;
pub const SDMA_AQL_PKT_COPY_LINEAR_DST_ADDR_HI_dst_addr_63_32_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_AQL_PKT_COPY_LINEAR_DST_ADDR_HI_dst_addr_63_32_shift: c_int = 0;

// define for RESERVED_DW10 word
// define for reserved_dw10 field
pub const SDMA_AQL_PKT_COPY_LINEAR_RESERVED_DW10_reserved_dw10_offset: c_int = 10;
pub const SDMA_AQL_PKT_COPY_LINEAR_RESERVED_DW10_reserved_dw10_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_AQL_PKT_COPY_LINEAR_RESERVED_DW10_reserved_dw10_shift: c_int = 0;

// define for RESERVED_DW11 word
// define for reserved_dw11 field
pub const SDMA_AQL_PKT_COPY_LINEAR_RESERVED_DW11_reserved_dw11_offset: c_int = 11;
pub const SDMA_AQL_PKT_COPY_LINEAR_RESERVED_DW11_reserved_dw11_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_AQL_PKT_COPY_LINEAR_RESERVED_DW11_reserved_dw11_shift: c_int = 0;

// define for RESERVED_DW12 word
// define for reserved_dw12 field
pub const SDMA_AQL_PKT_COPY_LINEAR_RESERVED_DW12_reserved_dw12_offset: c_int = 12;
pub const SDMA_AQL_PKT_COPY_LINEAR_RESERVED_DW12_reserved_dw12_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_AQL_PKT_COPY_LINEAR_RESERVED_DW12_reserved_dw12_shift: c_int = 0;

// define for RESERVED_DW13 word
// define for reserved_dw13 field
pub const SDMA_AQL_PKT_COPY_LINEAR_RESERVED_DW13_reserved_dw13_offset: c_int = 13;
pub const SDMA_AQL_PKT_COPY_LINEAR_RESERVED_DW13_reserved_dw13_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_AQL_PKT_COPY_LINEAR_RESERVED_DW13_reserved_dw13_shift: c_int = 0;

// define for COMPLETION_SIGNAL_LO word
// define for completion_signal_31_0 field
pub const SDMA_AQL_PKT_COPY_LINEAR_COMPLETION_SIGNAL_LO_completion_signal_31_0_offset: c_int = 14;
pub const SDMA_AQL_PKT_COPY_LINEAR_COMPLETION_SIGNAL_LO_completion_signal_31_0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_AQL_PKT_COPY_LINEAR_COMPLETION_SIGNAL_LO_completion_signal_31_0_shift: c_int = 0;

// define for COMPLETION_SIGNAL_HI word
// define for completion_signal_63_32 field
pub const SDMA_AQL_PKT_COPY_LINEAR_COMPLETION_SIGNAL_HI_completion_signal_63_32_offset: c_int = 15;
pub const SDMA_AQL_PKT_COPY_LINEAR_COMPLETION_SIGNAL_HI_completion_signal_63_32_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_AQL_PKT_COPY_LINEAR_COMPLETION_SIGNAL_HI_completion_signal_63_32_shift: c_int = 0;

//
// Definitions for SDMA_AQL_PKT_BARRIER_OR packet
//
// define for HEADER word
// define for format field
pub const SDMA_AQL_PKT_BARRIER_OR_HEADER_format_offset: c_int = 0;
pub const SDMA_AQL_PKT_BARRIER_OR_HEADER_format_mask: c_uint = 0x000000FF;
pub const SDMA_AQL_PKT_BARRIER_OR_HEADER_format_shift: c_int = 0;

// define for barrier field
pub const SDMA_AQL_PKT_BARRIER_OR_HEADER_barrier_offset: c_int = 0;
pub const SDMA_AQL_PKT_BARRIER_OR_HEADER_barrier_mask: c_uint = 0x00000001;
pub const SDMA_AQL_PKT_BARRIER_OR_HEADER_barrier_shift: c_int = 8;

// define for acquire_fence_scope field
pub const SDMA_AQL_PKT_BARRIER_OR_HEADER_acquire_fence_scope_offset: c_int = 0;
pub const SDMA_AQL_PKT_BARRIER_OR_HEADER_acquire_fence_scope_mask: c_uint = 0x00000003;
pub const SDMA_AQL_PKT_BARRIER_OR_HEADER_acquire_fence_scope_shift: c_int = 9;

// define for release_fence_scope field
pub const SDMA_AQL_PKT_BARRIER_OR_HEADER_release_fence_scope_offset: c_int = 0;
pub const SDMA_AQL_PKT_BARRIER_OR_HEADER_release_fence_scope_mask: c_uint = 0x00000003;
pub const SDMA_AQL_PKT_BARRIER_OR_HEADER_release_fence_scope_shift: c_int = 11;

// define for reserved field
pub const SDMA_AQL_PKT_BARRIER_OR_HEADER_reserved_offset: c_int = 0;
pub const SDMA_AQL_PKT_BARRIER_OR_HEADER_reserved_mask: c_uint = 0x00000007;
pub const SDMA_AQL_PKT_BARRIER_OR_HEADER_reserved_shift: c_int = 13;

// define for op field
pub const SDMA_AQL_PKT_BARRIER_OR_HEADER_op_offset: c_int = 0;
pub const SDMA_AQL_PKT_BARRIER_OR_HEADER_op_mask: c_uint = 0x0000000F;
pub const SDMA_AQL_PKT_BARRIER_OR_HEADER_op_shift: c_int = 16;

// define for subop field
pub const SDMA_AQL_PKT_BARRIER_OR_HEADER_subop_offset: c_int = 0;
pub const SDMA_AQL_PKT_BARRIER_OR_HEADER_subop_mask: c_uint = 0x00000007;
pub const SDMA_AQL_PKT_BARRIER_OR_HEADER_subop_shift: c_int = 20;

// define for RESERVED_DW1 word
// define for reserved_dw1 field
pub const SDMA_AQL_PKT_BARRIER_OR_RESERVED_DW1_reserved_dw1_offset: c_int = 1;
pub const SDMA_AQL_PKT_BARRIER_OR_RESERVED_DW1_reserved_dw1_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_AQL_PKT_BARRIER_OR_RESERVED_DW1_reserved_dw1_shift: c_int = 0;

// define for DEPENDENT_ADDR_0_LO word
// define for dependent_addr_0_31_0 field
pub const SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_0_LO_dependent_addr_0_31_0_offset: c_int = 2;
pub const SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_0_LO_dependent_addr_0_31_0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_0_LO_dependent_addr_0_31_0_shift: c_int = 0;

// define for DEPENDENT_ADDR_0_HI word
// define for dependent_addr_0_63_32 field
pub const SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_0_HI_dependent_addr_0_63_32_offset: c_int = 3;
pub const SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_0_HI_dependent_addr_0_63_32_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_0_HI_dependent_addr_0_63_32_shift: c_int = 0;

// define for DEPENDENT_ADDR_1_LO word
// define for dependent_addr_1_31_0 field
pub const SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_1_LO_dependent_addr_1_31_0_offset: c_int = 4;
pub const SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_1_LO_dependent_addr_1_31_0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_1_LO_dependent_addr_1_31_0_shift: c_int = 0;

// define for DEPENDENT_ADDR_1_HI word
// define for dependent_addr_1_63_32 field
pub const SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_1_HI_dependent_addr_1_63_32_offset: c_int = 5;
pub const SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_1_HI_dependent_addr_1_63_32_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_1_HI_dependent_addr_1_63_32_shift: c_int = 0;

// define for DEPENDENT_ADDR_2_LO word
// define for dependent_addr_2_31_0 field
pub const SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_2_LO_dependent_addr_2_31_0_offset: c_int = 6;
pub const SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_2_LO_dependent_addr_2_31_0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_2_LO_dependent_addr_2_31_0_shift: c_int = 0;

// define for DEPENDENT_ADDR_2_HI word
// define for dependent_addr_2_63_32 field
pub const SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_2_HI_dependent_addr_2_63_32_offset: c_int = 7;
pub const SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_2_HI_dependent_addr_2_63_32_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_2_HI_dependent_addr_2_63_32_shift: c_int = 0;

// define for DEPENDENT_ADDR_3_LO word
// define for dependent_addr_3_31_0 field
pub const SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_3_LO_dependent_addr_3_31_0_offset: c_int = 8;
pub const SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_3_LO_dependent_addr_3_31_0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_3_LO_dependent_addr_3_31_0_shift: c_int = 0;

// define for DEPENDENT_ADDR_3_HI word
// define for dependent_addr_3_63_32 field
pub const SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_3_HI_dependent_addr_3_63_32_offset: c_int = 9;
pub const SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_3_HI_dependent_addr_3_63_32_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_3_HI_dependent_addr_3_63_32_shift: c_int = 0;

// define for DEPENDENT_ADDR_4_LO word
// define for dependent_addr_4_31_0 field
pub const SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_4_LO_dependent_addr_4_31_0_offset: c_int = 10;
pub const SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_4_LO_dependent_addr_4_31_0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_4_LO_dependent_addr_4_31_0_shift: c_int = 0;

// define for DEPENDENT_ADDR_4_HI word
// define for dependent_addr_4_63_32 field
pub const SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_4_HI_dependent_addr_4_63_32_offset: c_int = 11;
pub const SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_4_HI_dependent_addr_4_63_32_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_4_HI_dependent_addr_4_63_32_shift: c_int = 0;

// define for RESERVED_DW12 word
// define for reserved_dw12 field
pub const SDMA_AQL_PKT_BARRIER_OR_RESERVED_DW12_reserved_dw12_offset: c_int = 12;
pub const SDMA_AQL_PKT_BARRIER_OR_RESERVED_DW12_reserved_dw12_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_AQL_PKT_BARRIER_OR_RESERVED_DW12_reserved_dw12_shift: c_int = 0;

// define for RESERVED_DW13 word
// define for reserved_dw13 field
pub const SDMA_AQL_PKT_BARRIER_OR_RESERVED_DW13_reserved_dw13_offset: c_int = 13;
pub const SDMA_AQL_PKT_BARRIER_OR_RESERVED_DW13_reserved_dw13_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_AQL_PKT_BARRIER_OR_RESERVED_DW13_reserved_dw13_shift: c_int = 0;

// define for COMPLETION_SIGNAL_LO word
// define for completion_signal_31_0 field
pub const SDMA_AQL_PKT_BARRIER_OR_COMPLETION_SIGNAL_LO_completion_signal_31_0_offset: c_int = 14;
pub const SDMA_AQL_PKT_BARRIER_OR_COMPLETION_SIGNAL_LO_completion_signal_31_0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_AQL_PKT_BARRIER_OR_COMPLETION_SIGNAL_LO_completion_signal_31_0_shift: c_int = 0;

// define for COMPLETION_SIGNAL_HI word
// define for completion_signal_63_32 field
pub const SDMA_AQL_PKT_BARRIER_OR_COMPLETION_SIGNAL_HI_completion_signal_63_32_offset: c_int = 15;
pub const SDMA_AQL_PKT_BARRIER_OR_COMPLETION_SIGNAL_HI_completion_signal_63_32_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_AQL_PKT_BARRIER_OR_COMPLETION_SIGNAL_HI_completion_signal_63_32_shift: c_int = 0;

