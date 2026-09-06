//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/include/ivsrcid/ivsrcid_vislands30.h
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
// Volcanic Islands IV SRC Register documentation
//
// Copyright (C) 2015  Advanced Micro Devices, Inc.
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
// IV Source IDs

pub const VISLANDS30_IV_EXTID_D1_V_UPDATE_INT: c_int = 0;

pub const VISLANDS30_IV_EXTID_D1_GRPH_PFLIP: c_int = 0;

pub const VISLANDS30_IV_EXTID_D2_V_UPDATE_INT: c_int = 0;

pub const VISLANDS30_IV_EXTID_D2_GRPH_PFLIP: c_int = 0;

pub const VISLANDS30_IV_EXTID_D3_V_UPDATE_INT: c_int = 0;

pub const VISLANDS30_IV_EXTID_D3_GRPH_PFLIP: c_int = 0;

pub const VISLANDS30_IV_EXTID_D4_V_UPDATE_INT: c_int = 0;

pub const VISLANDS30_IV_EXTID_D4_GRPH_PFLIP: c_int = 0;

pub const VISLANDS30_IV_EXTID_D5_V_UPDATE_INT: c_int = 0;

pub const VISLANDS30_IV_EXTID_D5_GRPH_PFLIP: c_int = 0;

pub const VISLANDS30_IV_EXTID_D6_V_UPDATE_INT: c_int = 0;

pub const VISLANDS30_IV_EXTID_D6_GRPH_PFLIP: c_int = 0;

pub const VISLANDS30_IV_EXTID_D1_VERTICAL_INTERRUPT0: c_int = 7;

pub const VISLANDS30_IV_EXTID_D1_VERTICAL_INTERRUPT1: c_int = 8;

pub const VISLANDS30_IV_EXTID_D1_VERTICAL_INTERRUPT2: c_int = 9;

pub const VISLANDS30_IV_EXTID_D1_EXT_TIMING_SYNC_LOSS: c_int = 10;

pub const VISLANDS30_IV_EXTID_D1_EXT_TIMING_SYNC: c_int = 11;

pub const VISLANDS30_IV_EXTID_D1_EXT_TIMING_SIGNAL: c_int = 12;

pub const VISLANDS30_IV_EXTID_D2_VERTICAL_INTERRUPT0: c_int = 7;

pub const VISLANDS30_IV_EXTID_D2_VERTICAL_INTERRUPT1: c_int = 8;

pub const VISLANDS30_IV_EXTID_D2_VERTICAL_INTERRUPT2: c_int = 9;

pub const VISLANDS30_IV_EXTID_D2_EXT_TIMING_SYNC_LOSS: c_int = 10;

pub const VISLANDS30_IV_EXTID_D2_EXT_TIMING_SYNC: c_int = 11;

pub const VISLANDS30_IV_EXTID_D2_EXT_TIMING_SIGNAL: c_int = 12;

pub const VISLANDS30_IV_EXTID_D3_VERTICAL_INTERRUPT0: c_int = 7;

pub const VISLANDS30_IV_EXTID_D3_VERTICAL_INTERRUPT1: c_int = 8;

pub const VISLANDS30_IV_EXTID_D3_VERTICAL_INTERRUPT2: c_int = 9;

pub const VISLANDS30_IV_EXTID_D3_EXT_TIMING_SYNC_LOSS: c_int = 10;

pub const VISLANDS30_IV_EXTID_D3_EXT_TIMING_SYNC: c_int = 11;

pub const VISLANDS30_IV_EXTID_D3_EXT_TIMING_SIGNAL: c_int = 12;

pub const VISLANDS30_IV_EXTID_D4_VERTICAL_INTERRUPT0: c_int = 7;

pub const VISLANDS30_IV_EXTID_D4_VERTICAL_INTERRUPT1: c_int = 8;

pub const VISLANDS30_IV_EXTID_D4_VERTICAL_INTERRUPT2: c_int = 9;

pub const VISLANDS30_IV_EXTID_D4_EXT_TIMING_SYNC_LOSS: c_int = 10;

pub const VISLANDS30_IV_EXTID_D4_EXT_TIMING_SYNC: c_int = 11;

pub const VISLANDS30_IV_EXTID_D4_EXT_TIMING_SIGNAL: c_int = 12;

pub const VISLANDS30_IV_EXTID_D5_VERTICAL_INTERRUPT0: c_int = 7;

pub const VISLANDS30_IV_EXTID_D5_VERTICAL_INTERRUPT1: c_int = 8;

pub const VISLANDS30_IV_EXTID_D5_VERTICAL_INTERRUPT2: c_int = 9;

pub const VISLANDS30_IV_EXTID_D5_EXT_TIMING_SYNC_LOSS: c_int = 10;

pub const VISLANDS30_IV_EXTID_D5_EXT_TIMING_SYNC: c_int = 11;

pub const VISLANDS30_IV_EXTID_D5_EXT_TIMING_SIGNAL: c_int = 12;

pub const VISLANDS30_IV_EXTID_D6_VERTICAL_INTERRUPT0: c_int = 7;

pub const VISLANDS30_IV_EXTID_D6_VERTICAL_INTERRUPT1: c_int = 8;

pub const VISLANDS30_IV_EXTID_D6_VERTICAL_INTERRUPT2: c_int = 9;

pub const VISLANDS30_IV_EXTID_HOTPLUG_DETECT_A: c_int = 0;

pub const VISLANDS30_IV_EXTID_HOTPLUG_DETECT_B: c_int = 1;

pub const VISLANDS30_IV_EXTID_HOTPLUG_DETECT_C: c_int = 2;

pub const VISLANDS30_IV_EXTID_HOTPLUG_DETECT_D: c_int = 3;

pub const VISLANDS30_IV_EXTID_HOTPLUG_DETECT_E: c_int = 4;

pub const VISLANDS30_IV_EXTID_HOTPLUG_DETECT_F: c_int = 5;

pub const VISLANDS30_IV_EXTID_HPD_RX_A: c_int = 6;

pub const VISLANDS30_IV_EXTID_HPD_RX_B: c_int = 7;

pub const VISLANDS30_IV_EXTID_HPD_RX_C: c_int = 8;

pub const VISLANDS30_IV_EXTID_HPD_RX_D: c_int = 9;

pub const VISLANDS30_IV_EXTID_HPD_RX_E: c_int = 10;

pub const VISLANDS30_IV_EXTID_HPD_RX_F: c_int = 11;
pub const VISLANDS30_IV_SRCID_GPIO_19: c_uint = 0x00000053  /* 83 */;
pub const VISLANDS30_IV_SRCID_SRBM_READ_TIMEOUT_ERR: c_uint = 0x00000060  /* 96 */;
pub const VISLANDS30_IV_SRCID_SRBM_CTX_SWITCH: c_uint = 0x00000061  /* 97 */;
pub const VISLANDS30_IV_SRBM_REG_ACCESS_ERROR: c_uint = 0x00000062  /* 98 */;
pub const VISLANDS30_IV_SRCID_UVD_ENC_GEN_PURP: c_uint = 0x00000077  /* 119 */;
pub const VISLANDS30_IV_SRCID_UVD_SYSTEM_MESSAGE: c_uint = 0x0000007c  /* 124 */;
pub const VISLANDS30_IV_SRCID_BIF_PF_VF_MSGBUF_VALID: c_uint = 0x00000087  /* 135 */;
pub const VISLANDS30_IV_SRCID_BIF_VF_PF_MSGBUF_ACK: c_uint = 0x0000008a  /* 138 */;
pub const VISLANDS30_IV_SRCID_SYS_PAGE_INV_FAULT: c_uint = 0x0000008c  /* 140 */;
pub const VISLANDS30_IV_SRCID_SYS_MEM_PROT_FAULT: c_uint = 0x0000008d  /* 141 */;
pub const VISLANDS30_IV_SRCID_SEM_PAGE_INV_FAULT: c_uint = 0x00000090  /* 144 */;
pub const VISLANDS30_IV_SRCID_SEM_MEM_PROT_FAULT: c_uint = 0x00000091  /* 145 */;
pub const VISLANDS30_IV_SRCID_GFX_PAGE_INV_FAULT: c_uint = 0x00000092  /* 146 */;
pub const VISLANDS30_IV_SRCID_GFX_MEM_PROT_FAULT: c_uint = 0x00000093  /* 147 */;
pub const VISLANDS30_IV_SRCID_ACP: c_uint = 0x000000a2  /* 162 */;
pub const VISLANDS30_IV_SRCID_VCE_TRAP: c_uint = 0x000000a7  /* 167 */;
pub const VISLANDS30_IV_EXTID_VCE_TRAP_GENERAL_PURPOSE: c_int = 0;
pub const VISLANDS30_IV_EXTID_VCE_TRAP_LOW_LATENCY: c_int = 1;
pub const VISLANDS30_IV_EXTID_VCE_TRAP_REAL_TIME: c_int = 2;
pub const VISLANDS30_IV_SRCID_CP_INT_RB: c_uint = 0x000000b0  /* 176 */;
pub const VISLANDS30_IV_SRCID_CP_INT_IB1: c_uint = 0x000000b1  /* 177 */;
pub const VISLANDS30_IV_SRCID_CP_INT_IB2: c_uint = 0x000000b2  /* 178 */;
pub const VISLANDS30_IV_SRCID_CP_PM4_RES_BITS_ERR: c_uint = 0x000000b4  /* 180 */;
pub const VISLANDS30_IV_SRCID_CP_END_OF_PIPE: c_uint = 0x000000b5  /* 181 */;
pub const VISLANDS30_IV_SRCID_CP_BAD_OPCODE: c_uint = 0x000000b7  /* 183 */;
pub const VISLANDS30_IV_SRCID_CP_PRIV_REG_FAULT: c_uint = 0x000000b8  /* 184 */;
pub const VISLANDS30_IV_SRCID_CP_PRIV_INSTR_FAULT: c_uint = 0x000000b9  /* 185 */;
pub const VISLANDS30_IV_SRCID_CP_WAIT_MEM_SEM_FAULT: c_uint = 0x000000ba  /* 186 */;
pub const VISLANDS30_IV_SRCID_CP_GUI_IDLE: c_uint = 0x000000bb  /* 187 */;
pub const VISLANDS30_IV_SRCID_CP_GUI_BUSY: c_uint = 0x000000bc  /* 188 */;
pub const VISLANDS30_IV_SRCID_CP_COMPUTE_QUERY_STATUS: c_uint = 0x000000bf  /* 191 */;
pub const VISLANDS30_IV_SRCID_CP_ECC_ERROR: c_uint = 0x000000c5  /* 197 */;
pub const CARRIZO_IV_SRCID_CP_COMPUTE_QUERY_STATUS: c_uint = 0x000000c7  /* 199 */;
pub const VISLANDS30_IV_SRCID_CP_WAIT_REG_MEM_POLL_TIMEOUT: c_uint = 0x000000c0  /* 192 */;
pub const VISLANDS30_IV_SRCID_CP_SEM_SIG_INCOMPL: c_uint = 0x000000c1  /* 193 */;
pub const VISLANDS30_IV_SRCID_CP_PREEMPT_ACK: c_uint = 0x000000c2  /* 194 */;
pub const VISLANDS30_IV_SRCID_CP_GENERAL_PROT_FAULT: c_uint = 0x000000c3  /* 195 */;
pub const VISLANDS30_IV_SRCID_CP_GDS_ALLOC_ERROR: c_uint = 0x000000c4  /* 196 */;
pub const VISLANDS30_IV_SRCID_CP_ECC_ERROR: c_uint = 0x000000c5  /* 197 */;
pub const VISLANDS30_IV_SRCID_RLC_STRM_PERF_MONITOR: c_uint = 0x000000ca  /* 202 */;
pub const VISLANDS30_IV_SDMA_ATOMIC_SRC_ID: c_uint = 0x000000da  /* 218 */;
pub const VISLANDS30_IV_SRCID_SDMA_ECC_ERROR: c_uint = 0x000000dc  /* 220 */;
pub const VISLANDS30_IV_SRCID_SDMA_TRAP: c_uint = 0x000000e0  /* 224 */;
pub const VISLANDS30_IV_SRCID_SDMA_SEM_INCOMPLETE: c_uint = 0x000000e1  /* 225 */;
pub const VISLANDS30_IV_SRCID_SDMA_SEM_WAIT: c_uint = 0x000000e2  /* 226 */;
pub const VISLANDS30_IV_SRCID_SMU_DISP_TIMER2_TRIGGER: c_uint = 0x000000e5  /* 229 */;
pub const VISLANDS30_IV_SRCID_CG_TSS_THERMAL_LOW_TO_HIGH: c_uint = 0x000000e6  /* 230 */;
pub const VISLANDS30_IV_SRCID_CG_TSS_THERMAL_HIGH_TO_LOW: c_uint = 0x000000e7  /* 231 */;
pub const VISLANDS30_IV_SRCID_GRBM_READ_TIMEOUT_ERR: c_uint = 0x000000e8  /* 232 */;
pub const VISLANDS30_IV_SRCID_GRBM_REG_GUI_IDLE: c_uint = 0x000000e9  /* 233 */;
pub const VISLANDS30_IV_SRCID_SQ_INTERRUPT_MSG: c_uint = 0x000000ef  /* 239 */;
pub const VISLANDS30_IV_SRCID_SDMA_PREEMPT: c_uint = 0x000000f0  /* 240 */;
pub const VISLANDS30_IV_SRCID_SDMA_VM_HOLE: c_uint = 0x000000f2  /* 242 */;
pub const VISLANDS30_IV_SRCID_SDMA_CTXEMPTY: c_uint = 0x000000f3  /* 243 */;
pub const VISLANDS30_IV_SRCID_SDMA_DOORBELL_INVALID: c_uint = 0x000000f4  /* 244 */;
pub const VISLANDS30_IV_SRCID_SDMA_FROZEN: c_uint = 0x000000f5  /* 245 */;
pub const VISLANDS30_IV_SRCID_SDMA_POLL_TIMEOUT: c_uint = 0x000000f6  /* 246 */;
pub const VISLANDS30_IV_SRCID_SDMA_SRBM_WRITE: c_uint = 0x000000f7  /* 247 */;
pub const VISLANDS30_IV_SRCID_CG_THERMAL_TRIG: c_uint = 0x000000f8  /* 248 */;
pub const VISLANDS30_IV_SRCID_SMU_DISP_TIMER_TRIGGER: c_uint = 0x000000fd  /* 253 */;
// These are not "real" source ids defined by HW
pub const VISLANDS30_IV_SRCID_VM_CONTEXT_ALL: c_uint = 0x00000100  /* 256 */;
pub const VISLANDS30_IV_EXTID_VM_CONTEXT0_ALL: c_int = 0;
pub const VISLANDS30_IV_EXTID_VM_CONTEXT1_ALL: c_int = 1;
// IV Extended IDs
pub const VISLANDS30_IV_EXTID_NONE: c_uint = 0x00000000;
pub const VISLANDS30_IV_EXTID_INVALID: c_uint = 0xffffffff;
