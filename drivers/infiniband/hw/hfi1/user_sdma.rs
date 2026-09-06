//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/hfi1/user_sdma.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Copyright(c) 2023 - Cornelis Networks, Inc.
// Copyright(c) 2015 - 2018 Intel Corporation.
//

// The maximum number of Data io vectors per message/request
pub const MAX_VECTORS_PER_REQ: c_int = 8;
//
// Maximum number of packet to send from each message/request
// before moving to the next one.
//
pub const MAX_PKTS_PER_QUEUE: c_int = 16;

// Number of BTH.PSN bits used for sequence number in expected rcvs
pub const BTH_SEQ_MASK: c_uint = 0x7ffull;
pub const AHG_KDETH_INTR_SHIFT: c_int = 12;
pub const AHG_KDETH_SH_SHIFT: c_int = 13;
pub const AHG_KDETH_ARRAY_SIZE: c_int = 9;

//
// Build an SDMA AHG header update descriptor and save it to an array.
// @arr        - Array to save the descriptor to.
// @idx        - Index of the array at which the descriptor will be saved.
// @array_size - Size of the array arr.
// @dw         - Update index into the header in DWs.
// @bit        - Start bit.
// @width      - Field width.
// @value      - 16 bits of immediate data to write into the field.
// Returns -ERANGE if idx is invalid. If successful, returns the next index
// (idx + 1) of the array to be used for the next descriptor.
//
// Tx request flag bits

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pkt_q_sdma_state {
    SDMA_PKT_Q_ACTIVE,
    SDMA_PKT_Q_DEFERRED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi1_user_sdma_pkt_q {
    pub ctxt: u16,
    pub subctxt: u16,
    pub n_max_reqs: u16,
    pub n_reqs: core::sync::atomic::AtomicI32,
    pub reqidx: u16,
    pub dd: *mut hfi1_devdata,
    pub txreq_cache: *mut kmem_cache,
    pub reqs: *mut user_sdma_request,
    pub req_in_use: *mut c_ulong,
    pub busy: iowait,
    pub state: pkt_q_sdma_state,
    pub wait: wait_queue_head_t,
    pub unpinned: c_ulong,
    pub handler: *mut mmu_rb_handler,
    pub n_locked: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi1_user_sdma_comp_q {
    pub nentries: u16,
    pub comps: *mut hfi1_sdma_comp_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_sdma_iovec {
    pub list: list_head,
    pub iov: iovec,
//
// offset into the virtual address space of the vector at
// which we last left off.
//
    pub offset: u64,
}

// evict operation argument
#[repr(C)]
#[derive(Copy, Clone)]
pub struct evict_data {
    pub /: *mut *mut u32 cleared; / count evicted so far,
    pub /: *mut *mut u32 target; / target count to evict,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_sdma_request {
// This is the original header from user space
    pub hdr: hfi1_pkt_header,
// Read mostly fields
    pub ____cacheline_aligned_in_smp: *mut *mut hfi1_user_sdma_pkt_q pq,
    pub cq: *mut hfi1_user_sdma_comp_q,
//
// Pointer to the SDMA engine for this request.
// Since different request could be on different VLs,
// each request will need it's own engine pointer.
//
    pub sde: *mut sdma_engine,
    pub info: sdma_req_info,
// TID array values copied from the tid_iov vector
    pub tids: *mut u32,
// total length of the data in the request
    pub data_len: u32,
// number of elements copied to the tids array
    pub n_tids: u16,
//
// We copy the iovs for this request (based on
// info.iovcnt). These are only the data vectors
//
    pub data_iovs: u8,
    pub ahg_idx: i8,
// Writeable fields shared with interrupt
    pub ____cacheline_aligned_in_smp: u16 seqcomp,
    pub seqsubmitted: u16,
// Send side fields
    pub ____cacheline_aligned_in_smp: list_head txps,
    pub seqnum: u16,
//
// KDETH.OFFSET (TID) field
// The offset can cover multiple packets, depending on the
// size of the TID entry.
//
    pub tidoffset: u32,
//
// KDETH.Offset (Eager) field
// We need to remember the initial value so the headers
// can be updated properly.
//
    pub koffset: u32,
    pub sent: u32,
// TID index copied from the tid_iov vector
    pub tididx: u16,
// progress index moving along the iovs array
    pub iov_idx: u8,
    pub has_error: u8,
    pub iovs: [user_sdma_iovec; MAX_VECTORS_PER_REQ],
    pub ____cacheline_aligned_in_smp: },
//
// A single txreq could span up to 3 physical pages when the MTU
// is sufficiently large (> 4K). Each of the IOV pointers also
// needs it's own set of flags so the vector has been handled
// independently of each other.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_sdma_txreq {
// Packet header for the txreq
    pub hdr: hfi1_pkt_header,
    pub txreq: sdma_txreq,
    pub list: list_head,
    pub req: *mut user_sdma_request,
    pub flags: u16,
    pub seqnum: u16,
}
