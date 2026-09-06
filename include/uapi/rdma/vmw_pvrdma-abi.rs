//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/rdma/vmw_pvrdma-abi.h
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


// SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-2-Clause)
//
// Copyright (c) 2012-2016 VMware, Inc.  All rights reserved.
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of EITHER the GNU General Public License
// version 2 as published by the Free Software Foundation or the BSD
// 2-Clause License. This program is distributed in the hope that it
// will be useful, but WITHOUT ANY WARRANTY; WITHOUT EVEN THE IMPLIED
// WARRANTY OF MERCHANTABILITY OR FITNESS FOR A PARTICULAR PURPOSE.
// See the GNU General Public License version 2 for more details at
// http://www.gnu.org/licenses/old-licenses/gpl-2.0.en.html.
//
// You should have received a copy of the GNU General Public License
// along with this program available in the file COPYING in the main
// directory of this source tree.
//
// The BSD 2-Clause License
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
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
// FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE
// COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT,
// INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
// (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
// HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
// STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
// ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED
// OF THE POSSIBILITY OF SUCH DAMAGE.
//

pub const PVRDMA_UAR_HANDLE_MASK: c_uint = 0x00FFFFFF	/* Bottom 24 bits. */;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pvrdma_wr_opcode {
    PVRDMA_WR_RDMA_WRITE,
    PVRDMA_WR_RDMA_WRITE_WITH_IMM,
    PVRDMA_WR_SEND,
    PVRDMA_WR_SEND_WITH_IMM,
    PVRDMA_WR_RDMA_READ,
    PVRDMA_WR_ATOMIC_CMP_AND_SWP,
    PVRDMA_WR_ATOMIC_FETCH_AND_ADD,
    PVRDMA_WR_LSO,
    PVRDMA_WR_SEND_WITH_INV,
    PVRDMA_WR_RDMA_READ_WITH_INV,
    PVRDMA_WR_LOCAL_INV,
    PVRDMA_WR_FAST_REG_MR,
    PVRDMA_WR_MASKED_ATOMIC_CMP_AND_SWP,
    PVRDMA_WR_MASKED_ATOMIC_FETCH_AND_ADD,
    PVRDMA_WR_BIND_MW,
    PVRDMA_WR_REG_SIG_MR,
    PVRDMA_WR_ERROR,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pvrdma_wc_status {
    PVRDMA_WC_SUCCESS,
    PVRDMA_WC_LOC_LEN_ERR,
    PVRDMA_WC_LOC_QP_OP_ERR,
    PVRDMA_WC_LOC_EEC_OP_ERR,
    PVRDMA_WC_LOC_PROT_ERR,
    PVRDMA_WC_WR_FLUSH_ERR,
    PVRDMA_WC_MW_BIND_ERR,
    PVRDMA_WC_BAD_RESP_ERR,
    PVRDMA_WC_LOC_ACCESS_ERR,
    PVRDMA_WC_REM_INV_REQ_ERR,
    PVRDMA_WC_REM_ACCESS_ERR,
    PVRDMA_WC_REM_OP_ERR,
    PVRDMA_WC_RETRY_EXC_ERR,
    PVRDMA_WC_RNR_RETRY_EXC_ERR,
    PVRDMA_WC_LOC_RDD_VIOL_ERR,
    PVRDMA_WC_REM_INV_RD_REQ_ERR,
    PVRDMA_WC_REM_ABORT_ERR,
    PVRDMA_WC_INV_EECN_ERR,
    PVRDMA_WC_INV_EEC_STATE_ERR,
    PVRDMA_WC_FATAL_ERR,
    PVRDMA_WC_RESP_TIMEOUT_ERR,
    PVRDMA_WC_GENERAL_ERR,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pvrdma_wc_opcode {
    PVRDMA_WC_SEND,
    PVRDMA_WC_RDMA_WRITE,
    PVRDMA_WC_RDMA_READ,
    PVRDMA_WC_COMP_SWAP,
    PVRDMA_WC_FETCH_ADD,
    PVRDMA_WC_BIND_MW,
    PVRDMA_WC_LSO,
    PVRDMA_WC_LOCAL_INV,
    PVRDMA_WC_FAST_REG_MR,
    PVRDMA_WC_MASKED_COMP_SWAP,
    PVRDMA_WC_MASKED_FETCH_ADD,
    PVRDMA_WC_RECV = 1 << 7,
    PVRDMA_WC_RECV_RDMA_WITH_IMM,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pvrdma_wc_flags {
    PVRDMA_WC_GRH			= 1 << 0,
    PVRDMA_WC_WITH_IMM		= 1 << 1,
    PVRDMA_WC_WITH_INVALIDATE	= 1 << 2,
    PVRDMA_WC_IP_CSUM_OK		= 1 << 3,
    PVRDMA_WC_WITH_SMAC		= 1 << 4,
    PVRDMA_WC_WITH_VLAN		= 1 << 5,
    PVRDMA_WC_WITH_NETWORK_HDR_TYPE	= 1 << 6,
    PVRDMA_WC_FLAGS_MAX		= PVRDMA_WC_WITH_NETWORK_HDR_TYPE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pvrdma_network_type {
    PVRDMA_NETWORK_IB,
    PVRDMA_NETWORK_ROCE_V1 = PVRDMA_NETWORK_IB,
    PVRDMA_NETWORK_IPV4,
    PVRDMA_NETWORK_IPV6
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_alloc_ucontext_resp {
    pub qp_tab_size: __u32,
    pub reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_alloc_pd_resp {
    pub pdn: __u32,
    pub reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_create_cq {
    pub buf_addr: __aligned_u64,
    pub buf_size: __u32,
    pub reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_create_cq_resp {
    pub cqn: __u32,
    pub reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_resize_cq {
    pub buf_addr: __aligned_u64,
    pub buf_size: __u32,
    pub reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_create_srq {
    pub buf_addr: __aligned_u64,
    pub buf_size: __u32,
    pub reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_create_srq_resp {
    pub srqn: __u32,
    pub reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_create_qp {
    pub rbuf_addr: __aligned_u64,
    pub sbuf_addr: __aligned_u64,
    pub rbuf_size: __u32,
    pub sbuf_size: __u32,
    pub qp_addr: __aligned_u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_create_qp_resp {
    pub qpn: __u32,
    pub qp_handle: __u32,
}

// PVRDMA masked atomic compare and swap
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_ex_cmp_swap {
    pub swap_val: __aligned_u64,
    pub compare_val: __aligned_u64,
    pub swap_mask: __aligned_u64,
    pub compare_mask: __aligned_u64,
}

// PVRDMA masked atomic fetch and add
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_ex_fetch_add {
    pub add_val: __aligned_u64,
    pub field_boundary: __aligned_u64,
}

// PVRDMA address vector.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_av {
    pub port_pd: __u32,
    pub sl_tclass_flowlabel: __u32,
    pub dgid: [__u8; 16],
    pub src_path_bits: __u8,
    pub gid_index: __u8,
    pub stat_rate: __u8,
    pub hop_limit: __u8,
    pub dmac: [__u8; 6],
    pub reserved: [__u8; 6],
}

// PVRDMA scatter/gather entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_sge {
    pub addr: __aligned_u64,
    pub length: __u32,
    pub lkey: __u32,
}

// PVRDMA receive queue work request
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_rq_wqe_hdr {
    pub /: *mut *mut __aligned_u64 wr_id; / wr id,
    pub /: *mut *mut __u32 num_sge; / size of s/g array,
    pub /: *mut *mut __u32 total_len; / reserved,
}

// Use pvrdma_sge (ib_sge) for receive queue s/g array elements.
// PVRDMA send queue work request
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_sq_wqe_hdr {
    pub /: *mut *mut __aligned_u64 wr_id; / wr id,
    pub /: *mut *mut __u32 num_sge; / size of s/g array,
    pub /: *mut *mut __u32 total_len; / reserved,
    pub /: *mut *mut __u32 opcode; / operation type,
    pub /: *mut *mut __u32 send_flags; / wr flags,
    pub imm_data: __be32,
    pub invalidate_rkey: __u32,
    pub ex: },
    pub reserved: __u32,
    pub remote_addr: __aligned_u64,
    pub rkey: __u32,
    pub reserved: [__u8; 4],
    pub rdma: },
    pub remote_addr: __aligned_u64,
    pub compare_add: __aligned_u64,
    pub swap: __aligned_u64,
    pub rkey: __u32,
    pub reserved: __u32,
    pub atomic: },
    pub remote_addr: __aligned_u64,
    pub log_arg_sz: __u32,
    pub rkey: __u32,
    pub cmp_swap: pvrdma_ex_cmp_swap,
    pub fetch_add: pvrdma_ex_fetch_add,
    pub wr_data: },
    pub masked_atomics: },
    pub iova_start: __aligned_u64,
    pub pl_pdir_dma: __aligned_u64,
    pub page_shift: __u32,
    pub page_list_len: __u32,
    pub length: __u32,
    pub access_flags: __u32,
    pub rkey: __u32,
    pub reserved: __u32,
    pub fast_reg: },
    pub remote_qpn: __u32,
    pub remote_qkey: __u32,
    pub av: pvrdma_av,
    pub ud: },
    pub wr: },
}

// Use pvrdma_sge (ib_sge) for send queue s/g array elements.
// Completion queue element.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_cqe {
    pub wr_id: __aligned_u64,
    pub qp: __aligned_u64,
    pub opcode: __u32,
    pub status: __u32,
    pub byte_len: __u32,
    pub imm_data: __be32,
    pub src_qp: __u32,
    pub wc_flags: __u32,
    pub vendor_err: __u32,
    pub pkey_index: __u16,
    pub slid: __u16,
    pub sl: __u8,
    pub dlid_path_bits: __u8,
    pub port_num: __u8,
    pub smac: [__u8; 6],
    pub network_hdr_type: __u8,
    pub /: *mut *mut __u8 reserved2[6]; / Pad to next power of 2 (64).,
}
