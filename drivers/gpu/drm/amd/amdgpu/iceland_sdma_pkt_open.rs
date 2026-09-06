//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/iceland_sdma_pkt_open.h
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
// Copyright (C) 2014  Advanced Micro Devices, Inc.
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
pub const SDMA_OP_GEN_PTEPDE: c_int = 12;
pub const SDMA_OP_TIMESTAMP: c_int = 13;
pub const SDMA_OP_SRBM_WRITE: c_int = 14;
pub const SDMA_OP_PRE_EXE: c_int = 15;
pub const SDMA_SUBOP_TIMESTAMP_SET: c_int = 0;
pub const SDMA_SUBOP_TIMESTAMP_GET: c_int = 1;
pub const SDMA_SUBOP_TIMESTAMP_GET_GLOBAL: c_int = 2;
pub const SDMA_SUBOP_COPY_LINEAR: c_int = 0;
pub const SDMA_SUBOP_COPY_LINEAR_SUB_WIND: c_int = 4;
pub const SDMA_SUBOP_COPY_TILED: c_int = 1;
pub const SDMA_SUBOP_COPY_TILED_SUB_WIND: c_int = 5;
pub const SDMA_SUBOP_COPY_T2T_SUB_WIND: c_int = 6;
pub const SDMA_SUBOP_COPY_SOA: c_int = 3;
pub const SDMA_SUBOP_WRITE_LINEAR: c_int = 0;
pub const SDMA_SUBOP_WRITE_TILED: c_int = 1;
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

// define for dst_ha field
pub const SDMA_PKT_COPY_LINEAR_PARAMETER_dst_ha_offset: c_int = 2;
pub const SDMA_PKT_COPY_LINEAR_PARAMETER_dst_ha_mask: c_uint = 0x00000001;
pub const SDMA_PKT_COPY_LINEAR_PARAMETER_dst_ha_shift: c_int = 22;

// define for src_sw field
pub const SDMA_PKT_COPY_LINEAR_PARAMETER_src_sw_offset: c_int = 2;
pub const SDMA_PKT_COPY_LINEAR_PARAMETER_src_sw_mask: c_uint = 0x00000003;
pub const SDMA_PKT_COPY_LINEAR_PARAMETER_src_sw_shift: c_int = 24;

// define for src_ha field
pub const SDMA_PKT_COPY_LINEAR_PARAMETER_src_ha_offset: c_int = 2;
pub const SDMA_PKT_COPY_LINEAR_PARAMETER_src_ha_mask: c_uint = 0x00000001;
pub const SDMA_PKT_COPY_LINEAR_PARAMETER_src_ha_shift: c_int = 30;

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

// define for dst2_ha field
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_PARAMETER_dst2_ha_offset: c_int = 2;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_PARAMETER_dst2_ha_mask: c_uint = 0x00000001;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_PARAMETER_dst2_ha_shift: c_int = 14;

// define for dst1_sw field
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_PARAMETER_dst1_sw_offset: c_int = 2;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_PARAMETER_dst1_sw_mask: c_uint = 0x00000003;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_PARAMETER_dst1_sw_shift: c_int = 16;

// define for dst1_ha field
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_PARAMETER_dst1_ha_offset: c_int = 2;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_PARAMETER_dst1_ha_mask: c_uint = 0x00000001;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_PARAMETER_dst1_ha_shift: c_int = 22;

// define for src_sw field
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_PARAMETER_src_sw_offset: c_int = 2;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_PARAMETER_src_sw_mask: c_uint = 0x00000003;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_PARAMETER_src_sw_shift: c_int = 24;

// define for src_ha field
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_PARAMETER_src_ha_offset: c_int = 2;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_PARAMETER_src_ha_mask: c_uint = 0x00000001;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_PARAMETER_src_ha_shift: c_int = 30;

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
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_4_src_pitch_mask: c_uint = 0x00003FFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_4_src_pitch_shift: c_int = 16;

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
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_9_dst_pitch_mask: c_uint = 0x00003FFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_9_dst_pitch_shift: c_int = 16;

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

// define for dst_ha field
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_12_dst_ha_offset: c_int = 12;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_12_dst_ha_mask: c_uint = 0x00000001;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_12_dst_ha_shift: c_int = 22;

// define for src_sw field
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_12_src_sw_offset: c_int = 12;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_12_src_sw_mask: c_uint = 0x00000003;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_12_src_sw_shift: c_int = 24;

// define for src_ha field
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_12_src_ha_offset: c_int = 12;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_12_src_ha_mask: c_uint = 0x00000001;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_12_src_ha_shift: c_int = 30;

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
// define for pitch_in_tile field
pub const SDMA_PKT_COPY_TILED_DW_3_pitch_in_tile_offset: c_int = 3;
pub const SDMA_PKT_COPY_TILED_DW_3_pitch_in_tile_mask: c_uint = 0x000007FF;
pub const SDMA_PKT_COPY_TILED_DW_3_pitch_in_tile_shift: c_int = 0;

// define for height field
pub const SDMA_PKT_COPY_TILED_DW_3_height_offset: c_int = 3;
pub const SDMA_PKT_COPY_TILED_DW_3_height_mask: c_uint = 0x00003FFF;
pub const SDMA_PKT_COPY_TILED_DW_3_height_shift: c_int = 16;

// define for DW_4 word
// define for slice_pitch field
pub const SDMA_PKT_COPY_TILED_DW_4_slice_pitch_offset: c_int = 4;
pub const SDMA_PKT_COPY_TILED_DW_4_slice_pitch_mask: c_uint = 0x003FFFFF;
pub const SDMA_PKT_COPY_TILED_DW_4_slice_pitch_shift: c_int = 0;

// define for DW_5 word
// define for element_size field
pub const SDMA_PKT_COPY_TILED_DW_5_element_size_offset: c_int = 5;
pub const SDMA_PKT_COPY_TILED_DW_5_element_size_mask: c_uint = 0x00000007;
pub const SDMA_PKT_COPY_TILED_DW_5_element_size_shift: c_int = 0;

// define for array_mode field
pub const SDMA_PKT_COPY_TILED_DW_5_array_mode_offset: c_int = 5;
pub const SDMA_PKT_COPY_TILED_DW_5_array_mode_mask: c_uint = 0x0000000F;
pub const SDMA_PKT_COPY_TILED_DW_5_array_mode_shift: c_int = 3;

// define for mit_mode field
pub const SDMA_PKT_COPY_TILED_DW_5_mit_mode_offset: c_int = 5;
pub const SDMA_PKT_COPY_TILED_DW_5_mit_mode_mask: c_uint = 0x00000007;
pub const SDMA_PKT_COPY_TILED_DW_5_mit_mode_shift: c_int = 8;

// define for tilesplit_size field
pub const SDMA_PKT_COPY_TILED_DW_5_tilesplit_size_offset: c_int = 5;
pub const SDMA_PKT_COPY_TILED_DW_5_tilesplit_size_mask: c_uint = 0x00000007;
pub const SDMA_PKT_COPY_TILED_DW_5_tilesplit_size_shift: c_int = 11;

// define for bank_w field
pub const SDMA_PKT_COPY_TILED_DW_5_bank_w_offset: c_int = 5;
pub const SDMA_PKT_COPY_TILED_DW_5_bank_w_mask: c_uint = 0x00000003;
pub const SDMA_PKT_COPY_TILED_DW_5_bank_w_shift: c_int = 15;

// define for bank_h field
pub const SDMA_PKT_COPY_TILED_DW_5_bank_h_offset: c_int = 5;
pub const SDMA_PKT_COPY_TILED_DW_5_bank_h_mask: c_uint = 0x00000003;
pub const SDMA_PKT_COPY_TILED_DW_5_bank_h_shift: c_int = 18;

// define for num_bank field
pub const SDMA_PKT_COPY_TILED_DW_5_num_bank_offset: c_int = 5;
pub const SDMA_PKT_COPY_TILED_DW_5_num_bank_mask: c_uint = 0x00000003;
pub const SDMA_PKT_COPY_TILED_DW_5_num_bank_shift: c_int = 21;

// define for mat_aspt field
pub const SDMA_PKT_COPY_TILED_DW_5_mat_aspt_offset: c_int = 5;
pub const SDMA_PKT_COPY_TILED_DW_5_mat_aspt_mask: c_uint = 0x00000003;
pub const SDMA_PKT_COPY_TILED_DW_5_mat_aspt_shift: c_int = 24;

// define for pipe_config field
pub const SDMA_PKT_COPY_TILED_DW_5_pipe_config_offset: c_int = 5;
pub const SDMA_PKT_COPY_TILED_DW_5_pipe_config_mask: c_uint = 0x0000001F;
pub const SDMA_PKT_COPY_TILED_DW_5_pipe_config_shift: c_int = 26;

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
pub const SDMA_PKT_COPY_TILED_DW_7_z_mask: c_uint = 0x00000FFF;
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

// define for COUNT word
// define for count field
pub const SDMA_PKT_COPY_TILED_COUNT_count_offset: c_int = 11;
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
// define for pitch_in_tile field
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_5_pitch_in_tile_offset: c_int = 5;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_5_pitch_in_tile_mask: c_uint = 0x000007FF;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_5_pitch_in_tile_shift: c_int = 0;

// define for height field
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_5_height_offset: c_int = 5;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_5_height_mask: c_uint = 0x00003FFF;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_5_height_shift: c_int = 16;

// define for DW_6 word
// define for slice_pitch field
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_6_slice_pitch_offset: c_int = 6;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_6_slice_pitch_mask: c_uint = 0x003FFFFF;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_6_slice_pitch_shift: c_int = 0;

// define for DW_7 word
// define for element_size field
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_7_element_size_offset: c_int = 7;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_7_element_size_mask: c_uint = 0x00000007;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_7_element_size_shift: c_int = 0;

// define for array_mode field
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_7_array_mode_offset: c_int = 7;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_7_array_mode_mask: c_uint = 0x0000000F;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_7_array_mode_shift: c_int = 3;

// define for mit_mode field
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_7_mit_mode_offset: c_int = 7;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_7_mit_mode_mask: c_uint = 0x00000007;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_7_mit_mode_shift: c_int = 8;

// define for tilesplit_size field
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_7_tilesplit_size_offset: c_int = 7;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_7_tilesplit_size_mask: c_uint = 0x00000007;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_7_tilesplit_size_shift: c_int = 11;

// define for bank_w field
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_7_bank_w_offset: c_int = 7;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_7_bank_w_mask: c_uint = 0x00000003;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_7_bank_w_shift: c_int = 15;

// define for bank_h field
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_7_bank_h_offset: c_int = 7;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_7_bank_h_mask: c_uint = 0x00000003;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_7_bank_h_shift: c_int = 18;

// define for num_bank field
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_7_num_bank_offset: c_int = 7;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_7_num_bank_mask: c_uint = 0x00000003;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_7_num_bank_shift: c_int = 21;

// define for mat_aspt field
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_7_mat_aspt_offset: c_int = 7;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_7_mat_aspt_mask: c_uint = 0x00000003;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_7_mat_aspt_shift: c_int = 24;

// define for pipe_config field
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_7_pipe_config_offset: c_int = 7;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_7_pipe_config_mask: c_uint = 0x0000001F;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_7_pipe_config_shift: c_int = 26;

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
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_9_z_mask: c_uint = 0x00000FFF;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_9_z_shift: c_int = 0;

// define for DW_10 word
// define for dst2_sw field
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_10_dst2_sw_offset: c_int = 10;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_10_dst2_sw_mask: c_uint = 0x00000003;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_10_dst2_sw_shift: c_int = 8;

// define for dst2_ha field
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_10_dst2_ha_offset: c_int = 10;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_10_dst2_ha_mask: c_uint = 0x00000001;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_10_dst2_ha_shift: c_int = 14;

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

// define for COUNT word
// define for count field
pub const SDMA_PKT_COPY_L2T_BROADCAST_COUNT_count_offset: c_int = 14;
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

// define for src_pitch_in_tile field
pub const SDMA_PKT_COPY_T2T_DW_4_src_pitch_in_tile_offset: c_int = 4;
pub const SDMA_PKT_COPY_T2T_DW_4_src_pitch_in_tile_mask: c_uint = 0x00000FFF;
pub const SDMA_PKT_COPY_T2T_DW_4_src_pitch_in_tile_shift: c_int = 16;

// define for DW_5 word
// define for src_slice_pitch field
pub const SDMA_PKT_COPY_T2T_DW_5_src_slice_pitch_offset: c_int = 5;
pub const SDMA_PKT_COPY_T2T_DW_5_src_slice_pitch_mask: c_uint = 0x003FFFFF;
pub const SDMA_PKT_COPY_T2T_DW_5_src_slice_pitch_shift: c_int = 0;

// define for DW_6 word
// define for src_element_size field
pub const SDMA_PKT_COPY_T2T_DW_6_src_element_size_offset: c_int = 6;
pub const SDMA_PKT_COPY_T2T_DW_6_src_element_size_mask: c_uint = 0x00000007;
pub const SDMA_PKT_COPY_T2T_DW_6_src_element_size_shift: c_int = 0;

// define for src_array_mode field
pub const SDMA_PKT_COPY_T2T_DW_6_src_array_mode_offset: c_int = 6;
pub const SDMA_PKT_COPY_T2T_DW_6_src_array_mode_mask: c_uint = 0x0000000F;
pub const SDMA_PKT_COPY_T2T_DW_6_src_array_mode_shift: c_int = 3;

// define for src_mit_mode field
pub const SDMA_PKT_COPY_T2T_DW_6_src_mit_mode_offset: c_int = 6;
pub const SDMA_PKT_COPY_T2T_DW_6_src_mit_mode_mask: c_uint = 0x00000007;
pub const SDMA_PKT_COPY_T2T_DW_6_src_mit_mode_shift: c_int = 8;

// define for src_tilesplit_size field
pub const SDMA_PKT_COPY_T2T_DW_6_src_tilesplit_size_offset: c_int = 6;
pub const SDMA_PKT_COPY_T2T_DW_6_src_tilesplit_size_mask: c_uint = 0x00000007;
pub const SDMA_PKT_COPY_T2T_DW_6_src_tilesplit_size_shift: c_int = 11;

// define for src_bank_w field
pub const SDMA_PKT_COPY_T2T_DW_6_src_bank_w_offset: c_int = 6;
pub const SDMA_PKT_COPY_T2T_DW_6_src_bank_w_mask: c_uint = 0x00000003;
pub const SDMA_PKT_COPY_T2T_DW_6_src_bank_w_shift: c_int = 15;

// define for src_bank_h field
pub const SDMA_PKT_COPY_T2T_DW_6_src_bank_h_offset: c_int = 6;
pub const SDMA_PKT_COPY_T2T_DW_6_src_bank_h_mask: c_uint = 0x00000003;
pub const SDMA_PKT_COPY_T2T_DW_6_src_bank_h_shift: c_int = 18;

// define for src_num_bank field
pub const SDMA_PKT_COPY_T2T_DW_6_src_num_bank_offset: c_int = 6;
pub const SDMA_PKT_COPY_T2T_DW_6_src_num_bank_mask: c_uint = 0x00000003;
pub const SDMA_PKT_COPY_T2T_DW_6_src_num_bank_shift: c_int = 21;

// define for src_mat_aspt field
pub const SDMA_PKT_COPY_T2T_DW_6_src_mat_aspt_offset: c_int = 6;
pub const SDMA_PKT_COPY_T2T_DW_6_src_mat_aspt_mask: c_uint = 0x00000003;
pub const SDMA_PKT_COPY_T2T_DW_6_src_mat_aspt_shift: c_int = 24;

// define for src_pipe_config field
pub const SDMA_PKT_COPY_T2T_DW_6_src_pipe_config_offset: c_int = 6;
pub const SDMA_PKT_COPY_T2T_DW_6_src_pipe_config_mask: c_uint = 0x0000001F;
pub const SDMA_PKT_COPY_T2T_DW_6_src_pipe_config_shift: c_int = 26;

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

// define for dst_pitch_in_tile field
pub const SDMA_PKT_COPY_T2T_DW_10_dst_pitch_in_tile_offset: c_int = 10;
pub const SDMA_PKT_COPY_T2T_DW_10_dst_pitch_in_tile_mask: c_uint = 0x00000FFF;
pub const SDMA_PKT_COPY_T2T_DW_10_dst_pitch_in_tile_shift: c_int = 16;

// define for DW_11 word
// define for dst_slice_pitch field
pub const SDMA_PKT_COPY_T2T_DW_11_dst_slice_pitch_offset: c_int = 11;
pub const SDMA_PKT_COPY_T2T_DW_11_dst_slice_pitch_mask: c_uint = 0x003FFFFF;
pub const SDMA_PKT_COPY_T2T_DW_11_dst_slice_pitch_shift: c_int = 0;

// define for DW_12 word
// define for dst_array_mode field
pub const SDMA_PKT_COPY_T2T_DW_12_dst_array_mode_offset: c_int = 12;
pub const SDMA_PKT_COPY_T2T_DW_12_dst_array_mode_mask: c_uint = 0x0000000F;
pub const SDMA_PKT_COPY_T2T_DW_12_dst_array_mode_shift: c_int = 3;

// define for dst_mit_mode field
pub const SDMA_PKT_COPY_T2T_DW_12_dst_mit_mode_offset: c_int = 12;
pub const SDMA_PKT_COPY_T2T_DW_12_dst_mit_mode_mask: c_uint = 0x00000007;
pub const SDMA_PKT_COPY_T2T_DW_12_dst_mit_mode_shift: c_int = 8;

// define for dst_tilesplit_size field
pub const SDMA_PKT_COPY_T2T_DW_12_dst_tilesplit_size_offset: c_int = 12;
pub const SDMA_PKT_COPY_T2T_DW_12_dst_tilesplit_size_mask: c_uint = 0x00000007;
pub const SDMA_PKT_COPY_T2T_DW_12_dst_tilesplit_size_shift: c_int = 11;

// define for dst_bank_w field
pub const SDMA_PKT_COPY_T2T_DW_12_dst_bank_w_offset: c_int = 12;
pub const SDMA_PKT_COPY_T2T_DW_12_dst_bank_w_mask: c_uint = 0x00000003;
pub const SDMA_PKT_COPY_T2T_DW_12_dst_bank_w_shift: c_int = 15;

// define for dst_bank_h field
pub const SDMA_PKT_COPY_T2T_DW_12_dst_bank_h_offset: c_int = 12;
pub const SDMA_PKT_COPY_T2T_DW_12_dst_bank_h_mask: c_uint = 0x00000003;
pub const SDMA_PKT_COPY_T2T_DW_12_dst_bank_h_shift: c_int = 18;

// define for dst_num_bank field
pub const SDMA_PKT_COPY_T2T_DW_12_dst_num_bank_offset: c_int = 12;
pub const SDMA_PKT_COPY_T2T_DW_12_dst_num_bank_mask: c_uint = 0x00000003;
pub const SDMA_PKT_COPY_T2T_DW_12_dst_num_bank_shift: c_int = 21;

// define for dst_mat_aspt field
pub const SDMA_PKT_COPY_T2T_DW_12_dst_mat_aspt_offset: c_int = 12;
pub const SDMA_PKT_COPY_T2T_DW_12_dst_mat_aspt_mask: c_uint = 0x00000003;
pub const SDMA_PKT_COPY_T2T_DW_12_dst_mat_aspt_shift: c_int = 24;

// define for dst_pipe_config field
pub const SDMA_PKT_COPY_T2T_DW_12_dst_pipe_config_offset: c_int = 12;
pub const SDMA_PKT_COPY_T2T_DW_12_dst_pipe_config_mask: c_uint = 0x0000001F;
pub const SDMA_PKT_COPY_T2T_DW_12_dst_pipe_config_shift: c_int = 26;

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

// define for pitch_in_tile field
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_4_pitch_in_tile_offset: c_int = 4;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_4_pitch_in_tile_mask: c_uint = 0x00000FFF;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_4_pitch_in_tile_shift: c_int = 16;

// define for DW_5 word
// define for slice_pitch field
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_5_slice_pitch_offset: c_int = 5;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_5_slice_pitch_mask: c_uint = 0x003FFFFF;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_5_slice_pitch_shift: c_int = 0;

// define for DW_6 word
// define for element_size field
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_6_element_size_offset: c_int = 6;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_6_element_size_mask: c_uint = 0x00000007;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_6_element_size_shift: c_int = 0;

// define for array_mode field
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_6_array_mode_offset: c_int = 6;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_6_array_mode_mask: c_uint = 0x0000000F;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_6_array_mode_shift: c_int = 3;

// define for mit_mode field
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_6_mit_mode_offset: c_int = 6;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_6_mit_mode_mask: c_uint = 0x00000007;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_6_mit_mode_shift: c_int = 8;

// define for tilesplit_size field
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_6_tilesplit_size_offset: c_int = 6;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_6_tilesplit_size_mask: c_uint = 0x00000007;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_6_tilesplit_size_shift: c_int = 11;

// define for bank_w field
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_6_bank_w_offset: c_int = 6;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_6_bank_w_mask: c_uint = 0x00000003;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_6_bank_w_shift: c_int = 15;

// define for bank_h field
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_6_bank_h_offset: c_int = 6;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_6_bank_h_mask: c_uint = 0x00000003;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_6_bank_h_shift: c_int = 18;

// define for num_bank field
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_6_num_bank_offset: c_int = 6;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_6_num_bank_mask: c_uint = 0x00000003;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_6_num_bank_shift: c_int = 21;

// define for mat_aspt field
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_6_mat_aspt_offset: c_int = 6;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_6_mat_aspt_mask: c_uint = 0x00000003;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_6_mat_aspt_shift: c_int = 24;

// define for pipe_config field
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_6_pipe_config_offset: c_int = 6;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_6_pipe_config_mask: c_uint = 0x0000001F;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_6_pipe_config_shift: c_int = 26;

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

// define for struct_sw field
pub const SDMA_PKT_COPY_STRUCT_DW_5_struct_sw_offset: c_int = 5;
pub const SDMA_PKT_COPY_STRUCT_DW_5_struct_sw_mask: c_uint = 0x00000003;
pub const SDMA_PKT_COPY_STRUCT_DW_5_struct_sw_shift: c_int = 16;

// define for struct_ha field
pub const SDMA_PKT_COPY_STRUCT_DW_5_struct_ha_offset: c_int = 5;
pub const SDMA_PKT_COPY_STRUCT_DW_5_struct_ha_mask: c_uint = 0x00000001;
pub const SDMA_PKT_COPY_STRUCT_DW_5_struct_ha_shift: c_int = 22;

// define for linear_sw field
pub const SDMA_PKT_COPY_STRUCT_DW_5_linear_sw_offset: c_int = 5;
pub const SDMA_PKT_COPY_STRUCT_DW_5_linear_sw_mask: c_uint = 0x00000003;
pub const SDMA_PKT_COPY_STRUCT_DW_5_linear_sw_shift: c_int = 24;

// define for linear_ha field
pub const SDMA_PKT_COPY_STRUCT_DW_5_linear_ha_offset: c_int = 5;
pub const SDMA_PKT_COPY_STRUCT_DW_5_linear_ha_mask: c_uint = 0x00000001;
pub const SDMA_PKT_COPY_STRUCT_DW_5_linear_ha_shift: c_int = 30;

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
pub const SDMA_PKT_WRITE_UNTILED_DW_3_count_mask: c_uint = 0x003FFFFF;
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
// define for pitch_in_tile field
pub const SDMA_PKT_WRITE_TILED_DW_3_pitch_in_tile_offset: c_int = 3;
pub const SDMA_PKT_WRITE_TILED_DW_3_pitch_in_tile_mask: c_uint = 0x000007FF;
pub const SDMA_PKT_WRITE_TILED_DW_3_pitch_in_tile_shift: c_int = 0;

// define for height field
pub const SDMA_PKT_WRITE_TILED_DW_3_height_offset: c_int = 3;
pub const SDMA_PKT_WRITE_TILED_DW_3_height_mask: c_uint = 0x00003FFF;
pub const SDMA_PKT_WRITE_TILED_DW_3_height_shift: c_int = 16;

// define for DW_4 word
// define for slice_pitch field
pub const SDMA_PKT_WRITE_TILED_DW_4_slice_pitch_offset: c_int = 4;
pub const SDMA_PKT_WRITE_TILED_DW_4_slice_pitch_mask: c_uint = 0x003FFFFF;
pub const SDMA_PKT_WRITE_TILED_DW_4_slice_pitch_shift: c_int = 0;

// define for DW_5 word
// define for element_size field
pub const SDMA_PKT_WRITE_TILED_DW_5_element_size_offset: c_int = 5;
pub const SDMA_PKT_WRITE_TILED_DW_5_element_size_mask: c_uint = 0x00000007;
pub const SDMA_PKT_WRITE_TILED_DW_5_element_size_shift: c_int = 0;

// define for array_mode field
pub const SDMA_PKT_WRITE_TILED_DW_5_array_mode_offset: c_int = 5;
pub const SDMA_PKT_WRITE_TILED_DW_5_array_mode_mask: c_uint = 0x0000000F;
pub const SDMA_PKT_WRITE_TILED_DW_5_array_mode_shift: c_int = 3;

// define for mit_mode field
pub const SDMA_PKT_WRITE_TILED_DW_5_mit_mode_offset: c_int = 5;
pub const SDMA_PKT_WRITE_TILED_DW_5_mit_mode_mask: c_uint = 0x00000007;
pub const SDMA_PKT_WRITE_TILED_DW_5_mit_mode_shift: c_int = 8;

// define for tilesplit_size field
pub const SDMA_PKT_WRITE_TILED_DW_5_tilesplit_size_offset: c_int = 5;
pub const SDMA_PKT_WRITE_TILED_DW_5_tilesplit_size_mask: c_uint = 0x00000007;
pub const SDMA_PKT_WRITE_TILED_DW_5_tilesplit_size_shift: c_int = 11;

// define for bank_w field
pub const SDMA_PKT_WRITE_TILED_DW_5_bank_w_offset: c_int = 5;
pub const SDMA_PKT_WRITE_TILED_DW_5_bank_w_mask: c_uint = 0x00000003;
pub const SDMA_PKT_WRITE_TILED_DW_5_bank_w_shift: c_int = 15;

// define for bank_h field
pub const SDMA_PKT_WRITE_TILED_DW_5_bank_h_offset: c_int = 5;
pub const SDMA_PKT_WRITE_TILED_DW_5_bank_h_mask: c_uint = 0x00000003;
pub const SDMA_PKT_WRITE_TILED_DW_5_bank_h_shift: c_int = 18;

// define for num_bank field
pub const SDMA_PKT_WRITE_TILED_DW_5_num_bank_offset: c_int = 5;
pub const SDMA_PKT_WRITE_TILED_DW_5_num_bank_mask: c_uint = 0x00000003;
pub const SDMA_PKT_WRITE_TILED_DW_5_num_bank_shift: c_int = 21;

// define for mat_aspt field
pub const SDMA_PKT_WRITE_TILED_DW_5_mat_aspt_offset: c_int = 5;
pub const SDMA_PKT_WRITE_TILED_DW_5_mat_aspt_mask: c_uint = 0x00000003;
pub const SDMA_PKT_WRITE_TILED_DW_5_mat_aspt_shift: c_int = 24;

// define for pipe_config field
pub const SDMA_PKT_WRITE_TILED_DW_5_pipe_config_offset: c_int = 5;
pub const SDMA_PKT_WRITE_TILED_DW_5_pipe_config_mask: c_uint = 0x0000001F;
pub const SDMA_PKT_WRITE_TILED_DW_5_pipe_config_shift: c_int = 26;

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
pub const SDMA_PKT_WRITE_TILED_DW_7_z_mask: c_uint = 0x00000FFF;
pub const SDMA_PKT_WRITE_TILED_DW_7_z_shift: c_int = 0;

// define for sw field
pub const SDMA_PKT_WRITE_TILED_DW_7_sw_offset: c_int = 7;
pub const SDMA_PKT_WRITE_TILED_DW_7_sw_mask: c_uint = 0x00000003;
pub const SDMA_PKT_WRITE_TILED_DW_7_sw_shift: c_int = 24;

// define for COUNT word
// define for count field
pub const SDMA_PKT_WRITE_TILED_COUNT_count_offset: c_int = 8;
pub const SDMA_PKT_WRITE_TILED_COUNT_count_mask: c_uint = 0x003FFFFF;
pub const SDMA_PKT_WRITE_TILED_COUNT_count_shift: c_int = 0;

// define for DATA0 word
// define for data0 field
pub const SDMA_PKT_WRITE_TILED_DATA0_data0_offset: c_int = 9;
pub const SDMA_PKT_WRITE_TILED_DATA0_data0_mask: c_uint = 0xFFFFFFFF;
pub const SDMA_PKT_WRITE_TILED_DATA0_data0_shift: c_int = 0;

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
pub const SDMA_PKT_SRBM_WRITE_ADDR_addr_mask: c_uint = 0x0000FFFF;
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

