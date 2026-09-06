//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/ulp/srp/ib_srp.h
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
// Copyright (c) 2005 Cisco Systems.  All rights reserved.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// OpenIB.org BSD license below:
//
// Redistribution and use in source and binary forms, with or
// without modification, are permitted provided that the following
// conditions are met:
//
// - Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer.
//
// - Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following
// disclaimer in the documentation and/or other materials
// provided with the distribution.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

//
// Choose the immediate data offset such that a 32 byte CDB still fits.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum srp_target_state {
    SRP_TARGET_SCANNING,
    SRP_TARGET_LIVE,
    SRP_TARGET_REMOVED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum srp_iu_type {
    SRP_IU_CMD,
    SRP_IU_TSK_MGMT,
    SRP_IU_RSP,
}

//
// RDMA adapter in the initiator system.
//
// @dev_list: List of RDMA ports associated with this RDMA adapter (srp_host).
// @mr_page_mask: HCA memory registration page mask.
// @mr_page_size: HCA memory registration page size.
// @mr_max_size: Maximum size in bytes of a single FR registration request.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct srp_device {
    pub dev_list: list_head,
    pub dev: *mut ib_device,
    pub pd: *mut ib_pd,
    pub global_rkey: u32,
    pub mr_page_mask: u64,
    pub mr_page_size: c_int,
    pub mr_max_size: c_int,
    pub max_pages_per_mr: c_int,
    pub has_fr: bool,
    pub use_fast_reg: bool,
}

//
// One port of an RDMA adapter in the initiator system.
//
// @target_list: List of connected target ports (struct srp_target_port).
// @target_lock: Protects @target_list.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct srp_host {
    pub srp_dev: *mut srp_device,
    pub port: u32,
    pub dev: device,
    pub target_list: list_head,
    pub target_lock: spinlock_t,
    pub list: list_head,
    pub add_target_mutex: mutex,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct srp_request {
    pub scmnd: *mut scsi_cmnd,
    pub cmd: *mut srp_iu,
    pub fr_list: *mut srp_fr_desc,
    pub indirect_desc: *mut srp_direct_buf,
    pub indirect_dma_addr: dma_addr_t,
    pub nmdesc: c_short,
    pub reg_cqe: ib_cqe,
}

//
// struct srp_rdma_ch
// @comp_vector: Completion vector used by this RDMA channel.
// @max_it_iu_len: Maximum initiator-to-target information unit length.
// @max_ti_iu_len: Maximum target-to-initiator information unit length.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct srp_rdma_ch {
// These are RW in the hot path, and commonly used together
    pub free_tx: list_head,
    pub lock: spinlock_t,
    pub req_lim: i32,
// These are read-only in the hot path
    pub ____cacheline_aligned_in_smp: *mut *mut srp_target_port target,
    pub send_cq: *mut ib_cq,
    pub recv_cq: *mut ib_cq,
    pub qp: *mut ib_qp,
    pub fr_pool: *mut srp_fr_pool,
    pub max_it_iu_len: u32,
    pub max_ti_iu_len: u32,
    pub max_imm_sge: u8,
    pub use_imm_data: bool,
// Everything above this point is used in the hot path of
// command processing. Try to keep them packed into cachelines.
//
    pub done: completion,
    pub status: c_int,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_cm {
    pub path: sa_path_rec,
    pub path_query: *mut ib_sa_query,
    pub path_query_id: c_int,
    pub cm_id: *mut ib_cm_id,
    pub ib_cm: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_cm {
    pub cm_id: *mut rdma_cm_id,
    pub rdma_cm: },
}

//
// struct srp_target_port - RDMA port in the SRP target system
// @comp_vector: Completion vector used by the first RDMA channel created for
// this target port.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct srp_target_port {
// read and written in the hot path
    pub lock: spinlock_t,
// read only in the hot path
    pub global_rkey: u32,
    pub ch: *mut srp_rdma_ch,
    pub net: *mut net,
    pub ch_count: u32,
    pub lkey: u32,
    pub state: srp_target_state,
    pub max_it_iu_size: u32,
    pub cmd_sg_cnt: c_uint,
    pub indirect_size: c_uint,
    pub allow_ext_sg: bool,
// other member variables
    pub sgid: ib_gid,
    pub id_ext: __be64,
    pub ioc_guid: __be64,
    pub initiator_ext: __be64,
    pub io_class: u16,
    pub srp_host: *mut srp_host,
    pub scsi_host: *mut Scsi_Host,
    pub rport: *mut srp_rport,
    pub target_name: [c_char; 32],
    pub scsi_id: c_uint,
    pub sg_tablesize: c_uint,
    pub target_can_queue: c_uint,
    pub mr_pool_size: c_int,
    pub mr_per_cmd: c_int,
    pub queue_size: c_int,
    pub comp_vector: c_int,
    pub tl_retry_count: c_int,
    pub using_rdma_cm: bool,
    pub service_id: __be64,
    pub orig_dgid: ib_gid,
    pub pkey: __be16,
    pub ib_cm: },
    pub ip4: sockaddr_in,
    pub ip6: sockaddr_in6,
    pub sa: sockaddr,
    pub ss: sockaddr_storage,
    pub src: },
    pub ip4: sockaddr_in,
    pub ip6: sockaddr_in6,
    pub sa: sockaddr,
    pub ss: sockaddr_storage,
    pub dst: },
    pub src_specified: bool,
    pub rdma_cm: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct srp_iu {
    pub list: list_head,
    pub dma: u64,
    pub buf: *mut c_void,
    pub size: usize,
    pub direction: dma_data_direction,
    pub num_sge: u32,
    pub sge: [ib_sge; SRP_MAX_SGE],
    pub cqe: ib_cqe,
}

//
// struct srp_fr_desc - fast registration work request arguments
// @entry: Entry in srp_fr_pool.free_list.
// @mr:    Memory region.
// @frpl:  Fast registration page list.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct srp_fr_desc {
    pub entry: list_head,
    pub mr: *mut ib_mr,
}

//
// struct srp_fr_pool - pool of fast registration descriptors
//
// An entry is available for allocation if and only if it occurs in @free_list.
//
// @size:      Number of descriptors in this pool.
// @max_page_list_len: Maximum fast registration work request page list length.
// @lock:      Protects free_list.
// @free_list: List of free descriptors.
// @desc:      Fast registration descriptor pool.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct srp_fr_pool {
    pub size: c_int,
    pub max_page_list_len: c_int,
    pub lock: spinlock_t,
    pub free_list: list_head,
    pub __counted_by(size): srp_fr_desc desc[],
}

//
// struct srp_map_state - per-request DMA memory mapping state
// @desc:	    Pointer to the element of the SRP buffer descriptor array
// that is being filled in.
// @pages:	    Array with DMA addresses of pages being considered for
// memory registration.
// @base_dma_addr:  DMA address of the first page that has not yet been mapped.
// @dma_len:	    Number of bytes that will be registered with the next FR
// memory registration call.
// @total_len:	    Total number of bytes in the sg-list being mapped.
// @npages:	    Number of page addresses in the pages[] array.
// @nmdesc:	    Number of FR memory descriptors used for mapping.
// @ndesc:	    Number of SRP buffer descriptors that have been filled in.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct srp_map_state {
    pub next: *mut srp_fr_desc,
    pub end: *mut srp_fr_desc,
    pub fr: },
    pub next: *mut c_void,
    pub end: *mut c_void,
    pub gen: },
}
