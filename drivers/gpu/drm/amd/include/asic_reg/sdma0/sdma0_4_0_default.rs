//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/include/asic_reg/sdma0/sdma0_4_0_default.h
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

// Macro flag: #define _sdma0_4_0_DEFAULT_HEADER
// addressBlock: sdma0_sdma0dec
pub const mmSDMA0_UCODE_ADDR_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_UCODE_DATA_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_VM_CNTL_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_VM_CTX_LO_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_VM_CTX_HI_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_ACTIVE_FCN_ID_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_VM_CTX_CNTL_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_VIRT_RESET_REQ_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_VF_ENABLE_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_CONTEXT_REG_TYPE0_DEFAULT: c_uint = 0xfffdf79f;
pub const mmSDMA0_CONTEXT_REG_TYPE1_DEFAULT: c_uint = 0x003fbcff;
pub const mmSDMA0_CONTEXT_REG_TYPE2_DEFAULT: c_uint = 0x000003ff;
pub const mmSDMA0_CONTEXT_REG_TYPE3_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_PUB_REG_TYPE0_DEFAULT: c_uint = 0x3c000000;
pub const mmSDMA0_PUB_REG_TYPE1_DEFAULT: c_uint = 0x30003882;
pub const mmSDMA0_PUB_REG_TYPE2_DEFAULT: c_uint = 0x0fc6e880;
pub const mmSDMA0_PUB_REG_TYPE3_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_MMHUB_CNTL_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_CONTEXT_GROUP_BOUNDARY_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_POWER_CNTL_DEFAULT: c_uint = 0x0003c000;
pub const mmSDMA0_CLK_CTRL_DEFAULT: c_uint = 0xff000100;
pub const mmSDMA0_CNTL_DEFAULT: c_uint = 0x00000002;
pub const mmSDMA0_CHICKEN_BITS_DEFAULT: c_uint = 0x00831f07;
pub const mmSDMA0_GB_ADDR_CONFIG_DEFAULT: c_uint = 0x00100012;
pub const mmSDMA0_GB_ADDR_CONFIG_READ_DEFAULT: c_uint = 0x00100012;
pub const mmSDMA0_RB_RPTR_FETCH_HI_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_SEM_WAIT_FAIL_TIMER_CNTL_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RB_RPTR_FETCH_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_IB_OFFSET_FETCH_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_PROGRAM_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_STATUS_REG_DEFAULT: c_uint = 0x46dee557;
pub const mmSDMA0_STATUS1_REG_DEFAULT: c_uint = 0x000003ff;
pub const mmSDMA0_RD_BURST_CNTL_DEFAULT: c_uint = 0x00000003;
pub const mmSDMA0_HBM_PAGE_CONFIG_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_UCODE_CHECKSUM_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_F32_CNTL_DEFAULT: c_uint = 0x00000001;
pub const mmSDMA0_FREEZE_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_PHASE0_QUANTUM_DEFAULT: c_uint = 0x00010002;
pub const mmSDMA0_PHASE1_QUANTUM_DEFAULT: c_uint = 0x00010002;
pub const mmSDMA_POWER_GATING_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA_PGFSM_CONFIG_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA_PGFSM_WRITE_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA_PGFSM_READ_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_EDC_CONFIG_DEFAULT: c_uint = 0x00000002;
pub const mmSDMA0_BA_THRESHOLD_DEFAULT: c_uint = 0x03ff03ff;
pub const mmSDMA0_ID_DEFAULT: c_uint = 0x00000001;
pub const mmSDMA0_VERSION_DEFAULT: c_uint = 0x00000400;
pub const mmSDMA0_EDC_COUNTER_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_EDC_COUNTER_CLEAR_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_STATUS2_REG_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_ATOMIC_CNTL_DEFAULT: c_uint = 0x00000200;
pub const mmSDMA0_ATOMIC_PREOP_LO_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_ATOMIC_PREOP_HI_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_UTCL1_CNTL_DEFAULT: c_uint = 0xd0003019;
pub const mmSDMA0_UTCL1_WATERMK_DEFAULT: c_uint = 0xfffbe1fe;
pub const mmSDMA0_UTCL1_RD_STATUS_DEFAULT: c_uint = 0x201001ff;
pub const mmSDMA0_UTCL1_WR_STATUS_DEFAULT: c_uint = 0x503001ff;
pub const mmSDMA0_UTCL1_INV0_DEFAULT: c_uint = 0x00000600;
pub const mmSDMA0_UTCL1_INV1_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_UTCL1_INV2_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_UTCL1_RD_XNACK0_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_UTCL1_RD_XNACK1_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_UTCL1_WR_XNACK0_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_UTCL1_WR_XNACK1_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_UTCL1_TIMEOUT_DEFAULT: c_uint = 0x00010001;
pub const mmSDMA0_UTCL1_PAGE_DEFAULT: c_uint = 0x000003e0;
pub const mmSDMA0_POWER_CNTL_IDLE_DEFAULT: c_uint = 0x06060200;
pub const mmSDMA0_RELAX_ORDERING_LUT_DEFAULT: c_uint = 0xc0000006;
pub const mmSDMA0_CHICKEN_BITS_2_DEFAULT: c_uint = 0x00000005;
pub const mmSDMA0_STATUS3_REG_DEFAULT: c_uint = 0x00100000;
pub const mmSDMA0_PHYSICAL_ADDR_LO_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_PHYSICAL_ADDR_HI_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_PHASE2_QUANTUM_DEFAULT: c_uint = 0x00010002;
pub const mmSDMA0_ERROR_LOG_DEFAULT: c_uint = 0x0000000f;
pub const mmSDMA0_PUB_DUMMY_REG0_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_PUB_DUMMY_REG1_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_PUB_DUMMY_REG2_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_PUB_DUMMY_REG3_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_F32_COUNTER_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_UNBREAKABLE_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_PERFMON_CNTL_DEFAULT: c_uint = 0x000ff7fd;
pub const mmSDMA0_PERFCOUNTER0_RESULT_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_PERFCOUNTER1_RESULT_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_PERFCOUNTER_TAG_DELAY_RANGE_DEFAULT: c_uint = 0x00640000;
pub const mmSDMA0_CRD_CNTL_DEFAULT: c_uint = 0x000085c0;
pub const mmSDMA0_MMHUB_TRUSTLVL_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_GPU_IOV_VIOLATION_LOG_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_ULV_CNTL_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_EA_DBIT_ADDR_DATA_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_EA_DBIT_ADDR_INDEX_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_GFX_RB_CNTL_DEFAULT: c_uint = 0x00040000;
pub const mmSDMA0_GFX_RB_BASE_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_GFX_RB_BASE_HI_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_GFX_RB_RPTR_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_GFX_RB_RPTR_HI_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_GFX_RB_WPTR_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_GFX_RB_WPTR_HI_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_GFX_RB_WPTR_POLL_CNTL_DEFAULT: c_uint = 0x00401000;
pub const mmSDMA0_GFX_RB_RPTR_ADDR_HI_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_GFX_RB_RPTR_ADDR_LO_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_GFX_IB_CNTL_DEFAULT: c_uint = 0x00000100;
pub const mmSDMA0_GFX_IB_RPTR_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_GFX_IB_OFFSET_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_GFX_IB_BASE_LO_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_GFX_IB_BASE_HI_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_GFX_IB_SIZE_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_GFX_SKIP_CNTL_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_GFX_CONTEXT_STATUS_DEFAULT: c_uint = 0x00000005;
pub const mmSDMA0_GFX_DOORBELL_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_GFX_CONTEXT_CNTL_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_GFX_STATUS_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_GFX_DOORBELL_LOG_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_GFX_WATERMARK_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_GFX_DOORBELL_OFFSET_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_GFX_CSA_ADDR_LO_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_GFX_CSA_ADDR_HI_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_GFX_IB_SUB_REMAIN_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_GFX_PREEMPT_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_GFX_DUMMY_REG_DEFAULT: c_uint = 0x0000000f;
pub const mmSDMA0_GFX_RB_WPTR_POLL_ADDR_HI_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_GFX_RB_WPTR_POLL_ADDR_LO_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_GFX_RB_AQL_CNTL_DEFAULT: c_uint = 0x00004000;
pub const mmSDMA0_GFX_MINOR_PTR_UPDATE_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_GFX_MIDCMD_DATA0_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_GFX_MIDCMD_DATA1_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_GFX_MIDCMD_DATA2_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_GFX_MIDCMD_DATA3_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_GFX_MIDCMD_DATA4_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_GFX_MIDCMD_DATA5_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_GFX_MIDCMD_DATA6_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_GFX_MIDCMD_DATA7_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_GFX_MIDCMD_DATA8_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_GFX_MIDCMD_CNTL_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_PAGE_RB_CNTL_DEFAULT: c_uint = 0x00040000;
pub const mmSDMA0_PAGE_RB_BASE_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_PAGE_RB_BASE_HI_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_PAGE_RB_RPTR_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_PAGE_RB_RPTR_HI_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_PAGE_RB_WPTR_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_PAGE_RB_WPTR_HI_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_PAGE_RB_WPTR_POLL_CNTL_DEFAULT: c_uint = 0x00401000;
pub const mmSDMA0_PAGE_RB_RPTR_ADDR_HI_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_PAGE_RB_RPTR_ADDR_LO_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_PAGE_IB_CNTL_DEFAULT: c_uint = 0x00000100;
pub const mmSDMA0_PAGE_IB_RPTR_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_PAGE_IB_OFFSET_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_PAGE_IB_BASE_LO_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_PAGE_IB_BASE_HI_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_PAGE_IB_SIZE_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_PAGE_SKIP_CNTL_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_PAGE_CONTEXT_STATUS_DEFAULT: c_uint = 0x00000004;
pub const mmSDMA0_PAGE_DOORBELL_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_PAGE_STATUS_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_PAGE_DOORBELL_LOG_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_PAGE_WATERMARK_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_PAGE_DOORBELL_OFFSET_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_PAGE_CSA_ADDR_LO_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_PAGE_CSA_ADDR_HI_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_PAGE_IB_SUB_REMAIN_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_PAGE_PREEMPT_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_PAGE_DUMMY_REG_DEFAULT: c_uint = 0x0000000f;
pub const mmSDMA0_PAGE_RB_WPTR_POLL_ADDR_HI_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_PAGE_RB_WPTR_POLL_ADDR_LO_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_PAGE_RB_AQL_CNTL_DEFAULT: c_uint = 0x00004000;
pub const mmSDMA0_PAGE_MINOR_PTR_UPDATE_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_PAGE_MIDCMD_DATA0_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_PAGE_MIDCMD_DATA1_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_PAGE_MIDCMD_DATA2_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_PAGE_MIDCMD_DATA3_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_PAGE_MIDCMD_DATA4_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_PAGE_MIDCMD_DATA5_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_PAGE_MIDCMD_DATA6_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_PAGE_MIDCMD_DATA7_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_PAGE_MIDCMD_DATA8_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_PAGE_MIDCMD_CNTL_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC0_RB_CNTL_DEFAULT: c_uint = 0x00040000;
pub const mmSDMA0_RLC0_RB_BASE_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC0_RB_BASE_HI_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC0_RB_RPTR_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC0_RB_RPTR_HI_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC0_RB_WPTR_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC0_RB_WPTR_HI_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC0_RB_WPTR_POLL_CNTL_DEFAULT: c_uint = 0x00401000;
pub const mmSDMA0_RLC0_RB_RPTR_ADDR_HI_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC0_RB_RPTR_ADDR_LO_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC0_IB_CNTL_DEFAULT: c_uint = 0x00000100;
pub const mmSDMA0_RLC0_IB_RPTR_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC0_IB_OFFSET_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC0_IB_BASE_LO_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC0_IB_BASE_HI_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC0_IB_SIZE_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC0_SKIP_CNTL_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC0_CONTEXT_STATUS_DEFAULT: c_uint = 0x00000004;
pub const mmSDMA0_RLC0_DOORBELL_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC0_STATUS_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC0_DOORBELL_LOG_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC0_WATERMARK_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC0_DOORBELL_OFFSET_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC0_CSA_ADDR_LO_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC0_CSA_ADDR_HI_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC0_IB_SUB_REMAIN_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC0_PREEMPT_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC0_DUMMY_REG_DEFAULT: c_uint = 0x0000000f;
pub const mmSDMA0_RLC0_RB_WPTR_POLL_ADDR_HI_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC0_RB_WPTR_POLL_ADDR_LO_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC0_RB_AQL_CNTL_DEFAULT: c_uint = 0x00004000;
pub const mmSDMA0_RLC0_MINOR_PTR_UPDATE_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC0_MIDCMD_DATA0_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC0_MIDCMD_DATA1_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC0_MIDCMD_DATA2_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC0_MIDCMD_DATA3_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC0_MIDCMD_DATA4_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC0_MIDCMD_DATA5_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC0_MIDCMD_DATA6_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC0_MIDCMD_DATA7_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC0_MIDCMD_DATA8_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC0_MIDCMD_CNTL_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC1_RB_CNTL_DEFAULT: c_uint = 0x00040000;
pub const mmSDMA0_RLC1_RB_BASE_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC1_RB_BASE_HI_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC1_RB_RPTR_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC1_RB_RPTR_HI_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC1_RB_WPTR_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC1_RB_WPTR_HI_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC1_RB_WPTR_POLL_CNTL_DEFAULT: c_uint = 0x00401000;
pub const mmSDMA0_RLC1_RB_RPTR_ADDR_HI_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC1_RB_RPTR_ADDR_LO_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC1_IB_CNTL_DEFAULT: c_uint = 0x00000100;
pub const mmSDMA0_RLC1_IB_RPTR_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC1_IB_OFFSET_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC1_IB_BASE_LO_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC1_IB_BASE_HI_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC1_IB_SIZE_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC1_SKIP_CNTL_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC1_CONTEXT_STATUS_DEFAULT: c_uint = 0x00000004;
pub const mmSDMA0_RLC1_DOORBELL_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC1_STATUS_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC1_DOORBELL_LOG_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC1_WATERMARK_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC1_DOORBELL_OFFSET_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC1_CSA_ADDR_LO_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC1_CSA_ADDR_HI_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC1_IB_SUB_REMAIN_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC1_PREEMPT_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC1_DUMMY_REG_DEFAULT: c_uint = 0x0000000f;
pub const mmSDMA0_RLC1_RB_WPTR_POLL_ADDR_HI_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC1_RB_WPTR_POLL_ADDR_LO_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC1_RB_AQL_CNTL_DEFAULT: c_uint = 0x00004000;
pub const mmSDMA0_RLC1_MINOR_PTR_UPDATE_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC1_MIDCMD_DATA0_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC1_MIDCMD_DATA1_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC1_MIDCMD_DATA2_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC1_MIDCMD_DATA3_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC1_MIDCMD_DATA4_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC1_MIDCMD_DATA5_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC1_MIDCMD_DATA6_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC1_MIDCMD_DATA7_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC1_MIDCMD_DATA8_DEFAULT: c_uint = 0x00000000;
pub const mmSDMA0_RLC1_MIDCMD_CNTL_DEFAULT: c_uint = 0x00000000;
