//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/cavium/cpt/request_manager.h
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
//
// Copyright (C) 2016 Cavium, Inc.
//

pub const TIME_IN_RESET_COUNT: c_int = 5;
pub const COMPLETION_CODE_SIZE: c_int = 8;
pub const COMPLETION_CODE_INIT: c_int = 0;
pub const PENDING_THOLD: c_int = 100;
pub const MAX_SG_IN_CNT: c_int = 12;
pub const MAX_SG_OUT_CNT: c_int = 13;
pub const SG_LIST_HDR_SIZE: c_int = 8;
pub const MAX_BUF_CNT: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub union ctrl_info {
    pub flags: u32,

    pub reserved0:26: u32,
    pub /: *mut *mut u32 grp:3; / Group bits,
    pub /: *mut *mut u32 dma_mode:2; / DMA mode,
    pub /: *mut *mut u32 se_req:1;/ To SE core,

    pub /: *mut *mut u32 se_req:1; / To SE core,
    pub /: *mut *mut u32 dma_mode:2; / DMA mode,
    pub /: *mut *mut u32 grp:3; / Group bits,
    pub reserved0:26: u32,

    pub s: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union opcode_info {
    pub flags: u16,
    pub major: u8,
    pub minor: u8,
    pub s: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cptvf_request {
    pub opcode: opcode_info,
    pub param1: u16,
    pub param2: u16,
    pub dlen: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct buf_ptr {
    pub vptr: *mut u8,
    pub dma_addr: dma_addr_t,
    pub size: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpt_request_info {
    pub /: *mut *mut u8 incnt; / Number of input buffers,
    pub /: *mut *mut u8 outcnt; / Number of output buffers,
    pub /: *mut *mut u16 rlen; / Output length,
    pub /: *mut *mut ctrl_info ctrl; / User control information,
    pub /: *mut *mut cptvf_request req; / Request Information (Core specific),
    pub may_sleep: bool,
    pub in: [buf_ptr; MAX_BUF_CNT],
    pub out: [buf_ptr; MAX_BUF_CNT],
    pub /: *mut *mut *mut *mut void (callback)(int, void ); / Kernel ASYNC request callabck,
    pub /: *mut *mut *mut void callback_arg; / Kernel ASYNC request callabck arg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sglist_component {
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
pub struct cpt_info_buffer {
    pub cptvf: *mut cpt_vf,
    pub time_in: c_ulong,
    pub extra_time: u8,
    pub req: *mut cpt_request_info,
    pub dptr_baddr: dma_addr_t,
    pub dlen: u32,
    pub rptr_baddr: dma_addr_t,
    pub comp_baddr: dma_addr_t,
    pub in_buffer: *mut u8,
    pub out_buffer: *mut u8,
    pub gather_components: *mut u8,
    pub scatter_components: *mut u8,
    pub pentry: *mut pending_entry,
    pub completion_addr: *mut volatile u64,
    pub alternate_caddr: *mut volatile u64,
}

//
// CPT_INST_S software command definitions
// Words EI (0-3)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union vq_cmd_word0 {
    pub u64: u64,
    pub opcode: __be16,
    pub param1: __be16,
    pub param2: __be16,
    pub dlen: __be16,
    pub s: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union vq_cmd_word3 {
    pub u64: u64,

    pub grp:3: u64,
    pub cptr:61: u64,

    pub cptr:61: u64,
    pub grp:3: u64,

    pub s: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpt_vq_command {
    pub cmd: vq_cmd_word0,
    pub dptr: u64,
    pub rptr: u64,
    pub cptr: vq_cmd_word3,
}

extern "C" {
    pub fn vq_post_process(cptvf: *mut cpt_vf, qno: u32);
}
extern "C" {
    pub fn process_request(cptvf: *mut cpt_vf, req: *mut cpt_request_info) -> c_int;
}
