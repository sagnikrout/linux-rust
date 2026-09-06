//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/rdma/rvt-abi.h
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
// This file contains defines, structures, etc. that are used
// to communicate between kernel and user code.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rvt_wqe_sge {
    pub addr: __aligned_u64,
    pub length: __u32,
    pub lkey: __u32,
}

//
// This structure is used to contain the head pointer, tail pointer,
// and completion queue entries as a single memory allocation so
// it can be mmap'ed into user space.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rvt_cq_wc {
// index of next entry to fill
    pub head): RDMA_ATOMIC_UAPI(__u32,,
// index of next ib_poll_cq() entry
    pub tail): RDMA_ATOMIC_UAPI(__u32,,
// these are actually size ibcq.cqe + 1
    pub uqueue: [ib_uverbs_wc; ],
}

//
// Receive work request queue entry.
// The size of the sg_list is determined when the QP (or SRQ) is created
// and stored in qp->r_rq.max_sge (or srq->rq.max_sge).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rvt_rwqe {
    pub wr_id: __u64,
    pub num_sge: __u8,
    pub padding: [__u8; 7],
    pub sg_list: [rvt_wqe_sge; ],
}

//
// This structure is used to contain the head pointer, tail pointer,
// and receive work queue entries as a single memory allocation so
// it can be mmap'ed into user space.
// Note that the wq array elements are variable size so you can't
// just index into the array to get the N'th element;
// use get_rwqe_ptr() for user space and rvt_get_rwqe_ptr()
// for kernel space.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rvt_rwq {
// new work requests posted to the head
    pub head): RDMA_ATOMIC_UAPI(__u32,,
// receives pull requests from here.
    pub tail): RDMA_ATOMIC_UAPI(__u32,,
    pub wq: [rvt_rwqe; ],
}
