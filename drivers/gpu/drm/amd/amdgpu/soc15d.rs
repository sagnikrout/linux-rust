//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/soc15d.h
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
pub const GFX9_NUM_GFX_RINGS: c_int = 1;
pub const GFX9_NUM_COMPUTE_RINGS: c_int = 8;
//
// PM4
//
pub const PACKET_TYPE0: c_int = 0;
pub const PACKET_TYPE1: c_int = 1;
pub const PACKET_TYPE2: c_int = 2;
pub const PACKET_TYPE3: c_int = 3;

pub const CP_PACKET2: c_uint = 0x80000000;
pub const PACKET2_PAD_SHIFT: c_int = 0;

pub const PACKETJ_CONDITION_CHECK0: c_int = 0;
pub const PACKETJ_CONDITION_CHECK1: c_int = 1;
pub const PACKETJ_CONDITION_CHECK2: c_int = 2;
pub const PACKETJ_CONDITION_CHECK3: c_int = 3;
pub const PACKETJ_CONDITION_CHECK4: c_int = 4;
pub const PACKETJ_CONDITION_CHECK5: c_int = 5;
pub const PACKETJ_CONDITION_CHECK6: c_int = 6;
pub const PACKETJ_CONDITION_CHECK7: c_int = 7;
pub const PACKETJ_TYPE0: c_int = 0;
pub const PACKETJ_TYPE1: c_int = 1;
pub const PACKETJ_TYPE2: c_int = 2;
pub const PACKETJ_TYPE3: c_int = 3;
pub const PACKETJ_TYPE4: c_int = 4;
pub const PACKETJ_TYPE5: c_int = 5;
pub const PACKETJ_TYPE6: c_int = 6;
pub const PACKETJ_TYPE7: c_int = 7;

pub const CP_PACKETJ_NOP: c_uint = 0x60000000;

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

pub const PACKET3_ATOMIC_MEM__COMMAND__SINGLE_PASS_ATOMIC: c_int = 0;
pub const PACKET3_ATOMIC_MEM__COMMAND__LOOP_UNTIL_COMPARE_SATISFIED: c_int = 1;
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

pub const PACKET3_WRITE_DATA__DST_SEL__MEM_MAPPED_REGISTER: c_int = 0;
pub const PACKET3_WRITE_DATA__DST_SEL__TC_L2: c_int = 2;
pub const PACKET3_WRITE_DATA__DST_SEL__GDS: c_int = 3;
pub const PACKET3_WRITE_DATA__DST_SEL__MEMORY: c_int = 5;
pub const PACKET3_WRITE_DATA__DST_SEL__MEMORY_MAPPED_ADC_PERSISTENT_STATE: c_int = 6;
pub const PACKET3_WRITE_DATA__ADDR_INCR__INCREMENT_ADDRESS: c_int = 0;
pub const PACKET3_WRITE_DATA__ADDR_INCR__DO_NOT_INCREMENT_ADDRESS: c_int = 1;
pub const PACKET3_WRITE_DATA__WR_CONFIRM__DO_NOT_WAIT_FOR_WRITE_CONFIRMATION: c_int = 0;
pub const PACKET3_WRITE_DATA__WR_CONFIRM__WAIT_FOR_WRITE_CONFIRMATION: c_int = 1;
pub const PACKET3_WRITE_DATA__CACHE_POLICY__LRU: c_int = 0;
pub const PACKET3_WRITE_DATA__CACHE_POLICY__STREAM: c_int = 1;
pub const PACKET3_WRITE_DATA__CACHE_POLICY__NOA: c_int = 2;
pub const PACKET3_WRITE_DATA__CACHE_POLICY__BYPASS: c_int = 3;
pub const PACKET3_DRAW_INDEX_INDIRECT_MULTI: c_uint = 0x38;
pub const PACKET3_MEM_SEMAPHORE: c_uint = 0x39;

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

pub const PACKET3_WAIT_REG_MEM__FUNCTION__ALWAYS_PASS: c_int = 0;
pub const PACKET3_WAIT_REG_MEM__FUNCTION__LESS_THAN_REF_VALUE: c_int = 1;
pub const PACKET3_WAIT_REG_MEM__FUNCTION__LESS_THAN_EQUAL_TO_THE_REF_VALUE: c_int = 2;
pub const PACKET3_WAIT_REG_MEM__FUNCTION__EQUAL_TO_THE_REFERENCE_VALUE: c_int = 3;
pub const PACKET3_WAIT_REG_MEM__FUNCTION__NOT_EQUAL_REFERENCE_VALUE: c_int = 4;
pub const PACKET3_WAIT_REG_MEM__FUNCTION__GREATER_THAN_OR_EQUAL_REFERENCE_VALUE: c_int = 5;
pub const PACKET3_WAIT_REG_MEM__FUNCTION__GREATER_THAN_REFERENCE_VALUE: c_int = 6;
pub const PACKET3_WAIT_REG_MEM__MEM_SPACE__REGISTER_SPACE: c_int = 0;
pub const PACKET3_WAIT_REG_MEM__MEM_SPACE__MEMORY_SPACE: c_int = 1;
pub const PACKET3_WAIT_REG_MEM__OPERATION__WAIT_REG_MEM: c_int = 0;
pub const PACKET3_WAIT_REG_MEM__OPERATION__WR_WAIT_WR_REG: c_int = 1;
pub const PACKET3_WAIT_REG_MEM__OPERATION__WAIT_MEM_PREEMPTABLE: c_int = 3;
pub const PACKET3_INDIRECT_BUFFER: c_uint = 0x3F;

// 0 - LRU
// 1 - Stream
// 2 - Bypass
//

pub const PACKET3_INDIRECT_BUFFER__CACHE_POLICY__LRU: c_int = 0;
pub const PACKET3_INDIRECT_BUFFER__CACHE_POLICY__STREAM: c_int = 1;
pub const PACKET3_COPY_DATA: c_uint = 0x40;

pub const PACKET3_COPY_DATA__SRC_SEL__MEM_MAPPED_REGISTER: c_int = 0;
pub const PACKET3_COPY_DATA__SRC_SEL__MEMORY: c_int = 1;
pub const PACKET3_COPY_DATA__SRC_SEL__TC_L2: c_int = 2;
pub const PACKET3_COPY_DATA__SRC_SEL__GDS: c_int = 3;
pub const PACKET3_COPY_DATA__SRC_SEL__PERFCOUNTERS: c_int = 4;
pub const PACKET3_COPY_DATA__SRC_SEL__IMMEDIATE_DATA: c_int = 5;
pub const PACKET3_COPY_DATA__SRC_SEL__ATOMIC_RETURN_DATA: c_int = 6;
pub const PACKET3_COPY_DATA__SRC_SEL__GDS_ATOMIC_RETURN_DATA0: c_int = 7;
pub const PACKET3_COPY_DATA__SRC_SEL__GDS_ATOMIC_RETURN_DATA1: c_int = 8;
pub const PACKET3_COPY_DATA__SRC_SEL__GPU_CLOCK_COUNT: c_int = 9;
pub const PACKET3_COPY_DATA__DST_SEL__MEM_MAPPED_REGISTER: c_int = 0;
pub const PACKET3_COPY_DATA__DST_SEL__TC_L2: c_int = 2;
pub const PACKET3_COPY_DATA__DST_SEL__GDS: c_int = 3;
pub const PACKET3_COPY_DATA__DST_SEL__PERFCOUNTERS: c_int = 4;
pub const PACKET3_COPY_DATA__DST_SEL__MEMORY: c_int = 5;
pub const PACKET3_COPY_DATA__DST_SEL__MEM_MAPPED_REG_DC: c_int = 6;
pub const PACKET3_COPY_DATA__SRC_CACHE_POLICY__LRU: c_int = 0;
pub const PACKET3_COPY_DATA__SRC_CACHE_POLICY__STREAM: c_int = 1;
pub const PACKET3_COPY_DATA__COUNT_SEL__32_BITS_OF_DATA: c_int = 0;
pub const PACKET3_COPY_DATA__COUNT_SEL__64_BITS_OF_DATA: c_int = 1;
pub const PACKET3_COPY_DATA__WR_CONFIRM__DO_NOT_WAIT_FOR_CONFIRMATION: c_int = 0;
pub const PACKET3_COPY_DATA__WR_CONFIRM__WAIT_FOR_CONFIRMATION: c_int = 1;
pub const PACKET3_COPY_DATA__DST_CACHE_POLICY__LRU: c_int = 0;
pub const PACKET3_COPY_DATA__DST_CACHE_POLICY__STREAM: c_int = 1;
pub const PACKET3_COPY_DATA__PQ_EXE_STATUS__DEFAULT: c_int = 0;
pub const PACKET3_COPY_DATA__PQ_EXE_STATUS__PHASE_UPDATE: c_int = 1;
pub const PACKET3_PFP_SYNC_ME: c_uint = 0x42;
pub const PACKET3_COND_WRITE: c_uint = 0x45;
pub const PACKET3_EVENT_WRITE: c_uint = 0x46;

// 0 - any non-TS event
// 1 - ZPASS_DONE, PIXEL_PIPE_STAT_
// 2 - SAMPLE_PIPELINESTAT
// 3 - SAMPLE_STREAMOUTSTAT
// 4 - *S_PARTIAL_FLUSH
//

pub const PACKET3_EVENT_WRITE__EVENT_INDEX__OTHER: c_int = 0;
pub const PACKET3_EVENT_WRITE__EVENT_INDEX__SAMPLE_PIPELINESTATS: c_int = 2;
pub const PACKET3_EVENT_WRITE__EVENT_INDEX__CS_PARTIAL_FLUSH: c_int = 4;
pub const PACKET3_RELEASE_MEM: c_uint = 0x49;

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
//

// 0 - DST_ADDR using DAS
// 1 - GDS
// 3 - DST_ADDR using L2
//

// 0 - LRU
// 1 - Stream
//

// 0 - SRC_ADDR using SAS
// 1 - GDS
// 2 - DATA
// 3 - SRC_ADDR using L2
//

// COMMAND

// 0 - memory
// 1 - register
//

// 0 - memory
// 1 - register
//

pub const PACKET3_ACQUIRE_MEM: c_uint = 0x58;
// 1.  HEADER
// 2.  COHER_CNTL [30:0]
// 2.1 ENGINE_SEL [31:31]
// 3.  COHER_SIZE [31:0]
// 4.  COHER_SIZE_HI [7:0]
// 5.  COHER_BASE_LO [31:0]
// 6.  COHER_BASE_HI [23:0]
// 7.  POLL_INTERVAL [15:0]
//
// COHER_CNTL fields for CP_COHER_CNTL

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
pub const PACKET3_FRAME_CONTROL: c_uint = 0x90;

//
// x=0: tmz_begin
// x=1: tmz_end
//
pub const PACKET3_INVALIDATE_TLBS: c_uint = 0x98;

pub const PACKET3_SET_RESOURCES: c_uint = 0xA0;
// 1. header
// 2. CONTROL
// 3. QUEUE_MASK_LO [31:0]
// 4. QUEUE_MASK_HI [31:0]
// 5. GWS_MASK_LO [31:0]
// 6. GWS_MASK_HI [31:0]
// 7. OAC_MASK [15:0]
// 8. GDS_HEAP_SIZE [16:11] | GDS_HEAP_BASE [5:0]
//

pub const PACKET3_MAP_QUEUES: c_uint = 0xA2;
// 1. header
// 2. CONTROL
// 3. CONTROL2
// 4. MQD_ADDR_LO [31:0]
// 5. MQD_ADDR_HI [31:0]
// 6. WPTR_ADDR_LO [31:0]
// 7. WPTR_ADDR_HI [31:0]
//
// CONTROL

// CONTROL2

pub const PACKET3_UNMAP_QUEUES: c_uint = 0xA3;
// 1. header
// 2. CONTROL
// 3. CONTROL2
// 4. CONTROL3
// 5. CONTROL4
// 6. CONTROL5
//
// CONTROL

// 0 - PREEMPT_QUEUES
// 1 - RESET_QUEUES
// 2 - DISABLE_PROCESS_QUEUES
// 3 - PREEMPT_QUEUES_NO_UNMAP
//

// CONTROL2a

// CONTROL2b

// CONTROL3a

// CONTROL3b

// CONTROL4

// CONTROL5

pub const PACKET3_QUERY_STATUS: c_uint = 0xA4;
// 1. header
// 2. CONTROL
// 3. CONTROL2
// 4. ADDR_LO [31:0]
// 5. ADDR_HI [31:0]
// 6. DATA_LO [31:0]
// 7. DATA_HI [31:0]
//
// CONTROL

// CONTROL2a

// CONTROL2b

pub const PACKET3_RUN_CLEANER_SHADER_9_0: c_uint = 0xD7;
// 1. header
// 2. RESERVED [31:0]
//
pub const PACKET3_RUN_CLEANER_SHADER: c_uint = 0xD2;
// 1. header
// 2. RESERVED [31:0]
//
pub const VCE_CMD_NO_OP: c_uint = 0x00000000;
pub const VCE_CMD_END: c_uint = 0x00000001;
pub const VCE_CMD_IB: c_uint = 0x00000002;
pub const VCE_CMD_FENCE: c_uint = 0x00000003;
pub const VCE_CMD_TRAP: c_uint = 0x00000004;
pub const VCE_CMD_IB_AUTO: c_uint = 0x00000005;
pub const VCE_CMD_SEMAPHORE: c_uint = 0x00000006;
pub const VCE_CMD_IB_VM: c_uint = 0x00000102;
pub const VCE_CMD_WAIT_GE: c_uint = 0x00000106;
pub const VCE_CMD_UPDATE_PTB: c_uint = 0x00000107;
pub const VCE_CMD_FLUSH_TLB: c_uint = 0x00000108;
pub const VCE_CMD_REG_WRITE: c_uint = 0x00000109;
pub const VCE_CMD_REG_WAIT: c_uint = 0x0000010a;
pub const HEVC_ENC_CMD_NO_OP: c_uint = 0x00000000;
pub const HEVC_ENC_CMD_END: c_uint = 0x00000001;
pub const HEVC_ENC_CMD_FENCE: c_uint = 0x00000003;
pub const HEVC_ENC_CMD_TRAP: c_uint = 0x00000004;
pub const HEVC_ENC_CMD_IB_VM: c_uint = 0x00000102;
pub const HEVC_ENC_CMD_REG_WRITE: c_uint = 0x00000109;
pub const HEVC_ENC_CMD_REG_WAIT: c_uint = 0x0000010a;
