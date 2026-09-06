//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/gfx_v12_1_pkt.h
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
// Copyright 2025 Advanced Micro Devices, Inc.
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
// PM4 definitions
//
pub const PACKET_TYPE0: c_int = 0;
pub const PACKET_TYPE1: c_int = 1;
pub const PACKET_TYPE2: c_int = 2;
pub const PACKET_TYPE3: c_int = 3;

pub const CP_PACKET2: c_uint = 0x80000000;
pub const PACKET2_PAD_SHIFT: c_int = 0;

// Packet 3 types
pub const PACKET3_NOP: c_uint = 0x10;
pub const PACKET3_CLEAR_STATE: c_uint = 0x12;
pub const PACKET3_INDEX_BUFFER_SIZE: c_uint = 0x13;
pub const PACKET3_DISPATCH_DIRECT: c_uint = 0x15;
pub const PACKET3_DISPATCH_INDIRECT: c_uint = 0x16;
pub const PACKET3_ATOMIC_MEM: c_uint = 0x1E;

pub const PACKET3_ATOMIC_MEM__COMMAND__SINGLE_PASS_ATOMIC: c_int = 0;
pub const PACKET3_ATOMIC_MEM__COMMAND__LOOP_UNTIL_COMPARE_SATISFIED: c_int = 1;
pub const PACKET3_ATOMIC_MEM__COMMAND__WAIT_FOR_WRITE_CONFIRMATION: c_int = 2;
pub const PACKET3_ATOMIC_MEM__COMMAND__SEND_AND_CONTINUE: c_int = 3;
pub const PACKET3_ATOMIC_MEM__SCOPE__CU: c_int = 0;
pub const PACKET3_ATOMIC_MEM__SCOPE__SE: c_int = 1;
pub const PACKET3_ATOMIC_MEM__SCOPE__DEVICE: c_int = 2;
pub const PACKET3_ATOMIC_MEM__SCOPE__SYSTEM: c_int = 3;
pub const PACKET3_ATOMIC_MEM__TEMPORAL__RT: c_int = 0;
pub const PACKET3_ATOMIC_MEM__TEMPORAL__NT: c_int = 1;
pub const PACKET3_ATOMIC_MEM__TEMPORAL__FW: c_int = 2;
pub const PACKET3_ATOMIC_MEM__TEMPORAL__UC: c_int = 3;
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
pub const PACKET3_DRAW_INDIRECT_MULTI: c_uint = 0x2C;
pub const PACKET3_DRAW_INDEX_AUTO: c_uint = 0x2D;
pub const PACKET3_NUM_INSTANCES: c_uint = 0x2F;
pub const PACKET3_DRAW_INDEX_MULTI_AUTO: c_uint = 0x30;
pub const PACKET3_DRAW_INDEX_OFFSET_2: c_uint = 0x35;
pub const PACKET3_WRITE_DATA: c_uint = 0x37;

pub const PACKET3_WRITE_DATA__DST_SEL__MEM_MAPPED_REGISTER: c_int = 0;
pub const PACKET3_WRITE_DATA__DST_SEL__TC_L2: c_int = 2;
pub const PACKET3_WRITE_DATA__DST_SEL__MEMORY: c_int = 5;
pub const PACKET3_WRITE_DATA__DST_SEL__MEMORY_MAPPED_ADC_PERSISTENT_STATE: c_int = 6;
// 0 - register
// 1 - reserved
// 2 - tc_l2
// 3 - reserved
// 4 - reserved
// 5 - memory (same as tc_l2)
// 6 - memory_mapped_adc_persistent_state
//

pub const PACKET3_WRITE_DATA__SCOPE__CU: c_int = 0;
pub const PACKET3_WRITE_DATA__SCOPE__SE: c_int = 1;
pub const PACKET3_WRITE_DATA__SCOPE__DEVICE: c_int = 2;
pub const PACKET3_WRITE_DATA__SCOPE__SYSTEM: c_int = 3;

pub const PACKET3_WRITE_DATA__MODE__LOCAL_XCD: c_int = 0;
pub const PACKET3_WRITE_DATA__MODE__REMOTE_OR_LOCAL_AID: c_int = 1;
pub const PACKET3_WRITE_DATA__MODE__REMOTE_XCD: c_int = 2;
pub const PACKET3_WRITE_DATA__MODE__REMOTE_MID: c_int = 3;
// 0 - local xcd
// 1 - remote/local aid
// 2 - remote xcd
// 3 - remote mid
//

pub const PACKET3_WRITE_DATA__ADDR_INCR__INCREMENT_ADDRESS: c_int = 0;
pub const PACKET3_WRITE_DATA__ADDR_INCR__DO_NOT_INCREMENT_ADDRESS: c_int = 1;

pub const PACKET3_WRITE_DATA__WR_CONFIRM__DO_NOT_WAIT_FOR_WRITE_CONFIRMATION: c_int = 0;
pub const PACKET3_WRITE_DATA__WR_CONFIRM__WAIT_FOR_WRITE_CONFIRMATION: c_int = 1;

pub const PACKET3_WRITE_DATA__TEMPORAL__RT: c_int = 0;
pub const PACKET3_WRITE_DATA__TEMPORAL__NT: c_int = 1;
pub const PACKET3_WRITE_DATA__TEMPORAL__HT: c_int = 2;
pub const PACKET3_WRITE_DATA__TEMPORAL__LU: c_int = 3;

pub const PACKET3_WRITE_DATA__COOP_DISABLE__MASTER_AND_SLAVE_COOP: c_int = 0;
pub const PACKET3_WRITE_DATA__COOP_DISABLE__MASTER_ONLY: c_int = 1;

pub const PACKET3_DRAW_INDEX_INDIRECT_MULTI: c_uint = 0x38;
pub const PACKET3_WAIT_REG_MEM: c_uint = 0x3C;

pub const PACKET3_WAIT_REG_MEM__FUNCTION__ALWAYS_PASS: c_int = 0;
pub const PACKET3_WAIT_REG_MEM__FUNCTION__LESS_THAN_REF_VALUE: c_int = 1;
pub const PACKET3_WAIT_REG_MEM__FUNCTION__LESS_THAN_EQUAL_TO_THE_REF_VALUE: c_int = 2;
pub const PACKET3_WAIT_REG_MEM__FUNCTION__EQUAL_TO_THE_REFERENCE_VALUE: c_int = 3;
pub const PACKET3_WAIT_REG_MEM__FUNCTION__NOT_EQUAL_REFERENCE_VALUE: c_int = 4;
pub const PACKET3_WAIT_REG_MEM__FUNCTION__GREATER_THAN_OR_EQUAL_REFERENCE_VALUE: c_int = 5;
pub const PACKET3_WAIT_REG_MEM__FUNCTION__GREATER_THAN_REFERENCE_VALUE: c_int = 6;
// 0 - always
// 1 - <
// 2 - <=
// 3 - ==
// 4 - !=
// 5 - >=
// 6 - >
//

pub const PACKET3_WAIT_REG_MEM__MEM_SPACE__REGISTER_SPACE: c_int = 0;
pub const PACKET3_WAIT_REG_MEM__MEM_SPACE__MEMORY_SPACE: c_int = 1;
// 0 - reg
// 1 - mem
//

pub const PACKET3_WAIT_REG_MEM__OPERATION__WAIT_REG_MEM: c_int = 0;
pub const PACKET3_WAIT_REG_MEM__OPERATION__WR_WAIT_WR_REG: c_int = 1;
pub const PACKET3_WAIT_REG_MEM__OPERATION__WAIT_MEM_PREEMPTABLE: c_int = 3;
// 0 - wait_reg_mem
// 1 - wr_wait_wr_reg
// 2 - reserved
// 3 - wait_mem_preemptable
//

pub const PACKET3_WAIT_REG_MEM__MODE__LOCAL_XCD: c_int = 0;
pub const PACKET3_WAIT_REG_MEM__MODE__REMOTE_OR_LOCAL_AID: c_int = 1;
pub const PACKET3_WAIT_REG_MEM__MODE__REMOTE_XCD: c_int = 2;
pub const PACKET3_WAIT_REG_MEM__MODE__REMOTE_MID: c_int = 3;
// 0 - local xcd
// 1 - remote/local aid
// 2 - remote xcd
// 3 - remote mid
//

pub const PACKET3_WAIT_REG_MEM__TEMPORAL__RT: c_int = 0;
pub const PACKET3_WAIT_REG_MEM__TEMPORAL__NT: c_int = 1;
pub const PACKET3_WAIT_REG_MEM__TEMPORAL__HT: c_int = 2;
pub const PACKET3_WAIT_REG_MEM__TEMPORAL__LU: c_int = 3;
// 0 - rt
// 1 - nt
// 2 - ht
// 3 - lu
//

pub const PACKET3_INDIRECT_BUFFER: c_uint = 0x3F;

pub const PACKET3_INDIRECT_BUFFER__TEMPORAL__RT: c_int = 0;
pub const PACKET3_INDIRECT_BUFFER__TEMPORAL__NT: c_int = 1;
pub const PACKET3_INDIRECT_BUFFER__TEMPORAL__HT: c_int = 2;
pub const PACKET3_INDIRECT_BUFFER__TEMPORAL__LU: c_int = 3;
pub const PACKET3_COND_INDIRECT_BUFFER: c_uint = 0x3F;
pub const PACKET3_COPY_DATA: c_uint = 0x40;

pub const PACKET3_COPY_DATA__SRC_SEL__MEM_MAPPED_REGISTER: c_int = 0;
pub const PACKET3_COPY_DATA__SRC_SEL__TC_L2_OBSOLETE: c_int = 1;
pub const PACKET3_COPY_DATA__SRC_SEL__TC_L2: c_int = 2;
pub const PACKET3_COPY_DATA__SRC_SEL__PERFCOUNTERS: c_int = 4;
pub const PACKET3_COPY_DATA__SRC_SEL__IMMEDIATE_DATA: c_int = 5;
pub const PACKET3_COPY_DATA__SRC_SEL__ATOMIC_RETURN_DATA: c_int = 6;
pub const PACKET3_COPY_DATA__SRC_SEL__GPU_CLOCK_COUNT: c_int = 9;
pub const PACKET3_COPY_DATA__SRC_SEL__SYSTEM_CLOCK_COUNT: c_int = 10;
pub const PACKET3_COPY_DATA__SRC_SCOPE__CU: c_int = 0;
pub const PACKET3_COPY_DATA__SRC_SCOPE__SE: c_int = 1;
pub const PACKET3_COPY_DATA__SRC_SCOPE__DEVICE: c_int = 2;
pub const PACKET3_COPY_DATA__SRC_SCOPE__SYSTEM: c_int = 3;
pub const PACKET3_COPY_DATA__MODE__LOCAL_XCD: c_int = 0;
pub const PACKET3_COPY_DATA__MODE__REMOTE_OR_LOCAL_AID: c_int = 1;
pub const PACKET3_COPY_DATA__MODE__REMOTE_XCD: c_int = 2;
pub const PACKET3_COPY_DATA__MODE__REMOTE_MID: c_int = 3;
pub const PACKET3_COPY_DATA__DST_SEL__MEM_MAPPED_REGISTER: c_int = 0;
pub const PACKET3_COPY_DATA__DST_SEL__TC_L2: c_int = 2;
pub const PACKET3_COPY_DATA__DST_SEL__PERFCOUNTERS: c_int = 4;
pub const PACKET3_COPY_DATA__DST_SEL__TC_L2_OBSOLETE: c_int = 5;
pub const PACKET3_COPY_DATA__DST_SEL__MEM_MAPPED_REG_DC: c_int = 6;
pub const PACKET3_COPY_DATA__SRC_TEMPORAL__RT: c_int = 0;
pub const PACKET3_COPY_DATA__SRC_TEMPORAL__NT: c_int = 1;
pub const PACKET3_COPY_DATA__SRC_TEMPORAL__HT: c_int = 2;
pub const PACKET3_COPY_DATA__SRC_TEMPORAL__LU: c_int = 3;
pub const PACKET3_COPY_DATA__COUNT_SEL__32_BITS_OF_DATA: c_int = 0;
pub const PACKET3_COPY_DATA__COUNT_SEL__64_BITS_OF_DATA: c_int = 1;
pub const PACKET3_COPY_DATA__SRC_DST_REMOTE_MODE__SRC_IS_REMOTE: c_int = 0;
pub const PACKET3_COPY_DATA__SRC_DST_REMOTE_MODE__DST_IS_REMOTE: c_int = 1;
pub const PACKET3_COPY_DATA__WR_CONFIRM__DO_NOT_WAIT_FOR_CONFIRMATION: c_int = 0;
pub const PACKET3_COPY_DATA__WR_CONFIRM__WAIT_FOR_CONFIRMATION: c_int = 1;
pub const PACKET3_COPY_DATA__DST_TEMPORAL__RT: c_int = 0;
pub const PACKET3_COPY_DATA__DST_TEMPORAL__NT: c_int = 1;
pub const PACKET3_COPY_DATA__DST_TEMPORAL__HT: c_int = 2;
pub const PACKET3_COPY_DATA__DST_TEMPORAL__LU: c_int = 3;
pub const PACKET3_COPY_DATA__DST_SCOPE__CU: c_int = 0;
pub const PACKET3_COPY_DATA__DST_SCOPE__SE: c_int = 1;
pub const PACKET3_COPY_DATA__DST_SCOPE__DEVICE: c_int = 2;
pub const PACKET3_COPY_DATA__DST_SCOPE__SYSTEM: c_int = 3;
pub const PACKET3_COPY_DATA__PQ_EXE_STATUS__DEFAULT: c_int = 0;
pub const PACKET3_COPY_DATA__PQ_EXE_STATUS__PHASE_UPDATE: c_int = 1;
pub const PACKET3_PFP_SYNC_ME: c_uint = 0x42;
pub const PACKET3_COND_WRITE: c_uint = 0x45;
pub const PACKET3_EVENT_WRITE: c_uint = 0x46;

pub const PACKET3_EVENT_WRITE__EVENT_INDEX__OTHER: c_int = 0;
pub const PACKET3_EVENT_WRITE__EVENT_INDEX__SAMPLE_PIPELINESTAT: c_int = 2;
pub const PACKET3_EVENT_WRITE__EVENT_INDEX__CS_PARTIAL_FLUSH: c_int = 4;
pub const PACKET3_EVENT_WRITE_EOP: c_uint = 0x47;
pub const PACKET3_EVENT_WRITE_EOS: c_uint = 0x48;
pub const PACKET3_RELEASE_MEM: c_uint = 0x49;

pub const PACKET3_RELEASE_MEM__EVENT_INDEX__END_OF_PIPE: c_int = 5;
pub const PACKET3_RELEASE_MEM__EVENT_INDEX__SHADER_DONE: c_int = 6;

pub const PACKET3_RELEASE_MEM__TEMPORAL__RT: c_int = 0;
pub const PACKET3_RELEASE_MEM__TEMPORAL__NT: c_int = 1;
pub const PACKET3_RELEASE_MEM__TEMPORAL__HT: c_int = 2;
pub const PACKET3_RELEASE_MEM__TEMPORAL__LU: c_int = 3;
// 0 - temporal__release_mem__rt
// 1 - temporal__release_mem__nt
// 2 - temporal__release_mem__ht
// 3 - temporal__release_mem__lu
//

pub const PACKET3_RELEASE_MEM__PQ_EXE_STATUS__DEFAULT: c_int = 0;
pub const PACKET3_RELEASE_MEM__PQ_EXE_STATUS__PHASE_UPDATE: c_int = 1;

pub const PACKET3_RELEASE_MEM__DST_SEL__MEMORY_CONTROLLER: c_int = 0;
pub const PACKET3_RELEASE_MEM__DST_SEL__TC_L2: c_int = 1;
pub const PACKET3_RELEASE_MEM__DST_SEL__QUQUE_WRITE_POINTER_REGISTER: c_int = 2;
pub const PACKET3_RELEASE_MEM__DST_SEL__QUQUE_WRITE_POINTER_POLL_MASK_BIT: c_int = 3;

pub const PACKET3_RELEASE_MEM__MES_ACTION_ID__NO_MES_NOTIFICATION: c_int = 0;
pub const PACKET3_RELEASE_MEM__MES_ACTION_ID__INTERRUPT_AND_FENCE: c_int = 1;
pub const PACKET3_RELEASE_MEM__MES_ACTION_ID__INTERRUPT_NO_FENCE_THEN_ADDRESS_PAYLOAD: c_int = 2;
pub const PACKET3_RELEASE_MEM__MES_ACTION_ID__INTERRUPT_AND_ADDRESS_PAYLOAD: c_int = 3;

pub const PACKET3_RELEASE_MEM__INT_SEL__NONE: c_int = 0;
pub const PACKET3_RELEASE_MEM__INT_SEL__SEND_INTERRUPT_ONLY: c_int = 1;
pub const PACKET3_RELEASE_MEM__INT_SEL__SEND_INTERRUPT_AFTER_WRITE_CONFIRM: c_int = 2;
pub const PACKET3_RELEASE_MEM__INT_SEL__SEND_DATA_AND_WRITE_CONFIRM: c_int = 3;
pub const PACKET3_RELEASE_MEM__INT_SEL__UNCONDITIONALLY_SEND_INT_CTXID: c_int = 4;
pub const PACKET3_RELEASE_MEM__INT_SEL__UNCONDITIONALLY_SEND_INT_CTXID_BASED_ON_32_BIT_COMPARE: c_int = 5;
pub const PACKET3_RELEASE_MEM__INT_SEL__UNCONDITIONALLY_SEND_INT_CTXID_BASED_ON_64_BIT_COMPARE: c_int = 6;

pub const PACKET3_RELEASE_MEM__DATA_SEL__NONE: c_int = 0;
pub const PACKET3_RELEASE_MEM__DATA_SEL__SEND_32_BIT_LOW: c_int = 1;
pub const PACKET3_RELEASE_MEM__DATA_SEL__SEND_64_BIT_DATA: c_int = 2;
pub const PACKET3_RELEASE_MEM__DATA_SEL__SEND_GPU_CLOCK_COUNTER: c_int = 3;
pub const PACKET3_RELEASE_MEM__DATA_SEL__SEND_SYSTEM_CLOCK_COUNTER: c_int = 4;
// 0 - discard
// 1 - send low 32bit data
// 2 - send 64bit data
// 3 - send 64bit GPU counter value
// 4 - send 64bit sys counter value
//

pub const PACKET3_PREAMBLE_CNTL: c_uint = 0x4A;

pub const PACKET3_DMA_DATA: c_uint = 0x50;
// 1. header
// 2. CONTROL
// 3. SRC_ADDR_LO or DATA [31:0]
// 4. SRC_ADDR_HI [31:0]
// 5. DST_ADDR_LO [31:0]
// 6. DST_ADDR_HI [7:0]
// 7. COMMAND [31:26] | BYTE_COUNT [25:0]
//
// CONTROL

// 0 - ME
// 1 - PFP
//

// 0 - rt
// 1 - nt
// 2 - ht
// 3 - lu
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

pub const PACKET3_CONTEXT_REG_RMW: c_uint = 0x51;
pub const PACKET3_ACQUIRE_MEM: c_uint = 0x58;
// 1.  HEADER

// 3.  COHER_SIZE [31:0]

// 4.  COHER_SIZE_HI [7:0]

// 5.  COHER_BASE_LO [31:0]

// 6.  COHER_BASE_HI [23:0]

// 7.  POLL_INTERVAL [15:0]

// 8.  GCR_CNTL [18:0]

pub const PACKET3_ACQUIRE_MEM__GCR_CNTL__GLI_INV__NOP: c_int = 0;
pub const PACKET3_ACQUIRE_MEM__GCR_CNTL__GLI_INV__ALL: c_int = 1;
pub const PACKET3_ACQUIRE_MEM__GCR_CNTL__GLI_INV__RANGE: c_int = 2;
pub const PACKET3_ACQUIRE_MEM__GCR_CNTL__GLI_INV__FIRST_LAST: c_int = 3;
//
// 0:NOP
// 1:ALL
// 2:RANGE
// 3:FIRST_LAST
//

pub const PACKET3_ACQUIRE_MEM__GCR_CNTL__GL1_RANGE__ALL: c_int = 0;
pub const PACKET3_ACQUIRE_MEM__GCR_CNTL__GL1_RANGE__RANGE: c_int = 2;
pub const PACKET3_ACQUIRE_MEM__GCR_CNTL__GL1_RANGE__FIRST_LAST: c_int = 3;
//
// 0:ALL
// 1:reserved
// 2:RANGE
// 3:FIRST_LAST
//

pub const PACKET3_ACQUIRE_MEM__GCR_CNTL__GL2_SCOPE__DEVICE: c_int = 0;
pub const PACKET3_ACQUIRE_MEM__GCR_CNTL__GL2_SCOPE__SYSTEM: c_int = 1;
pub const PACKET3_ACQUIRE_MEM__GCR_CNTL__GL2_SCOPE__FORCE_ALL: c_int = 2;
//
// 0:Device scope
// 1:System scope
// 2:Force INV/WB all
// 3:Reserved
//

pub const PACKET3_ACQUIRE_MEM__GCR_CNTL__GL2_RANGE__ALL: c_int = 0;
pub const PACKET3_ACQUIRE_MEM__GCR_CNTL__GL2_RANGE__VOL: c_int = 1;
pub const PACKET3_ACQUIRE_MEM__GCR_CNTL__GL2_RANGE__RANGE: c_int = 2;
pub const PACKET3_ACQUIRE_MEM__GCR_CNTL__GL2_RANGE__FIRST_LAST: c_int = 3;
//
// 0:ALL
// 1:VOL
// 2:RANGE
// 3:FIRST_LAST
//

pub const PACKET3_ACQUIRE_MEM__GCR_CNTL__SEQ__PARALLET: c_int = 0;
pub const PACKET3_ACQUIRE_MEM__GCR_CNTL__SEQ__FORWARD: c_int = 1;
pub const PACKET3_ACQUIRE_MEM__GCR_CNTL__SEQ__REVERSE: c_int = 2;
//
// 0: PARALLEL
// 1: FORWARD
// 2: REVERSE
//

pub const PACKET3_GEN_PDEPTE: c_uint = 0x5B;
pub const PACKET3_PRIME_UTCL2: c_uint = 0x5D;
pub const PACKET3_LOAD_UCONFIG_REG: c_uint = 0x5E;
pub const PACKET3_LOAD_SH_REG: c_uint = 0x5F;
pub const PACKET3_LOAD_CONFIG_REG: c_uint = 0x60;
pub const PACKET3_LOAD_CONTEXT_REG: c_uint = 0x61;
pub const PACKET3_LOAD_COMPUTE_STATE: c_uint = 0x62;
pub const PACKET3_LOAD_SH_REG_INDEX: c_uint = 0x63;
pub const PACKET3_SET_CONFIG_REG: c_uint = 0x68;
pub const PACKET3_SET_CONFIG_REG_START: c_uint = 0x00002000;
pub const PACKET3_SET_CONFIG_REG_END: c_uint = 0x00002c00;
pub const PACKET3_SET_CONTEXT_REG: c_uint = 0x69;
pub const PACKET3_SET_CONTEXT_REG_START: c_uint = 0x0000a000;
pub const PACKET3_SET_CONTEXT_REG_END: c_uint = 0x0000a400;
pub const PACKET3_SET_SH_REG: c_uint = 0x76;
pub const PACKET3_SET_SH_REG_START: c_uint = 0x00002c00;
pub const PACKET3_SET_SH_REG_END: c_uint = 0x00003000;

pub const PACKET3_SET_SH_REG__INDEX__DEFAULT: c_int = 0;
pub const PACKET3_SET_SH_REG__INDEX__INSERT_VMID: c_int = 1;
pub const PACKET3_SET_SH_REG_OFFSET: c_uint = 0x77;
pub const PACKET3_SET_QUEUE_REG: c_uint = 0x78;
pub const PACKET3_SET_UCONFIG_REG: c_uint = 0x79;
pub const PACKET3_SET_UCONFIG_REG_START: c_uint = 0x0000c000;
pub const PACKET3_SET_UCONFIG_REG_END: c_uint = 0x0000c400;

pub const PACKET3_SET_UCONFIG_REG_INDEX: c_uint = 0x7A;
pub const PACKET3_DISPATCH_DRAW_PREAMBLE: c_uint = 0x8C;
pub const PACKET3_DISPATCH_DRAW: c_uint = 0x8D;
pub const PACKET3_INDEX_ATTRIBUTES_INDIRECT: c_uint = 0x91;
pub const PACKET3_WAIT_REG_MEM64: c_uint = 0x93;
pub const PACKET3_HDP_FLUSH: c_uint = 0x95;
pub const PACKET3_INVALIDATE_TLBS: c_uint = 0x98;

pub const PACKET3_DMA_DATA_FILL_MULTI: c_uint = 0x9A;
pub const PACKET3_SET_SH_REG_INDEX: c_uint = 0x9B;
pub const PACKET3_LOAD_CONTEXT_REG_INDEX: c_uint = 0x9F;
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

