//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/cikd.h
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
// Copyright 2012 Advanced Micro Devices, Inc.
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
// Authors: Alex Deucher
//
pub const MC_SEQ_MISC0__MT__MASK: c_uint = 0xf0000000;
pub const MC_SEQ_MISC0__MT__GDDR1: c_uint = 0x10000000;
pub const MC_SEQ_MISC0__MT__DDR2: c_uint = 0x20000000;
pub const MC_SEQ_MISC0__MT__GDDR3: c_uint = 0x30000000;
pub const MC_SEQ_MISC0__MT__GDDR4: c_uint = 0x40000000;
pub const MC_SEQ_MISC0__MT__GDDR5: c_uint = 0x50000000;
pub const MC_SEQ_MISC0__MT__HBM: c_uint = 0x60000000;
pub const MC_SEQ_MISC0__MT__DDR3: c_uint = 0xB0000000;
pub const CP_ME_TABLE_SIZE: c_int = 96;
// display controller offsets used for crtc/cur/lut/grph/viewport/etc.

// hpd instance offsets

// audio endpt instance offsets

pub const mmCC_DRM_ID_STRAPS: c_uint = 0x1559;
pub const CC_DRM_ID_STRAPS__ATI_REV_ID_MASK: c_uint = 0xf0000000;
pub const mmCHUB_CONTROL: c_uint = 0x619;

pub const mmGRPH_LUT_10BIT_BYPASS_CONTROL: c_uint = 0x1a02;

// 8 BPP

// 16 BPP

// 32 BPP

pub const NO_AUTO: c_int = 0;
pub const ES_AUTO: c_int = 1;
pub const GS_AUTO: c_int = 2;
pub const ES_AND_GS_AUTO: c_int = 3;

pub const MSG_ENTER_RLC_SAFE_MODE: c_int = 1;
pub const MSG_EXIT_RLC_SAFE_MODE: c_int = 0;
//
// PM4
//
pub const PACKET_TYPE0: c_int = 0;
pub const PACKET_TYPE1: c_int = 1;
pub const PACKET_TYPE2: c_int = 2;
pub const PACKET_TYPE3: c_int = 3;

pub const CP_PACKET2: c_uint = 0x80000000;
pub const PACKET2_PAD_SHIFT: c_int = 0;

// Packet 3 types
pub const PACKET3_NOP: c_uint = 0x10;
pub const PACKET3_SET_BASE: c_uint = 0x11;

pub const CE_PARTITION_BASE: c_int = 3;
pub const PACKET3_CLEAR_STATE: c_uint = 0x12;
pub const PACKET3_INDEX_BUFFER_SIZE: c_uint = 0x13;
pub const PACKET3_DISPATCH_DIRECT: c_uint = 0x15;
pub const PACKET3_DISPATCH_INDIRECT: c_uint = 0x16;
pub const PACKET3_ATOMIC_GDS: c_uint = 0x1D;
pub const PACKET3_ATOMIC_MEM: c_uint = 0x1E;
pub const PACKET3_OCCLUSION_QUERY: c_uint = 0x1F;
pub const PACKET3_SET_PREDICATION: c_uint = 0x20;
pub const PACKET3_REG_RMW: c_uint = 0x21;
pub const PACKET3_COND_EXEC: c_uint = 0x22;
pub const PACKET3_PRED_EXEC: c_uint = 0x23;
pub const PACKET3_DRAW_INDIRECT: c_uint = 0x24;
pub const PACKET3_DRAW_INDEX_INDIRECT: c_uint = 0x25;
pub const PACKET3_INDEX_BASE: c_uint = 0x26;
pub const PACKET3_DRAW_INDEX_2: c_uint = 0x27;
pub const PACKET3_CONTEXT_CONTROL: c_uint = 0x28;
pub const PACKET3_INDEX_TYPE: c_uint = 0x2A;
pub const PACKET3_DRAW_INDIRECT_MULTI: c_uint = 0x2C;
pub const PACKET3_DRAW_INDEX_AUTO: c_uint = 0x2D;
pub const PACKET3_NUM_INSTANCES: c_uint = 0x2F;
pub const PACKET3_DRAW_INDEX_MULTI_AUTO: c_uint = 0x30;
pub const PACKET3_INDIRECT_BUFFER_CONST: c_uint = 0x33;
pub const PACKET3_STRMOUT_BUFFER_UPDATE: c_uint = 0x34;
pub const PACKET3_DRAW_INDEX_OFFSET_2: c_uint = 0x35;
pub const PACKET3_DRAW_PREAMBLE: c_uint = 0x36;
pub const PACKET3_WRITE_DATA: c_uint = 0x37;

// 0 - register
// 1 - memory (sync - via GRBM)
// 2 - gl2
// 3 - gds
// 4 - reserved
// 5 - memory (async - direct)
//

// 0 - LRU
// 1 - Stream
//

// 0 - me
// 1 - pfp
// 2 - ce
//
pub const PACKET3_DRAW_INDEX_INDIRECT_MULTI: c_uint = 0x38;
pub const PACKET3_MEM_SEMAPHORE: c_uint = 0x39;

pub const PACKET3_COPY_DW: c_uint = 0x3B;
pub const PACKET3_WAIT_REG_MEM: c_uint = 0x3C;

// 0 - always
// 1 - <
// 2 - <=
// 3 - ==
// 4 - !=
// 5 - >=
// 6 - >
//

// 0 - reg
// 1 - mem
//

// 0 - wait_reg_mem
// 1 - wr_wait_wr_reg
//

// 0 - me
// 1 - pfp
//
pub const PACKET3_INDIRECT_BUFFER: c_uint = 0x3F;

// 0 - LRU
// 1 - Stream
// 2 - Bypass
//
pub const PACKET3_COPY_DATA: c_uint = 0x40;
pub const PACKET3_PFP_SYNC_ME: c_uint = 0x42;
pub const PACKET3_SURFACE_SYNC: c_uint = 0x43;

pub const PACKET3_COND_WRITE: c_uint = 0x45;
pub const PACKET3_EVENT_WRITE: c_uint = 0x46;

// 0 - any non-TS event
// 1 - ZPASS_DONE, PIXEL_PIPE_STAT_
// 2 - SAMPLE_PIPELINESTAT
// 3 - SAMPLE_STREAMOUTSTAT
// 4 - *S_PARTIAL_FLUSH
// 5 - EOP events
// 6 - EOS events
//
pub const PACKET3_EVENT_WRITE_EOP: c_uint = 0x47;

// 0 - LRU
// 1 - Stream
// 2 - Bypass
//

// 0 - discard
// 1 - send low 32bit data
// 2 - send 64bit data
// 3 - send 64bit GPU counter value
// 4 - send 64bit sys counter value
//

// 0 - none
// 1 - interrupt only (DATA_SEL = 0)
// 2 - interrupt when data write is confirmed
//

// 0 - MC
// 1 - TC/L2
//
pub const PACKET3_EVENT_WRITE_EOS: c_uint = 0x48;
pub const PACKET3_RELEASE_MEM: c_uint = 0x49;
pub const PACKET3_PREAMBLE_CNTL: c_uint = 0x4A;

pub const PACKET3_DMA_DATA: c_uint = 0x50;
// 1. header
// 2. CONTROL
// 3. SRC_ADDR_LO or DATA [31:0]
// 4. SRC_ADDR_HI [31:0]
// 5. DST_ADDR_LO [31:0]
// 6. DST_ADDR_HI [7:0]
// 7. COMMAND [30:21] | BYTE_COUNT [20:0]
//
// CONTROL

// 0 - ME
// 1 - PFP
//

// 0 - LRU
// 1 - Stream
// 2 - Bypass
//

// 0 - DST_ADDR using DAS
// 1 - GDS
// 3 - DST_ADDR using L2
//

// 0 - LRU
// 1 - Stream
// 2 - Bypass
//

// 0 - SRC_ADDR using SAS
// 1 - GDS
// 2 - DATA
// 3 - SRC_ADDR using L2
//

// COMMAND

// 0 - none
// 1 - 8 in 16
// 2 - 8 in 32
// 3 - 8 in 64
//

// 0 - none
// 1 - 8 in 16
// 2 - 8 in 32
// 3 - 8 in 64
//

// 0 - memory
// 1 - register
//

// 0 - memory
// 1 - register
//

pub const PACKET3_ACQUIRE_MEM: c_uint = 0x58;
pub const PACKET3_REWIND: c_uint = 0x59;
pub const PACKET3_LOAD_UCONFIG_REG: c_uint = 0x5E;
pub const PACKET3_LOAD_SH_REG: c_uint = 0x5F;
pub const PACKET3_LOAD_CONFIG_REG: c_uint = 0x60;
pub const PACKET3_LOAD_CONTEXT_REG: c_uint = 0x61;
pub const PACKET3_SET_CONFIG_REG: c_uint = 0x68;
pub const PACKET3_SET_CONFIG_REG_START: c_uint = 0x00002000;
pub const PACKET3_SET_CONFIG_REG_END: c_uint = 0x00002c00;
pub const PACKET3_SET_CONTEXT_REG: c_uint = 0x69;
pub const PACKET3_SET_CONTEXT_REG_START: c_uint = 0x0000a000;
pub const PACKET3_SET_CONTEXT_REG_END: c_uint = 0x0000a400;
pub const PACKET3_SET_CONTEXT_REG_INDIRECT: c_uint = 0x73;
pub const PACKET3_SET_SH_REG: c_uint = 0x76;
pub const PACKET3_SET_SH_REG_START: c_uint = 0x00002c00;
pub const PACKET3_SET_SH_REG_END: c_uint = 0x00003000;
pub const PACKET3_SET_SH_REG_OFFSET: c_uint = 0x77;
pub const PACKET3_SET_QUEUE_REG: c_uint = 0x78;
pub const PACKET3_SET_UCONFIG_REG: c_uint = 0x79;
pub const PACKET3_SET_UCONFIG_REG_START: c_uint = 0x0000c000;
pub const PACKET3_SET_UCONFIG_REG_END: c_uint = 0x0000c400;
pub const PACKET3_SCRATCH_RAM_WRITE: c_uint = 0x7D;
pub const PACKET3_SCRATCH_RAM_READ: c_uint = 0x7E;
pub const PACKET3_LOAD_CONST_RAM: c_uint = 0x80;
pub const PACKET3_WRITE_CONST_RAM: c_uint = 0x81;
pub const PACKET3_DUMP_CONST_RAM: c_uint = 0x83;
pub const PACKET3_INCREMENT_CE_COUNTER: c_uint = 0x84;
pub const PACKET3_INCREMENT_DE_COUNTER: c_uint = 0x85;
pub const PACKET3_WAIT_ON_CE_COUNTER: c_uint = 0x86;
pub const PACKET3_WAIT_ON_DE_COUNTER_DIFF: c_uint = 0x88;
pub const PACKET3_SWITCH_BUFFER: c_uint = 0x8B;
// SDMA - first instance at 0xd000, second at 0xd800
pub const SDMA0_REGISTER_OFFSET: c_uint = 0x0 /* not a register */;
pub const SDMA1_REGISTER_OFFSET: c_uint = 0x200 /* not a register */;
pub const SDMA_MAX_INSTANCE: c_int = 2;

// sDMA opcodes
pub const SDMA_OPCODE_NOP: c_int = 0;

pub const SDMA_OPCODE_COPY: c_int = 1;

pub const SDMA_OPCODE_WRITE: c_int = 2;

pub const SDMA_OPCODE_INDIRECT_BUFFER: c_int = 4;
pub const SDMA_OPCODE_FENCE: c_int = 5;
pub const SDMA_OPCODE_TRAP: c_int = 6;
pub const SDMA_OPCODE_SEMAPHORE: c_int = 7;

// 0 - increment
// 1 - write 1
//

// 0 - wait
// 1 - signal
//

// mailbox
pub const SDMA_OPCODE_POLL_REG_MEM: c_int = 8;

// 0 - wait_reg_mem
// 1 - wr_wait_wr_reg
//

// 0 - always
// 1 - <
// 2 - <=
// 3 - ==
// 4 - !=
// 5 - >=
// 6 - >
//

// 0 = register
// 1 = memory
//
pub const SDMA_OPCODE_COND_EXEC: c_int = 9;
pub const SDMA_OPCODE_CONSTANT_FILL: c_int = 11;

// 0 = byte fill
// 2 = DW fill
//
pub const SDMA_OPCODE_GENERATE_PTE_PDE: c_int = 12;
pub const SDMA_OPCODE_TIMESTAMP: c_int = 13;

pub const SDMA_OPCODE_SRBM_WRITE: c_int = 14;

// byte mask
pub const VCE_CMD_NO_OP: c_uint = 0x00000000;
pub const VCE_CMD_END: c_uint = 0x00000001;
pub const VCE_CMD_IB: c_uint = 0x00000002;
pub const VCE_CMD_FENCE: c_uint = 0x00000003;
pub const VCE_CMD_TRAP: c_uint = 0x00000004;
pub const VCE_CMD_IB_AUTO: c_uint = 0x00000005;
pub const VCE_CMD_SEMAPHORE: c_uint = 0x00000006;
// if PTR32, these are the bases for scratch and lds

// valid for both DEFAULT_MTYPE and APE1_MTYPE
// mmPA_SC_RASTER_CONFIG mask

// mmPA_SC_RASTER_CONFIG_1 mask

