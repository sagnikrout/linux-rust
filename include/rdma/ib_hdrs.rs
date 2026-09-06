//! Automatically rewritten from C Header to Rust Module
//! Source: include/rdma/ib_hdrs.h
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
// Copyright(c) 2016 - 2018 Intel Corporation.
//

// AETH NAK opcode values
pub const IB_RNR_NAK: c_uint = 0x20;
pub const IB_NAK_PSN_ERROR: c_uint = 0x60;
pub const IB_NAK_INVALID_REQUEST: c_uint = 0x61;
pub const IB_NAK_REMOTE_ACCESS_ERROR: c_uint = 0x62;
pub const IB_NAK_REMOTE_OPERATIONAL_ERROR: c_uint = 0x63;
pub const IB_NAK_INVALID_RD_REQUEST: c_uint = 0x64;

pub const IB_GRH_VERSION: c_int = 6;
pub const IB_GRH_VERSION_MASK: c_uint = 0xF;
pub const IB_GRH_VERSION_SHIFT: c_int = 28;
pub const IB_GRH_TCLASS_MASK: c_uint = 0xFF;
pub const IB_GRH_TCLASS_SHIFT: c_int = 20;
pub const IB_GRH_FLOW_MASK: c_uint = 0xFFFFF;
pub const IB_GRH_FLOW_SHIFT: c_int = 0;
pub const IB_GRH_NEXT_HDR: c_uint = 0x1B;
pub const IB_FECN_SHIFT: c_int = 31;
pub const IB_FECN_MASK: c_int = 1;

pub const IB_BECN_SHIFT: c_int = 30;
pub const IB_BECN_MASK: c_int = 1;

pub const IB_AETH_CREDIT_SHIFT: c_int = 24;
pub const IB_AETH_CREDIT_MASK: c_uint = 0x1F;
pub const IB_AETH_CREDIT_INVAL: c_uint = 0x1F;
pub const IB_AETH_NAK_SHIFT: c_int = 29;
pub const IB_MSN_MASK: c_uint = 0xFFFFFF;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_reth {
    pub /: *mut *mut __be64 vaddr; / potentially unaligned,
    pub rkey: __be32,
    pub length: __be32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_atomic_eth {
    pub /: *mut *mut __be64 vaddr; / potentially unaligned,
    pub rkey: __be32,
    pub /: *mut *mut __be64 swap_data; / potentially unaligned,
    pub /: *mut *mut __be64 compare_data; / potentially unaligned,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub union ib_ehdrs {
    pub deth: [__be32; 2],
    pub imm_data: __be32,
    pub ud: },
    pub reth: ib_reth,
    pub imm_data: __be32,
    pub rc: },
    pub aeth: __be32,
    pub /: *mut *mut __be64 atomic_ack_eth; / potentially unaligned,
    pub at: } __packed,
    pub imm_data: __be32,
    pub aeth: __be32,
    pub ieth: __be32,
    pub atomic_eth: ib_atomic_eth,
// TID RDMA headers
    pub r_req: tid_rdma_read_req,
    pub r_rsp: tid_rdma_read_resp,
    pub w_req: tid_rdma_write_req,
    pub w_rsp: tid_rdma_write_resp,
    pub w_data: tid_rdma_write_data,
    pub resync: tid_rdma_resync,
    pub ack: tid_rdma_ack,
    pub tid_rdma: },
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_other_headers {
    pub bth: [__be32; 3],
    pub u: ib_ehdrs,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_header {
    pub lrh: [__be16; 4],
    pub grh: ib_grh,
    pub oth: ib_other_headers,
    pub l: },
    pub oth: ib_other_headers,
    pub u: },
    pub __packed: },
// accessors for unaligned __be64 items
    pub get_unaligned_be64(p): return,
    pub p): put_unaligned_be64(val,,
    pub ib_u64_get(&reth->vaddr): return,
    pub &reth->vaddr): ib_u64_put(val,,
    pub ib_u64_get(&ateth->vaddr): return,
    pub &ateth->vaddr): ib_u64_put(val,,
    pub ib_u64_get(&ateth->swap_data): return,
    pub &ateth->swap_data): ib_u64_put(val,,
    pub ib_u64_get(&ateth->compare_data): return,
    pub &ateth->compare_data): ib_u64_put(val,,
//
// 9B/IB Packet Format
//
pub const IB_LNH_MASK: c_int = 3;
pub const IB_SC_MASK: c_uint = 0xf;
pub const IB_SC_SHIFT: c_int = 12;
pub const IB_SC5_MASK: c_uint = 0x10;
pub const IB_SL_MASK: c_uint = 0xf;
pub const IB_SL_SHIFT: c_int = 4;
pub const IB_SL_SHIFT: c_int = 4;
pub const IB_LVER_MASK: c_uint = 0xf;
pub const IB_LVER_SHIFT: c_int = 8;
    pub IB_LNH_MASK): return (be16_to_cpu(hdr->lrh[0]) &,
    pub IB_SC_MASK): return ((be16_to_cpu(hdr->lrh[0]) >> IB_SC_SHIFT) &,
    pub IB_SC5_MASK): return !!(sc5 &,
    pub IB_SL_MASK): return ((be16_to_cpu(hdr->lrh[0]) >> IB_SL_SHIFT) &,
    pub (be16_to_cpu(hdr->lrh[1])): return,
    pub (be16_to_cpu(hdr->lrh[3])): return,
    pub be32_to_cpu(ohdr->u.ud.deth[0]): return,
    pub IB_QPN_MASK): return ((be32_to_cpu(ohdr->u.ud.deth[1])) &,
//
// BTH
//
pub const IB_BTH_OPCODE_MASK: c_uint = 0xff;
pub const IB_BTH_OPCODE_SHIFT: c_int = 24;
pub const IB_BTH_PAD_MASK: c_int = 3;
pub const IB_BTH_PKEY_MASK: c_uint = 0xffff;
pub const IB_BTH_PAD_SHIFT: c_int = 20;
pub const IB_BTH_A_MASK: c_int = 1;
pub const IB_BTH_A_SHIFT: c_int = 31;
pub const IB_BTH_M_MASK: c_int = 1;
pub const IB_BTH_M_SHIFT: c_int = 22;
pub const IB_BTH_SE_MASK: c_int = 1;
pub const IB_BTH_SE_SHIFT: c_int = 23;
pub const IB_BTH_TVER_MASK: c_uint = 0xf;
pub const IB_BTH_TVER_SHIFT: c_int = 16;
pub const IB_BTH_OPCODE_CNP: c_uint = 0x81;
    pub IB_BTH_PKEY_MASK): return (be32_to_cpu(ohdr->bth[0]) &,
    pub (u32)(be32_to_cpu(ohdr->bth[2])): return,
    pub IB_QPN_MASK): return (u32)((be32_to_cpu(ohdr->bth[1])) &,
    pub cpu_to_be32(IB_BECN_SMASK): return (ohdr->bth[1]) &,
    pub cpu_to_be32(IB_FECN_SMASK): return (ohdr->bth[1]) &,
    pub cpu_to_be32(IB_BTH_SOLICITED): return ohdr->bth[0] &,
    pub cpu_to_be32(IB_BTH_MIG_REQ): return ohdr->bth[0] &,
