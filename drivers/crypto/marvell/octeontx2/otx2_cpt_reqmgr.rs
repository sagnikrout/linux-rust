//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/marvell/octeontx2/otx2_cpt_reqmgr.h
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


// SPDX-License-Identifier: GPL-2.0-only
// Copyright (C) 2020 Marvell.
//

// Completion code size and initial value
pub const OTX2_CPT_COMPLETION_CODE_SIZE: c_int = 8;

//
// Maximum total number of SG buffers is 100, we divide it equally
// between input and output
//
pub const OTX2_CPT_MAX_SG_IN_CNT: c_int = 50;
pub const OTX2_CPT_MAX_SG_OUT_CNT: c_int = 50;
// DMA mode direct or SG
pub const OTX2_CPT_DMA_MODE_DIRECT: c_int = 0;
pub const OTX2_CPT_DMA_MODE_SG: c_int = 1;
// Context source CPTR or DPTR
pub const OTX2_CPT_FROM_CPTR: c_int = 0;
pub const OTX2_CPT_FROM_DPTR: c_int = 1;
pub const OTX2_CPT_MAX_REQ_SIZE: c_int = 65535;
pub const SG_COMPS_MAX: c_int = 4;
pub const SGV2_COMPS_MAX: c_int = 3;
pub const SG_COMP_3: c_int = 3;
pub const SG_COMP_2: c_int = 2;
pub const SG_COMP_1: c_int = 1;
pub const OTX2_CPT_DPTR_RPTR_ALIGN: c_int = 8;
pub const OTX2_CPT_RES_ADDR_ALIGN: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub union otx2_cpt_opcode {
    pub flags: u16,
    pub major: u8,
    pub minor: u8,
    pub s: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_cptvf_request {
    pub param1: u32,
    pub param2: u32,
    pub dlen: u16,
    pub opcode: otx2_cpt_opcode,
    pub cptr_dma: dma_addr_t,
    pub cptr: *mut c_void,
}

//
// CPT_INST_S software command definitions
// Words EI (0-3)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union otx2_cpt_iq_cmd_word0 {
    pub u: u64,
    pub opcode: __be16,
    pub param1: __be16,
    pub param2: __be16,
    pub dlen: __be16,
    pub s: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union otx2_cpt_iq_cmd_word3 {
    pub u: u64,
    pub cptr:61: u64,
    pub grp:3: u64,
    pub s: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_cpt_iq_command {
    pub cmd: otx2_cpt_iq_cmd_word0,
    pub dptr: u64,
    pub rptr: u64,
    pub cptr: otx2_cpt_iq_cmd_word3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_cpt_pending_entry {
    pub /: *mut *mut *mut void completion_addr; / Completion address,
    pub info: *mut c_void,
// Kernel async request callback
    pub arg2): *mut *mut *mut void (callback)(int status, void arg1, void,
    pub /: *mut *mut *mut crypto_async_request areq; / Async request callback arg,
    pub /: *mut *mut u8 resume_sender; / Notify sender to resume sending requests,
    pub /: *mut *mut u8 busy; / Entry status (free/busy),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_cpt_pending_queue {
    pub /: *mut *mut *mut otx2_cpt_pending_entry head; / Head of the queue,
    pub /: *mut *mut u32 front; / Process work from here,
    pub /: *mut *mut u32 rear; / Append new work here,
    pub /: *mut *mut u32 pending_count; / Pending requests count,
    pub /: *mut *mut u32 qlen; / Queue length,
    pub /: *mut *mut spinlock_t lock; / Queue lock,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_cpt_buf_ptr {
    pub vptr: *mut u8,
    pub dma_addr: dma_addr_t,
    pub size: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union otx2_cpt_ctrl_info {
    pub flags: u32,

    pub reserved_6_31:26: u32,
    pub /: *mut *mut u32 grp:3; / Group bits,
    pub /: *mut *mut u32 dma_mode:2; / DMA mode,
    pub /: *mut *mut u32 se_req:1; / To SE core,

    pub /: *mut *mut u32 se_req:1; / To SE core,
    pub /: *mut *mut u32 dma_mode:2; / DMA mode,
    pub /: *mut *mut u32 grp:3; / Group bits,
    pub reserved_6_31:26: u32,

    pub s: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_cpt_req_info {
// Kernel async request callback
    pub arg2): *mut *mut *mut void (callback)(int status, void arg1, void,
    pub /: *mut *mut *mut crypto_async_request areq; / Async request callback arg,
    pub /: *mut *mut otx2_cptvf_request req;/ Request information (core specific),
    pub /: *mut *mut otx2_cpt_ctrl_info ctrl;/ User control information,
    pub in: [otx2_cpt_buf_ptr; OTX2_CPT_MAX_SG_IN_CNT],
    pub out: [otx2_cpt_buf_ptr; OTX2_CPT_MAX_SG_OUT_CNT],
    pub /: *mut *mut *mut u8 iv_out; / IV to send back,
    pub /: *mut *mut u16 rlen; / Output length,
    pub /: *mut *mut u8 in_cnt; / Number of input buffers,
    pub /: *mut *mut u8 out_cnt; / Number of output buffers,
    pub /: *mut *mut u8 req_type; / Type of request,
    pub /: *mut *mut u8 is_enc; / Is a request an encryption request,
    pub /: *mut *mut u8 is_trunc_hmac;/ Is truncated hmac used,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_cpt_inst_info {
    pub pentry: *mut otx2_cpt_pending_entry,
    pub req: *mut otx2_cpt_req_info,
    pub pdev: *mut pci_dev,
    pub completion_addr: *mut c_void,
    pub out_buffer: *mut u8,
    pub in_buffer: *mut u8,
    pub dptr_baddr: dma_addr_t,
    pub rptr_baddr: dma_addr_t,
    pub comp_baddr: dma_addr_t,
    pub time_in: c_ulong,
    pub dlen: u32,
    pub dma_len: u32,
    pub gthr_sz: u64,
    pub sctr_sz: u64,
    pub extra_time: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_cpt_sglist_component {
    pub len0: __be16,
    pub len1: __be16,
    pub len2: __be16,
    pub len3: __be16,
    pub ptr0: __be64,
    pub ptr1: __be64,
    pub ptr2: __be64,
    pub ptr3: __be64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cn10kb_cpt_sglist_component {
    pub len0: u16,
    pub len1: u16,
    pub len2: u16,
    pub valid_segs: u16,
    pub ptr0: u64,
    pub ptr1: u64,
    pub ptr2: u64,
}

// Allocate memory to meet below alignment requirement:
// ------------------------------------
// |    struct otx2_cpt_inst_info       |
// |    (No alignment required)         |
// |    --------------------------------|
// |   | padding for ARCH_DMA_MINALIGN  |
// |   | alignment                      |
// |------------------------------------|
// |    SG List Gather/Input memory     |
// |    Length = multiple of 32Bytes    |
// |    Alignment = 8Byte               |
// |----------------------------------  |
// |    SG List Scatter/Output memory   |
// |    Length = multiple of 32Bytes    |
// |    Alignment = 8Byte               |
// |     -------------------------------|
// |    | padding for 32B alignment     |
// |------------------------------------|
// |    Result response memory          |
// |    Alignment = 32Byte              |
// ------------------------------------
//
// Allocate extra memory for SG and response address alignment
// Setup gather (input) components
//
// Get buffer for union otx2_cpt_res_s response
// structure and its physical address
//
// SG list header size in bytes
pub const SG_LIST_HDR_SIZE: c_int = 8;
// Allocate memory to meet below alignment requirement:
// ------------------------------------
// |    struct otx2_cpt_inst_info       |
// |    (No alignment required)         |
// |    --------------------------------|
// |   | padding for ARCH_DMA_MINALIGN  |
// |   | alignment                      |
// |------------------------------------|
// |    SG List Header of 8 Byte        |
// |------------------------------------|
// |    SG List Gather/Input memory     |
// |    Length = multiple of 32Bytes    |
// |    Alignment = 8Byte               |
// |----------------------------------  |
// |    SG List Scatter/Output memory   |
// |    Length = multiple of 32Bytes    |
// |    Alignment = 8Byte               |
// |     -------------------------------|
// |    | padding for 32B alignment     |
// |------------------------------------|
// |    Result response memory          |
// |    Alignment = 32Byte              |
// ------------------------------------
//
// Allocate extra memory for SG and response address alignment
// Setup gather (input) components
//
// Get buffer for union otx2_cpt_res_s response
// structure and its physical address
//
extern "C" {
    pub fn otx2_cpt_post_process(wqe: *mut otx2_cptlf_wqe);
}
