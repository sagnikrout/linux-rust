//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/rdma/erdma-abi.h
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


// SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause)
//
// Copyright (c) 2020-2022, Alibaba Group.
//

pub const ERDMA_ABI_VERSION: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_ureq_create_cq {
    pub db_record_va: __aligned_u64,
    pub qbuf_va: __aligned_u64,
    pub qbuf_len: __u32,
    pub rsvd0: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_uresp_create_cq {
    pub cq_id: __u32,
    pub num_cqe: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_ureq_create_qp {
    pub db_record_va: __aligned_u64,
    pub qbuf_va: __aligned_u64,
    pub qbuf_len: __u32,
    pub rsvd0: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_uresp_create_qp {
    pub qp_id: __u32,
    pub num_sqe: __u32,
    pub num_rqe: __u32,
    pub rq_offset: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_uresp_alloc_ctx {
    pub dev_id: __u32,
    pub pad: __u32,
    pub sdb_type: __u32,
    pub sdb_offset: __u32,
    pub sdb: __aligned_u64,
    pub rdb: __aligned_u64,
    pub cdb: __aligned_u64,
}
