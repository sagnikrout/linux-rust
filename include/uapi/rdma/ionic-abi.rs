//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/rdma/ionic-abi.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
// Copyright (C) 2018-2025, Advanced Micro Devices, Inc

pub const IONIC_ABI_VERSION: c_int = 1;
pub const IONIC_EXPDB_64: c_int = 1;
pub const IONIC_EXPDB_128: c_int = 2;
pub const IONIC_EXPDB_256: c_int = 4;
pub const IONIC_EXPDB_512: c_int = 8;
pub const IONIC_EXPDB_SQ: c_int = 1;
pub const IONIC_EXPDB_RQ: c_int = 2;
pub const IONIC_CMB_ENABLE: c_int = 1;
pub const IONIC_CMB_REQUIRE: c_int = 2;
pub const IONIC_CMB_EXPDB: c_int = 4;
pub const IONIC_CMB_WC: c_int = 8;
pub const IONIC_CMB_UC: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_ctx_req {
    pub rsvd: [__u32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_ctx_resp {
    pub rsvd: __u32,
    pub page_shift: __u32,
    pub dbell_offset: __aligned_u64,
    pub version: __u16,
    pub qp_opcodes: __u8,
    pub admin_opcodes: __u8,
    pub sq_qtype: __u8,
    pub rq_qtype: __u8,
    pub cq_qtype: __u8,
    pub admin_qtype: __u8,
    pub max_stride: __u8,
    pub max_spec: __u8,
    pub udma_count: __u8,
    pub expdb_mask: __u8,
    pub expdb_qtypes: __u8,
    pub rsvd2: [__u8; 3],
    pub phc_offset: __aligned_u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_qdesc {
    pub addr: __aligned_u64,
    pub size: __u32,
    pub mask: __u16,
    pub depth_log2: __u8,
    pub stride_log2: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_ah_resp {
    pub ahid: __u32,
    pub pad: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_cq_req {
    pub cq: [ionic_qdesc; 2],
    pub udma_mask: __u8,
    pub rsvd: [__u8; 7],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_cq_resp {
    pub cqid: [__u32; 2],
    pub udma_mask: __u8,
    pub rsvd: [__u8; 7],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_qp_req {
    pub sq: ionic_qdesc,
    pub rq: ionic_qdesc,
    pub sq_spec: __u8,
    pub rq_spec: __u8,
    pub sq_cmb: __u8,
    pub rq_cmb: __u8,
    pub udma_mask: __u8,
    pub rsvd: [__u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_qp_resp {
    pub qpid: __u32,
    pub sq_cmb: __u8,
    pub rq_cmb: __u8,
    pub udma_idx: __u8,
    pub rsvd: [__u8; 1],
    pub sq_cmb_offset: __aligned_u64,
    pub rq_cmb_offset: __aligned_u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_srq_req {
    pub rq: ionic_qdesc,
    pub rq_spec: __u8,
    pub rq_cmb: __u8,
    pub udma_mask: __u8,
    pub rsvd: [__u8; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_srq_resp {
    pub qpid: __u32,
    pub rq_cmb: __u8,
    pub udma_idx: __u8,
    pub rsvd: [__u8; 2],
    pub rq_cmb_offset: __aligned_u64,
}
