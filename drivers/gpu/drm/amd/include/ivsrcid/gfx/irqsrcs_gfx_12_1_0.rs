//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/include/ivsrcid/gfx/irqsrcs_gfx_12_1_0.h
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
// 0x0 UTCL2 has encountered a fault scenario
pub const GFX_12_1_0__SRCID__UTCL2_FAULT: c_int = 0;
// 0x1 UTCL2 has encountered a retry scenario
pub const GFX_12_1_0__SRCID__UTCL2_RETRY: c_int = 1;
// 0x2 UTCL2 for data poisoning
pub const GFX_12_1_0__SRCID__UTCL2_DATA_POISONING: c_int = 2;
// 0x30 SDMA atomic*_rtn ops complete
pub const GFX_12_1_0__SRCID__SDMA_ATOMIC_RTN_DONE: c_int = 48;
// 0x31 Trap
pub const GFX_12_1_0__SRCID__SDMA_TRAP: c_int = 49;
// 0x32 SRBM write Protection
pub const GFX_12_1_0__SRCID__SDMA_SRBMWRITE: c_int = 50;
// 0x33 Context Empty
pub const GFX_12_1_0__SRCID__SDMA_CTXEMPTY: c_int = 51;
// 0x34 SDMA New Run List
pub const GFX_12_1_0__SRCID__SDMA_PREEMPT: c_int = 52;
// 0x35 sdma mid - command buffer preempt interrupt
pub const GFX_12_1_0__SRCID__SDMA_IB_PREEMPT: c_int = 53;
// 0x36 Doorbell BE invalid
pub const GFX_12_1_0__SRCID__SDMA_DOORBELL_INVALID: c_int = 54;
// 0x37 Queue hang or Command timeout
pub const GFX_12_1_0__SRCID__SDMA_QUEUE_HANG: c_int = 55;
// 0x38 SDMA atomic CMPSWAP loop timeout
pub const GFX_12_1_0__SRCID__SDMA_ATOMIC_TIMEOUT: c_int = 56;
// 0x39 SRBM read poll timeout
pub const GFX_12_1_0__SRCID__SDMA_POLL_TIMEOUT: c_int = 57;
// 0x3A Page retry  timeout after UTCL2 return nack = 1
pub const GFX_12_1_0__SRCID__SDMA_PAGE_TIMEOUT: c_int = 58;
// 0x3B Page Null from UTCL2 when nack = 2
pub const GFX_12_1_0__SRCID__SDMA_PAGE_NULL: c_int = 59;
// 0x3C Page Fault Error from UTCL2 when nack = 3
pub const GFX_12_1_0__SRCID__SDMA_PAGE_FAULT: c_int = 60;
// 0x3D MC or SEM address in VM hole
pub const GFX_12_1_0__SRCID__SDMA_INVALID_ADDR: c_int = 61;
// 0x3E ECC Error
pub const GFX_12_1_0__SRCID__SDMA_ECC: c_int = 62;
// 0x3F SDMA Frozen
pub const GFX_12_1_0__SRCID__SDMA_FROZEN: c_int = 63;
// 0x40 SRAM ECC Error
pub const GFX_12_1_0__SRCID__SDMA_SRAM_ECC: c_int = 64;
// 0x41 GPF(Sem incomplete timeout)
pub const GFX_12_1_0__SRCID__SDMA_SEM_INCOMPLETE_TIMEOUT: c_int = 65;
// 0x42 Semaphore wait fail timeout
pub const GFX_12_1_0__SRCID__SDMA_SEM_WAIT_FAIL_TIMEOUT: c_int = 66;
// 0x43 Wptr less than Rptr in active queue
pub const GFX_12_1_0__SRCID__SDMA_INVALID_RB_PTR: c_int = 67;
// 0x44 BE command exception
pub const GFX_12_1_0__SRCID__SDMA_BE_EXCEPTION: c_int = 68;
// 0x46 User fence. inherit from gfx v12_0 for gfx user queue
pub const GFX_12_1_0__SRCID__SDMA_FENCE: c_int = 70;
// 0xB0 CP_INTERRUPT pkt in RB
pub const GFX_12_1_0__SRCID__CP_RB_INT_PKT: c_int = 176;
// 0xB1 CP_INTERRUPT pkt in IB1
pub const GFX_12_1_0__SRCID__CP_IB1_INT_PKT: c_int = 177;
// 0xB2 CP_INTERRUPT pkt in IB2
pub const GFX_12_1_0__SRCID__CP_IB2_INT_PKT: c_int = 178;
// 0xB3 DMA Watch Interrupt
pub const GFX_12_1_0__SRCID__CP_DMA_WATCH_INTERRUPT: c_int = 179;
// 0xB4 PM4 Pkt Rsvd Bits Error
pub const GFX_12_1_0__SRCID__CP_PM4_PKT_RSVD_BIT_ERROR: c_int = 180;
// 0xB5 End-of-Pipe Interrupt
pub const GFX_12_1_0__SRCID__CP_EOP_INTERRUPT: c_int = 181;
// 0xB7 Bad Opcode Error
pub const GFX_12_1_0__SRCID__CP_BAD_OPCODE_ERROR: c_int = 183;
// 0xB8 Privileged Register Fault
pub const GFX_12_1_0__SRCID__CP_PRIV_REG_FAULT: c_int = 184;
// 0xB9 Privileged Instr Fault
pub const GFX_12_1_0__SRCID__CP_PRIV_INSTR_FAULT: c_int = 185;
// 0xBA Wait Memory Semaphore Fault (Sync Object Fault)
pub const GFX_12_1_0__SRCID__CP_WAIT_MEM_SEM_FAULT: c_int = 186;
// 0xBB Context Empty Interrupt
pub const GFX_12_1_0__SRCID__CP_CTX_EMPTY_INTERRUPT: c_int = 187;
// 0xBC Context Busy Interrupt
pub const GFX_12_1_0__SRCID__CP_CTX_BUSY_INTERRUPT: c_int = 188;
// 0xC0 CP.ME Wait_Reg_Mem Poll Timeout
pub const GFX_12_1_0__SRCID__CP_ME_WAIT_REG_MEM_POLL_TIMEOUT: c_int = 192;
// 0xC1 Surface Probe Fault Signal Incomplete
pub const GFX_12_1_0__SRCID__CP_SIG_INCOMPLETE: c_int = 193;
// 0xC2 Preemption Ack-wledge
pub const GFX_12_1_0__SRCID__CP_PREEMPT_ACK: c_int = 194;
// 0xC3 General Protection Fault (GPF)
pub const GFX_12_1_0__SRCID__CP_GPF: c_int = 195;
// 0xC4 GDS Alloc Error
pub const GFX_12_1_0__SRCID__CP_GDS_ALLOC_ERROR: c_int = 196;
// 0xC5 ECC Error
pub const GFX_12_1_0__SRCID__CP_ECC_ERROR: c_int = 197;
// 0xC8 Unattached VM Doorbell Received
pub const GFX_12_1_0__SRCID__CP_VM_DOORBELL: c_int = 200;
// 0xC9 ECC FUE Error
pub const GFX_12_1_0__SRCID__CP_FUE_ERROR: c_int = 201;
// 0xCA Suspend Completion Interrupt
pub const GFX_12_1_0__SRCID__CP_SUSPEAND_REQ_INTERRUPT: c_int = 202;
// 0xCB Resume Completion Interrupt
pub const GFX_12_1_0__SRCID__CP_RESUME_REQ_INTERRUPT: c_int = 203;
// 0xCA RLC Streaming Perf Monitor Interrupt
// ContextID[15:0] each bit indicates poison is seen on respecive indexed VMID
// Ex: ContextID[3] == 1 means VMID-3 encountered poison consumption
// ContextID[16] == 1 indicates that complete VF need to reset with FLR
pub const GFX_12_1_0__SRCID__RLC_STRM_PERF_MONITOR_INTERRUPT: c_int = 202;
// 0xCB RLC Poison Interrupt
pub const GFX_12_1_0__SRCID__RLC_POISON_INTERRUPT: c_int = 203;
// 0xE7 High on ContextID[0] - nHT Error;  ContextID[1] - illegal Opcode Error
pub const GFX_12_1_0__SRCID__PMR_EA_ERROR_INTERRUPT: c_int = 231;
// 0xE8 CRead timeout error
pub const GFX_12_1_0__SRCID__GRBM_RD_TIMEOUT_ERROR: c_int = 232;
// 0xE9 Register GUI Idle
pub const GFX_12_1_0__SRCID__GRBM_REG_GUI_IDLE: c_int = 233;
// 0xEF SQ Interrupt (ttrace wrap, errors)
pub const GFX_12_1_0__SRCID__SQ_INTERRUPT_ID: c_int = 239;
