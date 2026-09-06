//! Automatically rewritten from C Header to Rust Module
//! Source: include/rdma/tid_rdma_defs.h
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


// SPDX-License-Identifier: (GPL-2.0 OR BSD-3-Clause)
//
// Copyright(c) 2018 Intel Corporation.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tid_rdma_read_req {
    pub kdeth0: __le32,
    pub kdeth1: __le32,
    pub reth: ib_reth,
    pub tid_flow_psn: __be32,
    pub tid_flow_qp: __be32,
    pub verbs_qp: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tid_rdma_read_resp {
    pub kdeth0: __le32,
    pub kdeth1: __le32,
    pub aeth: __be32,
    pub reserved: [__be32; 4],
    pub verbs_psn: __be32,
    pub verbs_qp: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tid_rdma_write_req {
    pub kdeth0: __le32,
    pub kdeth1: __le32,
    pub reth: ib_reth,
    pub reserved: [__be32; 2],
    pub verbs_qp: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tid_rdma_write_resp {
    pub kdeth0: __le32,
    pub kdeth1: __le32,
    pub aeth: __be32,
    pub reserved: [__be32; 3],
    pub tid_flow_psn: __be32,
    pub tid_flow_qp: __be32,
    pub verbs_qp: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tid_rdma_write_data {
    pub kdeth0: __le32,
    pub kdeth1: __le32,
    pub reserved: [__be32; 6],
    pub verbs_qp: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tid_rdma_resync {
    pub kdeth0: __le32,
    pub kdeth1: __le32,
    pub reserved: [__be32; 6],
    pub verbs_qp: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tid_rdma_ack {
    pub kdeth0: __le32,
    pub kdeth1: __le32,
    pub aeth: __be32,
    pub reserved: [__be32; 2],
    pub tid_flow_psn: __be32,
    pub verbs_psn: __be32,
    pub tid_flow_qp: __be32,
    pub verbs_qp: __be32,
}

//
// TID RDMA Opcodes
//
pub const IB_OPCODE_TID_RDMA: c_uint = 0xe0;

//
// Define TID RDMA specific WR opcodes. The ib_wr_opcode
// enum already provides some reserved values for use by
// low level drivers. Two of those are used but renamed
// to be more descriptive.
//

