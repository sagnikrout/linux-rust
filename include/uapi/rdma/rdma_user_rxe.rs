//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/rdma/rdma_user_rxe.h
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


// SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR Linux-OpenIB)
//
// Copyright (c) 2016 Mellanox Technologies Ltd. All rights reserved.
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

#[repr(C)]
#[derive(Copy, Clone)]
pub union rxe_gid {
    pub raw: [__u8; 16],
    pub subnet_prefix: __be64,
    pub interface_id: __be64,
    pub global: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxe_global_route {
    pub dgid: rxe_gid,
    pub flow_label: __u32,
    pub sgid_index: __u8,
    pub hop_limit: __u8,
    pub traffic_class: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxe_av {
    pub port_num: __u8,
// From RXE_NETWORK_TYPE_*
    pub network_type: __u8,
    pub dmac: [__u8; 6],
    pub grh: rxe_global_route,
    pub _sockaddr_in: sockaddr_in,
    pub _sockaddr_in6: sockaddr_in6,
    pub dgid_addr: } sgid_addr,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxe_send_wr {
    pub wr_id: __aligned_u64,
    pub reserved: __u32,
    pub opcode: __u32,
    pub send_flags: __u32,
    pub imm_data: __be32,
    pub invalidate_rkey: __u32,
    pub ex: },
    pub remote_addr: __aligned_u64,
    pub length: __u32,
    pub rkey: __u32,
    pub type: __u8,
    pub level: __u8,
    pub flush: },
    pub remote_addr: __aligned_u64,
    pub rkey: __u32,
    pub reserved: __u32,
    pub rdma: },
    pub remote_addr: __aligned_u64,
    pub compare_add: __aligned_u64,
    pub swap: __aligned_u64,
    pub rkey: __u32,
    pub reserved: __u32,
    pub atomic: },
    pub remote_qpn: __u32,
    pub remote_qkey: __u32,
    pub pkey_index: __u16,
    pub reserved: __u16,
    pub ah_num: __u32,
    pub pad: [__u32; 4],
    pub av: rxe_av,
    pub ud: },
    pub addr: __aligned_u64,
    pub length: __aligned_u64,
    pub mr_lkey: __u32,
    pub mw_rkey: __u32,
    pub rkey: __u32,
    pub access: __u32,
    pub mw: },
// reg is only used by the kernel and is not part of the uapi

    pub mr: *mut ib_mr,
    pub reserved: __aligned_u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxe_sge {
    pub addr: __aligned_u64,
    pub length: __u32,
    pub lkey: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mminfo {
    pub offset: __aligned_u64,
    pub size: __u32,
    pub pad: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxe_dma_info {
    pub length: __u32,
    pub resid: __u32,
    pub cur_sge: __u32,
    pub num_sge: __u32,
    pub sge_offset: __u32,
    pub reserved: __u32,
    pub inline_data): __DECLARE_FLEX_ARRAY(__u8,,
    pub atomic_wr): __DECLARE_FLEX_ARRAY(__u8,,
    pub sge): __DECLARE_FLEX_ARRAY(struct rxe_sge,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxe_send_wqe {
    pub wr: rxe_send_wr,
    pub status: __u32,
    pub state: __u32,
    pub iova: __aligned_u64,
    pub mask: __u32,
    pub first_psn: __u32,
    pub last_psn: __u32,
    pub ack_length: __u32,
    pub ssn: __u32,
    pub has_rd_atomic: __u32,
    pub dma: rxe_dma_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxe_recv_wqe {
    pub wr_id: __aligned_u64,
    pub reserved: __u32,
    pub padding: __u32,
    pub dma: rxe_dma_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxe_create_ah_resp {
    pub ah_num: __u32,
    pub reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxe_create_cq_resp {
    pub mi: mminfo,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxe_resize_cq_resp {
    pub mi: mminfo,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxe_create_qp_resp {
    pub rq_mi: mminfo,
    pub sq_mi: mminfo,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxe_create_srq_resp {
    pub mi: mminfo,
    pub srq_num: __u32,
    pub reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxe_modify_srq_cmd {
    pub mmap_info_addr: __aligned_u64,
}

// This data structure is stored at the base of work and
// completion queues shared between user space and kernel space.
// It contains the producer and consumer indices. Is also
// contains a copy of the queue size parameters for user space
// to use but the kernel must use the parameters in the
// rxe_queue struct. For performance reasons arrange to have
// producer and consumer indices in separate cache lines
// the kernel should always mask the indices to avoid accessing
// memory outside of the data area
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxe_queue_buf {
    pub log2_elem_size: __u32,
    pub index_mask: __u32,
    pub pad_1: [__u32; 30],
    pub producer_index: __u32,
    pub pad_2: [__u32; 31],
    pub consumer_index: __u32,
    pub pad_3: [__u32; 31],
    pub data: [__u8; ],
}
