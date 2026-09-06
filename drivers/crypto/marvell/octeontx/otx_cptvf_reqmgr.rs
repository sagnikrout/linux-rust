//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/marvell/octeontx/otx_cptvf_reqmgr.h
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


// SPDX-License-Identifier: GPL-2.0
// Marvell OcteonTX CPT driver
//
// Copyright (C) 2019 Marvell International Ltd.
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License version 2 as
// published by the Free Software Foundation.
//

//
// Maximum total number of SG buffers is 100, we divide it equally
// between input and output
//
pub const OTX_CPT_MAX_SG_IN_CNT: c_int = 50;
pub const OTX_CPT_MAX_SG_OUT_CNT: c_int = 50;
// DMA mode direct or SG
pub const OTX_CPT_DMA_DIRECT_DIRECT: c_int = 0;
pub const OTX_CPT_DMA_GATHER_SCATTER: c_int = 1;
// Context source CPTR or DPTR
pub const OTX_CPT_FROM_CPTR: c_int = 0;
pub const OTX_CPT_FROM_DPTR: c_int = 1;
// CPT instruction queue alignment
pub const OTX_CPT_INST_Q_ALIGNMENT: c_int = 128;
pub const OTX_CPT_MAX_REQ_SIZE: c_int = 65535;
// Default command timeout in seconds
pub const OTX_CPT_COMMAND_TIMEOUT: c_int = 4;
pub const OTX_CPT_TIMER_HOLD: c_uint = 0x03F;
pub const OTX_CPT_COUNT_HOLD: c_int = 32;
pub const OTX_CPT_TIME_IN_RESET_COUNT: c_int = 5;
// Minimum and maximum values for interrupt coalescing
pub const OTX_CPT_COALESC_MIN_TIME_WAIT: c_uint = 0x0;

pub const OTX_CPT_COALESC_MIN_NUM_WAIT: c_uint = 0x0;

#[repr(C)]
#[derive(Copy, Clone)]
pub union otx_cpt_opcode_info {
    pub flags: u16,
    pub major: u8,
    pub minor: u8,
    pub s: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx_cptvf_request {
    pub param1: u32,
    pub param2: u32,
    pub dlen: u16,
    pub opcode: otx_cpt_opcode_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx_cpt_buf_ptr {
    pub vptr: *mut u8,
    pub dma_addr: dma_addr_t,
    pub size: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union otx_cpt_ctrl_info {
    pub flags: u32,

    pub reserved0:26: u32,
    pub /: *mut *mut u32 grp:3; / Group bits,
    pub /: *mut *mut u32 dma_mode:2; / DMA mode,
    pub /: *mut *mut u32 se_req:1; / To SE core,

    pub /: *mut *mut u32 se_req:1; / To SE core,
    pub /: *mut *mut u32 dma_mode:2; / DMA mode,
    pub /: *mut *mut u32 grp:3; / Group bits,
    pub reserved0:26: u32,

    pub s: },
}

//
// CPT_INST_S software command definitions
// Words EI (0-3)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union otx_cpt_iq_cmd_word0 {
    pub u64: u64,
    pub opcode: __be16,
    pub param1: __be16,
    pub param2: __be16,
    pub dlen: __be16,
    pub s: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union otx_cpt_iq_cmd_word3 {
    pub u64: u64,

    pub grp:3: u64,
    pub cptr:61: u64,

    pub cptr:61: u64,
    pub grp:3: u64,

    pub s: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx_cpt_iq_cmd {
    pub cmd: otx_cpt_iq_cmd_word0,
    pub dptr: u64,
    pub rptr: u64,
    pub cptr: otx_cpt_iq_cmd_word3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx_cpt_sglist_component {
    pub len: u64,
    pub len0: __be16,
    pub len1: __be16,
    pub len2: __be16,
    pub len3: __be16,
    pub s: },
    pub u: },
    pub ptr0: __be64,
    pub ptr1: __be64,
    pub ptr2: __be64,
    pub ptr3: __be64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx_cpt_pending_entry {
    pub /: *mut *mut *mut u64 completion_addr; / Completion address,
    pub info: *mut otx_cpt_info_buffer,
// Kernel async request callback
    pub arg2): *mut *mut *mut void (callback)(int status, void arg1, void,
    pub /: *mut *mut *mut crypto_async_request areq; / Async request callback arg,
    pub /: *mut *mut u8 resume_sender; / Notify sender to resume sending requests,
    pub /: *mut *mut u8 busy; / Entry status (free/busy),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx_cpt_pending_queue {
    pub /: *mut *mut *mut otx_cpt_pending_entry head; / Head of the queue,
    pub /: *mut *mut u32 front; / Process work from here,
    pub /: *mut *mut u32 rear; / Append new work here,
    pub /: *mut *mut u32 pending_count; / Pending requests count,
    pub /: *mut *mut u32 qlen; / Queue length,
    pub /: *mut *mut spinlock_t lock; / Queue lock,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx_cpt_req_info {
// Kernel async request callback
    pub arg2): *mut *mut *mut void (callback)(int status, void arg1, void,
    pub /: *mut *mut *mut crypto_async_request areq; / Async request callback arg,
    pub /: *mut *mut otx_cptvf_request req;/ Request information (core specific),
    pub /: *mut *mut otx_cpt_ctrl_info ctrl;/ User control information,
    pub in: [otx_cpt_buf_ptr; OTX_CPT_MAX_SG_IN_CNT],
    pub out: [otx_cpt_buf_ptr; OTX_CPT_MAX_SG_OUT_CNT],
    pub /: *mut *mut *mut u8 iv_out; / IV to send back,
    pub /: *mut *mut u16 rlen; / Output length,
    pub /: *mut *mut u8 incnt; / Number of input buffers,
    pub /: *mut *mut u8 outcnt; / Number of output buffers,
    pub /: *mut *mut u8 req_type; / Type of request,
    pub /: *mut *mut u8 is_enc; / Is a request an encryption request,
    pub /: *mut *mut u8 is_trunc_hmac;/ Is truncated hmac used,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx_cpt_info_buffer {
    pub pentry: *mut otx_cpt_pending_entry,
    pub req: *mut otx_cpt_req_info,
    pub pdev: *mut pci_dev,
    pub completion_addr: *mut u64,
    pub out_buffer: *mut u8,
    pub in_buffer: *mut u8,
    pub dptr_baddr: dma_addr_t,
    pub rptr_baddr: dma_addr_t,
    pub comp_baddr: dma_addr_t,
    pub time_in: c_ulong,
    pub dlen: u32,
    pub dma_len: u32,
    pub extra_time: u8,
}

extern "C" {
    pub fn otx_cpt_dump_sg_list(pdev: *mut pci_dev, req: *mut otx_cpt_req_info);
}
extern "C" {
    pub fn otx_cpt_post_process(wqe: *mut otx_cptvf_wqe);
}
