//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/nvme-rdma.h
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
//
// Copyright (c) 2015 Mellanox Technologies. All rights reserved.
//
pub const NVME_RDMA_IP_PORT: c_int = 4420;
pub const NVME_RDMA_MAX_QUEUE_SIZE: c_int = 256;
pub const NVME_RDMA_MAX_METADATA_QUEUE_SIZE: c_int = 128;
pub const NVME_RDMA_DEFAULT_QUEUE_SIZE: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvme_rdma_cm_fmt {
    NVME_RDMA_CM_FMT_1_0 = 0x0,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvme_rdma_cm_status {
    NVME_RDMA_CM_INVALID_LEN	= 0x01,
    NVME_RDMA_CM_INVALID_RECFMT	= 0x02,
    NVME_RDMA_CM_INVALID_QID	= 0x03,
    NVME_RDMA_CM_INVALID_HSQSIZE	= 0x04,
    NVME_RDMA_CM_INVALID_HRQSIZE	= 0x05,
    NVME_RDMA_CM_NO_RSC		= 0x06,
    NVME_RDMA_CM_INVALID_IRD	= 0x07,
    NVME_RDMA_CM_INVALID_ORD	= 0x08,
    NVME_RDMA_CM_INVALID_CNTLID	= 0x09,
}

//
// struct nvme_rdma_cm_req - rdma connect request
//
// @recfmt:        format of the RDMA Private Data
// @qid:           queue Identifier for the Admin or I/O Queue
// @hrqsize:       host receive queue size to be created
// @hsqsize:       host send queue size to be created
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_rdma_cm_req {
    pub recfmt: __le16,
    pub qid: __le16,
    pub hrqsize: __le16,
    pub hsqsize: __le16,
    pub cntlid: __le16,
    pub rsvd: [u8; 22],
}

//
// struct nvme_rdma_cm_rep - rdma connect reply
//
// @recfmt:        format of the RDMA Private Data
// @crqsize:       controller receive queue size
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_rdma_cm_rep {
    pub recfmt: __le16,
    pub crqsize: __le16,
    pub rsvd: [u8; 28],
}

//
// struct nvme_rdma_cm_rej - rdma connect reject
//
// @recfmt:        format of the RDMA Private Data
// @sts:           error status for the associated connect request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_rdma_cm_rej {
    pub recfmt: __le16,
    pub sts: __le16,
}
